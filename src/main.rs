use components::{init_ui_context, Button, GeistMeta};
use dioxus::prelude::*;

mod api;
mod components;
mod views;

fn main() {
    launch(app);
}

fn app() -> Element {
    init_ui_context();
    rsx! {
        GeistMeta {}
        Button {
            onclick: |_| println!("1"),
            "hi"
        }
    }
}

// TODOs:
// button ripple
// theme switching (restart as no reactivity is added?)
// geist meta tree shake
