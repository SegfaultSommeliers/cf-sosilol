// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod app;
mod auth;
mod layouts;
#[allow(dead_code, unused_imports)]
mod models;
mod pages;
mod paste;

use crate::app::App;
use axum::response::Response;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_web::MakeWebConsoleWriter;
use worker::{
    Context,
    Env,
    HttpRequest,
    event,
};

#[event(start)]
pub fn start() {
    fmt()
        .json()
        .with_writer(MakeWebConsoleWriter::new())
        .with_max_level(Level::INFO)
        .with_current_span(false)
        .with_span_list(true)
        .without_time()
        .init();
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> worker::Result<Response> {
    Ok(App::new(env).call(req).await)
}
