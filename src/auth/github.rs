// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::auth::Provider;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GithubUser {
    pub id: i64,
    pub login: String,
    pub avatar_url: String,
}

impl Provider {
    pub fn github(
        client_id: String,
        client_secret: String,
    ) -> Provider {
        Self::new(
            "https://github.com/login/oauth/authorize",
            "https://github.com/login/oauth/access_token",
            client_id,
            client_secret,
        )
    }

    pub async fn get_github_user(
        &self,
        token: &str,
    ) -> Result<GithubUser, reqwest::Error> {
        Client::new()
            .get("https://api.github.com/user")
            .header("User-Agent", "cf-sosilol")
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
