// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use worker::Env;

#[derive(Clone)]
pub struct AppState {
    pub env: Env,
    pub cookie_key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(input: &AppState) -> Self {
        input.cookie_key.clone()
    }
}
