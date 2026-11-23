// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;
use worker::{
    Bucket,
    D1Database,
    query,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("accessing paste storage")]
    Worker(#[from] worker::Error),

    #[error("R2 object has no body")]
    MissingBody,
}

pub struct Storage<'a> {
    db: &'a D1Database,
    bucket: &'a Bucket,
}

impl<'a> Storage<'a> {
    pub fn new(
        db: &'a D1Database,
        bucket: &'a Bucket,
    ) -> Self {
        Self { db, bucket }
    }

    pub async fn put_object(
        &self,
        key: &str,
        text: &str,
    ) -> Result<(), Error> {
        self.bucket
            .put(key, text.as_bytes().to_vec())
            .execute()
            .await?;
        Ok(())
    }

    pub async fn delete_object(
        &self,
        key: &str,
    ) -> Result<(), Error> {
        self.bucket.delete(key).await?;
        Ok(())
    }

    pub async fn insert(
        &self,
        id: &str,
        author: Option<i64>,
    ) -> Result<bool, Error> {
        let result = query!(self.db, "INSERT OR IGNORE INTO pastes (paste_id, author_id) VALUES (?1, (SELECT profile_id FROM profiles WHERE github_id = ?2))", id, author)?
        .run().await?;
        Ok(result.meta()?.and_then(|meta| meta.changes).unwrap_or(0) > 0)
    }

    pub async fn delete_paste(
        &self,
        id: &str,
    ) -> Result<(), Error> {
        query!(self.db, "DELETE FROM pastes WHERE paste_id = ?1", id)?
            .run()
            .await?;
        Ok(())
    }

    pub async fn get_object(
        &self,
        key: &str,
    ) -> Result<Option<String>, Error> {
        let Some(object) = self.bucket.get(key).execute().await? else {
            return Ok(None);
        };
        let body = object.body().ok_or(Error::MissingBody)?;
        Ok(Some(body.text().await?))
    }
}
