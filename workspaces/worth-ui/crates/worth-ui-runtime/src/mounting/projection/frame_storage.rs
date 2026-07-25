use worth_ui_host_contract::{
    UiMountedClipProjection, UiMountedLayerProjection, UiMountedLayerReference, UiMountedLayerRow,
    UiMountedPaintBatchReference, UiMountedPaintBatchRow, UiMountedPaintPrimitiveKind,
    UiMountedRealtimeBatchRow, UiMountedResourceEntry, UiMountedResourceKind,
    UiMountedResourceReference, UiMountedSpatialBatchRow, UiSurfaceBindingGeneration,
};

use super::UiMountedProjectionDenial;

mod semantic_projection;
mod view;

pub(super) use semantic_projection::{
    UiMountedProjectionNodeRecord, UiMountedProjectionSurface, UiMountedSemanticProjection,
};
use view::UiMountedPaintSelector;

const TABLE_LIMIT: usize = 2_048;
const RESOURCE_LIMIT: usize = 1_024;

#[derive(Clone)]
pub struct UiMountedProjectionFrame {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    receipt_basis: super::super::UiMountedNodeReceiptBasis,
    plan_digest: u64,
    semantic: UiMountedSemanticProjection,
    paint_batches: Vec<UiMountedPaintBatchRow>,
    layers: Vec<UiMountedLayerRow>,
    spatial_batches: Vec<UiMountedSpatialBatchRow>,
    realtime_batches: Vec<UiMountedRealtimeBatchRow>,
    resources: Vec<UiMountedResourceEntry>,
    ordinary_recorded: bool,
    virtualized_recorded: bool,
    canvas_recorded: bool,
    realtime_recorded: bool,
    paint_selectors: Vec<UiMountedPaintSelector>,
    preview: Option<super::lowering::UiMountedPreviewProjectionInput>,
    counters: super::super::UiMountStageCounters,
}

impl UiMountedProjectionFrame {
    pub(super) fn new(
        frame: worth_ui_host_contract::UiMountedFrameIdentity,
        receipt_basis: super::super::UiMountedNodeReceiptBasis,
        plan_digest: u64,
        semantic: UiMountedSemanticProjection,
        counters: super::super::UiMountStageCounters,
    ) -> Self {
        Self {
            frame,
            receipt_basis,
            plan_digest,
            semantic,
            paint_batches: Vec::new(),
            layers: Vec::new(),
            spatial_batches: Vec::new(),
            realtime_batches: Vec::new(),
            resources: Vec::new(),
            ordinary_recorded: false,
            virtualized_recorded: false,
            canvas_recorded: false,
            realtime_recorded: false,
            paint_selectors: Vec::new(),
            preview: None,
            counters,
        }
    }

    pub fn frame_identity(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.frame
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
        self.paint_selectors.push(UiMountedPaintSelector::Ordinary {
            receipt: receipt.clone(),
            batch,
        });
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
        self.push_plan_index_selector([receipt.touched_plan_index()], batch);
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
        self.push_plan_index_selector(receipt.touched_plan_indexes().iter().copied(), batch);
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
        self.push_plan_index_selector(receipt.touched_plan_indexes().iter().copied(), batch);
        Ok(())
    }

    pub(super) fn record_preview(
        &mut self,
        preview: super::lowering::UiMountedPreviewProjectionInput,
    ) -> Result<(), UiMountedProjectionDenial> {
        let node = self
            .semantic
            .nodes
            .get(&preview.mounted_instance)
            .ok_or(UiMountedProjectionDenial::PreviewInstanceMismatch)?;
        if node.receipt.graph_node() != preview.graph_node {
            return Err(UiMountedProjectionDenial::PreviewInstanceMismatch);
        }
        self.preview = Some(preview);
        Ok(())
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
            let mut surface = rebound
                .semantic
                .surfaces
                .get(affected)
                .copied()
                .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
            if surface.surface != replacement.semantic_surface_identity() {
                return Err(UiMountedProjectionDenial::MissingSurfaceBinding);
            }
            rebound.semantic.surfaces.remove(affected);
            surface.binding = replacement.binding_generation();
            rebound.semantic.surfaces.insert(surface.binding, surface);
        }
        Ok(rebound)
    }

    pub(super) fn semantic_projection(&self) -> &UiMountedSemanticProjection {
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
        self.paint_selectors
            .push(UiMountedPaintSelector::PlanIndexes {
                indexes: indexes.into_iter().collect(),
                batch,
            });
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
