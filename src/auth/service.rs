// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    app::AppError,
    auth::{
        OAuthSession,
        Pkce,
        Provider,
        Storage,
        github::GithubUser,
        oauth,
        pkce,
        storage,
    },
};
use thiserror::Error;
use worker::Date;

#[derive(Error, Debug)]
pub enum Error {
    #[error("generating pkce")]
    GeneratingPkce(#[source] pkce::Error),

    #[error("generating state code")]
    GeneratingNonce(#[source] pkce::Error),

    #[error("exchanging OAuth code")]
    ExchangingCode(#[source] oauth::Error),

    #[error("fetching GitHub user")]
    FetchingUser(#[source] reqwest::Error),

    #[error("accessing profile storage")]
    Storage(#[source] storage::Error),
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        match value {
            Error::GeneratingPkce(err) => Self::internal(err),
            Error::GeneratingNonce(err) => Self::internal(err),
            Error::ExchangingCode(err) => Self::internal(err),
            Error::FetchingUser(err) => Self::internal(err),
            Error::Storage(err) => Self::internal(err),
        }
    }
}

pub struct Service<'a> {
    storage: &'a Storage<'a>,
}

impl<'a> Service<'a> {
    pub fn new(storage: &'a Storage<'a>) -> Self {
        Self { storage }
    }

    pub fn generate_auth_cookie(&self) -> Result<OAuthSession, Error> {
        let pkce = Pkce::generate().map_err(Error::GeneratingPkce)?;
        let nonce = pkce::generate_nonce().map_err(Error::GeneratingNonce)?;

        Ok(OAuthSession {
            code_verifier: pkce.verifier,
            state: nonce,
            redirect: "/profile".to_owned(),
            exp: Date::now().as_millis() / 1000 + 600,
        })
    }

    pub async fn authorize(
        &self,
        provider: &Provider,
        verifier: &str,
        code: &str,
    ) -> Result<(String, GithubUser), Error> {
        let token = provider
            .get_access_token(verifier, code)
            .await
            .map_err(Error::ExchangingCode)?;
        let user = provider
            .get_github_user(&token)
            .await
            .map_err(Error::FetchingUser)?;
        self.storage
            .ensure_profile(user.id)
            .await
            .map_err(Error::Storage)?;
        Ok((token, user))
    }

    pub async fn profile_pastes(
        &self,
        github_id: i64,
    ) -> Result<Vec<String>, Error> {
        self.storage
            .profile_pastes(github_id)
            .await
            .map_err(Error::Storage)
    }
}
