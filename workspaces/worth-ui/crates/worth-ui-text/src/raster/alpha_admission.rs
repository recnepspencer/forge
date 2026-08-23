//! Effect-free admission for the exact alpha/Last Resort miss set.

use std::collections::HashSet;

use worth_ui_host_contract::UiGlyphRasterKey;

use super::capacity::{MAX_BATCH_RECORDS, MAX_RASTER_EDGE, MAX_STAGED_BYTES};
use super::demand::UiGlyphRasterDemandBatch;
use super::denial::UiGlyphRasterizationDenial;
use super::qualified_raster_admission::{
    candidate_for_record, predicted_outline_extent, validate_demand,
};
use crate::UiQualifiedTextLayout;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAlphaRasterAdmission {
    unique_records: usize,
    predicted_bytes: u64,
    validation_checks: u32,
    provenance_checks: u32,
}

impl UiAlphaRasterAdmission {
    pub const fn unique_records(self) -> usize {
        self.unique_records
    }

    pub const fn predicted_bytes(self) -> u64 {
        self.predicted_bytes
    }

    pub const fn validation_checks(self) -> u32 {
        self.validation_checks
    }

    pub const fn provenance_checks(self) -> u32 {
        self.provenance_checks
    }
}

pub fn admit_alpha_outline(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
) -> Result<UiAlphaRasterAdmission, UiGlyphRasterizationDenial> {
    validate_demand(layout, demand)?;
    let mut unique_keys = HashSet::with_capacity(demand.records().len());
    let mut predicted_bytes = 0_u64;
    for (index, record) in demand.records().iter().copied().enumerate() {
        if !is_alpha_source(record.key()) || !unique_keys.insert(record.key()) {
            continue;
        }
        if unique_keys.len() > MAX_BATCH_RECORDS {
            return Err(UiGlyphRasterizationDenial::BatchCapacityExceeded);
        }
        let candidate = candidate_for_record(layout, demand, index, record)?;
        let (width, height) = predicted_outline_extent(&candidate, record.key())
            .ok_or(UiGlyphRasterizationDenial::ExtentExceeded)?;
        if width > MAX_RASTER_EDGE || height > MAX_RASTER_EDGE {
            return Err(UiGlyphRasterizationDenial::ExtentExceeded);
        }
        predicted_bytes = predicted_bytes
            .checked_add(u64::from(width) * u64::from(height))
            .ok_or(UiGlyphRasterizationDenial::StagedByteCapacityExceeded)?;
        if predicted_bytes > MAX_STAGED_BYTES {
            return Err(UiGlyphRasterizationDenial::StagedByteCapacityExceeded);
        }
    }
    Ok(UiAlphaRasterAdmission {
        unique_records: unique_keys.len(),
        predicted_bytes,
        validation_checks: u32::try_from(demand.records().len()).unwrap_or(u32::MAX),
        provenance_checks: u32::try_from(demand.provenance().len()).unwrap_or(u32::MAX),
    })
}

pub(super) fn is_alpha_source(key: UiGlyphRasterKey) -> bool {
    matches!(
        key.source(),
        worth_ui_host_contract::UiGlyphRasterSource::AlphaOutline
            | worth_ui_host_contract::UiGlyphRasterSource::LastResort
    )
}
