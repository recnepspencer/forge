//! Start the native text-presentation transaction from mounted authority.

use std::num::NonZeroU32;

use worth_ui_host_contract::{
    UiMountedLogicalDamage, UiMountedPaintCommand, UiMountedPaintCommandChange,
    UiMountedPaintCommandIdentity, UiMountedPresentationWorkView, UiMountedSemanticTextMechanic,
    UiMountedSurfaceBindingRequirement,
};
use worth_ui_text::{
    derive_glyph_raster_demand, UiGlyphRasterDemandBatch, UiGlyphRasterDemandDenial,
    UiGlyphRasterDemandRequest, UiGlyphRasterLane, UiGlyphRasterPlacement, UiGlyphRasterScale,
};

use super::rasterization::UiNativeTextRasterWorkReport;

type MountedSemanticTextCommand<'work> = (
    UiMountedPaintCommandIdentity,
    &'work UiMountedSemanticTextMechanic,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeTextPresentationReadiness {
    SemanticTextLayoutMismatch,
    SemanticTextSourceMismatch,
    SemanticTextDemandDenied(UiGlyphRasterDemandDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeTextDemandInspection {
    demand_batches: u32,
    demand_records: u32,
    key_checks: u32,
}

impl UiNativeTextDemandInspection {
    pub(crate) const fn demand_batches(self) -> u32 {
        self.demand_batches
    }

    pub(crate) const fn demand_records(self) -> u32 {
        self.demand_records
    }

    pub(crate) const fn key_checks(self) -> u32 {
        self.key_checks
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiMountedEventTimeDpiAuthority(NonZeroU32);

impl UiMountedEventTimeDpiAuthority {
    pub(crate) fn from_requirement(
        requirement: UiMountedSurfaceBindingRequirement,
    ) -> Option<Self> {
        NonZeroU32::new(requirement.device_scale_milli()).map(Self)
    }

    pub(crate) const fn dpi_milli(self) -> u32 {
        self.0.get()
    }
}

pub(crate) enum UiNativeTextPresentationPreparation {
    Prepared(UiNativeTextPresentationPrepared),
    Denied(UiNativeTextPresentationDenial),
}

pub(crate) struct UiNativeTextPresentationPrepared {
    layout_count: usize,
    paint_span_count: usize,
    demand_batches: Box<[UiGlyphRasterDemandBatch]>,
    pin_commands: Box<[UiMountedPaintCommandIdentity]>,
    pin_removals: Box<[UiMountedPaintCommandIdentity]>,
    pin_set_complete: bool,
    planning: Option<UiNativeTextDemandInspection>,
    raster_work: UiNativeTextRasterWorkReport,
}

pub(crate) struct UiNativeTextPresentationDenial {
    layout_count: usize,
    paint_span_count: usize,
    readiness: UiNativeTextPresentationReadiness,
}

pub(crate) fn prepare_mounted_semantic_text<'work>(
    work: UiMountedPresentationWorkView<'work>,
    dpi: UiMountedEventTimeDpiAuthority,
    resolve: impl Fn(
        worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
    ) -> Option<&'work worth_ui_text::UiQualifiedTextLayout>,
) -> Option<UiNativeTextPresentationPreparation> {
    let pin_work = mounted_semantic_text(work);
    // A delta removal carries only its command identity.  Preserve it here so
    // the runtime's committed command-to-pin owner can decide whether it is a
    // text release.  Arbitrary non-text removals remain insufficient because
    // the mounted text coordinator rejects zero-demand candidates that do not
    // change committed text ownership.
    if pin_work.mechanics.is_empty() && pin_work.removals.is_empty() {
        return None;
    }
    let lane = lane_for(work);
    let join = MountedTextDemandJoin {
        dpi,
        lane,
        damage: logical_damage(work),
        resolve,
    };
    Some(match prepare_demands(&pin_work.mechanics, &join) {
        Ok(demands) => inspect_demand_boundary(&pin_work, demands),
        Err(readiness) => denied_preparation(&pin_work.mechanics, readiness),
    })
}

struct PreparedDemand {
    demands: Box<[UiGlyphRasterDemandBatch]>,
}

fn prepare_demands<'work, Resolve>(
    mechanics: &[MountedSemanticTextCommand<'_>],
    join: &MountedTextDemandJoin<'work, Resolve>,
) -> Result<PreparedDemand, UiNativeTextPresentationReadiness>
where
    Resolve: Fn(
        worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
    ) -> Option<&'work worth_ui_text::UiQualifiedTextLayout>,
{
    let layouts = mechanics
        .iter()
        .map(|(_, mechanic)| join.layout_for(mechanic))
        .collect::<Result<Vec<_>, _>>()?;
    let demands = mechanics
        .iter()
        .zip(&layouts)
        .map(|((_, mechanic), layout)| join.demand_for(layout, mechanic))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PreparedDemand {
        demands: demands.into_boxed_slice(),
    })
}

fn inspect_demand_boundary(
    pin_work: &MountedSemanticTextWork<'_>,
    prepared: PreparedDemand,
) -> UiNativeTextPresentationPreparation {
    let demand_batches = u32::try_from(prepared.demands.len()).unwrap_or(u32::MAX);
    let demand_records = prepared
        .demands
        .iter()
        .map(|demand| u32::try_from(demand.records().len()).unwrap_or(u32::MAX))
        .fold(0_u32, u32::saturating_add);
    let inspection = UiNativeTextDemandInspection {
        demand_batches,
        demand_records,
        key_checks: demand_records,
    };
    UiNativeTextPresentationPreparation::Prepared(UiNativeTextPresentationPrepared {
        layout_count: prepared.demands.len(),
        paint_span_count: paint_span_count(&pin_work.mechanics),
        demand_batches: prepared.demands,
        pin_commands: pin_work
            .mechanics
            .iter()
            .map(|(identity, _)| *identity)
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        pin_removals: pin_work.removals.clone().into_boxed_slice(),
        pin_set_complete: pin_work.complete,
        planning: Some(inspection),
        raster_work: UiNativeTextRasterWorkReport::not_admitted(),
    })
}

impl UiNativeTextPresentationPrepared {
    pub(crate) fn layout_count(&self) -> usize {
        self.layout_count
    }

    pub(crate) fn paint_span_count(&self) -> usize {
        self.paint_span_count
    }

    pub(crate) fn demand_batches(&self) -> &[UiGlyphRasterDemandBatch] {
        &self.demand_batches
    }

    pub(crate) fn pin_commands(&self) -> &[UiMountedPaintCommandIdentity] {
        &self.pin_commands
    }

    pub(crate) fn pin_removals(&self) -> &[UiMountedPaintCommandIdentity] {
        &self.pin_removals
    }

    pub(crate) const fn pin_set_complete(&self) -> bool {
        self.pin_set_complete
    }

    pub(crate) const fn planning_inspection(&self) -> Option<UiNativeTextDemandInspection> {
        self.planning
    }

    pub(crate) const fn raster_work(&self) -> UiNativeTextRasterWorkReport {
        self.raster_work
    }
}

impl UiNativeTextPresentationDenial {
    pub(crate) const fn readiness(&self) -> UiNativeTextPresentationReadiness {
        self.readiness
    }
}

/// Joins mounted mechanics to durable layouts under the event-time DPI basis.
/// Demand preparation performs no host/native effect.
struct MountedTextDemandJoin<'work, Resolve> {
    dpi: UiMountedEventTimeDpiAuthority,
    lane: UiGlyphRasterLane,
    damage: &'work [UiMountedLogicalDamage],
    resolve: Resolve,
}

impl<'work, Resolve> MountedTextDemandJoin<'work, Resolve>
where
    Resolve: Fn(
        worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
    ) -> Option<&'work worth_ui_text::UiQualifiedTextLayout>,
{
    fn layout_for(
        &self,
        mechanic: &UiMountedSemanticTextMechanic,
    ) -> Result<&'work worth_ui_text::UiQualifiedTextLayout, UiNativeTextPresentationReadiness>
    {
        let layout = (self.resolve)(mechanic.qualified_layout_identity())
            .ok_or(UiNativeTextPresentationReadiness::SemanticTextLayoutMismatch)?;
        validate_mounted_layout(layout, mechanic)?;
        Ok(layout)
    }

    fn demand_for(
        &self,
        layout: &worth_ui_text::UiQualifiedTextLayout,
        mechanic: &UiMountedSemanticTextMechanic,
    ) -> Result<UiGlyphRasterDemandBatch, UiNativeTextPresentationReadiness> {
        let scale =
            UiGlyphRasterScale::new(self.dpi.dpi_milli(), mechanic.qualified_layout_scale())
                .ok_or(UiNativeTextPresentationReadiness::SemanticTextDemandDenied(
                    UiGlyphRasterDemandDenial::ZeroDpi,
                ))?;
        let demand = derive_glyph_raster_demand(
            layout,
            UiGlyphRasterDemandRequest {
                paint_spans: mechanic.foregrounds(),
                logical_damage: self.damage,
                scale,
                placement: UiGlyphRasterPlacement::from_mounted_logical(
                    mechanic.origin_x(),
                    mechanic.origin_y(),
                )
                .ok_or(
                    UiNativeTextPresentationReadiness::SemanticTextDemandDenied(
                        UiGlyphRasterDemandDenial::OriginOverflow,
                    ),
                )?,
                lane: self.lane,
            },
        )
        .map_err(UiNativeTextPresentationReadiness::SemanticTextDemandDenied)?;
        Ok(demand)
    }
}

fn validate_mounted_layout(
    layout: &worth_ui_text::UiQualifiedTextLayout,
    mechanic: &UiMountedSemanticTextMechanic,
) -> Result<(), UiNativeTextPresentationReadiness> {
    if layout.identity() != mechanic.qualified_layout_identity()
        || layout.view().request_identity() != mechanic.qualified_layout_request()
        || layout.view().profile_generation() != mechanic.qualified_layout_profile()
        || layout.view().font_collection_generation() != mechanic.qualified_layout_fonts()
        || layout.view().text_scale_generation() != mechanic.qualified_layout_scale()
    {
        return Err(UiNativeTextPresentationReadiness::SemanticTextLayoutMismatch);
    }
    if layout.source() != mechanic.text() {
        return Err(UiNativeTextPresentationReadiness::SemanticTextSourceMismatch);
    }
    Ok(())
}

fn denied_preparation(
    mechanics: &[MountedSemanticTextCommand<'_>],
    readiness: UiNativeTextPresentationReadiness,
) -> UiNativeTextPresentationPreparation {
    UiNativeTextPresentationPreparation::Denied(UiNativeTextPresentationDenial {
        layout_count: 0,
        paint_span_count: paint_span_count(mechanics),
        readiness,
    })
}

fn paint_span_count(mechanics: &[MountedSemanticTextCommand<'_>]) -> usize {
    mechanics
        .iter()
        .map(|(_, mechanic)| mechanic.foregrounds().len())
        .sum()
}

fn lane_for(work: UiMountedPresentationWorkView<'_>) -> UiGlyphRasterLane {
    match work {
        UiMountedPresentationWorkView::Reconstruction(_) => UiGlyphRasterLane::Reconstruction,
        UiMountedPresentationWorkView::Initial(_)
        | UiMountedPresentationWorkView::Delta(_)
        | UiMountedPresentationWorkView::Unchanged(_) => UiGlyphRasterLane::Ordinary,
    }
}

struct MountedSemanticTextWork<'work> {
    mechanics: Vec<MountedSemanticTextCommand<'work>>,
    removals: Vec<UiMountedPaintCommandIdentity>,
    complete: bool,
}

fn mounted_semantic_text(work: UiMountedPresentationWorkView<'_>) -> MountedSemanticTextWork<'_> {
    match work {
        UiMountedPresentationWorkView::Initial(initial) => complete_text_work(initial.commands()),
        UiMountedPresentationWorkView::Reconstruction(reconstruction) => {
            complete_text_work(reconstruction.commands())
        }
        UiMountedPresentationWorkView::Delta(delta) => {
            let mut mechanics = Vec::new();
            let mut removals = Vec::new();
            for change in delta.changes() {
                match change {
                    UiMountedPaintCommandChange::Insert(command)
                    | UiMountedPaintCommandChange::Replace(command) => {
                        if let Some(mechanic) = semantic_text_mechanic(command) {
                            mechanics.push((command.identity(), mechanic));
                        } else {
                            removals.push(command.identity());
                        }
                    }
                    UiMountedPaintCommandChange::Remove(identity) => removals.push(*identity),
                }
            }
            MountedSemanticTextWork {
                mechanics,
                removals,
                complete: false,
            }
        }
        UiMountedPresentationWorkView::Unchanged(_) => MountedSemanticTextWork {
            mechanics: Vec::new(),
            removals: Vec::new(),
            complete: false,
        },
    }
}

fn complete_text_work(commands: &[UiMountedPaintCommand]) -> MountedSemanticTextWork<'_> {
    MountedSemanticTextWork {
        mechanics: commands
            .iter()
            .filter_map(|command| {
                semantic_text_mechanic(command).map(|mechanic| (command.identity(), mechanic))
            })
            .collect(),
        removals: Vec::new(),
        complete: true,
    }
}

fn semantic_text_mechanic(
    command: &UiMountedPaintCommand,
) -> Option<&UiMountedSemanticTextMechanic> {
    match command {
        UiMountedPaintCommand::SemanticText { mechanic, .. } => Some(mechanic),
        UiMountedPaintCommand::FilledRect { .. } => None,
    }
}

fn logical_damage(work: UiMountedPresentationWorkView<'_>) -> &[UiMountedLogicalDamage] {
    match work {
        UiMountedPresentationWorkView::Initial(initial) => initial.damage(),
        UiMountedPresentationWorkView::Delta(delta) => delta.damage(),
        UiMountedPresentationWorkView::Reconstruction(reconstruction) => reconstruction.damage(),
        UiMountedPresentationWorkView::Unchanged(_) => &[],
    }
}

#[cfg(test)]
#[path = "preparation_tests.rs"]
mod tests;
