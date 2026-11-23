// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    auth::{
        generate_nonce,
        pkce,
    },
    paste::storage,
};
use rand::{
    RngExt,
    distr::Alphanumeric,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("text is required or too large")]
    InvalidText,

    #[error("generating paste id")]
    GeneratingId(#[source] getrandom::Error),

    #[error("generating storage key")]
    GeneratingKey(#[source] pkce::Error),

    #[error("accessing paste storage")]
    Storage(#[source] storage::Error),

    #[error("failed to generate unique paste id")]
    IdCollision,
}

pub struct Service<'a> {
    storage: &'a storage::Storage<'a>,
    max_paste_size_bytes: usize,
}

pub fn valid_id(id: &str) -> bool {
    id.len() == 14 && id.bytes().all(|b| b.is_ascii_alphanumeric())
}

fn generate_id() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(14)
        .map(char::from)
        .collect()
}

impl<'a> Service<'a> {
    pub fn new(
        storage: &'a storage::Storage<'a>,
        max_paste_size_bytes: usize,
    ) -> Self {
        Self {
            storage,
            max_paste_size_bytes,
        }
    }

    pub async fn save(
        &self,
        text: &str,
        author: Option<i64>,
    ) -> Result<String, Error> {
        if text.trim().is_empty() || text.len() > self.max_paste_size_bytes {
            return Err(Error::InvalidText);
        }
        for _ in 0..5 {
            let id = generate_id();
            let key = format!("pastes/{}", generate_nonce().map_err(Error::GeneratingKey)?);
            self.storage
                .put_object(&key, text)
                .await
                .map_err(Error::Storage)?;
            if self
                .storage
                .insert(&id, &key, author)
                .await
                .map_err(Error::Storage)?
            {
                return Ok(id);
            }
            if let Err(error) = self.storage.delete_object(&key).await {
                tracing::error!(?error, key, "failed to clean up R2 object");
            }
        }
        Err(Error::IdCollision)
    }

    pub async fn load(
        &self,
        id: &str,
    ) -> Result<Option<String>, Error> {
        let Some(key) = self.storage.find_key(id).await.map_err(Error::Storage)? else {
            return Ok(None);
        };
        Ok(Some(
            self.storage
                .get_object(&key)
                .await
                .map_err(Error::Storage)?,
        ))
    }
}
