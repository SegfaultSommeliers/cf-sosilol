// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    auth,
    pages,
    state::AppState,
};
use anyhow::Result;
use axum::{
    Router,
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
    routing::get,
};
use axum_extra::extract::cookie::Key;
use base64::{
    Engine,
    engine::general_purpose::STANDARD,
};
use tower_service::Service;
use worker::{
    Env,
    HttpRequest,
};

pub struct App {
    state: AppState,
}

impl App {
    pub fn new(env: Env) -> Result<App> {
        let cookie_key = env.secret("COOKIE_KEY")?;
        let bytes = STANDARD.decode(cookie_key.to_string())?;
        let cookie_key = Key::from(&bytes);

        Ok(App {
            state: AppState { env, cookie_key },
        })
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
