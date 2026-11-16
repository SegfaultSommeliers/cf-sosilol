mod app;
mod layouts;
mod models;
mod pages;

use crate::app::App;
use anyhow::Result;
use axum::{
    body::Body,
    response::Response,
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
    _env: Env,
    _ctx: Context,
) -> Result<Response<Body>> {
    let app = App::new();

    Ok(app.call(req).await?)
}
