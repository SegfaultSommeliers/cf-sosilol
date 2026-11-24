// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::sync::Arc;
use thiserror::Error;
use worker::RateLimiter;

#[derive(Debug, Error)]
pub enum Error {
    #[error("checking {scope} rate limiter")]
    CheckFailed {
        scope: &'static str,
        #[source]
        source: worker::Error,
    },
}

#[derive(Clone)]
pub struct Service {
    global: Arc<RateLimiter>,
    save: Arc<RateLimiter>,
    auth_request: Arc<RateLimiter>,
}

impl Service {
    pub fn new(
        global: RateLimiter,
        save: RateLimiter,
        auth_request: RateLimiter,
    ) -> Self {
        Self {
            global: Arc::new(global),
            save: Arc::new(save),
            auth_request: Arc::new(auth_request),
        }
    }

    pub async fn check_request(
        &self,
        client_ip: &str,
        method: &str,
        path: &str,
    ) -> Result<bool, Error> {
        if !self.check(&self.global, client_ip, "global").await? {
            return Ok(false);
        }

        let route_limiter = match (method, path) {
            ("POST", "/save") => Some((&self.save, "save")),
            ("GET", "/auth/request") => Some((&self.auth_request, "auth_request")),
            _ => None,
        };

        if let Some((limiter, scope)) = route_limiter {
            return self.check(limiter, client_ip, scope).await;
        }

        Ok(true)
    }

    async fn check(
        &self,
        limiter: &RateLimiter,
        client_ip: &str,
        scope: &'static str,
    ) -> Result<bool, Error> {
        match limiter.limit(client_ip.to_owned()).await {
            Ok(outcome) => Ok(outcome.success),
            Err(source) => Err(Error::CheckFailed { scope, source }),
        }
    }
}
