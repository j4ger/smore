use crate::components::{use_scale, use_theme, Scale};
use dioxus::prelude::*;

pub const PAGE_STYLE_COMMON: &'static str = "
    main {
        font-size: var(--g-font-size);
        width: var(--g-width);
        height: var(--g-height);
        padding: var(--g-padding);
        margin: var(--g-margin);
    }

    :global(body) {
        background-image: radial-gradient(#e3e3e3 var(--g-dot-size), transparent 0),
        radial-gradient(#e3e3e3 var(--g-dot-size), transparent 0);
        background-position: 0 0, calc(var(--g-dot-space) * 25px) calc(var(--g-dot-space) * 25px);
        background-attachment: fixed;
        background-size: calc(var(--g-dot-space) * 50px) calc(var(--g-dot-space) * 50px);
    }

    section {
        max-width: 100vw;
        min-height: 100vh;
        box-sizing: border-box;
        position: relative;
        font-size: var(--g-font-size);
        width: var(--g-width);
        height: var(--g-height);
        padding: var(--g-padding);
        margin: var(--g-margin);
    }
";

#[component]
pub fn PageContent(scale: Option<Scale>, children: Element) -> Element {
    let scales = scale.unwrap_or(use_scale());

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

    let pt = scales.pt(3.125);
    let pr = scales.pr(0.);
    let pb = scales.pb(3.125);
    let pl = scales.pl(0.);

    let mt = scales.mt(0.);
    let mb = scales.mb(0.);
    let ml = scales.ml(0.);
    let mr = scales.mr(0.);

    let style = format!(
        "
        --g-font-size: {font_size};
        --g-width: {width};
        --g-height: {height};
        --g-padding: {pt} {pr} {pb} {pl};
        --g-margin: {mt} {mr} {mb} {ml};
    "
    );

    rsx! {
        main {
            style,
            { children }
        }
    }
}

#[derive(Clone, Props, Debug, PartialEq)]
pub struct PageProps {
    #[props(default = false)]
    dot_backdrop: bool,
    #[props(default = 1)]
    dot_size: u32,
    #[props(default = 1)]
    dot_space: u32,
    scale: Option<Scale>,
    children: Element,
}

pub fn Page(props: PageProps) -> Element {
    let theme = use_theme();
    let scales = props.scale.unwrap_or(use_scale());

    let dot_size = props.dot_size;
    let dot_space = props.dot_space;
    let dot_style = if theme.r#type != "dark" && props.dot_backdrop {
        format!(
            "
            --g-dot-size: {dot_size};
            --g-dot-space: {dot_space};
            "
        )
    } else {
        "".into()
    };

    let font_size = scales.font(1.);

    let width = if scales.width.is_some() {
        scales.width(1.)
    } else {
        "calc(100% - 100pt)".into()
    };
    let height = if scales.height.is_some() {
        scales.height(1.)
    } else {
        "auto".into()
    };

    let pt = scales.pt(0.);
    let pr = scales.pr(1.34);
    let pb = scales.pb(0.);
    let pl = scales.pl(1.34);

    let mt = scales.mt(0.);
    let mb = scales.mb(0.);
    let mr = if scales.height.is_some() {
        scales.mr(0.)
    } else {
        "auto".into()
    };
    let ml = if scales.width.is_some() {
        scales.ml(0.)
    } else {
        "auto".into()
    };

    let style = format!(
        "
        {dot_style}
        --g-font-size: {font_size};
        --g-width: {width};
        --g-height: {height};
        --g-padding: {pt} {pr} {pb} {pl};
        --g-margin: {mt} {mr} {mb} {ml};
    "
    );

    rsx! {
        section {
            style,
            PageContent {
                {props.children}
            }
        }
    }
}
