use worth_ui_host_contract::{
    UiMountedClipProjection, UiMountedLayerProjection, UiMountedLayerReference, UiMountedLayerRow,
    UiMountedPaintBatchReference, UiMountedPaintBatchRow, UiMountedPaintPrimitiveKind,
    UiMountedRealtimeBatchRow, UiMountedResourceEntry, UiMountedResourceKind,
    UiMountedResourceReference, UiMountedSpatialBatchRow,
};

use super::UiMountedProjectionDenial;

pub(in crate::mounting) mod diagnostic_source;
mod drawable_order;
mod lane_recording;
mod layout_reconstruction;
mod mechanic_source;
#[cfg(test)]
pub(crate) mod mechanic_source_tests;
mod node_changes;
mod presentation_effects;
pub(crate) mod presentation_sources;
mod rebind;
mod semantic_mechanics;
mod semantic_projection;
mod view;

use diagnostic_source::UiMountedDiagnosticSource;
use lane_recording::require_once;
use mechanic_source::{UiMountedMechanicCompletion, UiMountedMechanicSource};
use presentation_effects::{
    UiMountedPresentationEffectCompletion, UiMountedPresentationEffectSource,
};
pub(in crate::mounting) use semantic_projection::UiMountedSemanticProjection;
pub(super) use semantic_projection::{UiMountedProjectionNodeRecord, UiMountedProjectionSurface};
use view::{UiMountedOrdinaryPaintSelector, UiMountedPlanIndexPaintSelector};

const TABLE_LIMIT: usize = 2_048;
const RESOURCE_LIMIT: usize = 1_024;

#[derive(Clone)]
pub struct UiMountedProjectionFrame {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    content_generation: worth_ui_host_contract::UiMountedContentGeneration,
    receipt_basis: super::super::UiMountedNodeReceiptBasis,
    plan_digest: u64,
    semantic: UiMountedSemanticProjection,
    mechanics: UiMountedMechanicSource,
    presentation_effects: UiMountedPresentationEffectSource,
    diagnostics: UiMountedDiagnosticSource,
    changed_instances: std::rc::Rc<[worth_ui_host_contract::UiMountedInstanceIdentity]>,
    precise_command_instances: std::rc::Rc<[worth_ui_host_contract::UiMountedInstanceIdentity]>,
    presentation_command_changes:
        std::rc::Rc<[worth_ui_host_contract::UiMountedPaintCommandChange]>,
    paint_batches: Vec<UiMountedPaintBatchRow>,
    layers: Vec<UiMountedLayerRow>,
    spatial_batches: Vec<UiMountedSpatialBatchRow>,
    realtime_batches: Vec<UiMountedRealtimeBatchRow>,
    resources: Vec<UiMountedResourceEntry>,
    ordinary_recorded: bool,
    virtualized_recorded: bool,
    canvas_recorded: bool,
    realtime_recorded: bool,
    ordinary_paint_selector: Option<UiMountedOrdinaryPaintSelector>,
    plan_index_paint_selectors: Vec<UiMountedPlanIndexPaintSelector>,
    preview: Option<super::lowering::UiMountedPreviewProjectionInput>,
    visual_overlay: Option<super::super::UiMountedVisualOverlayProjectionInput>,
    counters: super::super::UiMountStageCounters,
    capability_generation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    font_collection: std::sync::Arc<worth_ui_text::UiGlobalFontCollection>,
    materialized_projection_rows: std::rc::Rc<std::cell::Cell<u64>>,
}

pub(super) struct UiMountedProjectionFrameInput {
    pub frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub content_generation: worth_ui_host_contract::UiMountedContentGeneration,
    pub receipt_basis: super::super::UiMountedNodeReceiptBasis,
    pub plan_digest: u64,
    pub semantic: UiMountedSemanticProjection,
    pub counters: super::super::UiMountStageCounters,
    pub capability_generation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    pub capability_profile_digest: u64,
    pub font_collection: std::sync::Arc<worth_ui_text::UiGlobalFontCollection>,
    pub mechanics: UiMountedMechanicSource,
    pub presentation_effects: UiMountedPresentationEffectSource,
    pub diagnostics: UiMountedDiagnosticSource,
    pub changed_instances: std::rc::Rc<[worth_ui_host_contract::UiMountedInstanceIdentity]>,
}

impl UiMountedProjectionFrame {
    pub(super) fn new(input: UiMountedProjectionFrameInput) -> Self {
        Self {
            frame: input.frame,
            content_generation: input.content_generation,
            receipt_basis: input.receipt_basis,
            plan_digest: input.plan_digest,
            semantic: input.semantic,
            mechanics: input.mechanics,
            presentation_effects: input.presentation_effects,
            diagnostics: input.diagnostics,
            changed_instances: input.changed_instances,
            precise_command_instances: std::rc::Rc::from([]),
            presentation_command_changes: std::rc::Rc::from([]),
            paint_batches: Vec::new(),
            layers: Vec::new(),
            spatial_batches: Vec::new(),
            realtime_batches: Vec::new(),
            resources: Vec::new(),
            ordinary_recorded: false,
            virtualized_recorded: false,
            canvas_recorded: false,
            realtime_recorded: false,
            ordinary_paint_selector: None,
            plan_index_paint_selectors: Vec::new(),
            preview: None,
            visual_overlay: None,
            counters: input.counters,
            capability_generation: input.capability_generation,
            capability_profile_digest: input.capability_profile_digest,
            font_collection: input.font_collection,
            materialized_projection_rows: std::rc::Rc::new(std::cell::Cell::new(0)),
        }
    }

    pub fn frame_identity(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
    }
    pub(in crate::mounting) fn content_generation(
        &self,
    ) -> worth_ui_host_contract::UiMountedContentGeneration {
        self.content_generation
    }
    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
    }
    pub(crate) fn mounted_instances(
        &self,
    ) -> impl ExactSizeIterator<Item = worth_ui_host_contract::UiMountedInstanceIdentity> + '_ {
        self.semantic.order.iter().copied()
    }

    pub fn cost_report(&self) -> super::super::UiMountCostReport {
        self.counters.finish()
    }

    pub(in crate::mounting) fn table_range_digest(&self) -> u64 {
        [
            self.semantic.node_count() as u64,
            self.layers.len() as u64,
            self.paint_batches.len() as u64,
            self.spatial_batches.len() as u64,
            self.realtime_batches.len() as u64,
            self.resources.len() as u64,
            self.mechanics.table_digest(),
        ]
        .into_iter()
        .fold(0x7461_626c_6572_616e_u64, |digest, value| {
            digest.rotate_left(11) ^ value
        })
    }

    pub(in crate::mounting) fn visual_region_basis(
        &self,
    ) -> super::super::UiMountedVisualRegionBasis {
        self.mechanics.visual_region_basis()
    }

    pub(in crate::mounting) fn identity_trace_basis(
        &self,
        source: crate::facade::prepared_application_authority::WorthUiPreparedVisualTraceSource,
    ) -> super::super::UiMountedIdentityTraceBasis {
        super::super::UiMountedIdentityTraceBasis::new(
            self.receipt_basis.clone(),
            self.semantic.clone(),
            source,
        )
    }

    pub(super) fn record_ordinary(
        &mut self,
        receipt: &crate::runtime::WorthUiOrdinaryLaneFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.ordinary_recorded)?;
        let batch = self.push_lane_batch(
            receipt.touch().row_count() as u32,
            0,
            None,
            UiMountedPaintPrimitiveKind::OrdinaryLaneSummary,
        )?;
        self.ordinary_paint_selector =
            Some(UiMountedOrdinaryPaintSelector::new(receipt.clone(), batch));
        Ok(())
    }

    pub(super) fn complete_mechanics(&mut self) -> Result<(), UiMountedProjectionDenial> {
        let mutation = self.mechanics.apply(UiMountedMechanicCompletion {
            frame: self.frame,
            content: self.content_generation,
            receipts: &self.receipt_basis,
            semantic: &self.semantic,
            changed: &self.changed_instances,
            capability_generation: self.capability_generation,
            capability_profile_digest: self.capability_profile_digest,
            font_collection: &self.font_collection,
        })?;
        self.record_rows::<worth_ui_host_contract::UiMountedFilledRectMechanic>(
            mutation.filled_rects,
        )?;
        self.record_rows::<worth_ui_host_contract::UiMountedSemanticTextMechanic>(
            mutation.semantic_text,
        )?;
        self.record_rows::<worth_ui_host_contract::UiMountedHitTestMechanic>(mutation.hit_tests)?;
        self.precise_command_instances = mutation.precise_instances.into();
        self.presentation_command_changes = mutation.command_changes.into();
        Ok(())
    }

    pub(super) fn mechanic_source(&self) -> UiMountedMechanicSource {
        self.mechanics.clone()
    }

    pub(super) fn presentation_effect_source(&self) -> UiMountedPresentationEffectSource {
        self.presentation_effects.clone()
    }

    pub(super) fn complete_presentation_effects(&mut self) {
        self.presentation_effects
            .apply(UiMountedPresentationEffectCompletion {
                semantic: &self.semantic,
                mechanics: &self.mechanics,
                changed: &self.changed_instances,
                preview: self.preview.as_ref(),
                overlay: self.visual_overlay.as_ref(),
                canvas: !self.spatial_batches.is_empty(),
                realtime: !self.realtime_batches.is_empty(),
            });
    }

    pub(super) fn complete_diagnostics(&mut self) {
        self.diagnostics
            .apply(&self.semantic, &self.changed_instances);
    }

    pub(in crate::mounting) fn presentation_commands_for_instance(
        &self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
        surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> std::sync::Arc<[worth_ui_host_contract::UiMountedPaintCommand]> {
        self.mechanics
            .commands_for_instance(instance, surface, binding)
    }

    pub(in crate::mounting) fn has_precise_command_delta(
        &self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> bool {
        self.precise_command_instances.contains(&instance)
    }

    pub(in crate::mounting) fn presentation_command_changes(
        &self,
    ) -> &[worth_ui_host_contract::UiMountedPaintCommandChange] {
        if self.precise_command_instances.len() == self.changed_instances.len() {
            &self.presentation_command_changes
        } else {
            &[]
        }
    }

    pub(in crate::mounting) fn presentation_order_position(
        &self,
        instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> Option<u64> {
        self.semantic.order.position(instance)
    }

    pub(in crate::mounting) fn presentation_instance_order(
        &self,
    ) -> crate::runtime::persistent_index::UiPersistentOrder<
        worth_ui_host_contract::UiMountedInstanceIdentity,
    > {
        self.semantic.order.clone()
    }

    pub(in crate::mounting) fn materialized_projection_rows(&self) -> u64 {
        self.materialized_projection_rows.get()
    }

    pub(in crate::mounting) fn semantic_projection(&self) -> &UiMountedSemanticProjection {
        &self.semantic
    }

    fn push_paint_batch(
        &mut self,
        primitive_count: u32,
        layer: UiMountedLayerReference,
        resource: Option<UiMountedResourceReference>,
        primitive_kind: UiMountedPaintPrimitiveKind,
    ) -> Result<UiMountedPaintBatchReference, UiMountedProjectionDenial> {
        if self.paint_batches.len() >= TABLE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedPaintBatchRow>(1)?;
        let batch_index = u16::try_from(self.paint_batches.len())
            .map_err(|_| UiMountedProjectionDenial::TableCapacityExceeded)?;
        self.paint_batches.push(UiMountedPaintBatchRow::new(
            primitive_count,
            UiMountedLayerProjection::Layer(layer),
            resource,
            primitive_kind,
        ));
        Ok(UiMountedPaintBatchReference::new(batch_index))
    }

    fn push_lane_batch(
        &mut self,
        primitive_count: u32,
        semantic_order: u32,
        resource: Option<UiMountedResourceReference>,
        primitive_kind: UiMountedPaintPrimitiveKind,
    ) -> Result<UiMountedPaintBatchReference, UiMountedProjectionDenial> {
        let layer = self.push_layer(semantic_order)?;
        self.push_paint_batch(primitive_count, layer, resource, primitive_kind)
    }

    fn push_plan_index_selector(
        &mut self,
        indexes: impl IntoIterator<Item = u32>,
        batch: UiMountedPaintBatchReference,
    ) {
        self.plan_index_paint_selectors
            .push(UiMountedPlanIndexPaintSelector::new(
                indexes.into_iter().collect(),
                batch,
            ));
    }

    fn push_layer(
        &mut self,
        semantic_order: u32,
    ) -> Result<UiMountedLayerReference, UiMountedProjectionDenial> {
        if self.layers.len() >= TABLE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedLayerRow>(1)?;
        let index = u16::try_from(self.layers.len())
            .map_err(|_| UiMountedProjectionDenial::TableCapacityExceeded)?;
        self.layers.push(UiMountedLayerRow::new(
            semantic_order,
            UiMountedClipProjection::Unclipped,
        ));
        Ok(UiMountedLayerReference::new(index))
    }

    fn intern_canvas_resource(
        &mut self,
        content_identity: u64,
    ) -> Result<UiMountedResourceReference, UiMountedProjectionDenial> {
        if let Some(index) = self
            .resources
            .iter()
            .position(|entry| entry.content_identity() == content_identity)
        {
            return u16::try_from(index)
                .map(UiMountedResourceReference::new)
                .map_err(|_| UiMountedProjectionDenial::TableCapacityExceeded);
        }
        if self.resources.len() >= RESOURCE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedResourceEntry>(1)?;
        let index = u16::try_from(self.resources.len())
            .map_err(|_| UiMountedProjectionDenial::TableCapacityExceeded)?;
        self.resources.push(UiMountedResourceEntry::new(
            content_identity,
            UiMountedResourceKind::CanvasContract,
            0,
        ));
        Ok(UiMountedResourceReference::new(index))
    }

    fn record_rows<Row>(&mut self, count: usize) -> Result<(), UiMountedProjectionDenial> {
        self.counters
            .replace_rows::<Row>(count)
            .map_err(|_| UiMountedProjectionDenial::CostCounterOverflow)
    }
}
