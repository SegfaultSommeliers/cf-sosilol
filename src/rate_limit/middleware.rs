// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    app::AppError,
    rate_limit::service::{
        self,
        Service,
    },
};
use axum::{
    extract::{
        Request,
        State,
    },
    middleware::Next,
    response::Response,
};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("rate limit exceeded")]
    RateLimited,

    #[error("rate-limit service")]
    Service(#[source] service::Error),
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        match error {
            Error::RateLimited => Self::too_many_requests("Rate limit exceeded"),
            error @ Error::Service(_) => Self::internal(error),
        }
    }
}

pub async fn rate_limit(
    State(service): State<Service>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let client_ip = request
        .headers()
        .get("cf-connecting-ip")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown");

    let allowed = service
        .check_request(client_ip, request.method().as_str(), request.uri().path())
        .await
        .map_err(Error::Service)?;

    if !allowed {
        return Err(Error::RateLimited.into());
    }

    Ok(next.run(request).await)
}
