//! Shared qualified-demand and provenance admission for text raster lanes.

use worth_ui_host_contract::{UiGlyphRasterDemandRecord, UiGlyphRasterKey};

use super::capacity::{MAX_BATCH_RECORDS, MAX_RASTER_EDGE, MAX_STAGED_BYTES};
use super::demand::UiGlyphRasterDemandBatch;
use super::demand_candidate::{candidate_for_positioned, UiGlyphRasterCandidate};
use super::denial::UiGlyphRasterizationDenial;
use crate::UiQualifiedTextLayout;

pub(super) fn validate_demand(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
) -> Result<(), UiGlyphRasterizationDenial> {
    if demand.layout_identity() != layout.identity() {
        return Err(UiGlyphRasterizationDenial::ForeignLayout);
    }
    if demand.scale().text_scale_generation() != layout.view().text_scale_generation() {
        return Err(UiGlyphRasterizationDenial::ForeignScale);
    }
    if demand.records().len() > MAX_BATCH_RECORDS {
        return Err(UiGlyphRasterizationDenial::BatchCapacityExceeded);
    }
    let lineage = layout.pinned_font_collection().identity_digest();
    for record in demand.records().iter().copied() {
        validate_record_metadata(layout, demand, lineage, record)?;
    }
    validate_provenance(layout, demand)
}

pub(super) fn candidate_for_record(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
    index: usize,
    record: UiGlyphRasterDemandRecord,
) -> Result<UiGlyphRasterCandidate, UiGlyphRasterizationDenial> {
    let provenance = demand
        .provenance()
        .get(index)
        .ok_or(UiGlyphRasterizationDenial::MissingProvenance)?;
    let candidate = candidate_for_positioned(
        layout,
        provenance.positioned_glyph_index,
        demand.scale(),
        demand.placement(),
    )
    .map_err(UiGlyphRasterizationDenial::Demand)?
    .ok_or(UiGlyphRasterizationDenial::ForeignDemandRecord)?;
    if candidate.key != record.key()
        || candidate.original_range != record.attribution().original_range()
        || super::planning_geometry::predicted_extent(layout, &candidate)? != record.extent()
    {
        return Err(UiGlyphRasterizationDenial::ForeignDemandRecord);
    }
    Ok(candidate)
}

pub(super) fn predicted_outline_extent(
    candidate: &UiGlyphRasterCandidate,
    key: UiGlyphRasterKey,
) -> Option<(u32, u32)> {
    let width_units = u128::try_from(
        i64::from(candidate.ink_bounds.x_max()) - i64::from(candidate.ink_bounds.x_min()),
    )
    .ok()?;
    let height_units = u128::try_from(
        i64::from(candidate.ink_bounds.y_max()) - i64::from(candidate.ink_bounds.y_min()),
    )
    .ok()?;
    if width_units == 0 || height_units == 0 || candidate.units_per_em == 0 {
        return None;
    }
    let numerator = u128::from(key.size().millipoints()) * u128::from(key.dpi_milli());
    let denominator = u128::from(candidate.units_per_em) * 1_000_000;
    let width = ceil_ratio(width_units * numerator, denominator)?.saturating_add(2);
    let height = ceil_ratio(height_units * numerator, denominator)?.saturating_add(2);
    Some((u32::try_from(width).ok()?, u32::try_from(height).ok()?))
}

pub(super) fn admit_extent(
    width: u32,
    height: u32,
    staged_bytes: &mut u64,
) -> Result<(), UiGlyphRasterizationDenial> {
    if width == 0 || height == 0 || width > MAX_RASTER_EDGE || height > MAX_RASTER_EDGE {
        return Err(UiGlyphRasterizationDenial::ExtentExceeded);
    }
    let bytes = u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or(UiGlyphRasterizationDenial::StagedByteCapacityExceeded)?;
    *staged_bytes = staged_bytes
        .checked_add(bytes)
        .ok_or(UiGlyphRasterizationDenial::StagedByteCapacityExceeded)?;
    if *staged_bytes > MAX_STAGED_BYTES {
        return Err(UiGlyphRasterizationDenial::StagedByteCapacityExceeded);
    }
    Ok(())
}

fn validate_record_metadata(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
    lineage: [u8; 32],
    record: UiGlyphRasterDemandRecord,
) -> Result<(), UiGlyphRasterizationDenial> {
    let view = layout.view();
    let key = record.key();
    if key.font_collection_generation() != view.font_collection_generation()
        || key.font_collection_lineage().digest() != lineage
    {
        return Err(UiGlyphRasterizationDenial::ForeignCollectionLineage);
    }
    if key.profile_generation() != view.profile_generation() {
        return Err(UiGlyphRasterizationDenial::ForeignProfile);
    }
    if key.dpi_milli() != demand.scale().dpi_milli() {
        return Err(UiGlyphRasterizationDenial::ForeignScale);
    }
    if record.attribution().layout() != layout.identity() {
        return Err(UiGlyphRasterizationDenial::ForeignLayout);
    }
    Ok(())
}

fn validate_provenance(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
) -> Result<(), UiGlyphRasterizationDenial> {
    if !demand.provenance().is_empty() && demand.provenance().len() != demand.records().len() {
        return Err(UiGlyphRasterizationDenial::MissingProvenance);
    }
    if !demand.records().is_empty() && demand.provenance().is_empty() {
        return Err(UiGlyphRasterizationDenial::MissingProvenance);
    }
    for (index, record) in demand.records().iter().copied().enumerate() {
        let provenance = demand
            .provenance()
            .get(index)
            .ok_or(UiGlyphRasterizationDenial::MissingProvenance)?;
        let candidate = candidate_for_positioned(
            layout,
            provenance.positioned_glyph_index,
            demand.scale(),
            demand.placement(),
        )
        .map_err(UiGlyphRasterizationDenial::Demand)?
        .ok_or(UiGlyphRasterizationDenial::ForeignDemandRecord)?;
        if candidate.glyph_index != provenance.glyph_index
            || candidate.key != record.key()
            || candidate.original_range != record.attribution().original_range()
        {
            return Err(UiGlyphRasterizationDenial::ForeignDemandRecord);
        }
    }
    Ok(())
}

fn ceil_ratio(numerator: u128, denominator: u128) -> Option<u128> {
    numerator
        .checked_add(denominator.checked_sub(1)?)
        .map(|value| value / denominator)
}
