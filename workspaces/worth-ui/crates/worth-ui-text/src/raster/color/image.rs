//! Qualified intrinsic-color source selection and outline image production.

use worth_ui_host_contract::UiGlyphRasterKey;

use super::super::demand_candidate::UiGlyphRasterCandidate;
use super::super::denial::UiGlyphRasterizationDenial;
use super::admission::{validate_color_palette, validate_color_source};
use super::bitmap::{bitmap_geometry, render_color_bitmap};
use super::colr::render_colr;
use super::pixels::UiCanonicalColorImage;
use crate::UiQualifiedTextLayout;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiColorRasterGeometry {
    pub(crate) width: u32,
    pub(crate) height: u32,
}

pub(crate) fn predicted_geometry(
    layout: &UiQualifiedTextLayout,
    candidate: &UiGlyphRasterCandidate,
) -> Result<UiColorRasterGeometry, UiGlyphRasterizationDenial> {
    let key = candidate.key;
    let (width, height) = match key.source() {
        worth_ui_host_contract::UiGlyphRasterSource::ColorOutline => {
            super::super::qualified_raster_admission::predicted_outline_extent(candidate, key)
                .ok_or(UiGlyphRasterizationDenial::ExtentExceeded)?
        }
        worth_ui_host_contract::UiGlyphRasterSource::ColorBitmap => {
            let geometry = bitmap_geometry(layout, key)?;
            (geometry.width, geometry.height)
        }
        _ => return Err(UiGlyphRasterizationDenial::UnsupportedColorSource),
    };
    Ok(UiColorRasterGeometry { width, height })
}

pub(super) fn render_color_image(
    layout: &UiQualifiedTextLayout,
    candidate: &UiGlyphRasterCandidate,
    key: UiGlyphRasterKey,
) -> Result<UiCanonicalColorImage, UiGlyphRasterizationDenial> {
    let resource = layout
        .artifact()
        .face_resource(key.face())
        .ok_or(UiGlyphRasterizationDenial::InvalidFaceResource)?;
    validate_color_source(layout, key)?;
    validate_color_palette(layout, key)?;
    let geometry = predicted_geometry(layout, candidate)?;
    match key.source() {
        worth_ui_host_contract::UiGlyphRasterSource::ColorOutline => {
            render_color_outline(resource, candidate, key, geometry)
        }
        worth_ui_host_contract::UiGlyphRasterSource::ColorBitmap => {
            render_color_bitmap(resource, key)
        }
        _ => Err(UiGlyphRasterizationDenial::UnsupportedColorSource),
    }
}

fn render_color_outline(
    resource: &crate::layout_artifact::UiQualifiedTextFaceResource,
    candidate: &UiGlyphRasterCandidate,
    key: UiGlyphRasterKey,
    geometry: UiColorRasterGeometry,
) -> Result<UiCanonicalColorImage, UiGlyphRasterizationDenial> {
    render_colr(resource, candidate, key, geometry)
}
