// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::auth::oauth::Provider;
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use std::sync::Arc;
use worker::Env;

#[derive(Clone)]
pub struct AppState {
    pub env: Env,
    pub cookie_key: Key,
    pub auth_provider: Arc<Provider>,
}

impl FromRef<AppState> for Key {
    fn from_ref(input: &AppState) -> Self {
        input.cookie_key.clone()
    }
}
