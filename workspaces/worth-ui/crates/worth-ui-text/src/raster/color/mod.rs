//! Text-owned qualified intrinsic-color raster production.

pub(crate) mod admission;
mod bitmap;
mod colr;
pub(crate) mod completion;
mod compositing;
#[cfg(test)]
mod compositing_tests;
mod image;
mod pixels;
mod transform;

#[cfg(test)]
mod tests;

pub(crate) use image::predicted_geometry;
#[cfg(test)]
mod transaction_tests;

use std::collections::HashSet;

use worth_ui_host_contract::{UiGlyphRasterDemandRecord, UiGlyphRasterKey};

use self::admission::{
    admit_intrinsic_color, is_color_source, transaction_identity, validate_color_source,
    UiColorRasterAdmission, UiColorRasterTransactionAdmission,
};
use self::completion::{complete_color_raster_transaction, UiColorRasterTransactionCompletion};
use self::image::render_color_image;
use super::batch::{
    UiColorRasterBatch, UiGlyphRasterAdmissionDenial, UiGlyphRasterRecord, UiGlyphRasterRecordInput,
};
use super::cost::UiGlyphRasterLaneCostInput;
use super::demand::UiGlyphRasterDemandBatch;
use super::denial::UiGlyphRasterizationDenial;
use super::qualified_raster_admission::candidate_for_record;
use super::source::UiColorRasterKind;
use super::UiGlyphRasterCost;
use crate::UiQualifiedTextLayout;

pub struct UiColorRasterization {
    batch: UiColorRasterBatch,
    cost: UiGlyphRasterCost,
}

pub struct UiColorRasterTransaction {
    batches: Box<[UiColorRasterization]>,
    completion: UiColorRasterTransactionCompletion,
}

impl UiColorRasterTransaction {
    pub fn batches(&self) -> &[UiColorRasterization] {
        &self.batches
    }

    pub const fn completion(&self) -> &UiColorRasterTransactionCompletion {
        &self.completion
    }

    pub fn into_batches(self) -> Box<[UiColorRasterization]> {
        self.batches
    }
}

impl UiColorRasterization {
    pub fn batch(&self) -> &UiColorRasterBatch {
        &self.batch
    }

    pub const fn cost(&self) -> UiGlyphRasterCost {
        self.cost
    }

    pub fn into_parts(self) -> (UiColorRasterBatch, UiGlyphRasterCost) {
        (self.batch, self.cost)
    }
}

pub fn rasterize_intrinsic_color(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
) -> Result<UiColorRasterization, UiGlyphRasterizationDenial> {
    let admission = admit_intrinsic_color(layout, demand)?;
    rasterize_candidates(layout, demand, admission, &mut HashSet::new(), None)
}

/// Rasterize only native-admitted color keys while retaining complete demand
/// and source validation against the runtime-owned layout.
pub fn rasterize_intrinsic_color_selection(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
    keys: &[UiGlyphRasterKey],
) -> Result<UiColorRasterization, UiGlyphRasterizationDenial> {
    let admission = admit_intrinsic_color(layout, demand)?;
    let selected = keys.iter().copied().collect::<HashSet<_>>();
    if selected
        .iter()
        .any(|key| !demand.records().iter().any(|record| record.key() == *key))
    {
        return Err(UiGlyphRasterizationDenial::ForeignDemandRecord);
    }
    rasterize_candidates(
        layout,
        demand,
        admission,
        &mut HashSet::new(),
        Some(&selected),
    )
}

pub fn rasterize_intrinsic_color_transaction(
    batches: &[(&UiQualifiedTextLayout, &UiGlyphRasterDemandBatch)],
    admission: &UiColorRasterTransactionAdmission,
) -> Result<UiColorRasterTransaction, UiGlyphRasterizationDenial> {
    if admission.identity() != transaction_identity(batches)
        || usize::try_from(admission.demand_batches()).ok() != Some(batches.len())
    {
        return Err(UiGlyphRasterizationDenial::ForeignDemandRecord);
    }
    let mut rasterized =
        HashSet::with_capacity(usize::try_from(admission.unique_records()).unwrap_or(0));
    let rasters = batches
        .iter()
        .map(|&(layout, demand)| {
            let batch_admission = admit_intrinsic_color(layout, demand)?;
            rasterize_candidates(layout, demand, batch_admission, &mut rasterized, None)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    let completion = complete_color_raster_transaction(admission, &rasters)?;
    Ok(UiColorRasterTransaction {
        batches: rasters,
        completion,
    })
}

fn rasterize_candidates(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
    admission: UiColorRasterAdmission,
    rasterized: &mut HashSet<UiGlyphRasterKey>,
    selected: Option<&HashSet<UiGlyphRasterKey>>,
) -> Result<UiColorRasterization, UiGlyphRasterizationDenial> {
    let mut records = Vec::with_capacity(admission.unique_records());
    let mut local_keys = HashSet::with_capacity(admission.unique_records());
    let mut transaction_cache_hits = 0_u32;
    for (index, record) in demand.records().iter().copied().enumerate() {
        if selected.is_some_and(|keys| !keys.contains(&record.key())) {
            continue;
        }
        if !is_color_source(record.key()) || !local_keys.insert(record.key()) {
            continue;
        }
        let candidate = candidate_for_record(layout, demand, index, record)?;
        validate_color_source(layout, record.key())?;
        if rasterized.insert(record.key()) {
            records.push((record, candidate));
        } else {
            transaction_cache_hits = transaction_cache_hits.saturating_add(1);
        }
    }
    let record_count = records.len();
    let mut job = ColorRasterJob::new(layout);
    let raster_records = records
        .into_iter()
        .map(|(record, candidate)| job.rasterize(record, candidate))
        .collect::<Result<Vec<_>, _>>()?;
    let batch = UiColorRasterBatch::from_text_mechanics(
        demand.identity(),
        layout.identity(),
        demand.scale(),
        demand.lane(),
        raster_records,
    )
    .map_err(UiGlyphRasterizationDenial::Record)?;
    let mut cost = demand.cost();
    cost.add_lane_work(
        demand.lane(),
        job.cost_input(record_count, transaction_cache_hits, admission),
    );
    Ok(UiColorRasterization { batch, cost })
}

struct ColorRasterJob<'layout> {
    layout: &'layout UiQualifiedTextLayout,
    actual_bytes: u64,
    actual_texels: u64,
    outline_evaluations: u32,
    bitmap_source_evaluations: u32,
}

impl<'layout> ColorRasterJob<'layout> {
    fn new(layout: &'layout UiQualifiedTextLayout) -> Self {
        Self {
            layout,
            actual_bytes: 0,
            actual_texels: 0,
            outline_evaluations: 0,
            bitmap_source_evaluations: 0,
        }
    }

    fn rasterize(
        &mut self,
        record: UiGlyphRasterDemandRecord,
        candidate: super::demand_candidate::UiGlyphRasterCandidate,
    ) -> Result<UiGlyphRasterRecord<UiColorRasterKind>, UiGlyphRasterizationDenial> {
        match record.key().source() {
            worth_ui_host_contract::UiGlyphRasterSource::ColorOutline => {
                self.outline_evaluations = self.outline_evaluations.saturating_add(1);
            }
            worth_ui_host_contract::UiGlyphRasterSource::ColorBitmap => {
                self.bitmap_source_evaluations = self.bitmap_source_evaluations.saturating_add(1);
            }
            _ => {}
        }
        let image = render_color_image(self.layout, &candidate, record.key())?;
        self.actual_bytes = self
            .actual_bytes
            .checked_add(u64::try_from(image.pixels.len()).unwrap_or(u64::MAX))
            .ok_or(UiGlyphRasterizationDenial::StagedByteCapacityExceeded)?;
        self.actual_texels = self
            .actual_texels
            .checked_add(u64::from(image.extent.width()) * u64::from(image.extent.height()))
            .ok_or(UiGlyphRasterizationDenial::StagedByteCapacityExceeded)?;
        if self.actual_bytes > super::capacity::MAX_STAGED_BYTES {
            return Err(UiGlyphRasterizationDenial::StagedByteCapacityExceeded);
        }
        UiGlyphRasterRecord::<UiColorRasterKind>::from_text_mechanics(UiGlyphRasterRecordInput {
            key: record.key(),
            attribution: record.attribution(),
            bearing: image.bearing,
            extent: image.extent,
            stride: image.extent.width() * 4,
            pixels: image.pixels,
            digest: image.digest,
        })
        .map_err(|denial| match denial {
            UiGlyphRasterAdmissionDenial::StagedByteCapacityExceeded => {
                UiGlyphRasterizationDenial::StagedByteCapacityExceeded
            }
            other => UiGlyphRasterizationDenial::Record(other),
        })
    }

    fn cost_input(
        &self,
        record_count: usize,
        transaction_cache_hits: u32,
        admission: UiColorRasterAdmission,
    ) -> UiGlyphRasterLaneCostInput {
        let count = u32::try_from(record_count).unwrap_or(u32::MAX);
        UiGlyphRasterLaneCostInput {
            layout_visits: 0,
            outer_traversals: 0,
            validation_checks: admission.validation_checks(),
            provenance_checks: admission.provenance_checks(),
            demanded_glyphs: 0,
            face_resource_lookups: count,
            outline_evaluations: self.outline_evaluations,
            bitmap_source_evaluations: self.bitmap_source_evaluations,
            retained_scans: 0,
            cache_hits: transaction_cache_hits,
            cache_misses: count,
            rasterized_glyphs: count,
            rasterized_texels: self.actual_texels,
            produced_bytes: self.actual_bytes,
        }
    }
}
