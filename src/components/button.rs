use dioxus::prelude::*;

use crate::components::{add_alpha_3, theme::Palette, use_scale, use_theme};

pub const BUTTON_STYLE_COMMON: &'static str =
        ".btn {
            box-sizing: border-box;
            display: inline-block;
            line-height: var(--g-line-height);
            border-radius: var(--g-border-radius);
            font-weight: 400;
            font-size: var(--g-font-size);
            user-select: none;
            outline: none;
            text-transform: capitalize;
            justify-content: center;
            text-align: center;
            white-space: nowrap;
            transition: background-color 200ms ease 0ms, box-shadow 200ms ease 0ms, border 200ms ease 0ms, color 200ms ease 0ms;
            position: relative;
            overflow: hidden;
            color: var(--g-color);
            background-color: var(--g-bg);
            border: 1px solid var(--g-border);
            cursor: var(--g-cursor);
            pointer-events: var(--g-event);
            box-shadow: var(--g-box-shadow);
            min-width: var(--g-min-width);
            width: var(--g-width);
            height: var(--g-height);
            padding: var(--g-pt) var(--g-pr) var(--g-pb) var(--g-pl);
            margin: var(--g-mt) var(--g-mr) var(--g-mb) var(--g-ml);
        }

        .btn:hover,
        .btn:focus {
            color: var(--g-hover-color);
            background-color: var(--g-hover-bg);
            border-color: var(--g-hover-border);
            box-shadow: var(--g-hover-box-shadow);
            transform: translate3d(0px, var(--g-hover-transform), 0px);
        }

        .btn :global(.text) {
            position: relative;
            z-index: 1;
            display: inline-flex;
            justify-content: center;
            align-items: center;
            text-align: center;
            line-height: inherit;
            top: -1px;
        }

        .btn :global(.text p),
        .btn :global(.text pre),
        .btn :global(.text div) {
            margin: 0;
        }";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonTypes {
    Default,
    Secondary,
    Success,
    Warning,
    Error,
    Abort,
    SecondaryLight,
    SuccessLight,
    WarningLight,
    ErrorLight,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct ButtonProps {
    #[props(default = ButtonTypes::Default)]
    r#type: ButtonTypes,
    #[props(default = false)]
    ghost: bool,
    #[props(default = false)]
    loading: bool,
    #[props(default = false)]
    shadow: bool,
    #[props(default = false)]
    auto: bool,
    #[props(default = false)]
    disabled: bool,
    #[props(default = EventHandler::default())]
    onclick: EventHandler<MouseEvent>,
    children: Element,
}

pub fn Button(props: ButtonProps) -> Element {
    let theme = use_theme();
    let palette = theme.palette;
    let SCALES = use_scale();

    let line_height = SCALES.height(2.5);
    let border_radius = theme.layout.radius;
    let font_size = SCALES.font(0.875);

    // -light types share the same background color as their dark counterparts
    // they only have different hover colors
    let (bg, border, color) = if props.disabled {
        (palette.accents_1, palette.accents_2, "#ccc")
    } else {
        match props.r#type {
            ButtonTypes::Default => (palette.background, palette.border, palette.accents_5),
            ButtonTypes::Secondary | ButtonTypes::SecondaryLight => {
                (palette.foreground, palette.foreground, palette.background)
            }
            ButtonTypes::Success | ButtonTypes::SuccessLight => {
                (palette.success, palette.success, "#fff")
            }
            ButtonTypes::Warning | ButtonTypes::WarningLight => {
                (palette.warning, palette.warning, "#fff")
            }
            ButtonTypes::Error | ButtonTypes::ErrorLight => (palette.error, palette.error, "#fff"),
            ButtonTypes::Abort => ("transparent", "transparent", palette.accents_5),
        }
    };

    let box_shadow = if props.shadow {
        theme.expressiveness.shadowSmall
    } else {
        "none"
    };

    let hover_box_shadow = if props.shadow {
        theme.expressiveness.shadowMedium
    } else {
        "none"
    };

    let button_icon_padding = SCALES.pl(0.727);
    let button_height = SCALES.height(2.5);

    let min_width = if props.auto {
        "min-content".to_string()
    } else {
        SCALES.width(10.5)
    };
    let width = if props.auto { "auto" } else { "initial" };
    let height = SCALES.height(2.5);
    let pl = if props.auto {
        SCALES.pl(1.15)
    } else {
        SCALES.pl(1.375)
    };
    let pr = if props.auto {
        SCALES.pr(1.15)
    } else {
        SCALES.pr(1.375)
    };
    let pt = SCALES.pt(0.);
    let pb = SCALES.pb(0.);
    let mt = SCALES.mt(0.);
    let mr = SCALES.mr(0.);
    let mb = SCALES.mb(0.);
    let ml = SCALES.ml(0.);

    let (cursor, event) = match (props.disabled, props.loading) {
        (true, _) => ("not-allowed", "auto"),
        (_, true) => ("default", "none"),
        _ => ("pointer", "auto"),
    };

    let alpha_bg = add_alpha_3(bg, 0.85);

    let (hover_bg, hover_border, hover_color) = {
        let colors = match props.r#type {
            ButtonTypes::Default => (palette.background, palette.foreground, palette.foreground),
            ButtonTypes::Secondary => (palette.background, palette.foreground, palette.foreground),
            ButtonTypes::Success => (palette.background, palette.success, palette.success),
            ButtonTypes::Warning => (palette.background, palette.warning, palette.warning),
            ButtonTypes::Error => (palette.background, palette.error, palette.error),
            ButtonTypes::Abort => ("transparent", "transparent", palette.accents_5),
            _ => (alpha_bg.as_str(), border, color),
        };

        if props.disabled {
            (palette.accents_1, palette.accents_2, "#ccc")
        } else if props.loading {
            (colors.0, colors.1, "transparent")
        } else if props.shadow {
            (bg, border, color)
        } else {
            if props.ghost {
                match props.r#type {
                    ButtonTypes::Secondary | ButtonTypes::SecondaryLight => {
                        (palette.foreground, palette.background, palette.background)
                    }
                    ButtonTypes::Success | ButtonTypes::SuccessLight => {
                        (palette.success, palette.background, "white")
                    }
                    ButtonTypes::Warning | ButtonTypes::WarningLight => {
                        (palette.warning, palette.background, "white")
                    }
                    ButtonTypes::Error | ButtonTypes::ErrorLight => {
                        (palette.error, palette.background, "white")
                    }
                    _ => colors,
                }
            } else {
                colors
            }
        }
    };

    let hover_transform = if props.shadow { "-1px" } else { "0px" };

    let style = format!(
        "
        --g-line-height: {line_height};
        --g-border-radius: {border_radius};
        --g-font-size: {font_size};
        --g-color: {color};
        --g-bg: {bg};
        --g-border: {border};
        --g-cursor: {cursor};
        --g-event: {event};
        --g-box-shadow: {box_shadow};
        --geist-ui-button-icon-padding: {button_icon_padding};
        --geist-ui-button-height: {button_height};
        --geist-ui-button-color: {color};
        --geist-ui-button-bg: {bg};
        --g-min-width: {min_width};
        --g-width: {width};
        --g-height: {height};
        --g-pt: {pt};
        --g-pb: {pb};
        --g-pr: {pr};
        --g-pl: {pl};
        --g-mt: {mt};
        --g-mr: {mr};
        --g-mb: {mb};
        --g-ml: {ml};
        --g-hover-color: {hover_color};
        --geist-ui-button-color: {hover_color};
        --g-hover-bg: {hover_bg};
        --g-hover-border: {hover_border};
        box-shadow: var(--g-hover-box-shadow);
        --g-hover-box-shadow: {hover_box_shadow};
        --g-hover-transform: {hover_transform};"
    );

    rsx! {
        button {
            class: "btn",
            style: style,
            disabled: props.disabled,
            onclick: props.onclick,
            {props.children}
        }
    }
}
