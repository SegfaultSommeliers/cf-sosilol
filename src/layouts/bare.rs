// SPDX-FileCopyrightText: 2025 Konstantin Konovalov
// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT

use maud::{
    DOCTYPE,
    Markup,
    html,
};

pub fn bare_layout(children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content="width=device-width, initial-scale=1.0";

                link rel="shortcut icon" href="/favicon.ico" type="image/x-icon";
                link rel="stylesheet" href="/assets/css/main.css";

                script defer src="/assets/js/main.js" {}

                title { "соси лол" }
            }

            body {
                (children)
            }
        }
    }
}
