use worth_ui_host_contract::{
    UiMountedPaintPrimitiveKind, UiMountedRealtimeBatchRow, UiMountedSpatialBatchRow,
};

use super::{UiMountedProjectionDenial, UiMountedProjectionFrame, TABLE_LIMIT};

impl UiMountedProjectionFrame {
    pub(in crate::mounting::projection) fn record_virtualized(
        &mut self,
        receipt: &crate::runtime::WorthUiVirtualizedDataFrameReceipt,
    ) -> Result<(), UiMountedProjectionDenial> {
        require_once(&mut self.virtualized_recorded)?;
        let range = receipt.visible_range();
        let count = range
            .row_count()
            .checked_mul(range.column_count())
            .ok_or(UiMountedProjectionDenial::TableCapacityExceeded)?;
        let batch = self.push_lane_batch(
            count,
            1,
            None,
            UiMountedPaintPrimitiveKind::VirtualizedLaneSummary,
        )?;
        self.push_plan_index_selector([receipt.touched_plan_index()], batch);
        Ok(())
    }

    pub(in crate::mounting::projection) fn record_canvas(
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

    pub(in crate::mounting::projection) fn record_realtime(
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

    pub(in crate::mounting::projection) fn record_preview(
        &mut self,
        preview: super::super::lowering::UiMountedPreviewProjectionInput,
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

    pub(in crate::mounting::projection) fn record_visual_overlay(
        &mut self,
        overlay: Option<crate::mounting::UiMountedVisualOverlayProjectionInput>,
    ) -> Result<(), UiMountedProjectionDenial> {
        if let Some(overlay) = overlay {
            let instance = overlay.target_receipt.mounted_instance();
            let target = self
                .semantic
                .nodes
                .get(&instance)
                .ok_or(UiMountedProjectionDenial::VisualOverlayTargetMissing)?;
            if target.receipt.semantic_surface() != overlay.surface {
                return Err(UiMountedProjectionDenial::VisualOverlaySurfaceMismatch);
            }
            self.visual_overlay = Some(overlay);
        }
        Ok(())
    }
}

pub(super) fn require_once(recorded: &mut bool) -> Result<(), UiMountedProjectionDenial> {
    if *recorded {
        return Err(UiMountedProjectionDenial::DuplicateLaneContribution);
    }
    *recorded = true;
    Ok(())
}
