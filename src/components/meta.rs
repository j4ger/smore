use dioxus::prelude::*;

use crate::components::button::BUTTON_STYLE_COMMON;
use crate::components::loading::LOADING_STYLE_COMMON;
use crate::components::page::PAGE_STYLE_COMMON;

const RESET: Asset = asset!("assets/reset.css");

pub fn GeistMeta() -> Element {
    rsx! {
        document::Style {
            href: RESET
        }
        document::Style {
            "
            {BUTTON_STYLE_COMMON}
            {LOADING_STYLE_COMMON}
            {PAGE_STYLE_COMMON}
            "
        }
    }
}
