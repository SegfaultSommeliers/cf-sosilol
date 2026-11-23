// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    app::{
        AppError,
        AppState,
    },
    auth,
    pages,
    paste::service,
};
use axum::{
    Json,
    extract::{
        Path,
        State,
    },
    http::StatusCode,
    response::{
        IntoResponse,
        Redirect,
        Response,
    },
};
use axum_extra::extract::PrivateCookieJar;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("invalid paste id")]
    InvalidId,
    #[error("paste not found")]
    NotFound,
    #[error("paste service")]
    Service(#[source] service::Error),
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        match error {
            Error::InvalidId => Self::bad_request(error.to_string()),
            Error::NotFound => Self::not_found(error.to_string()),
            Error::Service(service::Error::InvalidText) => {
                Self::bad_request(service::Error::InvalidText.to_string())
            }
            error => Self::internal(error),
        }
    }
}

#[derive(Deserialize)]
pub struct SaveRequest {
    text: String,
}

#[worker::send]
pub async fn save(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(request): Json<SaveRequest>,
) -> Result<Response, AppError> {
    let author: Option<i64> = auth::session(&jar).and_then(|session| session.user_id.parse().ok());
    let id = state
        .paste_service(&state.paste_storage())
        .save(&request.text, author)
        .await
        .map_err(Error::Service)?;
    Ok((StatusCode::CREATED, Json(serde_json::json!({"id": id}))).into_response())
}

#[worker::send]
pub async fn view(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    if !service::valid_id(&id) {
        return Ok(Redirect::to("/").into_response());
    }
    match state
        .paste_service(&state.paste_storage())
        .load(&id)
        .await
        .map_err(Error::Service)?
    {
        Some(text) => Ok(pages::paste(&text).into_response()),
        None => Ok(Redirect::to("/").into_response()),
    }
}

#[worker::send]
pub async fn raw(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    if !service::valid_id(&id) {
        return Err(Error::InvalidId.into());
    }
    let text = state
        .paste_service(&state.paste_storage())
        .load(&id)
        .await
        .map_err(Error::Service)?
        .ok_or(Error::NotFound)?;
    Ok(text.into_response())
}
