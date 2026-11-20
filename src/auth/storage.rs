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
pub enum Error {
    #[error("preparing query")]
    PreparingQuery(#[source] worker::Error),

    #[error("creating profile")]
    CreatingProfile(#[source] worker::Error),

    #[error("finding profile")]
    FindingProfile(#[source] worker::Error),
}

pub struct Storage<'a> {
    db: &'a D1Database,
}

#[derive(Deserialize)]
pub struct Profile {
    pub profile_id: String,
    pub github_id: String,
}

impl<'a> Storage<'a> {
    pub fn new(db: &'a D1Database) -> Self {
        Self { db }
    }

    pub async fn create_profile(
        &self,
        github_id: &str,
    ) -> Result<(), Error> {
        query!(
            self.db,
            "INSERT INTO profiles (github_id) VALUES (?1)",
            github_id
        )
        .map_err(Error::PreparingQuery)?
        .run()
        .await
        .map_err(Error::CreatingProfile)?;

        Ok(())
    }

    pub async fn find_profile_by_github_id(
        &self,
        github_id: &str,
    ) -> Result<Option<Profile>, Error> {
        let profile: Option<Profile> = query!(
            self.db,
            "SELECT profile_id, github_id, created_at
             FROM profiles
             WHERE github_id = ?1",
            github_id
        )
        .map_err(Error::PreparingQuery)?
        .first(None)
        .await
        .map_err(Error::FindingProfile)?;
        Ok(profile)
    }
}
