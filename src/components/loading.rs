use crate::components::{use_scale, use_theme, Scale};
use dioxus::prelude::*;

pub const LOADING_STYLE_COMMON: &'static str = "
    .loading-container {
        display: inline-flex;
        align-items: center;
        position: relative;
        font-size: var(--g-font-size);
        width: var(--g-width);
        height: var(--g-height);
        min-height: 1em;
        padding: var(--g-padding);
        margin: var(--g-margin);
    }

    .loading-child {
        margin-right: 0.5em;
        color: var(--g-color);
        line-height: 1;
    }

    label :global(*) {
        margin: 0;
    }

    .loading {
        position: absolute;
        top: 50%;
        left: 50%;
        width: 100%;
        height: 100%;
        transform: translate(-50%, -50%);
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: transparent;
        user-select: none;
    }

    .loading > i {
        width: 0.25em;
        height: 0.25em;
        border-radius: 50%;
        background-color: var(--g-bg);
        margin: 0 calc(0.25em / 2 * var(--g-space-ratio));
        display: inline-block;
        animation: loading-blink 1.4s infinite both;
    }

    .loading > i:nth-child(2) {
        animation-delay: 0.2s;
    }

    .loading > i:nth-child(3) {
        animation-delay: 0.4s;
    }

    @keyframes loading-blink {
        0% {
            opacity: 0.2;
        }

        20% {
            opacity: 1;
        }

        100% {
            opacity: 0.2;
        }
    }";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LoadingTypes {
    Default,
    Secondary,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Props, Debug, PartialEq)]
pub struct LoadingProps {
    #[props(default = LoadingTypes::Default)]
    r#type: LoadingTypes,
    #[props(into)]
    color: Option<String>,
    #[props(default = 1.)]
    space_ratio: f32,
    scale: Option<Scale>,
    children: Option<Element>,
}

pub fn Loading(props: LoadingProps) -> Element {
    let theme = use_theme();
    let palette = theme.palette;
    let scales = props.scale.unwrap_or(use_scale());

    let bg = props.color.unwrap_or(
        (match props.r#type {
            LoadingTypes::Default => palette.accents_6,
            LoadingTypes::Secondary => palette.secondary,
            LoadingTypes::Success => palette.success,
            LoadingTypes::Warning => palette.warning,
            LoadingTypes::Error => palette.error,
        })
        .to_string(),
    );
    let color = palette.accents_5;

    let font_size = scales.font(1.);
    let width = if scales.width.is_some() {
        scales.width(1.)
    } else {
        "100%".into()
    };
    let height = if scales.height.is_some() {
        scales.height(1.)
    } else {
        "100%".into()
    };

    let pt = scales.pt(0.);
    let pr = scales.pr(0.);
    let pb = scales.pb(0.);
    let pl = scales.pl(0.);

    let mt = scales.mt(0.);
    let mb = scales.mb(0.);
    let ml = scales.ml(0.);
    let mr = scales.mr(0.);

    let space_ratio = format!("{:.2}", props.space_ratio);

    let style = format!(
        "
        --g-font-size: {font_size};
        --g-width: {width};
        --g-height: {height};
        --g-padding: {pt} {pr} {pb} {pl};
        --g-margin: {mt} {mr} {mb} {ml};
        --g-color: {color};
        --g-bg: {bg};
        --g-space-ratio: {space_ratio};
    "
    );

    rsx! {
        div {
            class: "loading-container",
            style,
            span {
                class: "loading",
                if let Some(children) = props.children {
                    label {
                        class: "loading-child",
                        {children}
                    }
                }
                i {}
                i {}
                i {}
            }
        }
    }
}
