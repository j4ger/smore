use dioxus::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub r#type: &'static str,
    pub palette: Palette,
    pub expressiveness: Expressiveness,
    pub layout: Layout,
    pub font: Font,
    pub breakpoints: Breakpoints,
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub accents_1: &'static str,
    pub accents_2: &'static str,
    pub accents_3: &'static str,
    pub accents_4: &'static str,
    pub accents_5: &'static str,
    pub accents_6: &'static str,
    pub accents_7: &'static str,
    pub accents_8: &'static str,
    pub background: &'static str,
    pub foreground: &'static str,
    pub selection: &'static str,
    pub secondary: &'static str,
    pub code: &'static str,
    pub border: &'static str,
    pub error: &'static str,
    pub errorLight: &'static str,
    pub errorLighter: &'static str,
    pub errorDark: &'static str,
    pub success: &'static str,
    pub successLight: &'static str,
    pub successLighter: &'static str,
    pub successDark: &'static str,
    pub warning: &'static str,
    pub warningLight: &'static str,
    pub warningLighter: &'static str,
    pub warningDark: &'static str,
    pub cyan: &'static str,
    pub cyanLighter: &'static str,
    pub cyanLight: &'static str,
    pub cyanDark: &'static str,
    pub violet: &'static str,
    pub violetLighter: &'static str,
    pub violetLight: &'static str,
    pub violetDark: &'static str,
    pub purple: &'static str,
    pub alert: &'static str,
    pub magenta: &'static str,
    pub link: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Expressiveness {
    pub linkStyle: &'static str,
    pub linkHoverStyle: &'static str,
    pub dropdownBoxShadow: &'static str,
    pub scrollerStart: &'static str,
    pub scrollerEnd: &'static str,
    pub shadowSmall: &'static str,
    pub shadowMedium: &'static str,
    pub shadowLarge: &'static str,
    pub portalOpacity: f32,
}

pub const LIGHT_THEME: Theme = Theme {
    r#type: "light",
    palette: Palette {
        accents_1: "#fafafa",
        accents_2: "#eaeaea",
        accents_3: "#999",
        accents_4: "#888",
        accents_5: "#666",
        accents_6: "#444",
        accents_7: "#333",
        accents_8: "#111",
        background: "#fff",
        foreground: "#000",
        selection: "#79ffe1",
        secondary: "#666",
        code: "#f81ce5",
        border: "#eaeaea",
        error: "#e00",
        errorLight: "#ff1a1a",
        errorLighter: "#f7d4d6",
        errorDark: "#c50000",
        success: "#0070f3",
        successLight: "#3291ff",
        successLighter: "#d3e5ff",
        successDark: "#0761d1",
        warning: "#f5a623",
        warningLight: "#f7b955",
        warningLighter: "#ffefcf",
        warningDark: "#ab570a",
        cyan: "#50e3c2",
        cyanLighter: "#aaffec",
        cyanLight: "#79ffe1",
        cyanDark: "#29bc9b",
        violet: "#7928ca",
        violetLighter: "#e3d7fc",
        violetLight: "#8a63d2",
        violetDark: "#4c2889",
        purple: "#f81ce5",
        alert: "#ff0080",
        magenta: "#eb367f",
        link: "#0070f3",
    },
    expressiveness: Expressiveness {
        linkStyle: "none",
        linkHoverStyle: "none",
        dropdownBoxShadow: "0 4px 4px 0 rgba(0, 0, 0, 0.02)",
        scrollerStart: "rgba(255, 255, 255, 1)",
        scrollerEnd: "rgba(255, 255, 255, 0)",
        shadowSmall: "0 5px 10px rgba(0, 0, 0, 0.12)",
        shadowMedium: "0 8px 30px rgba(0, 0, 0, 0.12)",
        shadowLarge: "0 30px 60px rgba(0, 0, 0, 0.12)",
        portalOpacity: 0.25,
    },
    breakpoints: DEFAULT_BREAKPOINTS,
    font: DEFAULT_FONT,
    layout: DEFAULT_LAYOUT,
};

#[derive(Clone, Copy, Debug)]
pub struct Layout {
    pub gap: &'static str,
    pub gapNegative: &'static str,
    pub gapHalf: &'static str,
    pub gapHalfNegative: &'static str,
    pub gapQuarter: &'static str,
    pub gapQuarterNegative: &'static str,
    pub pageMargin: &'static str,
    pub pageWidth: &'static str,
    pub pageWidthWithMargin: &'static str,
    pub breakpointMobile: &'static str,
    pub breakpointTablet: &'static str,
    pub radius: &'static str,
    pub unit: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Font {
    pub sans: &'static str,
    pub mono: &'static str,
    pub prism: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Breakpoints {
    pub xs: Breakpoint,
    pub sm: Breakpoint,
    pub md: Breakpoint,
    pub lg: Breakpoint,
    pub xl: Breakpoint,
}

#[derive(Clone, Copy, Debug)]
pub struct Breakpoint {
    pub min: &'static str,
    pub max: &'static str,
}

pub const DEFAULT_LAYOUT: Layout = Layout {
    gap: "16pt",
    gapNegative: "-16pt",
    gapHalf: "8pt",
    gapHalfNegative: "-8pt",
    gapQuarter: "4pt",
    gapQuarterNegative: "-4pt",
    pageMargin: "16pt",
    pageWidth: "750pt",
    pageWidthWithMargin: "782pt",
    breakpointMobile: "650px",
    breakpointTablet: "900px",
    radius: "6px",
    unit: "16px",
};

pub const DEFAULT_FONT: Font = Font {
    sans: "\"Inter\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", \"Roboto\", \"Oxygen\", \"Ubuntu\", \"Cantarell\", \"Fira Sans\", \"Droid Sans\", \"Helvetica Neue\", sans-serif",
    mono: "Menlo, Monaco, Lucida Console, Liberation Mono, DejaVu Sans Mono, Bitstream Vera Sans Mono, Courier New, monospace",
    prism: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,\"Liberation Mono\", \"Courier New\", monospace",
};

pub const DEFAULT_BREAKPOINTS: Breakpoints = Breakpoints {
    xs: Breakpoint {
        min: "0",
        max: "650px",
    },
    sm: Breakpoint {
        min: "650px",
        max: "900px",
    },
    md: Breakpoint {
        min: "900px",
        max: "1280px",
    },
    lg: Breakpoint {
        min: "1280px",
        max: "1920px",
    },
    xl: Breakpoint {
        min: "1920px",
        max: "10000px",
    },
};

pub fn provide_theme() {
    use_context_provider(|| LIGHT_THEME);
}

pub fn use_theme() -> Theme {
    use_context::<Theme>()
}
