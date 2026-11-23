// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::AppState;
use axum::{
    extract::{
        Request,
        State,
    },
    middleware::Next,
    response::{
        IntoResponse,
        Response,
    },
};

pub async fn rate_limit(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let client_ip = request
        .headers()
        .get("cf-connecting-ip")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown")
        .to_owned();

    if !check_limit(&state.global_rate_limiter, &client_ip, "global").await {
        return super::AppError::too_many_requests("Rate limit exceeded").into_response();
    }

    let route_limiter = match (request.method().as_str(), request.uri().path()) {
        ("POST", "/save") => Some((&state.save_rate_limiter, "save")),
        ("GET", "/auth/request") => Some((&state.auth_request_rate_limiter, "auth_request")),
        _ => None,
    };

    if let Some((limiter, scope)) = route_limiter
        && !check_limit(limiter, &client_ip, scope).await
    {
        return super::AppError::too_many_requests("Rate limit exceeded").into_response();
    }

    next.run(request).await
}

async fn check_limit(
    limiter: &worker::RateLimiter,
    client_ip: &str,
    scope: &str,
) -> bool {
    match limiter.limit(client_ip.to_owned()).await {
        Ok(outcome) => outcome.success,
        Err(error) => {
            tracing::error!(
                target: "rate_limit",
                scope,
                error = ?error,
                "Rate-limit check failed; allowing request"
            );
            true
        }
    }
}
