//! Text-owned glyph-raster demand derivation.
//!
//! Demand is derived from the qualified layout and mounted damage. Runtime
//! selects the layout and supplies inert paint/damage views; it cannot mint a
//! key or reinterpret a glyph. The private provenance beside each borrowed
//! record lets raster admission validate only the selected paragraph-local
//! records.

use std::collections::HashSet;

use worth_ui_host_contract::{
    UiGlyphRasterAttribution, UiGlyphRasterDemandBatchView, UiGlyphRasterDemandBatchViewInput,
    UiGlyphRasterDemandIdentity, UiGlyphRasterDemandRecord, UiGlyphRasterLane,
    UiMountedLogicalDamage, UiMountedTextForegroundSpan, UiQualifiedTextLayoutIdentity,
    UiTextOriginalRange, UiTextScaleGeneration,
};

use super::cost::{UiGlyphRasterLaneCost, UiGlyphRasterLaneCostInput};
use super::demand_candidate::candidate_for_positioned;
use super::demand_geometry::{contains_range, damage_intersects};
use super::demand_identity::demand_identity;
use super::key::UiGlyphRasterKeyDenial;
use super::placement::UiGlyphRasterPlacement;
use super::planning_geometry::predicted_extent;
use crate::{UiGlobalTextProfile, UiQualifiedTextLayout};
use worth_ui_host_contract::UiGlyphRasterKey;

const MAX_DEMAND_RECORDS: usize = UiGlobalTextProfile::MAX_GLYPHS;

struct DemandBatchAdmission {
    identity: UiGlyphRasterDemandIdentity,
    layout: UiQualifiedTextLayoutIdentity,
    scale: UiGlyphRasterScale,
    placement: UiGlyphRasterPlacement,
    lane: UiGlyphRasterLane,
    records: Vec<UiGlyphRasterDemandRecord>,
    provenance: Vec<UiGlyphRasterDemandProvenance>,
    lane_cost: UiGlyphRasterLaneCost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterScale {
    dpi_milli: u32,
    text_scale: UiTextScaleGeneration,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiGlyphRasterDemandRequest<'a> {
    pub paint_spans: &'a [UiMountedTextForegroundSpan],
    pub logical_damage: &'a [UiMountedLogicalDamage],
    pub scale: UiGlyphRasterScale,
    pub placement: UiGlyphRasterPlacement,
    pub lane: UiGlyphRasterLane,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterDemandDenial {
    Key(UiGlyphRasterKeyDenial),
    ZeroDpi,
    ForeignLayout,
    ForeignKey,
    LayoutScaleMismatch,
    ForeignCollectionLineage,
    DemandIdentityMismatch,
    DemandCapacityExceeded,
    PaintSpanMismatch,
    MissingCoverage,
    ForeignFace,
    OriginOverflow,
    RasterGeometryUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiGlyphRasterDemandProvenance {
    pub(crate) positioned_glyph_index: usize,
    pub(crate) glyph_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterDemandBatch {
    identity: UiGlyphRasterDemandIdentity,
    layout: UiQualifiedTextLayoutIdentity,
    scale: UiGlyphRasterScale,
    placement: UiGlyphRasterPlacement,
    lane: UiGlyphRasterLane,
    records: Box<[UiGlyphRasterDemandRecord]>,
    provenance: Box<[UiGlyphRasterDemandProvenance]>,
    cost: super::UiGlyphRasterCost,
}

impl UiGlyphRasterScale {
    pub const fn new(dpi_milli: u32, text_scale: UiTextScaleGeneration) -> Option<Self> {
        if dpi_milli == 0 {
            None
        } else {
            Some(Self {
                dpi_milli,
                text_scale,
            })
        }
    }

    pub const fn dpi_milli(self) -> u32 {
        self.dpi_milli
    }

    pub const fn text_scale_generation(self) -> UiTextScaleGeneration {
        self.text_scale
    }
}

impl UiGlyphRasterDemandBatch {
    pub(crate) fn from_text_mechanics(
        identity: UiGlyphRasterDemandIdentity,
        layout: UiQualifiedTextLayoutIdentity,
        scale: UiGlyphRasterScale,
        placement: UiGlyphRasterPlacement,
        lane: UiGlyphRasterLane,
        records: impl IntoIterator<Item = UiGlyphRasterDemandRecord>,
    ) -> Result<Self, UiGlyphRasterDemandDenial> {
        let records: Vec<_> = records.into_iter().collect();
        let expected = demand_identity(layout, scale, placement, lane, &records);
        if identity != expected {
            return Err(UiGlyphRasterDemandDenial::DemandIdentityMismatch);
        }
        Self::admit_records(DemandBatchAdmission {
            identity,
            layout,
            scale,
            placement,
            lane,
            records,
            provenance: Vec::new(),
            lane_cost: Default::default(),
        })
    }

    fn from_derived_records(derived: DerivedDemand) -> Result<Self, UiGlyphRasterDemandDenial> {
        let identity = demand_identity(
            derived.layout,
            derived.scale,
            derived.placement,
            derived.lane,
            &derived.records,
        );
        Self::admit_records(DemandBatchAdmission {
            identity,
            layout: derived.layout,
            scale: derived.scale,
            placement: derived.placement,
            lane: derived.lane,
            records: derived.records,
            provenance: derived.provenance,
            lane_cost: derived.cost,
        })
    }

    fn admit_records(input: DemandBatchAdmission) -> Result<Self, UiGlyphRasterDemandDenial> {
        let DemandBatchAdmission {
            identity,
            layout,
            scale,
            placement,
            lane,
            records,
            provenance,
            lane_cost,
        } = input;
        if records.len() > MAX_DEMAND_RECORDS {
            return Err(UiGlyphRasterDemandDenial::DemandCapacityExceeded);
        }
        if !provenance.is_empty() && provenance.len() != records.len() {
            return Err(UiGlyphRasterDemandDenial::DemandIdentityMismatch);
        }
        if records.iter().any(|record| {
            record.attribution().layout() != layout || record.key().dpi_milli() != scale.dpi_milli()
        }) {
            return Err(UiGlyphRasterDemandDenial::ForeignKey);
        }
        let cost = match lane {
            UiGlyphRasterLane::Ordinary => {
                super::UiGlyphRasterCost::from_text_mechanics(lane_cost, Default::default())
            }
            UiGlyphRasterLane::Reconstruction => {
                super::UiGlyphRasterCost::from_text_mechanics(Default::default(), lane_cost)
            }
        };
        Ok(Self {
            identity,
            layout,
            scale,
            placement,
            lane,
            records: records.into_boxed_slice(),
            provenance: provenance.into_boxed_slice(),
            cost,
        })
    }

    pub const fn identity(&self) -> UiGlyphRasterDemandIdentity {
        self.identity
    }

    pub const fn layout_identity(&self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }

    pub const fn scale(&self) -> UiGlyphRasterScale {
        self.scale
    }

    pub(crate) const fn placement(&self) -> UiGlyphRasterPlacement {
        self.placement
    }

    pub const fn lane(&self) -> UiGlyphRasterLane {
        self.lane
    }

    pub fn records(&self) -> &[UiGlyphRasterDemandRecord] {
        &self.records
    }

    pub(crate) fn provenance(&self) -> &[UiGlyphRasterDemandProvenance] {
        &self.provenance
    }

    pub const fn cost(&self) -> super::UiGlyphRasterCost {
        self.cost
    }

    pub fn as_view(&self) -> UiGlyphRasterDemandBatchView<'_> {
        UiGlyphRasterDemandBatchView::from_text_mechanics(UiGlyphRasterDemandBatchViewInput {
            identity: self.identity,
            layout: self.layout,
            dpi_milli: self.scale.dpi_milli,
            text_scale: self.scale.text_scale,
            lane: self.lane,
            records: &self.records,
        })
        .expect("admitted demand preserves a nonzero DPI")
    }
}

struct DerivedDemand {
    layout: UiQualifiedTextLayoutIdentity,
    scale: UiGlyphRasterScale,
    placement: UiGlyphRasterPlacement,
    lane: UiGlyphRasterLane,
    records: Vec<UiGlyphRasterDemandRecord>,
    provenance: Vec<UiGlyphRasterDemandProvenance>,
    cost: UiGlyphRasterLaneCost,
}

#[derive(Default)]
struct DemandCounters {
    layout_visits: u32,
    demanded_glyphs: u32,
    face_resource_lookups: u32,
    unique_keys: HashSet<UiGlyphRasterKey>,
}

impl DemandCounters {
    fn cost(self) -> UiGlyphRasterLaneCost {
        UiGlyphRasterLaneCost::from_text_mechanics(UiGlyphRasterLaneCostInput {
            layout_visits: self.layout_visits,
            outer_traversals: self.layout_visits,
            validation_checks: self.demanded_glyphs,
            provenance_checks: 0,
            demanded_glyphs: self.demanded_glyphs,
            face_resource_lookups: self.face_resource_lookups,
            outline_evaluations: 0,
            bitmap_source_evaluations: 0,
            retained_scans: 0,
            cache_hits: self
                .demanded_glyphs
                .saturating_sub(u32::try_from(self.unique_keys.len()).unwrap_or(u32::MAX)),
            cache_misses: 0,
            rasterized_glyphs: 0,
            rasterized_texels: 0,
            produced_bytes: 0,
        })
    }
}

/// Derive exact paragraph-local raster demand from the durable qualified
/// layout and the mounted damage/paint attribution views.
pub fn derive_glyph_raster_demand(
    layout: &UiQualifiedTextLayout,
    request: UiGlyphRasterDemandRequest<'_>,
) -> Result<UiGlyphRasterDemandBatch, UiGlyphRasterDemandDenial> {
    let derived = collect_demand(layout, request)?;
    UiGlyphRasterDemandBatch::from_derived_records(derived)
}

fn collect_demand(
    layout: &UiQualifiedTextLayout,
    request: UiGlyphRasterDemandRequest<'_>,
) -> Result<DerivedDemand, UiGlyphRasterDemandDenial> {
    if request.scale.dpi_milli() == 0 {
        return Err(UiGlyphRasterDemandDenial::ZeroDpi);
    }
    if request.scale.text_scale_generation() != layout.view().text_scale_generation() {
        return Err(UiGlyphRasterDemandDenial::LayoutScaleMismatch);
    }
    let (records, provenance, counters) = collect_demand_records(layout, request)?;
    Ok(DerivedDemand {
        layout: layout.identity(),
        scale: request.scale,
        placement: request.placement,
        lane: request.lane,
        records,
        provenance,
        cost: counters.cost(),
    })
}

fn collect_demand_records(
    layout: &UiQualifiedTextLayout,
    request: UiGlyphRasterDemandRequest<'_>,
) -> Result<
    (
        Vec<UiGlyphRasterDemandRecord>,
        Vec<UiGlyphRasterDemandProvenance>,
        DemandCounters,
    ),
    UiGlyphRasterDemandDenial,
> {
    let layout_identity = layout.identity();
    let mut records = Vec::new();
    let mut provenance = Vec::new();
    let mut counters = DemandCounters::default();

    for (positioned_index, positioned) in layout.positioned_glyphs().iter().enumerate() {
        counters.layout_visits = counters.layout_visits.saturating_add(1);
        if !damage_intersects(
            positioned.ink_bounds(),
            request.placement,
            request.logical_damage,
        ) {
            continue;
        }
        let Some(candidate) =
            candidate_for_positioned(layout, positioned_index, request.scale, request.placement)?
        else {
            continue;
        };
        let span_count = request
            .paint_spans
            .iter()
            .filter(|span| contains_range(span.original_range(), candidate.original_range))
            .count();
        if span_count != 1 {
            return Err(UiGlyphRasterDemandDenial::PaintSpanMismatch);
        }
        counters.face_resource_lookups = counters.face_resource_lookups.saturating_add(1);
        counters.demanded_glyphs = counters.demanded_glyphs.saturating_add(1);
        if records.len() == MAX_DEMAND_RECORDS {
            return Err(UiGlyphRasterDemandDenial::DemandCapacityExceeded);
        }
        let extent = predicted_extent(layout, &candidate)
            .map_err(|_| UiGlyphRasterDemandDenial::RasterGeometryUnavailable)?;
        records.push(demand_record(
            candidate.key,
            layout_identity,
            candidate.original_range,
            extent,
        ));
        provenance.push(UiGlyphRasterDemandProvenance {
            positioned_glyph_index: candidate.positioned_glyph_index,
            glyph_index: candidate.glyph_index,
        });
        counters.unique_keys.insert(candidate.key);
    }
    Ok((records, provenance, counters))
}

pub(crate) fn demand_record(
    key: UiGlyphRasterKey,
    layout: UiQualifiedTextLayoutIdentity,
    original_range: UiTextOriginalRange,
    extent: worth_ui_host_contract::UiGlyphRasterExtent,
) -> UiGlyphRasterDemandRecord {
    UiGlyphRasterDemandRecord::from_text_mechanics(
        key,
        UiGlyphRasterAttribution::from_text_mechanics(layout, original_range),
        extent,
    )
    .expect("qualified raster geometry has bounded staged bytes")
}
