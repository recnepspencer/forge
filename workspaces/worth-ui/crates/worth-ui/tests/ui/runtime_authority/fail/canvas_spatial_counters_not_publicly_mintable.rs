use worth_ui::facade::WorthUiCanvasSpatialCounters;

fn main() {
    let _counters = WorthUiCanvasSpatialCounters {
        canvas_plan_row_count: 1,
        skipped_noncanvas_plan_row_count: 0,
        draw_hook_count: 1,
        hit_test_hook_count: 1,
        tool_state_hook_count: 1,
        overlay_plan_count: 0,
        viewport_transform_count: 0,
        draw_pass_count: 0,
        spatial_hit_test_count: 0,
        tool_state_attachment_count: 0,
        command_identity_preservation_count: 0,
        selection_identity_preservation_count: 0,
        diagnostics_posture_count: 0,
        renderer_reference_count: 0,
        domain_geometry_truth_read_count: 0,
        renderer_internal_read_count: 0,
        certification_failure_count: 0,
        denial_count: 0,
    };
}
