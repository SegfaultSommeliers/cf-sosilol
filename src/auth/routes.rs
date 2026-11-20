// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    app::state::AppState,
    auth::session::Session,
};
use axum::{
    Json,
    extract::{
        Request,
        State,
    },
    response::{
        IntoResponse,
        Redirect,
        Response,
    },
};
use axum_extra::extract::PrivateCookieJar;

pub async fn profile(
    State(_state): State<AppState>,
    jar: PrivateCookieJar,
    _req: Request,
) -> Result<Response, Response> {
    let redirect = || Redirect::to("").into_response();
    let cookie = jar.get("session").ok_or_else(redirect)?;
    let _session: Session = serde_json::from_str(cookie.value()).map_err(|_| redirect())?;

    Ok(Json("123").into_response())
}
