//! Candidate key authority for one qualified positioned glyph.
//!
//! This module is the single place where layout-owned face, coverage, style,
//! variation, and fractional-origin meaning become a raster key. It is
//! shared by demand derivation and raster admission so those phases cannot
//! silently disagree about the glyph being rendered.

use worth_ui_host_contract::{
    UiGlyphRasterKeyInput, UiGlyphRasterSource, UiPositionedTextGlyphRecord,
    UiQualifiedTextCoverageRecord, UiQualifiedTextGlyphRecord, UiQualifiedTextRunRecord,
    UiQualifiedTextStyleRecord, UiTextCoverageDisposition,
};

use super::demand::{UiGlyphRasterDemandDenial, UiGlyphRasterScale};
use super::demand_geometry::fractional_origin;
use super::key::admit_raster_key;
use super::placement::UiGlyphRasterPlacement;
use crate::layout_artifact::UiQualifiedTextColorSource;
use crate::UiQualifiedTextLayout;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiGlyphRasterCandidate {
    pub(crate) key: worth_ui_host_contract::UiGlyphRasterKey,
    pub(crate) original_range: worth_ui_host_contract::UiTextOriginalRange,
    pub(crate) glyph_index: usize,
    pub(crate) positioned_glyph_index: usize,
    pub(crate) units_per_em: u16,
    pub(crate) ink_bounds: worth_ui_host_contract::UiTextFontUnitRect,
}

struct CandidateContext<'layout> {
    positioned: UiPositionedTextGlyphRecord,
    glyph: UiQualifiedTextGlyphRecord,
    run: UiQualifiedTextRunRecord,
    style: &'layout UiQualifiedTextStyleRecord,
    coverage: UiQualifiedTextCoverageRecord,
    glyph_index: usize,
    positioned_glyph_index: usize,
}

pub(crate) fn candidate_for_positioned(
    layout: &UiQualifiedTextLayout,
    positioned_glyph_index: usize,
    scale: UiGlyphRasterScale,
    placement: UiGlyphRasterPlacement,
) -> Result<Option<UiGlyphRasterCandidate>, UiGlyphRasterDemandDenial> {
    let Some(context) = context_for_positioned(layout, positioned_glyph_index)? else {
        return Ok(None);
    };
    let key = raster_key(layout, &context, scale, placement)?;
    Ok(Some(UiGlyphRasterCandidate {
        key,
        original_range: context.glyph.original_range(),
        glyph_index: context.glyph_index,
        positioned_glyph_index: context.positioned_glyph_index,
        units_per_em: context.run.units_per_em(),
        ink_bounds: context.glyph.ink_bounds_font_units(),
    }))
}

fn context_for_positioned<'layout>(
    layout: &'layout UiQualifiedTextLayout,
    positioned_glyph_index: usize,
) -> Result<Option<CandidateContext<'layout>>, UiGlyphRasterDemandDenial> {
    let positioned = layout
        .positioned_glyphs()
        .get(positioned_glyph_index)
        .ok_or(UiGlyphRasterDemandDenial::ForeignLayout)?;
    let glyph_index = usize::try_from(positioned.source_glyph_index())
        .map_err(|_| UiGlyphRasterDemandDenial::ForeignLayout)?;
    let glyph = layout
        .glyphs()
        .get(glyph_index)
        .ok_or(UiGlyphRasterDemandDenial::ForeignLayout)?;
    if glyph.original_range().is_empty() {
        return Ok(None);
    }
    let run = layout
        .view()
        .logical_runs()
        .iter()
        .copied()
        .find(|run| {
            run.glyph_range()
                .contains(&u32::try_from(glyph_index).unwrap_or(u32::MAX))
        })
        .ok_or(UiGlyphRasterDemandDenial::ForeignLayout)?;
    let style = layout
        .styles()
        .get(usize::from(run.style_index()))
        .ok_or(UiGlyphRasterDemandDenial::ForeignLayout)?;
    let coverage = coverage_for_glyph(layout, *glyph)?;
    if coverage.disposition() == UiTextCoverageDisposition::LayoutControl {
        return Ok(None);
    }
    let ink_bounds = glyph.ink_bounds_font_units();
    if ink_bounds.x_min() >= ink_bounds.x_max() || ink_bounds.y_min() >= ink_bounds.y_max() {
        return Ok(None);
    }
    Ok(Some(CandidateContext {
        positioned: *positioned,
        glyph: *glyph,
        run,
        style,
        coverage,
        glyph_index,
        positioned_glyph_index,
    }))
}

fn coverage_for_glyph(
    layout: &UiQualifiedTextLayout,
    glyph: UiQualifiedTextGlyphRecord,
) -> Result<UiQualifiedTextCoverageRecord, UiGlyphRasterDemandDenial> {
    let glyph_range = glyph.original_range();
    let mut matching = layout
        .artifact()
        .coverage()
        .iter()
        .copied()
        .filter(|coverage| {
            let range = coverage.original_range();
            range.end() > glyph_range.start() && range.start() < glyph_range.end()
        });
    let first = matching
        .next()
        .filter(|coverage| coverage.original_range().start() == glyph_range.start())
        .ok_or(UiGlyphRasterDemandDenial::MissingCoverage)?;
    let mut covered_end = first.original_range().end();
    for coverage in matching {
        if covered_end >= glyph_range.end() {
            return Err(UiGlyphRasterDemandDenial::MissingCoverage);
        }
        let range = coverage.original_range();
        if range.start() != covered_end
            || coverage.face() != first.face()
            || coverage.disposition() != first.disposition()
            || coverage.attempted_collection() != first.attempted_collection()
        {
            return Err(UiGlyphRasterDemandDenial::MissingCoverage);
        }
        covered_end = range.end();
    }
    if covered_end != glyph_range.end() {
        return Err(UiGlyphRasterDemandDenial::MissingCoverage);
    }
    Ok(first)
}

fn raster_key(
    layout: &UiQualifiedTextLayout,
    context: &CandidateContext<'_>,
    scale: UiGlyphRasterScale,
    placement: UiGlyphRasterPlacement,
) -> Result<worth_ui_host_contract::UiGlyphRasterKey, UiGlyphRasterDemandDenial> {
    let face_resource = layout
        .artifact()
        .face_resource(context.run.face())
        .ok_or(UiGlyphRasterDemandDenial::ForeignFace)?;
    let source = if context.coverage.disposition() == UiTextCoverageDisposition::MissingCluster {
        UiGlyphRasterSource::LastResort
    } else if let Some(source) = face_resource.color_source(context.glyph.glyph_id()) {
        match source {
            UiQualifiedTextColorSource::Outline => UiGlyphRasterSource::ColorOutline,
            UiQualifiedTextColorSource::Bitmap => UiGlyphRasterSource::ColorBitmap,
        }
    } else {
        UiGlyphRasterSource::AlphaOutline
    };
    let origin = fractional_origin(
        context
            .positioned
            .origin_x_millipoints()
            .checked_add(placement.origin_x_millipoints())
            .ok_or(UiGlyphRasterDemandDenial::OriginOverflow)?,
        context
            .positioned
            .origin_y_millipoints()
            .checked_add(placement.origin_y_millipoints())
            .ok_or(UiGlyphRasterDemandDenial::OriginOverflow)?,
        scale.dpi_milli(),
    )
    .ok_or(UiGlyphRasterDemandDenial::OriginOverflow)?;
    let size = worth_ui_host_contract::UiGlyphRasterSize::from_millipoints(
        context.style.font_size_millipoints(),
    )
    .ok_or(UiGlyphRasterDemandDenial::ForeignLayout)?;
    admit_raster_key(UiGlyphRasterKeyInput {
        font_collection: layout.view().font_collection_generation(),
        font_collection_lineage:
            worth_ui_host_contract::UiFontCollectionLineageIdentity::from_text_mechanics(
                layout.pinned_font_collection().identity_digest(),
            ),
        profile: layout.view().profile_generation(),
        face: context.run.face(),
        glyph_id: context.glyph.glyph_id(),
        variations: layout
            .pinned_font_collection()
            .raster_variations(context.run.face(), context.style)
            .ok_or(UiGlyphRasterDemandDenial::ForeignFace)?,
        palette: worth_ui_host_contract::UiGlyphRasterPalette::new(0),
        size,
        source,
        dpi_milli: scale.dpi_milli(),
        origin,
    })
    .map_err(UiGlyphRasterDemandDenial::Key)
}
