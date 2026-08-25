use worth_ui_host_contract::{
    UiMountedClipProjection, UiMountedLayerProjection, UiMountedLayerReference, UiMountedLayerRow,
    UiMountedPaintBatchReference, UiMountedPaintBatchRow, UiMountedPaintPrimitiveKind,
    UiMountedResourceEntry, UiMountedResourceKind, UiMountedResourceReference,
};

use super::{
    UiMountedPlanIndexPaintSelector, UiMountedProjectionDenial, UiMountedProjectionFrame,
    RESOURCE_LIMIT, TABLE_LIMIT,
};

impl UiMountedProjectionFrame {
    pub(super) fn push_lane_batch(
        &mut self,
        primitive_count: u32,
        semantic_order: u32,
        resource: Option<UiMountedResourceReference>,
        primitive_kind: UiMountedPaintPrimitiveKind,
    ) -> Result<UiMountedPaintBatchReference, UiMountedProjectionDenial> {
        let layer = self.push_layer(semantic_order)?;
        self.push_paint_batch(primitive_count, layer, resource, primitive_kind)
    }

    pub(super) fn push_plan_index_selector(
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

    pub(super) fn intern_canvas_resource(
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

    pub(super) fn record_rows<Row>(
        &mut self,
        count: usize,
    ) -> Result<(), UiMountedProjectionDenial> {
        self.counters
            .replace_rows::<Row>(count)
            .map_err(|_| UiMountedProjectionDenial::CostCounterOverflow)
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
}
