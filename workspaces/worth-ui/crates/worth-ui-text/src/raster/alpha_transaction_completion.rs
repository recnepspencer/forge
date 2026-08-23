//! Proof that alpha raster production exactly completed an admitted transaction.

use std::collections::HashSet;

use worth_ui_host_contract::{
    UiGlyphRasterAttribution, UiGlyphRasterBatchIdentity, UiGlyphRasterDemandIdentity,
    UiGlyphRasterKey,
};

use super::alpha::UiAlphaRasterization;
use super::alpha_transaction_admission::UiAlphaRasterTransactionAdmission;
use super::capacity::MAX_STAGED_BYTES;
use super::denial::UiGlyphRasterizationDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAlphaRasterBatchCompletion {
    demand: UiGlyphRasterDemandIdentity,
    miss: UiGlyphRasterDemandIdentity,
    batch: UiGlyphRasterBatchIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAlphaRasterTransactionCompletion {
    admission_identity: [u8; 32],
    batches: Box<[UiAlphaRasterBatchCompletion]>,
    unique_records: u32,
    actual_bytes: u64,
}

impl UiAlphaRasterBatchCompletion {
    pub const fn demand_identity(self) -> UiGlyphRasterDemandIdentity {
        self.demand
    }

    pub const fn miss_identity(self) -> UiGlyphRasterDemandIdentity {
        self.miss
    }

    pub const fn batch_identity(self) -> UiGlyphRasterBatchIdentity {
        self.batch
    }
}

impl UiAlphaRasterTransactionCompletion {
    pub const fn admission_identity(&self) -> [u8; 32] {
        self.admission_identity
    }

    pub fn batches(&self) -> &[UiAlphaRasterBatchCompletion] {
        &self.batches
    }

    pub const fn unique_records(&self) -> u32 {
        self.unique_records
    }

    pub const fn actual_bytes(&self) -> u64 {
        self.actual_bytes
    }
}

pub(super) fn complete_alpha_raster_transaction(
    admission: &UiAlphaRasterTransactionAdmission,
    rasters: &[UiAlphaRasterization],
) -> Result<UiAlphaRasterTransactionCompletion, UiGlyphRasterizationDenial> {
    if usize::try_from(admission.demand_batches()).ok() != Some(rasters.len()) {
        return Err(UiGlyphRasterizationDenial::TransactionOutputMismatch);
    }
    if rasters.iter().enumerate().any(|(index, raster)| {
        let batch = raster.batch();
        admission.expected_batch(index).is_none_or(|expected| {
            batch.demand_identity() != expected.demand
                || batch.layout_identity() != expected.layout
                || batch.scale() != expected.scale
                || batch.lane() != expected.lane
        })
    }) {
        return Err(UiGlyphRasterizationDenial::TransactionOutputMismatch);
    }
    let mut batches = Vec::with_capacity(rasters.len());
    for raster in rasters {
        let batch = raster.batch();
        batches.push(UiAlphaRasterBatchCompletion {
            demand: batch.demand_identity(),
            miss: batch.miss_identity(),
            batch: batch.batch_identity(),
        });
    }
    let records = rasters
        .iter()
        .enumerate()
        .flat_map(|(batch_index, raster)| {
            raster.batch().records().iter().map(move |record| {
                (
                    batch_index,
                    record.key(),
                    record.attribution(),
                    u64::try_from(record.pixels().len()).unwrap_or(u64::MAX),
                )
            })
        });
    let actual_bytes = validate_produced_keys(admission, records)?;
    Ok(UiAlphaRasterTransactionCompletion {
        admission_identity: admission.identity(),
        batches: batches.into_boxed_slice(),
        unique_records: admission.unique_records(),
        actual_bytes,
    })
}

pub(super) fn validate_produced_keys(
    admission: &UiAlphaRasterTransactionAdmission,
    records: impl IntoIterator<Item = (usize, UiGlyphRasterKey, UiGlyphRasterAttribution, u64)>,
) -> Result<u64, UiGlyphRasterizationDenial> {
    let mut produced =
        HashSet::with_capacity(usize::try_from(admission.unique_records()).unwrap_or_default());
    let mut actual_bytes = 0_u64;
    for (batch_index, key, attribution, bytes) in records {
        if !admission.admits_key(key)
            || admission.expected_attribution(batch_index, key) != Some(attribution)
            || !produced.insert(key)
        {
            return Err(UiGlyphRasterizationDenial::TransactionOutputMismatch);
        }
        actual_bytes = actual_bytes
            .checked_add(bytes)
            .ok_or(UiGlyphRasterizationDenial::StagedByteCapacityExceeded)?;
    }
    let complete = produced.len() == admission.admitted_keys().len()
        && admission
            .admitted_keys()
            .iter()
            .all(|key| produced.contains(key));
    if !complete {
        return Err(UiGlyphRasterizationDenial::TransactionOutputMismatch);
    }
    if actual_bytes > admission.predicted_bytes() || actual_bytes > MAX_STAGED_BYTES {
        return Err(UiGlyphRasterizationDenial::StagedByteCapacityExceeded);
    }
    Ok(actual_bytes)
}
