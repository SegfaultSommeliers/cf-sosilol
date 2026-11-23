// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use maud::{
    Markup,
    html,
};

use crate::layouts::base_layout;

pub fn authors() -> Markup {
    base_layout(html! {
        section class="default-section" {
            "Copyright:\n2026 "
            a href="https://github.com/SegfaultSommeliers" { "SegfaultSommeliers" }
            "\n- developed the Go port (May 2026)"
            "\n- developed the Rust + Cloudflare Workers port (November 2026)"
            "\n2026 "
            a href="https://github.com/fiwka" { "fiwka" }
            "\n- developed the original implementation in Kotlin"
        }
    })
}
