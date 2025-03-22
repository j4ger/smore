use dioxus::hooks::{use_context, use_context_provider};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scale {
    unit: u32,
    pl: Option<u32>,
    pr: Option<u32>,
    pb: Option<u32>,
    pt: Option<u32>,
    px: Option<u32>,
    py: Option<u32>,
    mb: Option<u32>,
    mt: Option<u32>,
    ml: Option<u32>,
    mr: Option<u32>,
    mx: Option<u32>,
    my: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
    font: Option<u32>,
}

impl Scale {
    pub fn new(unit: u32) -> Self {
        Scale {
            unit,
            pl: None,
            pr: None,
            pb: None,
            pt: None,
            px: None,
            py: None,
            mb: None,
            mt: None,
            ml: None,
            mr: None,
            mx: None,
            my: None,
            width: None,
            height: None,
            font: None,
        }
    }

    pub fn with_unit(mut self, unit: u32) -> Self {
        self.unit = unit;
        self
    }

    pub fn with_pl(mut self, pl: u32) -> Self {
        self.pl = Some(pl);
        self
    }

    pub fn with_pr(mut self, pr: u32) -> Self {
        self.pr = Some(pr);
        self
    }

    pub fn with_pb(mut self, pb: u32) -> Self {
        self.pb = Some(pb);
        self
    }

    pub fn with_pt(mut self, pt: u32) -> Self {
        self.pt = Some(pt);
        self
    }

    pub fn with_px(mut self, px: u32) -> Self {
        self.px = Some(px);
        self
    }

    pub fn with_py(mut self, py: u32) -> Self {
        self.py = Some(py);
        self
    }

    pub fn with_mb(mut self, mb: u32) -> Self {
        self.mb = Some(mb);
        self
    }

    pub fn with_mt(mut self, mt: u32) -> Self {
        self.mt = Some(mt);
        self
    }

    pub fn with_ml(mut self, ml: u32) -> Self {
        self.ml = Some(ml);
        self
    }

    pub fn with_mr(mut self, mr: u32) -> Self {
        self.mr = Some(mr);
        self
    }

    pub fn with_mx(mut self, mx: u32) -> Self {
        self.mx = Some(mx);
        self
    }

    pub fn with_my(mut self, my: u32) -> Self {
        self.my = Some(my);
        self
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_font(mut self, font: u32) -> Self {
        self.font = Some(font);
        self
    }

    pub fn pl(&self, scale: f32) -> String {
        format!("{}px", (self.pl.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn pr(&self, scale: f32) -> String {
        format!("{}px", (self.pr.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn pb(&self, scale: f32) -> String {
        format!("{}px", (self.pb.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn pt(&self, scale: f32) -> String {
        format!("{}px", (self.pt.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn px(&self, scale: f32) -> String {
        format!("{}px", (self.px.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn py(&self, scale: f32) -> String {
        format!("{}px", (self.py.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn mb(&self, scale: f32) -> String {
        format!("{}px", (self.mb.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn mt(&self, scale: f32) -> String {
        format!("{}px", (self.mt.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn ml(&self, scale: f32) -> String {
        format!("{}px", (self.ml.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn mr(&self, scale: f32) -> String {
        format!("{}px", (self.mr.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn mx(&self, scale: f32) -> String {
        format!("{}px", (self.mx.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn my(&self, scale: f32) -> String {
        format!("{}px", (self.my.unwrap_or(self.unit) as f32 * scale) as u32)
    }

    pub fn width(&self, scale: f32) -> String {
        format!(
            "{}px",
            (self.width.unwrap_or(self.unit) as f32 * scale) as u32
        )
    }

    pub fn height(&self, scale: f32) -> String {
        format!(
            "{}px",
            (self.height.unwrap_or(self.unit) as f32 * scale) as u32
        )
    }

    pub fn font(&self, scale: f32) -> String {
        format!(
            "{}px",
            (self.font.unwrap_or(self.unit) as f32 * scale) as u32
        )
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale::new(16)
    }
}

pub fn provide_scale(scale: Scale) {
    use_context_provider(|| scale);
}

pub fn use_scale() -> Scale {
    use_context::<Scale>()
}
