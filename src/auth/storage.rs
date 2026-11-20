// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::Deserialize;
use thiserror::Error;
use worker::{
    D1Database,
    query,
};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] worker::Error),
}

pub struct Storage<'a> {
    db: &'a D1Database,
}

#[derive(Deserialize)]
pub struct Profile {
    pub profile_id: String,
    pub github_id: String,
}

impl Storage<'_> {
    pub fn new(db: &D1Database) -> Storage {
        Storage { db }
    }

    pub async fn create_profile(
        &self,
        github_id: &str,
    ) -> Result<(), StorageError> {
        query!(
            self.db,
            "INSERT INTO profiles (github_id) VALUES (?1)",
            github_id
        )?
        .run()
        .await?;

        Ok(())
    }

    pub async fn find_profile_by_github_id(
        &self,
        github_id: &str,
    ) -> Result<Option<Profile>, StorageError> {
        let profile: Option<Profile> = query!(
            self.db,
            "SELECT profile_id, github_id, created_at
             FROM profiles
             WHERE github_id = ?1",
            github_id
        )?
        .first(None)
        .await?;
        Ok(profile)
    }
}
