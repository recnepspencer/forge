//! Mounted placement carried into glyph-demand geometry and subpixel keys.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiGlyphRasterPlacement {
    origin_x_millipoints: i64,
    origin_y_millipoints: i64,
}

impl UiGlyphRasterPlacement {
    pub fn from_mounted_logical(origin_x: f32, origin_y: f32) -> Option<Self> {
        Some(Self {
            origin_x_millipoints: mounted_millipoints(origin_x)?,
            origin_y_millipoints: mounted_millipoints(origin_y)?,
        })
    }

    pub const fn origin_x_millipoints(self) -> i64 {
        self.origin_x_millipoints
    }

    pub const fn origin_y_millipoints(self) -> i64 {
        self.origin_y_millipoints
    }
}

fn mounted_millipoints(value: f32) -> Option<i64> {
    let scaled = f64::from(value) * 1_000.0;
    if !scaled.is_finite() || scaled < i64::MIN as f64 || scaled > i64::MAX as f64 {
        return None;
    }
    Some(scaled.round() as i64)
}
