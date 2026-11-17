// SPDX-FileCopyrightText: 2025 Konstantin Konovalov
// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT

use maud::{
    Markup,
    html,
};

use crate::{
    layouts::base_layout,
    models::Paste,
};

pub fn profile(
    login: &str,
    avatar_url: &str,
    pastes: &[Paste],
) -> Markup {
    base_layout(html! {
        section class="default-section" id="profile" {
            header class="profile-header" {
                @if !avatar_url.is_empty() {
                    img src=(avatar_url) alt="Avatar" class="avatar";
                } @else {
                    div class="avatar profile-avatar-placeholder" {}
                }

                h3 {
                    "Профиль " (login)
                }
            }

            div class="pastes profile-pastes" {
                @if pastes.is_empty() {
                    h4 class="profile-pastes-empty" {
                        "У вас нет сохранённых паст."
                    }
                } @else {
                    h4 class="profile-pastes-title" {
                        "Ваши пасты:"
                    }

                    div class="collection profile-collection" {
                        @for paste in pastes {
                            a
                                href=(format!("/view/{}", paste.id))
                                class="collection-item grey darken-3"
                            {
                                (&paste.id)
                            }
                        }
                    }
                }
            }
        }
    })
}
