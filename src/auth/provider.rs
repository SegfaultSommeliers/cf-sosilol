// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use reqwest::Url;

pub struct ExtractAuth {
    pub code: String,
    pub state: String,
}

pub trait Provider {
    fn get_auth_url(
        &self,
        code_challenge: &str,
        state: &str,
        redirect_uri: &str,
        scope: &[&str],
    ) -> Url;

    fn extract_code(
        &self,
        redirect_url: &str,
    ) -> ExtractAuth;

    async fn get_access_token(
        &self,
        code: &str,
    ) -> String;
}
