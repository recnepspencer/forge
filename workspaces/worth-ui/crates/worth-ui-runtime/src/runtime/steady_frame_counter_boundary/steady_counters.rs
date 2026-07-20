use crate::runtime::{
    WorthUiCanvasSpatialCounters, WorthUiOrdinaryLaneCounters, WorthUiRealtimeLaneCounters,
    WorthUiVirtualizedDataCounters,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiSteadyFrameCounters {
    ordinary: WorthUiOrdinaryLaneCounters,
    virtualized_data: WorthUiVirtualizedDataCounters,
    canvas_spatial: WorthUiCanvasSpatialCounters,
    realtime_overlay: WorthUiRealtimeLaneCounters,
}

impl WorthUiSteadyFrameCounters {
    pub(crate) fn record_ordinary(&mut self, counters: WorthUiOrdinaryLaneCounters) {
        self.ordinary = counters;
    }

    pub(crate) fn record_virtualized_data(&mut self, counters: WorthUiVirtualizedDataCounters) {
        self.virtualized_data = counters;
    }

    pub(crate) fn record_canvas_spatial(&mut self, counters: WorthUiCanvasSpatialCounters) {
        self.canvas_spatial = counters;
    }

    pub(crate) fn record_realtime_overlay(&mut self, counters: WorthUiRealtimeLaneCounters) {
        self.realtime_overlay = counters;
    }

    pub fn ordinary(&self) -> WorthUiOrdinaryLaneCounters {
        self.ordinary
    }

    pub fn virtualized_data(&self) -> WorthUiVirtualizedDataCounters {
        self.virtualized_data
    }

    pub fn canvas_spatial(&self) -> WorthUiCanvasSpatialCounters {
        self.canvas_spatial
    }

    pub fn realtime_overlay(&self) -> WorthUiRealtimeLaneCounters {
        self.realtime_overlay
    }

    pub fn total_nodes_visited(&self) -> u64 {
        self.ordinary.ordinary_frame_row_touch_count() as u64
            + self.virtualized_data.visible_row_touch_count() as u64
            + self.canvas_spatial.spatial_hit_test_count() as u64
            + self.realtime_overlay.frame_synchronized_pass_count() as u64
    }

    pub fn total_layout_recompute_breadth(&self) -> u64 {
        self.ordinary.child_range_touch_count() as u64
            + self.realtime_overlay.ordinary_layout_pass_count() as u64
    }

    pub fn total_hit_test_breadth(&self) -> u64 {
        self.canvas_spatial.spatial_hit_test_count() as u64
    }

    pub fn total_virtualized_rows_touched(&self) -> u64 {
        self.virtualized_data.visible_row_touch_count() as u64
    }

    pub fn total_virtualized_columns_touched(&self) -> u64 {
        self.virtualized_data.visible_column_touch_count() as u64
    }

    pub fn total_draw_batches(&self) -> u64 {
        self.canvas_spatial.draw_pass_count() as u64
            + self.realtime_overlay.renderer_surface_handoff_count() as u64
    }

    pub fn total_render_passes(&self) -> u64 {
        self.canvas_spatial.draw_pass_count() as u64
            + self.realtime_overlay.frame_synchronized_pass_count() as u64
    }

    pub fn total_text_shape_count(&self) -> u64 {
        self.ordinary.text_shape_count() as u64
    }

    pub fn total_glyph_upload_count(&self) -> u64 {
        self.ordinary.glyph_upload_count() as u64
    }

    pub fn total_allocation_count(&self) -> u64 {
        self.realtime_overlay.allocation_count() as u64
    }

    /// General-purpose allocations declared by active-plan executors only.
    /// Host-adapter translation and renderer/native mechanics are outside this
    /// counter boundary and require their own independent observation.
    pub fn executor_allocation_count(&self) -> u64 {
        self.total_allocation_count()
    }

    pub fn total_diagnostic_materialization_count(&self) -> u64 {
        self.realtime_overlay.diagnostic_materialization_count() as u64
    }

    pub fn total_forbidden_source_or_registry_work(&self) -> u64 {
        self.ordinary.source_parse_count() as u64
            + self.ordinary.registry_lookup_count() as u64
            + self.realtime_overlay.source_parse_count() as u64
            + self.realtime_overlay.registry_lookup_count() as u64
    }
}
