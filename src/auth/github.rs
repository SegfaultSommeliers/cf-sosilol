// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::auth::provider::{
    ExtractAuth,
    Provider,
};
use reqwest::Url;

const GITHUB_API_URL: &str = "https://github.com/login/oauth/authorize";

pub struct Github<'a> {
    client_id: &'a str,
    client_secret: &'a str,
}

impl<'a> Github<'a> {
    pub fn new(
        client_id: &'a str,
        client_secret: &'a str,
    ) -> Self {
        Github {
            client_id,
            client_secret,
        }
    }
}

impl Provider for Github<'_> {
    fn get_auth_url(
        &self,
        code_challenge: &str,
        state: &str,
        redirect_uri: &str,
        scope: &[&str],
    ) -> Url {
        let mut url = Url::parse(GITHUB_API_URL).expect("Invalid Github OAuth URL");
        let scope = scope.join(" ");

        let params = [
            ("client_id", self.client_id),
            ("redirect_uri", redirect_uri),
            ("response_type", "code"),
            ("state", state),
            ("code_challenge", code_challenge),
            ("code_challenge_method", "S256"),
            ("scope", &scope),
        ];

        url.query_pairs_mut().extend_pairs(params);
        url
    }

    fn extract_code(
        &self,
        redirect_url: &str,
    ) -> ExtractAuth {
        todo!()
    }

    async fn get_access_token(
        &self,
        code: &str,
    ) -> String {
        todo!()
    }
}
