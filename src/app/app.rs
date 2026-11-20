// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    app::state::AppState,
    auth,
    auth::oauth::Provider,
    pages,
};
use axum::{
    Router,
    response::Response,
    routing::get,
};
use axum_extra::extract::cookie::Key;
use base64::{
    Engine,
    engine::general_purpose::STANDARD,
};
use std::sync::Arc;
use tower_service::Service;
use worker::{
    Env,
    HttpRequest,
};

pub struct App {
    state: AppState,
}

impl App {
    pub fn new(env: Env) -> Self {
        let cookie_key = env.secret("COOKIE_KEY").expect("`COOKIE_KEY` is invalid");
        let bytes = STANDARD
            .decode(cookie_key.to_string())
            .expect("`COOKIE_KEY` is invalid");
        let cookie_key = Key::from(&bytes);
        let auth_provider = Arc::new(Provider::github(
            env.secret("GITHUB_CLIENT_ID")
                .expect("`GITHUB_CLIENT_ID` is invalid")
                .to_string(),
            env.secret("GITHUB_CLIENT_SECRET")
                .expect("`GITHUB_CLIENT_SECRET` is invalid")
                .to_string(),
        ));

        Self {
            state: AppState {
                env,
                cookie_key,
                auth_provider,
            },
        }
    }

    fn router(&self) -> Router {
        Router::new()
            .route("/", get(pages::main()))
            .route("/profile", get(auth::routes::profile))
            .with_state(self.state.clone())
    }

    pub async fn call(
        &self,
        req: HttpRequest,
    ) -> Response {
        self.router().call(req).await.unwrap()
    }
}
