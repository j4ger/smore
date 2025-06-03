use crate::components::{add_alpha_3, use_scale, use_theme, Loading, Scale};
use dioxus::prelude::*;
use std::time::Duration;

pub const BUTTON_STYLE_COMMON: &'static str = "
    .btn {
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
        transition: background-color 200ms ease 0ms,
                    box-shadow 200ms ease 0ms,
                    border 200ms ease 0ms,
                    color 200ms ease 0ms;
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
        padding: var(--g-padding);
        margin: var(--g-margin);
        border-radius: var(--g-border-radius);
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
    }

    .drip {
        position: absolute;
        left: 0;
        right: 0;
        top: 0;
        bottom: 0;
    }

    .drip-svg {
        position: absolute;
        animation: 350ms ease-in expand;
        animation-fill-mode: forwards;
        width: 1rem;
        height: 1rem;
    }

    @keyframes expand {
        0% {
            opacity: 0;
            transform: scale(1);
        }
        30% {
            opacity: 1;
        }
        80% {
            opacity: 0.5;
        }
        100% {
            transform: scale(28);
            opacity: 0;
        }
    }

    .btn-loading {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        z-index: 2;
        background-color: var(--g-bg);
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
    scale: Option<Scale>,
    children: Element,
}

pub fn Button(props: ButtonProps) -> Element {
    let theme = use_theme();
    let palette = theme.palette;
    let scales = props.scale.unwrap_or(use_scale());

    let line_height = scales.height(2.5);
    let border_radius = theme.layout.radius;
    let font_size = scales.font(0.875);

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

    let button_icon_padding = scales.pl(0.727);
    let button_height = scales.height(2.5);

    let min_width = if props.auto {
        "min-content".to_string()
    } else {
        scales.width(10.5)
    };
    let width = if props.auto { "auto" } else { "initial" };
    let height = scales.height(2.5);
    let pl = if props.auto {
        scales.pl(1.15)
    } else {
        scales.pl(1.375)
    };
    let pr = if props.auto {
        scales.pr(1.15)
    } else {
        scales.pr(1.375)
    };
    let pt = scales.pt(0.);
    let pb = scales.pb(0.);
    let mt = scales.mt(0.);
    let mr = scales.mr(0.);
    let mb = scales.mb(0.);
    let ml = scales.ml(0.);

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
        --g-padding: {pt} {pr} {pb} {pl};
        --g-margin: {mt} {mr} {mb} {ml};
        --g-hover-color: {hover_color};
        --g-hover-bg: {hover_bg};
        --g-hover-border: {hover_border};
        box-shadow: var(--g-hover-box-shadow);
        --g-hover-box-shadow: {hover_box_shadow};
        --g-hover-transform: {hover_transform};"
    );

    let mut drip_x = use_signal(|| 0f32);
    let mut drip_y = use_signal(|| 0f32);
    let mut drip_show = use_signal(|| false);
    let drip_color = match props.r#type {
        ButtonTypes::ErrorLight
        | ButtonTypes::SuccessLight
        | ButtonTypes::WarningLight
        | ButtonTypes::SecondaryLight => add_alpha_3(palette.accents_2, 0.65),
        _ => add_alpha_3(hover_bg, 0.65),
    };

    let mut countdown: Signal<Option<Task>> = use_signal(|| None);

    let onclick = move |ev: Event<MouseData>| async move {
        if props.disabled || props.loading {
            return;
        }
        if let Some(countdown) = countdown().take() {
            countdown.cancel();
            drip_show.set(false);
        }
        let coods = ev.element_coordinates();
        drip_x.set(coods.x as f32);
        drip_y.set(coods.y as f32);
        drip_show.set(true);
        countdown.set(Some(spawn(async move {
            smol::Timer::after(Duration::from_millis(350)).await;
            drip_show.set(false);
            drip_x.set(0.);
            drip_y.set(0.);
        })));
        props.onclick.call(ev);
    };

    rsx! {
        button {
            class: "btn",
            style: style,
            disabled: props.disabled,
            onclick,
            if props.loading {
                ButtonLoading {
                    color
                }
            }
            {props.children}
            if drip_show() {
                ButtonDrip {
                    x: drip_x,
                    y: drip_y,
                    color: drip_color,
                }
            }
        }
    }
}

#[component]
fn ButtonDrip(x: Signal<f32>, y: Signal<f32>, color: String) -> Element {
    let style = use_memo(move || {
        let x = x() - 10.;
        let y = y() - 10.;
        format!("top: {y}; left: {x};")
    });
    rsx! {
        div {
            class: "drip",
            svg {
                class: "drip-svg",
                view_box: "0 0 20 20",
                width: "20",
                height: "20",
                style,
                g {
                    stroke : "none",
                    stroke_width: "1",
                    fill: "none",
                    fill_rule: "evenodd",
                    g {
                        fill: color,
                        rect {
                            width: "100%",
                            height: "100%",
                            rx: "10"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ButtonLoading(color: String) -> Element {
    rsx! {
        div {
            class: "btn-loading",
            Loading {
                color
            }
        }
    }
}
