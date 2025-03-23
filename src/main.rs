use components::{init_ui_context, Button, ButtonTypes, GeistMeta};
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
            "Default"
        }
        Button {
            type: ButtonTypes::Secondary,
            "Secondary"
        }
        Button {
            type: ButtonTypes::Success,
            "Success"
        }
        Button {
            type: ButtonTypes::Warning,
            "Warning"
        }
        Button {
            type: ButtonTypes::Error,
            "Error"
        }
        Button {
            type: ButtonTypes::Abort,
            "Abort"
        }
        Button {
            type: ButtonTypes::SecondaryLight,
            "SecondaryLight"
        }
        Button {
            type: ButtonTypes::SuccessLight,
            "SuccessLight"
        }
        Button {
            type: ButtonTypes::WarningLight,
            "WarningLight"
        }
        Button {
            type: ButtonTypes::ErrorLight,
            "ErrorLight"
        }
    }
}

// TODOs:
// button ripple
// theme switching (restart as no reactivity is added?)
// geist meta tree shake
