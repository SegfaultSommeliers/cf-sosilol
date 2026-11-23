// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::paste::storage;
use rand::{
    RngExt,
    distr::Alphanumeric,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("text is required or too large")]
    InvalidText,

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
            if self
                .storage
                .insert(&id, author)
                .await
                .map_err(Error::Storage)?
            {
                if let Err(error) = self.storage.put_object(&id, text).await {
                    if let Err(cleanup_error) = self.storage.delete_object(&id).await {
                        tracing::error!(?cleanup_error, %id, "failed to clean up R2 object");
                    }
                    if let Err(cleanup_error) = self.storage.delete_paste(&id).await {
                        tracing::error!(?cleanup_error, %id, "failed to clean up reserved paste ID");
                    }
                    return Err(Error::Storage(error));
                }
                return Ok(id);
            }
        }
        Err(Error::IdCollision)
    }

    pub async fn load(
        &self,
        id: &str,
    ) -> Result<Option<String>, Error> {
        self.storage.get_object(id).await.map_err(Error::Storage)
    }
}
