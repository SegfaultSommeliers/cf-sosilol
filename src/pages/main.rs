// SPDX-FileCopyrightText: 2026 Konstantin Konovalov
// SPDX-FileCopyrightText: 2026 SegfaultSommeliers
//
// SPDX-License-Identifier: MIT

use maud::{
    Markup,
    html,
};

use crate::layouts::base_layout;

pub fn main() -> Markup {
    base_layout(html! {
        div
            x-data="editor"
            "@keydown.ctrl.s.window.prevent"="save()"
        {
            button
                class="btn-floating btn-large waves-effect waves-light red save-btn save-fab"
                ":disabled"="saving"
                "@click"="save()"
            {
                i class="fas fa-paste save-fab-icon" {}
            }

            div class="editor" {
                textarea
                    class="editor-textarea"
                    placeholder="Пиши сюда"
                    x-model="text"
                    "@keydown.tab.prevent"="handleTab($event)"
                {}
            }
        }
    })
}
