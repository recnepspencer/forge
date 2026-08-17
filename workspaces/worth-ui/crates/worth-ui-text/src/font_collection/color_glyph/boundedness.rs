use skrifa::{
    color::{
        Brush, ColorGlyphFormat, ColorPainter, CompositeMode, PaintCachedColorGlyph, PaintError,
        Transform,
    },
    instance::LocationRef,
    raw::types::BoundingBox,
    GlyphId, MetadataProvider,
};

use crate::font_collection::UiFontCollectionAdmissionDenial;

use super::malformed;

pub(super) fn validate(
    font: &harfrust::FontRef<'_>,
    glyphs: impl IntoIterator<Item = u16>,
) -> Result<(), UiFontCollectionAdmissionDenial> {
    let collection = font.color_glyphs();
    for glyph in glyphs {
        let glyph = collection
            .get_with_format(GlyphId::new(glyph.into()), ColorGlyphFormat::ColrV1)
            .ok_or_else(malformed)?;
        let mut painter = BoundednessPainter::default();
        glyph
            .paint(LocationRef::default(), &mut painter)
            .map_err(|_| malformed())?;
        if !painter.is_bounded() {
            return Err(malformed());
        }
    }
    Ok(())
}

struct BoundednessPainter {
    bounded: bool,
    clip_depth: u8,
    layers: Vec<Layer>,
}

impl Default for BoundednessPainter {
    fn default() -> Self {
        Self {
            bounded: true,
            clip_depth: 0,
            layers: Vec::new(),
        }
    }
}

struct Layer {
    backdrop_bounded: bool,
    mode: CompositeMode,
}

impl BoundednessPainter {
    fn is_bounded(&self) -> bool {
        self.bounded && self.layers.is_empty() && self.clip_depth == 0
    }
}

impl ColorPainter for BoundednessPainter {
    fn push_transform(&mut self, _transform: Transform) {}

    fn pop_transform(&mut self) {}

    fn push_clip_glyph(&mut self, _glyph_id: GlyphId) {
        self.clip_depth = self.clip_depth.saturating_add(1);
    }

    fn push_clip_box(&mut self, _clip_box: BoundingBox<f32>) {
        self.clip_depth = self.clip_depth.saturating_add(1);
    }

    fn pop_clip(&mut self) {
        self.clip_depth = self.clip_depth.saturating_sub(1);
    }

    fn fill(&mut self, _brush: Brush<'_>) {
        self.bounded &= self.clip_depth != 0;
    }

    fn push_layer(&mut self, mode: CompositeMode) {
        self.layers.push(Layer {
            backdrop_bounded: core::mem::replace(&mut self.bounded, true),
            mode,
        });
    }

    fn pop_layer(&mut self) {
        let Some(layer) = self.layers.pop() else {
            self.bounded = false;
            return;
        };
        self.bounded = composite_is_bounded(self.bounded, layer.backdrop_bounded, layer.mode);
    }

    fn paint_cached_color_glyph(
        &mut self,
        _glyph: GlyphId,
    ) -> Result<PaintCachedColorGlyph, PaintError> {
        Ok(PaintCachedColorGlyph::Unimplemented)
    }
}

fn composite_is_bounded(source: bool, backdrop: bool, mode: CompositeMode) -> bool {
    use CompositeMode::*;
    match mode {
        Clear => true,
        Src | SrcOut => source,
        Dest | DestOut => backdrop,
        SrcIn | DestIn => source || backdrop,
        SrcAtop => backdrop,
        DestAtop => source,
        SrcOver | DestOver | Xor | Plus | Screen | Overlay | Darken | Lighten | ColorDodge
        | ColorBurn | HardLight | SoftLight | Difference | Exclusion | Multiply | HslHue
        | HslSaturation | HslColor | HslLuminosity => source && backdrop,
        Unknown => false,
    }
}

#[cfg(test)]
mod tests {
    use super::composite_is_bounded;
    use skrifa::color::CompositeMode;

    #[test]
    fn atop_result_is_bounded_by_the_operand_it_preserves() {
        assert!(composite_is_bounded(false, true, CompositeMode::SrcAtop));
        assert!(composite_is_bounded(true, false, CompositeMode::DestAtop));
        assert!(!composite_is_bounded(true, false, CompositeMode::SrcAtop));
        assert!(!composite_is_bounded(false, true, CompositeMode::DestAtop));
    }
}
