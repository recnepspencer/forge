//! Qualified alpha-outline raster production.
//!
//! This module consumes only the exact face bytes pinned by a qualified
//! layout. It intentionally has no color-table, atlas, GPU, or presentation
//! path; intrinsic-color demand is owned by the sibling color-raster lane.

use std::collections::HashSet;
use swash::zeno::{Format, Vector};
use swash::{scale::image::Image, scale::*, FontRef, GlyphId};
use worth_ui_host_contract::{UiGlyphRasterDemandRecord, UiGlyphRasterKey, UiGlyphRasterSource};

use super::alpha_admission::{admit_alpha_outline, UiAlphaRasterAdmission};
use super::alpha_record::{build_raster_record, content_digest, validate_image};
use super::alpha_transaction_admission::{transaction_identity, UiAlphaRasterTransactionAdmission};
use super::alpha_transaction_completion::{
    complete_alpha_raster_transaction, UiAlphaRasterTransactionCompletion,
};
use super::batch::{UiAlphaRasterBatch, UiGlyphRasterRecord};
use super::capacity::MAX_STAGED_BYTES;
use super::cost::UiGlyphRasterLaneCostInput;
use super::demand::UiGlyphRasterDemandBatch;
use super::denial::UiGlyphRasterizationDenial;
use super::UiGlyphRasterCost;
use crate::UiQualifiedTextLayout;

pub struct UiAlphaRasterization {
    batch: UiAlphaRasterBatch,
    cost: UiGlyphRasterCost,
}

pub struct UiAlphaRasterTransaction {
    batches: Box<[UiAlphaRasterization]>,
    completion: UiAlphaRasterTransactionCompletion,
}

impl UiAlphaRasterTransaction {
    pub fn batches(&self) -> &[UiAlphaRasterization] {
        &self.batches
    }

    pub const fn completion(&self) -> &UiAlphaRasterTransactionCompletion {
        &self.completion
    }

    pub fn into_batches(self) -> Box<[UiAlphaRasterization]> {
        self.batches
    }
}

impl UiAlphaRasterization {
    pub fn batch(&self) -> &UiAlphaRasterBatch {
        &self.batch
    }

    pub const fn cost(&self) -> UiGlyphRasterCost {
        self.cost
    }

    pub fn into_parts(self) -> (UiAlphaRasterBatch, UiGlyphRasterCost) {
        (self.batch, self.cost)
    }
}

/// Rasterize the admitted alpha/Last Resort miss subset from exact layout
/// bytes. Repeated keys are rasterized once; their demand records remain
/// attributable outside raster equivalence.
pub fn rasterize_alpha_outline(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
) -> Result<UiAlphaRasterization, UiGlyphRasterizationDenial> {
    let admission = admit_alpha_outline(layout, demand)?;
    rasterize_candidates(layout, demand, admission, &mut HashSet::new(), None, None)
}

/// Rasterize only the keys admitted by the native atlas plan.  Demand and
/// provenance are still validated against the complete layout-owned batch;
/// the key filter controls which records are allowed to reach the renderer.
pub fn rasterize_alpha_outline_selection(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
    keys: &[UiGlyphRasterKey],
) -> Result<UiAlphaRasterization, UiGlyphRasterizationDenial> {
    let admission = admit_alpha_outline(layout, demand)?;
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
        None,
    )
}

pub fn rasterize_alpha_outline_selection_cached(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
    keys: &[UiGlyphRasterKey],
    cache: &mut super::UiGlyphRasterCache,
) -> Result<UiAlphaRasterization, UiGlyphRasterizationDenial> {
    let admission = admit_alpha_outline(layout, demand)?;
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
        Some(cache),
    )
}

/// Rasterizes an atomically admitted mounted transaction. Equal keys cross
/// the outline renderer once even when several layouts demand them.
pub fn rasterize_alpha_outline_transaction(
    batches: &[(&UiQualifiedTextLayout, &UiGlyphRasterDemandBatch)],
    admission: &UiAlphaRasterTransactionAdmission,
) -> Result<UiAlphaRasterTransaction, UiGlyphRasterizationDenial> {
    if admission.identity() != transaction_identity(batches)
        || usize::try_from(admission.demand_batches()).ok() != Some(batches.len())
    {
        return Err(UiGlyphRasterizationDenial::ForeignDemandRecord);
    }
    let mut rasterized =
        HashSet::with_capacity(usize::try_from(admission.unique_records()).unwrap_or(0));
    let batches = batches
        .iter()
        .map(|&(layout, demand)| {
            let batch_admission = admit_alpha_outline(layout, demand)?;
            rasterize_candidates(layout, demand, batch_admission, &mut rasterized, None, None)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();
    let completion = complete_alpha_raster_transaction(admission, &batches)?;
    Ok(UiAlphaRasterTransaction {
        batches,
        completion,
    })
}

fn rasterize_candidates(
    layout: &UiQualifiedTextLayout,
    demand: &UiGlyphRasterDemandBatch,
    admission: UiAlphaRasterAdmission,
    rasterized: &mut HashSet<UiGlyphRasterKey>,
    selected: Option<&HashSet<UiGlyphRasterKey>>,
    mut cache: Option<&mut super::UiGlyphRasterCache>,
) -> Result<UiAlphaRasterization, UiGlyphRasterizationDenial> {
    let mut records = Vec::with_capacity(admission.unique_records());
    let mut cached_records = Vec::with_capacity(admission.unique_records());
    let mut local_keys = HashSet::with_capacity(admission.unique_records());
    let mut transaction_cache_hits = 0_u32;
    for record in demand.records().iter().copied() {
        if selected.is_some_and(|keys| !keys.contains(&record.key())) {
            continue;
        }
        if !matches!(
            record.key().source(),
            UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort
        ) || !local_keys.insert(record.key())
        {
            continue;
        }
        if rasterized.insert(record.key()) {
            match cache
                .as_deref()
                .and_then(|cache| cache.alpha_record(record))
            {
                Some(Ok(cached)) => {
                    transaction_cache_hits = transaction_cache_hits.saturating_add(1);
                    cached_records.push(cached);
                }
                Some(Err(denial)) => {
                    return Err(UiGlyphRasterizationDenial::Record(denial));
                }
                None => records.push(record),
            }
        } else {
            transaction_cache_hits = transaction_cache_hits.saturating_add(1);
        }
    }
    let record_count = records.len();
    let mut job = AlphaRasterJob::new(layout);
    let mut raster_records = records
        .into_iter()
        .map(|record| job.rasterize(record))
        .collect::<Result<Vec<_>, _>>()?;
    if let Some(cache) = cache.as_deref_mut() {
        for record in &raster_records {
            cache.insert(record);
        }
    }
    raster_records.extend(cached_records);
    let batch = UiAlphaRasterBatch::from_text_mechanics(
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
    Ok(UiAlphaRasterization { batch, cost })
}

struct AlphaRasterJob<'layout> {
    layout: &'layout UiQualifiedTextLayout,
    context: ScaleContext,
    actual_bytes: u64,
    actual_texels: u64,
}

impl<'layout> AlphaRasterJob<'layout> {
    fn new(layout: &'layout UiQualifiedTextLayout) -> Self {
        Self {
            layout,
            context: ScaleContext::new(),
            actual_bytes: 0,
            actual_texels: 0,
        }
    }

    fn rasterize(
        &mut self,
        record: UiGlyphRasterDemandRecord,
    ) -> Result<UiGlyphRasterRecord<super::UiAlphaRasterKind>, UiGlyphRasterizationDenial> {
        let image = self.render_image(record.key())?;
        self.record_image(record, image)
    }

    fn render_image(&mut self, key: UiGlyphRasterKey) -> Result<Image, UiGlyphRasterizationDenial> {
        let resource = self
            .layout
            .artifact()
            .face_resource(key.face())
            .ok_or(UiGlyphRasterizationDenial::InvalidFaceResource)?;
        let face = FontRef::from_index(
            resource.bytes(),
            usize::try_from(key.face().face_index())
                .map_err(|_| UiGlyphRasterizationDenial::InvalidFaceResource)?,
        )
        .ok_or(UiGlyphRasterizationDenial::InvalidFaceResource)?;
        let glyph_id = u16::try_from(key.glyph_id())
            .map_err(|_| UiGlyphRasterizationDenial::InvalidGlyphId)?;
        let settings = key
            .variations()
            .records()
            .map(|variation| {
                (
                    swash::Tag::from_be_bytes(variation.axis()),
                    variation.value_milli() as f32 / 1_000.0,
                )
            })
            .collect::<Vec<_>>();
        let mut scaler = self
            .context
            .builder(face)
            .size(pixels_per_em(key))
            .hint(true)
            .variations(settings)
            .build();
        if !scaler.has_outlines() {
            return Err(UiGlyphRasterizationDenial::OutlineUnavailable);
        }
        let mut renderer = Render::new(&[Source::Outline]);
        renderer.format(Format::Alpha);
        renderer.offset(Vector::new(
            fractional_pixel(key.fractional_origin().x_over_64()),
            fractional_pixel(key.fractional_origin().y_over_64()),
        ));
        renderer
            .render(&mut scaler, GlyphId::from(glyph_id))
            .ok_or(UiGlyphRasterizationDenial::EmptyRaster)
    }

    fn record_image(
        &mut self,
        record: UiGlyphRasterDemandRecord,
        image: Image,
    ) -> Result<UiGlyphRasterRecord<super::UiAlphaRasterKind>, UiGlyphRasterizationDenial> {
        let shape = validate_image(&image)?;
        self.actual_bytes = self
            .actual_bytes
            .checked_add(u64::try_from(image.data.len()).unwrap_or(u64::MAX))
            .ok_or(UiGlyphRasterizationDenial::StagedByteCapacityExceeded)?;
        self.actual_texels = self
            .actual_texels
            .checked_add(u64::from(shape.width) * u64::from(shape.height))
            .ok_or(UiGlyphRasterizationDenial::StagedByteCapacityExceeded)?;
        if self.actual_bytes > MAX_STAGED_BYTES {
            return Err(UiGlyphRasterizationDenial::StagedByteCapacityExceeded);
        }
        let digest = content_digest(&image);
        build_raster_record(record, image, shape, digest)
    }

    fn cost_input(
        &self,
        record_count: usize,
        transaction_cache_hits: u32,
        admission: UiAlphaRasterAdmission,
    ) -> UiGlyphRasterLaneCostInput {
        let count = u32::try_from(record_count).unwrap_or(u32::MAX);
        UiGlyphRasterLaneCostInput {
            layout_visits: 0,
            outer_traversals: 0,
            validation_checks: admission.validation_checks(),
            provenance_checks: admission.provenance_checks(),
            demanded_glyphs: 0,
            face_resource_lookups: count,
            outline_evaluations: count,
            bitmap_source_evaluations: 0,
            retained_scans: 0,
            cache_hits: transaction_cache_hits,
            cache_misses: count,
            rasterized_glyphs: count,
            rasterized_texels: self.actual_texels,
            produced_bytes: self.actual_bytes,
        }
    }
}

fn pixels_per_em(key: UiGlyphRasterKey) -> f32 {
    (key.size().millipoints() as f64 * f64::from(key.dpi_milli()) / 1_000_000.0) as f32
}

fn fractional_pixel(value_over_64: i16) -> f32 {
    f32::from(value_over_64) / 64.0
}
