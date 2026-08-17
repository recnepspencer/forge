//! Borrowed glyph-run draw attribution.
//!
//! Each run names the mounted mechanic, layout, paint span, and original
//! range together with the exact raster key. Attribution is not a cache key.

use super::{UiGlyphRasterKey, UiQualifiedTextLayoutIdentity, UiTextOriginalRange};
use crate::{UiMountedPaintCommandIdentity, UiMountedTextPaintSpanIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRunView {
    mechanic: UiMountedPaintCommandIdentity,
    layout: UiQualifiedTextLayoutIdentity,
    paint_span: UiMountedTextPaintSpanIdentity,
    original_range: UiTextOriginalRange,
    raster_key: UiGlyphRasterKey,
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct UiGlyphRunViewInput {
    pub mechanic: UiMountedPaintCommandIdentity,
    pub layout: UiQualifiedTextLayoutIdentity,
    pub paint_span: UiMountedTextPaintSpanIdentity,
    pub original_range: UiTextOriginalRange,
    pub raster_key: UiGlyphRasterKey,
}

impl UiGlyphRunView {
    #[doc(hidden)]
    pub const fn from_text_mechanics(input: UiGlyphRunViewInput) -> Self {
        Self {
            mechanic: input.mechanic,
            layout: input.layout,
            paint_span: input.paint_span,
            original_range: input.original_range,
            raster_key: input.raster_key,
        }
    }

    pub const fn mechanic(self) -> UiMountedPaintCommandIdentity {
        self.mechanic
    }

    pub const fn layout_identity(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }

    pub const fn paint_span(self) -> UiMountedTextPaintSpanIdentity {
        self.paint_span
    }

    pub const fn original_range(self) -> UiTextOriginalRange {
        self.original_range
    }

    pub const fn raster_key(self) -> UiGlyphRasterKey {
        self.raster_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qualified_text::raster_key::{
        UiGlyphRasterFractionalOrigin, UiGlyphRasterKeyInput, UiGlyphRasterPalette,
        UiGlyphRasterSize, UiGlyphRasterSource, UiGlyphVariationCoordinates,
    };
    use crate::{
        UiFontCollectionGeneration, UiFontCollectionLineageIdentity, UiMountedAllocationBasis,
        UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedCoordinateSpace,
        UiMountedFilledRectCompletionInput, UiMountedFilledRectMechanic, UiMountedFrameIdentity,
        UiMountedInstanceIdentity, UiMountedNodeReceiptIssuer, UiMountedRgba8,
        UiMountedTransformProjection, UiQualifiedFontFaceIdentity, UiSemanticSurfaceIdentity,
        UiSurfaceBindingGeneration, UiTextProfileGeneration,
    };

    fn sample_key() -> UiGlyphRasterKey {
        UiGlyphRasterKey::from_text_mechanics(UiGlyphRasterKeyInput {
            font_collection: UiFontCollectionGeneration::new(2).unwrap(),
            font_collection_lineage: UiFontCollectionLineageIdentity::from_text_mechanics([5; 32]),
            profile: UiTextProfileGeneration::new(4).unwrap(),
            face: UiQualifiedFontFaceIdentity::from_text_mechanics([6; 32], 1),
            glyph_id: 21,
            variations: UiGlyphVariationCoordinates::empty(),
            palette: UiGlyphRasterPalette::new(1),
            size: UiGlyphRasterSize::from_millipoints(16_000).unwrap(),
            source: UiGlyphRasterSource::ColorBitmap,
            dpi_milli: 2_000,
            origin: UiGlyphRasterFractionalOrigin::from_sixty_fourths(16, 0),
        })
        .unwrap()
    }

    fn mechanic_identity() -> UiMountedPaintCommandIdentity {
        let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
        let mounted_instance = UiMountedInstanceIdentity::mint_unbound().unwrap();
        let bounds = UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
            x: 0.0,
            y: 0.0,
            width: 16.0,
            height: 16.0,
            coordinate_space: UiMountedCoordinateSpace::HostSurface,
        })
        .unwrap();
        let mechanic = UiMountedFilledRectMechanic::complete_from_runtime_mounting(
            UiMountedFilledRectCompletionInput {
                frame,
                surface: UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
                binding: UiSurfaceBindingGeneration::mint_unbound().unwrap(),
                mounted_instance,
                node_receipt: UiMountedNodeReceiptIssuer::mint_for(frame)
                    .unwrap()
                    .receipt_for(mounted_instance),
                allocation_basis: UiMountedAllocationBasis::new(
                    1,
                    1,
                    1,
                    UiMountedTransformProjection::Identity,
                ),
                bounds,
                color: UiMountedRgba8::new(0, 0, 0, 255),
                layer_semantic_order: 0,
                clip_bounds: bounds,
            },
        )
        .unwrap();
        UiMountedPaintCommandIdentity::filled_rect(&mechanic)
    }

    #[test]
    fn glyph_run_view_ties_draw_attribution_to_the_exact_key() {
        let mechanic = mechanic_identity();
        let layout = UiQualifiedTextLayoutIdentity::from_text_mechanics([9; 32]);
        let view = UiGlyphRunView::from_text_mechanics(UiGlyphRunViewInput {
            mechanic,
            layout,
            paint_span: UiMountedTextPaintSpanIdentity::from_runtime_mounting([3; 32]),
            original_range: UiTextOriginalRange::new(4, 8).unwrap(),
            raster_key: sample_key(),
        });
        assert_eq!(view.mechanic(), mechanic);
        assert_eq!(view.layout_identity(), layout);
        assert_eq!(view.paint_span().digest(), [3; 32]);
        assert_eq!(view.original_range().start(), 4);
        assert_eq!(view.raster_key().glyph_id(), 21);
        assert_eq!(view.raster_key().source(), UiGlyphRasterSource::ColorBitmap);
    }
}
