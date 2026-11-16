use crate::pages;
use anyhow::Result;
use axum::{
    Router,
    body::Body,
    response::Response,
    routing::get,
};
use tower_service::Service;
use worker::HttpRequest;

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    fn router(&self) -> Router {
        Router::new().route("/", get(pages::main()))
    }

    pub async fn call(
        &self,
        req: HttpRequest,
    ) -> Result<Response<Body>> {
        Ok(self.router().call(req).await?)
    }
}
