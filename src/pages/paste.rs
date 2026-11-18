// SPDX-FileCopyrightText: 2026 Konstantin Konovalov
// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT

use maud::{
    Markup,
    html,
};

use crate::layouts::bare_layout;

pub fn paste(code: &str) -> Markup {
    bare_layout(html! {
        div class="default-section code-view" {
            (code)
        }
    })
}
