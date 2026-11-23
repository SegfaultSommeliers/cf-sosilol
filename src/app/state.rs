// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    auth,
    paste,
};
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use std::sync::Arc;
use worker::{
    Bucket,
    D1Database,
    Env,
};

#[derive(Clone)]
pub struct AppState {
    pub env: Env,
    pub db: Arc<D1Database>,
    pub bucket: Arc<Bucket>,
    pub cookie_key: Key,
    pub auth_provider: auth::Provider,
    pub redirect_url: String,
}

impl FromRef<AppState> for Key {
    fn from_ref(input: &AppState) -> Self {
        input.cookie_key.clone()
    }
}

impl AppState {
    pub fn new(
        env: Env,
        db: D1Database,
        bucket: Bucket,
        cookie_key: Key,
        auth_provider: auth::Provider,
        redirect_url: String,
    ) -> Self {
        Self {
            env,
            db: Arc::new(db),
            bucket: Arc::new(bucket),
            cookie_key,
            auth_provider,
            redirect_url,
        }
    }

    pub fn auth_storage(&self) -> auth::Storage<'_> {
        auth::Storage::new(&self.db)
    }

    pub fn auth_service<'a>(
        &self,
        storage: &'a auth::Storage<'a>,
    ) -> auth::Service<'a> {
        auth::Service::new(storage)
    }

    pub fn paste_storage(&self) -> paste::Storage<'_> {
        paste::Storage::new(&self.db, &self.bucket)
    }

    pub fn paste_service<'a>(
        &self,
        storage: &'a paste::Storage<'a>,
    ) -> paste::Service<'a> {
        paste::Service::new(storage)
    }
}
