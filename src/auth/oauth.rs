// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use reqwest::{
    Client,
    StatusCode,
    Url,
};
use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("serializing auth url query")]
    SerializingQuery(#[source] serde_urlencoded::ser::Error),

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("provider internal error: HTTP {0}")]
    Internal(StatusCode),

    #[error("invalid provider response")]
    InvalidResponse(#[source] reqwest::Error),
}

pub struct Provider {
    authorize_url: Url,
    access_token_url: Url,
    client_id: String,
    client_secret: String,
}

#[derive(Serialize)]
struct AuthorizeRequest<'a> {
    client_id: &'a str,
    redirect_uri: &'a str,
    response_type: &'static str,
    state: &'a str,
    code_challenge: &'a str,
    code_challenge_method: &'static str,
    scope: &'a str,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
    code_verifier: &'a str,
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub _token_type: String,
    pub _scope: String,
}

impl Provider {
    pub fn new(
        authorize_url: &str,
        access_token_url: &str,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            authorize_url: Url::parse(authorize_url).expect("invalid authorize URL"),
            access_token_url: Url::parse(access_token_url).expect("invalid access_token URL"),
            client_id,
            client_secret,
        }
    }

    pub fn get_auth_url(
        &self,
        code_challenge: &str,
        state: &str,
        redirect_uri: &str,
        scope: &[&str],
    ) -> Result<Url, Error> {
        let mut url = self.authorize_url.clone();
        let scope = scope.join(" ");

        let query = serde_urlencoded::to_string(&AuthorizeRequest {
            client_id: &self.client_id,
            redirect_uri,
            response_type: "code",
            state,
            code_challenge,
            code_challenge_method: "S256",
            scope: &scope,
        })
        .map_err(Error::SerializingQuery)?;

        url.set_query(Some(&query));
        Ok(url)
    }

    pub async fn get_access_token(
        &self,
        code_verifier: &str,
        code: &str,
    ) -> Result<String, Error> {
        let request = TokenRequest {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            code,
            code_verifier,
        };

        let response = Client::new()
            .post(self.access_token_url.clone())
            .header("Accept", "application/json")
            .form(&request)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {}

            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                return Err(Error::InvalidCredentials);
            }

            status => {
                return Err(Error::Internal(status));
            }
        }

        let response: TokenResponse = response.json().await.map_err(|err| {
            if err.is_decode() {
                Error::InvalidResponse(err)
            } else {
                Error::Http(err)
            }
        })?;

        Ok(response.access_token)
    }
}
