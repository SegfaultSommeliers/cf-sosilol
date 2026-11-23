// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    app::{
        AppState,
        request_logger,
    },
    auth,
    auth::Provider,
    pages,
    paste,
};
use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware,
    response::Response,
    routing::{
        get,
        post,
    },
};
use axum_extra::extract::cookie::Key;
use base64::{
    Engine,
    prelude::BASE64_STANDARD,
};
use tower_http::request_id::{
    MakeRequestUuid,
    PropagateRequestIdLayer,
    SetRequestIdLayer,
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
    pub fn new(env: Env) -> Self {
        let cookie_key = env.secret("COOKIE_KEY").expect("`COOKIE_KEY` is invalid");
        let bytes = BASE64_STANDARD
            .decode(cookie_key.to_string())
            .expect("`COOKIE_KEY` is invalid");
        let cookie_key = Key::from(&bytes);
        let auth_provider = Provider::github(
            env.secret("GITHUB_CLIENT_ID")
                .expect("`GITHUB_CLIENT_ID` is invalid")
                .to_string(),
            env.secret("GITHUB_CLIENT_SECRET")
                .expect("`GITHUB_CLIENT_SECRET` is invalid")
                .to_string(),
        );
        let redirect_url = env
            .secret("GITHUB_REDIRECT_URL")
            .expect("`GITHUB_REDIRECT_URL` is invalid")
            .to_string();
        let db = env.d1("DB").expect("`DB` is invalid");
        let bucket = env.bucket("BUCKET").expect("`BUCKET` is invalid");

        Self {
            state: AppState::new(env, db, bucket, cookie_key, auth_provider, redirect_url),
        }
    }

    fn router(&self) -> Router {
        Router::new()
            .route("/", get(pages::main()))
            .route("/profile", get(auth::profile))
            .route("/auth/request", get(auth::request))
            .route("/auth/redirect", get(auth::callback))
            .route("/auth/logout", post(auth::logout))
            .route(
                "/save",
                post(paste::save).layer(DefaultBodyLimit::max(6 * 1_048_576 + 1024)),
            )
            .route("/view/{id}", get(paste::view))
            .route("/raw/{id}", get(paste::raw))
            .with_state(self.state.clone())
            .layer(PropagateRequestIdLayer::x_request_id())
            .layer(middleware::from_fn(request_logger))
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
    }

    pub async fn call(
        &self,
        req: HttpRequest,
    ) -> Response {
        self.router().call(req).await.unwrap()
    }
}
