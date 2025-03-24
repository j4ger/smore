mod button;
pub use button::{Button, ButtonTypes};

mod scale;
pub use scale::{use_scale, Scale};

mod theme;
pub use theme::{use_theme, Theme};

mod utils;
pub use utils::*;

mod meta;
pub use meta::GeistMeta;

mod loading;
pub use loading::{Loading, LoadingTypes};

mod page;
pub use page::Page;

pub fn init_ui_context() {
    theme::provide_theme();
    scale::provide_scale(Scale::default());
}
