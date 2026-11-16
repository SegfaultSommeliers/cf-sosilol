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
