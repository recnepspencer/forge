//! Exact effect-free geometry shared by demand planning and raster admission.

use worth_ui_host_contract::{UiGlyphRasterExtent, UiGlyphRasterSource};

use super::demand_candidate::UiGlyphRasterCandidate;
use super::denial::UiGlyphRasterizationDenial;
use crate::UiQualifiedTextLayout;

pub(super) fn predicted_extent(
    layout: &UiQualifiedTextLayout,
    candidate: &UiGlyphRasterCandidate,
) -> Result<UiGlyphRasterExtent, UiGlyphRasterizationDenial> {
    let (width, height) = match candidate.key.source() {
        UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort => {
            super::qualified_raster_admission::predicted_outline_extent(candidate, candidate.key)
                .ok_or(UiGlyphRasterizationDenial::ExtentExceeded)?
        }
        UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap => {
            let geometry = super::color::predicted_geometry(layout, candidate)?;
            (geometry.width, geometry.height)
        }
    };
    UiGlyphRasterExtent::new(width, height).ok_or(UiGlyphRasterizationDenial::ExtentExceeded)
}
