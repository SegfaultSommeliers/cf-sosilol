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

    #[error("listing profile pastes")]
    ListingPastes(#[source] worker::Error),
}

pub struct Storage<'a> {
    db: &'a D1Database,
}

#[derive(Deserialize)]
pub struct Profile {
    pub profile_id: String,
    pub github_id: String,
}

#[derive(Deserialize)]
struct PasteId {
    paste_id: String,
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

    pub async fn ensure_profile(
        &self,
        github_id: i64,
    ) -> Result<(), Error> {
        query!(
            self.db,
            "INSERT INTO profiles (github_id) VALUES (?1) ON CONFLICT(github_id) DO NOTHING",
            github_id
        )
        .map_err(Error::PreparingQuery)?
        .run()
        .await
        .map_err(Error::CreatingProfile)?;
        Ok(())
    }

    pub async fn profile_pastes(
        &self,
        github_id: i64,
    ) -> Result<Vec<String>, Error> {
        let result = query!(
            self.db,
            "SELECT p.paste_id FROM pastes p JOIN profiles u ON u.profile_id = p.author_id WHERE u.github_id = ?1 ORDER BY p.paste_id DESC LIMIT 100",
            github_id
        )
        .map_err(Error::PreparingQuery)?
        .all()
        .await
        .map_err(Error::ListingPastes)?;
        let pastes: Vec<PasteId> = result.results().map_err(Error::ListingPastes)?;
        Ok(pastes.into_iter().map(|paste| paste.paste_id).collect())
    }
}
