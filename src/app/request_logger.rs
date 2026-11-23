// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::Instrument;

pub async fn request_logger(
    request: Request,
    next: Next,
) -> Response {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_owned();

    let span = tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        path = %request.uri().path(),
    );

    let response = next.run(request).instrument(span.clone()).await;

    span.in_scope(|| tracing::info!(status = response.status().as_u16()));
    response
}
