use worth_ui_host_contract::{
    UiMountedAccessibilityProjection, UiMountedClipProjection, UiMountedClipTable,
    UiMountedDiagnosticProjection, UiMountedLayerProjection, UiMountedLayerReference,
    UiMountedLayerRow, UiMountedLayerTable, UiMountedNodeProjectionView,
    UiMountedNodeProjectionViewInput, UiMountedOmissionReason, UiMountedPaintBatchReference,
    UiMountedPaintBatchRow, UiMountedPaintBatchTable, UiMountedPaintPrimitiveKind,
    UiMountedParticipationStatus, UiMountedProjectionAudience, UiMountedProjectionView,
    UiMountedProjectionViewInput, UiMountedRealtimeBatchRow, UiMountedRealtimeBatchTable,
    UiMountedResourceEntry, UiMountedResourceKind, UiMountedResourceReference,
    UiMountedResourceTable, UiMountedSpatialBatchRow, UiMountedSpatialBatchTable,
    UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

use super::{UiMountedNodeReceipt, UiMountedProjectionDenial};

const TABLE_LIMIT: usize = 2_048;
const RESOURCE_LIMIT: usize = 1_024;

#[derive(Clone)]
pub(super) struct UiMountedProjectionNodeRecord {
    pub receipt: UiMountedNodeReceipt,
    pub plan_index: Option<u32>,
}

#[derive(Clone, Copy)]
pub(super) struct UiMountedProjectionSurface {
    pub surface: UiSemanticSurfaceIdentity,
    pub binding: UiSurfaceBindingGeneration,
    pub audience: UiMountedProjectionAudience,
}

#[derive(Clone)]
pub struct UiMountedProjectionFrame {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    plan_digest: u64,
    nodes: Vec<UiMountedProjectionNodeRecord>,
    surfaces: Vec<UiMountedProjectionSurface>,
    paint_batches: Vec<UiMountedPaintBatchRow>,
    layers: Vec<UiMountedLayerRow>,
    spatial_batches: Vec<UiMountedSpatialBatchRow>,
    realtime_batches: Vec<UiMountedRealtimeBatchRow>,
    resources: Vec<UiMountedResourceEntry>,
    ordinary_recorded: bool,
    virtualized_recorded: bool,
    canvas_recorded: bool,
    realtime_recorded: bool,
    counters: super::super::UiMountStageCounters,
}

impl UiMountedProjectionFrame {
    pub(super) fn new(
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        plan_digest: u64,
        nodes: Vec<UiMountedProjectionNodeRecord>,
        surfaces: Vec<UiMountedProjectionSurface>,
        counters: super::super::UiMountStageCounters,
    ) -> Self {
        Self {
            frame,
            plan_digest,
            nodes,
            surfaces,
            paint_batches: Vec::new(),
            layers: Vec::new(),
            spatial_batches: Vec::new(),
            realtime_batches: Vec::new(),
            resources: Vec::new(),
            ordinary_recorded: false,
            virtualized_recorded: false,
            canvas_recorded: false,
            realtime_recorded: false,
            counters,
        }
    }

    pub fn frame_identity(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
    }
    pub fn plan_digest(&self) -> u64 {
        self.plan_digest
    }
    pub fn node_receipts(&self) -> impl ExactSizeIterator<Item = &UiMountedNodeReceipt> {
        self.nodes.iter().map(|record| &record.receipt)
    }

    pub fn cost_report(&self) -> super::super::UiMountCostReport {
        self.counters.finish()
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
            UiMountedPaintPrimitiveKind::FilledRect,
        )?;
        for node in &mut self.nodes {
            if node
                .plan_index
                .is_some_and(|index| receipt.touch().names_plan_index(index))
                && node.receipt.participation().paint().status()
                    == UiMountedParticipationStatus::Admitted
            {
                node.receipt.attach_paint(batch);
            }
        }
        Ok(())
    }

    pub(super) fn record_virtualized(
        &mut self,
        receipt: &crate::runtime::WorthUiVirtualizedDataFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.virtualized_recorded)?;
        let range = receipt.visible_range();
        let count = range
            .row_count()
            .checked_mul(range.column_count())
            .ok_or(UiMountedProjectionDenial::TableCapacityExceeded)?;
        let batch =
            self.push_lane_batch(count, 1, None, UiMountedPaintPrimitiveKind::FilledRect)?;
        self.attach_exact_plan_index(receipt.touched_plan_index(), batch);
        Ok(())
    }

    pub(super) fn record_canvas(
        &mut self,
        receipt: &crate::runtime::WorthUiCanvasSpatialFrameReceipt,
        resource_content_identity: u64,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.canvas_recorded)?;
        if self.spatial_batches.len() >= TABLE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedSpatialBatchRow>(1)?;
        self.spatial_batches.push(UiMountedSpatialBatchRow::new(
            receipt.visible_primitive_count(),
            receipt.queried_hit_test_region_count(),
            receipt.touched_overlay_row_count(),
            receipt.touched_tool_state_row_count(),
        ));
        let resource = self.intern_canvas_resource(resource_content_identity)?;
        let batch = self.push_lane_batch(
            receipt.visible_primitive_count(),
            2,
            Some(resource),
            UiMountedPaintPrimitiveKind::CanvasSpatialBatch,
        )?;
        for plan_index in receipt.touched_plan_indexes() {
            self.attach_exact_plan_index(*plan_index, batch);
        }
        Ok(())
    }

    pub(super) fn record_realtime(
        &mut self,
        receipt: &crate::runtime::WorthUiRealtimeFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.realtime_recorded)?;
        if self.realtime_batches.len() >= TABLE_LIMIT {
            return Err(UiMountedProjectionDenial::TableCapacityExceeded);
        }
        self.record_rows::<UiMountedRealtimeBatchRow>(1)?;
        self.realtime_batches.push(UiMountedRealtimeBatchRow::new(
            receipt.touched_overlay_row_count(),
        ));
        let batch = self.push_lane_batch(
            u32::from(receipt.touched_overlay_row_count()),
            3,
            None,
            UiMountedPaintPrimitiveKind::RealtimeBatch,
        )?;
        for plan_index in receipt.touched_plan_indexes() {
            self.attach_exact_plan_index(*plan_index, batch);
        }
        Ok(())
    }

    pub(super) fn record_preview(
        &mut self,
        preview: super::lowering::UiMountedPreviewProjectionInput,
    ) -> Result<(), UiMountedProjectionDenial> {
        let node = self
            .nodes
            .iter_mut()
            .find(|node| node.receipt.mounted_instance() == preview.mounted_instance)
            .ok_or(UiMountedProjectionDenial::PreviewInstanceMismatch)?;
        if node.receipt.graph_node() != preview.graph_node {
            return Err(UiMountedProjectionDenial::PreviewInstanceMismatch);
        }
        node.receipt
            .attach_preview(worth_ui_host_contract::UiMountedPreviewProjection::resize(
                preview.frame_epoch,
                preview.extent_subpixels,
                preview.candidate_count,
                preview.all_candidates_admitted,
            ));
        Ok(())
    }

    pub fn view_for(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> Result<UiMountedProjectionView, UiMountedProjectionDenial> {
        let surface = self
            .surfaces
            .iter()
            .find(|surface| surface.binding == binding)
            .copied()
            .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
        let nodes = self
            .nodes
            .iter()
            .filter(|node| node.receipt.semantic_surface() == surface.surface)
            .map(|node| audience_node_view(&node.receipt, surface.audience))
            .collect();
        Ok(UiMountedProjectionView::new(UiMountedProjectionViewInput {
            frame: self.frame,
            surface: surface.surface,
            binding,
            nodes,
            clips: UiMountedClipTable::produced(Vec::new()),
            layers: UiMountedLayerTable::produced(self.layers.clone()),
            paint_batches: UiMountedPaintBatchTable::new(self.paint_batches.clone()),
            spatial_batches: UiMountedSpatialBatchTable::new(self.spatial_batches.clone()),
            realtime_batches: UiMountedRealtimeBatchTable::new(self.realtime_batches.clone()),
            resources: UiMountedResourceTable::new(self.resources.clone()),
        }))
    }

    pub(crate) fn rebound(
        &self,
        replacements: &[(
            UiSurfaceBindingGeneration,
            super::super::UiSurfaceBindingIdentityView,
        )],
    ) -> Result<Self, UiMountedProjectionDenial> {
        let mut rebound = self.clone();
        for (affected, replacement) in replacements {
            let surface = rebound
                .surfaces
                .iter_mut()
                .find(|surface| surface.binding == *affected)
                .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
            if surface.surface != replacement.semantic_surface_identity() {
                return Err(UiMountedProjectionDenial::MissingSurfaceBinding);
            }
            surface.binding = replacement.binding_generation();
        }
        Ok(rebound)
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

    fn attach_exact_plan_index(&mut self, plan_index: u32, batch: UiMountedPaintBatchReference) {
        for node in &mut self.nodes {
            if node.plan_index == Some(plan_index)
                && node.receipt.participation().paint().status()
                    == UiMountedParticipationStatus::Admitted
            {
                node.receipt.attach_paint(batch);
            }
        }
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

fn require_once(recorded: &mut bool) -> Result<(), UiMountedProjectionDenial> {
    if *recorded {
        return Err(UiMountedProjectionDenial::DuplicateLaneContribution);
    }
    *recorded = true;
    Ok(())
}

fn audience_node_view(
    receipt: &UiMountedNodeReceipt,
    audience: UiMountedProjectionAudience,
) -> UiMountedNodeProjectionView {
    let accessibility = if audience.accessibility_disclosed() {
        receipt.accessibility()
    } else {
        UiMountedAccessibilityProjection::Omitted(UiMountedOmissionReason::SurfacePolicyWithheld)
    };
    let diagnostic = if audience.diagnostics_disclosed() {
        receipt.diagnostic()
    } else {
        UiMountedDiagnosticProjection::Omitted(UiMountedOmissionReason::SurfacePolicyWithheld)
    };
    UiMountedNodeProjectionView::new(UiMountedNodeProjectionViewInput {
        mounted_instance: receipt.mounted_instance(),
        node_receipt: receipt.identity(),
        role: receipt.role(),
        participation: receipt.participation(),
        allocation: receipt.allocation(),
        preview: receipt.preview(),
        paint: receipt.paint(),
        accessibility,
        motion: receipt.motion(),
        diagnostic,
    })
}
