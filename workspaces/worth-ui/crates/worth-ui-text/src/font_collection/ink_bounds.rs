#[path = "ink_bounds/bitmap.rs"]
pub(super) mod bitmap;
#[path = "ink_bounds/color.rs"]
pub(super) mod color;
#[path = "ink_bounds/color_path.rs"]
mod color_path;
#[path = "ink_bounds/color_region.rs"]
mod color_region;
#[cfg(test)]
#[path = "ink_bounds/color_tests.rs"]
pub(super) mod color_tests;

use harfrust::{FontRef, ShaperInstance};
use skrifa::{
    instance::{LocationRef, Size},
    outline::{DrawSettings, OutlinePen},
    GlyphId, MetadataProvider,
};

use super::UiFontGlyphInkBounds;

pub(super) fn for_glyph(
    font: &FontRef<'_>,
    instance: &ShaperInstance,
    glyph_id: u32,
) -> UiFontGlyphInkBounds {
    let glyph_id = GlyphId::new(glyph_id);
    if let Some(bounds) = color::bounds(font, instance.coords(), glyph_id) {
        return bounds.unwrap_or_default();
    }
    if let Some(bounds) = bitmap::bounds(font, glyph_id) {
        return bounds.unwrap_or_default();
    }
    outline_bounds(font, instance.coords(), glyph_id).unwrap_or_default()
}

pub(super) fn outline_bounds(
    font: &FontRef<'_>,
    coords: &[skrifa::instance::NormalizedCoord],
    glyph_id: GlyphId,
) -> Option<UiFontGlyphInkBounds> {
    let mut pen = InkBoundsPen::default();
    font.outline_glyphs()
        .get(glyph_id)?
        .draw(
            DrawSettings::unhinted(Size::unscaled(), LocationRef::new(coords)),
            &mut pen,
        )
        .ok()?;
    pen.finish()
}

#[derive(Default)]
struct InkBoundsPen {
    current: Option<(f64, f64)>,
    first: Option<(f64, f64)>,
    bounds: Option<[f64; 4]>,
}

impl InkBoundsPen {
    fn include(&mut self, x: f64, y: f64) {
        match &mut self.bounds {
            Some(bounds) => {
                bounds[0] = bounds[0].min(x);
                bounds[1] = bounds[1].min(y);
                bounds[2] = bounds[2].max(x);
                bounds[3] = bounds[3].max(y);
            }
            None => self.bounds = Some([x, y, x, y]),
        }
    }

    fn include_x(&mut self, x: f64) {
        if let Some(bounds) = &mut self.bounds {
            bounds[0] = bounds[0].min(x);
            bounds[2] = bounds[2].max(x);
        }
    }

    fn include_y(&mut self, y: f64) {
        if let Some(bounds) = &mut self.bounds {
            bounds[1] = bounds[1].min(y);
            bounds[3] = bounds[3].max(y);
        }
    }

    fn finish(self) -> Option<UiFontGlyphInkBounds> {
        let [x_min, y_min, x_max, y_max] = self.bounds?;
        Some(UiFontGlyphInkBounds {
            x_min: x_min.floor() as i32,
            y_min: y_min.floor() as i32,
            x_max: x_max.ceil() as i32,
            y_max: y_max.ceil() as i32,
        })
    }
}

impl OutlinePen for InkBoundsPen {
    fn move_to(&mut self, x: f32, y: f32) {
        let point = (f64::from(x), f64::from(y));
        self.include(point.0, point.1);
        self.current = Some(point);
        self.first = Some(point);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        let point = (f64::from(x), f64::from(y));
        self.include(point.0, point.1);
        self.current = Some(point);
    }

    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        let start = self.current.unwrap_or_default();
        let control = (f64::from(cx), f64::from(cy));
        let end = (f64::from(x), f64::from(y));
        include_quadratic_extrema(self, start, control, end);
        self.include(end.0, end.1);
        self.current = Some(end);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let start = self.current.unwrap_or_default();
        let first = (f64::from(cx0), f64::from(cy0));
        let second = (f64::from(cx1), f64::from(cy1));
        let end = (f64::from(x), f64::from(y));
        include_cubic_extrema(self, start, first, second, end);
        self.include(end.0, end.1);
        self.current = Some(end);
    }

    fn close(&mut self) {
        self.current = self.first;
    }
}

fn include_quadratic_extrema(
    pen: &mut InkBoundsPen,
    start: (f64, f64),
    control: (f64, f64),
    end: (f64, f64),
) {
    if let Some(t) = quadratic_extremum(start.0, control.0, end.0) {
        pen.include_x(quadratic_at(start.0, control.0, end.0, t));
    }
    if let Some(t) = quadratic_extremum(start.1, control.1, end.1) {
        pen.include_y(quadratic_at(start.1, control.1, end.1, t));
    }
}

fn quadratic_extremum(start: f64, control: f64, end: f64) -> Option<f64> {
    let denominator = start - 2.0 * control + end;
    let t = (start - control) / denominator;
    denominator
        .ne(&0.0)
        .then_some(t)
        .filter(|t| (0.0..1.0).contains(t))
}

fn quadratic_at(start: f64, control: f64, end: f64, t: f64) -> f64 {
    let inverse = 1.0 - t;
    inverse * inverse * start + 2.0 * inverse * t * control + t * t * end
}

fn include_cubic_extrema(
    pen: &mut InkBoundsPen,
    start: (f64, f64),
    first: (f64, f64),
    second: (f64, f64),
    end: (f64, f64),
) {
    for t in cubic_extrema(start.0, first.0, second.0, end.0) {
        pen.include_x(cubic_at(start.0, first.0, second.0, end.0, t));
    }
    for t in cubic_extrema(start.1, first.1, second.1, end.1) {
        pen.include_y(cubic_at(start.1, first.1, second.1, end.1, t));
    }
}

fn cubic_extrema(start: f64, first: f64, second: f64, end: f64) -> Vec<f64> {
    let a = -start + 3.0 * first - 3.0 * second + end;
    let b = 2.0 * (start - 2.0 * first + second);
    let c = first - start;
    if a.abs() < f64::EPSILON {
        return (b.abs() >= f64::EPSILON)
            .then_some(-c / b)
            .filter(|t| (0.0..1.0).contains(t))
            .into_iter()
            .collect();
    }
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return Vec::new();
    }
    let root = discriminant.sqrt();
    [(-b + root) / (2.0 * a), (-b - root) / (2.0 * a)]
        .into_iter()
        .filter(|t| (0.0..1.0).contains(t))
        .collect()
}

fn cubic_at(start: f64, first: f64, second: f64, end: f64, t: f64) -> f64 {
    let inverse = 1.0 - t;
    inverse.powi(3) * start
        + 3.0 * inverse.powi(2) * t * first
        + 3.0 * inverse * t * t * second
        + t.powi(3) * end
}
