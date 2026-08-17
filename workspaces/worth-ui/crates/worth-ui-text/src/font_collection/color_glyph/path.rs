use kurbo::{Affine as KurboAffine, BezPath, Point, Rect, Shape};
use skrifa::{
    instance::{LocationRef, NormalizedCoord, Size},
    outline::{DrawSettings, HintingInstance, OutlinePen},
    GlyphId, MetadataProvider,
};

pub(crate) fn glyph(
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

pub(crate) fn hinted_glyph(
    font: &harfrust::FontRef<'_>,
    glyph_id: GlyphId,
    hinting: &HintingInstance,
    design_to_pixel_scale: f64,
    transform: KurboAffine,
) -> Option<BezPath> {
    if !design_to_pixel_scale.is_finite() || design_to_pixel_scale <= 0.0 {
        return None;
    }
    let mut pen = PathPen::default();
    font.outline_glyphs()
        .get(glyph_id)?
        .draw(DrawSettings::hinted(hinting, false), &mut pen)
        .ok()?;
    let path = pen.finish();
    (!path.is_empty()).then(|| transform * KurboAffine::scale(1.0 / design_to_pixel_scale) * path)
}

pub(crate) fn rectangle(rect: Rect, transform: KurboAffine) -> BezPath {
    transform * rect.to_path(0.0)
}

#[cfg(test)]
pub(crate) fn rectangles(rects: impl IntoIterator<Item = Rect>) -> BezPath {
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

#[cfg(test)]
mod tests {
    use read_fonts::TableProvider;
    use skrifa::outline::Engine;

    use super::*;
    use crate::font_collection::profile_inputs_from_repository;

    #[test]
    fn qualified_interpreter_changes_the_pinned_outline_at_small_size() {
        let input = profile_inputs_from_repository()
            .into_vec()
            .into_iter()
            .find(|input| input.id.as_ref() == "noto-sans-roman")
            .unwrap();
        let font = harfrust::FontRef::from_index(&input.bytes, 0).unwrap();
        let glyph_id = GlyphId::new(font.cmap().unwrap().map_codepoint('H').unwrap().to_u32());
        let pixels_per_em = 9.0;
        let scale = pixels_per_em / f64::from(font.head().unwrap().units_per_em());
        let unhinted = glyph(&font, &[], glyph_id, KurboAffine::IDENTITY).unwrap();
        let outlines = font.outline_glyphs();
        let hinting = HintingInstance::new(
            &outlines,
            Size::new(pixels_per_em as f32),
            LocationRef::default(),
            Engine::Interpreter,
        )
        .unwrap();
        let hinted = hinted_glyph(&font, glyph_id, &hinting, scale, KurboAffine::IDENTITY).unwrap();

        assert_ne!(unhinted.bounding_box(), hinted.bounding_box());
    }
}
