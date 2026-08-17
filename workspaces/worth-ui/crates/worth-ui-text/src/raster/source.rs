//! Raster source and pixel-format posture owned by text mechanics.
//!
//! Source selection is identity, not a production implementation. This module
//! does not interpret outlines, color tables, or bitmap strikes.

use std::marker::PhantomData;

pub use worth_ui_host_contract::UiGlyphRasterSource;

pub struct UiAlphaRasterKind(PhantomData<()>);
pub struct UiColorRasterKind(PhantomData<()>);

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::UiAlphaRasterKind {}
    impl Sealed for super::UiColorRasterKind {}
}

pub trait UiGlyphRasterFormat: sealed::Sealed {
    const CHANNELS: usize;
    fn source_matches(source: UiGlyphRasterSource) -> bool;
}

impl UiGlyphRasterFormat for UiAlphaRasterKind {
    const CHANNELS: usize = 1;

    fn source_matches(source: UiGlyphRasterSource) -> bool {
        matches!(
            source,
            UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort
        )
    }
}

impl UiGlyphRasterFormat for UiColorRasterKind {
    const CHANNELS: usize = 4;

    fn source_matches(source: UiGlyphRasterSource) -> bool {
        matches!(
            source,
            UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_kinds_reject_the_opposite_source_class() {
        assert!(UiAlphaRasterKind::source_matches(
            UiGlyphRasterSource::AlphaOutline
        ));
        assert!(!UiAlphaRasterKind::source_matches(
            UiGlyphRasterSource::ColorOutline
        ));
        assert!(UiColorRasterKind::source_matches(
            UiGlyphRasterSource::ColorBitmap
        ));
        assert!(!UiColorRasterKind::source_matches(
            UiGlyphRasterSource::LastResort
        ));
        assert_eq!(UiAlphaRasterKind::CHANNELS, 1);
        assert_eq!(UiColorRasterKind::CHANNELS, 4);
    }
}
