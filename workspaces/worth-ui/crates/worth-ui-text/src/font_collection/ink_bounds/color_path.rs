use kurbo::{Affine as KurboAffine, BezPath, Point, Rect, Shape};
use skrifa::{
    instance::{LocationRef, NormalizedCoord, Size},
    outline::{DrawSettings, OutlinePen},
    GlyphId, MetadataProvider,
};

pub(super) fn glyph(
    font: &harfrust::FontRef<'_>,
    coords: &[NormalizedCoord],
    glyph_id: GlyphId,
    transform: KurboAffine,
) -> Option<BezPath> {
    let mut pen = PathPen::default();
    font.outline_glyphs()
        .get(glyph_id)?
        .draw(
            DrawSettings::unhinted(Size::unscaled(), LocationRef::new(coords)),
            &mut pen,
        )
        .ok()?;
    let path = pen.finish();
    (!path.is_empty()).then(|| transform * path)
}

pub(super) fn rectangle(rect: Rect, transform: KurboAffine) -> BezPath {
    transform * rect.to_path(0.0)
}

#[cfg(test)]
pub(super) fn rectangles(rects: impl IntoIterator<Item = Rect>) -> BezPath {
    let mut path = BezPath::new();
    for rect in rects {
        path.extend(rect.path_elements(0.0));
    }
    path
}

#[derive(Default)]
struct PathPen {
    path: BezPath,
    open: bool,
}

impl PathPen {
    fn finish(mut self) -> BezPath {
        if self.open {
            self.path.close_path();
        }
        self.path
    }
}

impl OutlinePen for PathPen {
    fn move_to(&mut self, x: f32, y: f32) {
        if self.open {
            self.path.close_path();
        }
        self.path.move_to(Point::new(f64::from(x), f64::from(y)));
        self.open = true;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(Point::new(f64::from(x), f64::from(y)));
    }

    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.path.quad_to(
            Point::new(f64::from(cx), f64::from(cy)),
            Point::new(f64::from(x), f64::from(y)),
        );
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.path.curve_to(
            Point::new(f64::from(cx0), f64::from(cy0)),
            Point::new(f64::from(cx1), f64::from(cy1)),
            Point::new(f64::from(x), f64::from(y)),
        );
    }

    fn close(&mut self) {
        self.path.close_path();
        self.open = false;
    }
}
