use dioxus::prelude::*;

use crate::components::button::BUTTON_STYLE_COMMON;

pub fn GeistMeta() -> Element {
    rsx! {
        document::Style {
            "{BUTTON_STYLE_COMMON}"
        }
    }
}
