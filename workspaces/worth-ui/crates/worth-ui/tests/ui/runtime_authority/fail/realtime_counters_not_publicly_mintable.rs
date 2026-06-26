use worth_ui::facade::WorthUiRealtimeLaneCounters;

fn main() {
    let _counters = WorthUiRealtimeLaneCounters {
        hud_plan_row_count: 1,
        skipped_nonrealtime_plan_row_count: 0,
        overlay_hook_count: 1,
        renderer_surface_admission_count: 1,
        frame_synchronized_pass_count: 0,
        renderer_surface_handoff_count: 0,
        command_identity_preservation_count: 0,
        accessibility_posture_count: 0,
        diagnostics_posture_count: 0,
        ordinary_layout_pass_count: 0,
        source_parse_count: 0,
        registry_lookup_count: 0,
        allocation_count: 0,
        diagnostic_materialization_count: 0,
        certification_failure_count: 0,
        denial_count: 0,
    };
}
