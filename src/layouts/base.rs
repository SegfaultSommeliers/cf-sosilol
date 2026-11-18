// SPDX-FileCopyrightText: 2026 Konstantin Konovalov
// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT

use maud::{
    DOCTYPE,
    Markup,
    html,
};

pub fn base_layout(children: Markup) -> Markup {
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
                nav {
                    div class="nav-wrapper nav-brand-wrapper" {
                        a href="/" class="brand-logo nav-brand-logo" {
                            "соси лол"
                        }

                        a href="#" data-target="mobile" class="sidenav-trigger" {
                            i class="material-icons" { "menu" }
                        }

                        ul id="nav-mobile" class="right hide-on-med-and-down" {
                            li {
                                a href="/v1/swagger-ui/index.html" { "API" }
                            }
                            li {
                                a href="https://vk.com/fiwka" { "Автор" }
                            }
                            li {
                                a href="/profile" { "Профиль" }
                            }
                        }
                    }
                }

                ul class="sidenav" id="mobile" {
                    li {
                        a href="/v1/swagger-ui/index.html" { "API" }
                    }
                    li {
                        a href="https://vk.com/fiwka" { "Автор" }
                    }
                    li {
                        a href="/profile" { "Профиль" }
                    }
                }

                (children)
            }
        }
    }
}
