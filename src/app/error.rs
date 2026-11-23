// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use axum::{
    Json,
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
};
use serde::Serialize;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug)]
pub struct AppError {
    status: StatusCode,
    message: String,
    data: Option<Value>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl AppError {
    fn new(
        status: StatusCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            status,
            message: message.into(),
            data: None,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, message)
    }

    pub fn too_many_requests(message: impl Into<String>) -> Self {
        Self::new(StatusCode::TOO_MANY_REQUESTS, message)
    }

    pub fn internal(error: impl Debug) -> Self {
        tracing::error!(
            target: "app_error",
            error = ?error,
            "Internal server error"
        );

        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    }

    pub fn with_data(
        mut self,
        data: Value,
    ) -> Self {
        self.data = Some(data);
        self
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = ErrorResponse {
            code: self.status.as_u16(),
            message: self.message,
            data: self.data,
        };

        (self.status, Json(body)).into_response()
    }
}
