// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod middleware;
mod service;

pub use middleware::rate_limit;
pub use service::Service;
