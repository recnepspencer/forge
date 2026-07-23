use crate::runtime::{
    WorthUiCanvasSpatialCounters, WorthUiFrameCostCounter, WorthUiOrdinaryLaneCounters,
    WorthUiRealtimeLaneCounters, WorthUiSteadyFrameCounters, WorthUiVirtualizedDataCounters,
};

use super::counter_schema;

pub(crate) fn ordinary_rows(counters: WorthUiOrdinaryLaneCounters) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            counter_schema::ORDINARY_NODES_VISITED,
            counters.ordinary_frame_row_touch_count(),
        ),
        count(
            counter_schema::ORDINARY_LAYOUT_RECOMPUTE_BREADTH,
            counters.child_range_touch_count(),
        ),
        count(
            counter_schema::ORDINARY_COMMAND_SURFACES_TOUCHED,
            counters.command_surface_touch_count(),
        ),
        count(
            counter_schema::ORDINARY_TOKEN_SUPPORT_TOUCHED,
            counters.token_support_touch_count(),
        ),
        count(
            counter_schema::ORDINARY_TEXT_SHAPES,
            counters.text_shape_count(),
        ),
        count(
            counter_schema::ORDINARY_GLYPH_UPLOADS,
            counters.glyph_upload_count(),
        ),
        count(
            counter_schema::ORDINARY_SOURCE_PARSE_COUNT,
            counters.source_parse_count(),
        ),
        count(
            counter_schema::ORDINARY_REGISTRY_LOOKUP_COUNT,
            counters.registry_lookup_count(),
        ),
        count(
            counter_schema::ORDINARY_ARTIFACT_TREE_SCAN_COUNT,
            counters.artifact_tree_scan_count(),
        ),
        count(
            counter_schema::ORDINARY_FULL_PLAN_SCAN_COUNT,
            counters.full_plan_scan_count(),
        ),
    ]
}

pub(crate) fn virtualized_rows(
    counters: WorthUiVirtualizedDataCounters,
) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            counter_schema::VIRTUALIZED_VISIBLE_ROWS_TOUCHED,
            counters.visible_row_touch_count(),
        ),
        count(
            counter_schema::VIRTUALIZED_VISIBLE_COLUMNS_TOUCHED,
            counters.visible_column_touch_count(),
        ),
        count(
            counter_schema::VIRTUALIZED_VISIBLE_CELLS_TOUCHED,
            counters.visible_cell_touch_count(),
        ),
        count(
            counter_schema::VIRTUALIZED_DIRECT_ROW_LOOKUPS,
            counters.direct_row_lookup_count(),
        ),
        count(
            counter_schema::VIRTUALIZED_EVIDENCE_REFERENCE_LOOKUPS,
            counters.evidence_reference_lookup_count(),
        ),
        count(
            counter_schema::VIRTUALIZED_FULL_COLLECTION_SCAN_COUNT,
            counters.full_collection_scan_count(),
        ),
        count(
            counter_schema::VIRTUALIZED_OFFSET_PAGINATION_SUBSTITUTE_COUNT,
            counters.offset_pagination_substitute_count(),
        ),
        count(
            counter_schema::VIRTUALIZED_QUERY_COLLECTION_EXECUTION_COUNT,
            counters.query_collection_execution_count(),
        ),
        count(
            counter_schema::VIRTUALIZED_DIAGNOSTIC_MATERIALIZATION_COUNT,
            counters.diagnostic_materialization_count(),
        ),
    ]
}

pub(crate) fn canvas_rows(counters: WorthUiCanvasSpatialCounters) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            counter_schema::CANVAS_DRAW_HOOK_COUNT,
            counters.draw_hook_count(),
        ),
        count(
            counter_schema::CANVAS_SPATIAL_HIT_TESTS,
            counters.spatial_hit_test_count(),
        ),
        count(
            counter_schema::CANVAS_OVERLAY_PLANS,
            counters.overlay_plan_count(),
        ),
        count(
            counter_schema::CANVAS_VIEWPORT_TRANSFORMS,
            counters.viewport_transform_count(),
        ),
        count(
            counter_schema::CANVAS_DRAW_PASSES,
            counters.draw_pass_count(),
        ),
        count(
            counter_schema::CANVAS_RENDERER_REFERENCES,
            counters.renderer_reference_count(),
        ),
        count(
            counter_schema::CANVAS_DOMAIN_GEOMETRY_TRUTH_READS,
            counters.domain_geometry_truth_read_count(),
        ),
        count(
            counter_schema::CANVAS_RENDERER_INTERNAL_READS,
            counters.renderer_internal_read_count(),
        ),
    ]
}

pub(crate) fn realtime_rows(counters: WorthUiRealtimeLaneCounters) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count(
            counter_schema::REALTIME_OVERLAY_HOOKS,
            counters.overlay_hook_count(),
        ),
        count(
            counter_schema::REALTIME_FRAME_SYNCHRONIZED_PASSES,
            counters.frame_synchronized_pass_count(),
        ),
        count(
            counter_schema::REALTIME_RENDERER_SURFACE_HANDOFFS,
            counters.renderer_surface_handoff_count(),
        ),
        count(
            counter_schema::REALTIME_TARGETED_OVERLAY_ROWS,
            counters.targeted_overlay_row_count(),
        ),
        count(
            counter_schema::REALTIME_ORDINARY_LAYOUT_PASSES,
            counters.ordinary_layout_pass_count(),
        ),
        count(
            counter_schema::REALTIME_SOURCE_PARSE_COUNT,
            counters.source_parse_count(),
        ),
        count(
            counter_schema::REALTIME_REGISTRY_LOOKUP_COUNT,
            counters.registry_lookup_count(),
        ),
        count(
            counter_schema::REALTIME_ALLOCATION_COUNT,
            counters.allocation_count(),
        ),
        count(
            counter_schema::REALTIME_DIAGNOSTIC_MATERIALIZATION_COUNT,
            counters.diagnostic_materialization_count(),
        ),
    ]
}

pub(crate) fn aggregate_rows(counters: WorthUiSteadyFrameCounters) -> Vec<WorthUiFrameCostCounter> {
    vec![
        count_u64(
            counter_schema::STEADY_NODES_VISITED,
            counters.total_nodes_visited(),
        ),
        count_u64(
            counter_schema::STEADY_LAYOUT_RECOMPUTE_BREADTH,
            counters.total_layout_recompute_breadth(),
        ),
        count_u64(
            counter_schema::STEADY_HIT_TEST_BREADTH,
            counters.total_hit_test_breadth(),
        ),
        count_u64(
            counter_schema::STEADY_VIRTUALIZED_ROWS_TOUCHED,
            counters.total_virtualized_rows_touched(),
        ),
        count_u64(
            counter_schema::STEADY_VIRTUALIZED_COLUMNS_TOUCHED,
            counters.total_virtualized_columns_touched(),
        ),
        count_u64(
            counter_schema::STEADY_DRAW_BATCHES,
            counters.total_draw_batches(),
        ),
        count_u64(
            counter_schema::STEADY_RENDER_PASSES,
            counters.total_render_passes(),
        ),
        count_u64(
            counter_schema::STEADY_TEXT_SHAPES,
            counters.total_text_shape_count(),
        ),
        count_u64(
            counter_schema::STEADY_GLYPH_UPLOADS,
            counters.total_glyph_upload_count(),
        ),
        count_u64(
            counter_schema::STEADY_EXECUTOR_ALLOCATIONS,
            counters.total_allocation_count(),
        ),
        count_u64(
            counter_schema::STEADY_DIAGNOSTIC_MATERIALIZATIONS,
            counters.total_diagnostic_materialization_count(),
        ),
        count_u64(
            counter_schema::STEADY_SOURCE_OR_REGISTRY_WORK,
            counters.total_forbidden_source_or_registry_work(),
        ),
    ]
}

fn count(name: &'static str, value: usize) -> WorthUiFrameCostCounter {
    count_u64(name, value as u64)
}

fn count_u64(name: &'static str, value: u64) -> WorthUiFrameCostCounter {
    WorthUiFrameCostCounter::count(name, value)
}
