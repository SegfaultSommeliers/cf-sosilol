// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod app;
mod auth;
mod layouts;
mod models;
mod pages;
mod paste;
mod state;

use crate::app::App;
use anyhow::Result;
use axum::{
    handler::Handler,
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
};
use worker::{
    Context,
    Env,
    HttpRequest,
    event,
};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let app = match App::new(env) {
        Ok(app) => app,
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };

    Ok(app.call(req).await)
}
