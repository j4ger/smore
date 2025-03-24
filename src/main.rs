use components::{init_ui_context, Button, ButtonTypes, GeistMeta, Loading, Page, Scale};
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
        Page {
            dot_backdrop: true,
            Button {
                "Default"
            }
            Button {
                loading: true,
                "Loading"
            }
            Button {
                disabled: true,
                "Disabled"
            }
            Button {
                ghost: true,
                "Ghost"
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
            Loading {

            }
        }
    }
}

// TODOs:
// button ripple
// theme switching (restart as no reactivity is added?)
// geist meta tree shake
// scale css variable inheritance (omit if not needed)
// this can be done by assigning a default value for the variable in the COMMON_STYLE part and overwrite if needed
