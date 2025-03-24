use dioxus::prelude::*;

use crate::components::button::BUTTON_STYLE_COMMON;
use crate::components::loading::LOADING_STYLE_COMMON;

pub fn GeistMeta() -> Element {
    rsx! {
        document::Style {
            "
            {BUTTON_STYLE_COMMON}
            {LOADING_STYLE_COMMON}
            "
        }
    }
}
