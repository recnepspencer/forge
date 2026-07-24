use worth_ui::facade::app::WorthUiPreparedApplicationAuthority;
use worth_ui::facade::mounted::{
    UiMountedNodeReceipt, UiMountedProjectionView, UiProjectedMountedFrameCandidate,
};

fn extract(authority: WorthUiPreparedApplicationAuthority) {
    let _ = authority.into_canonical_artifact();
}

fn extract_forbidden_mounted_authority(view: &UiMountedProjectionView) {
    let _ = view.query_key();
    let _ = view.query_artifact();
    let _ = view.query_settlement();
    let _ = view.query_rows();
    let _ = view.query_patches();
    let _ = view.query_operational_identity();
    let _ = view.native_widget_handle();
    let _ = view.native_texture_handle();
    let _ = view.native_resource_handle();
}

fn forge_node_receipt() -> UiMountedNodeReceipt {
    UiMountedNodeReceipt {}
}

fn forge_projected_candidate() -> UiProjectedMountedFrameCandidate {
    UiProjectedMountedFrameCandidate {}
}

fn main() {}

// runtime authority denials share one compiler process.
mod covered_001 { include!("prepared_application_constituent_cannot_be_replaced.rs"); }
mod covered_002 { include!("runtime_internal_module_is_not_public.rs"); }
mod covered_003 { include!("raw_runtime_cannot_lower_artifact_only_replacement.rs"); }
mod covered_004 { include!("raw_runtime_cannot_open_source_ingress.rs"); }
mod covered_005 { include!("prepared_application_cannot_launch_twice.rs"); }
mod covered_006 { include!("prepared_application_has_no_active_inspection.rs"); }
mod covered_007 { include!("active_application_session_cannot_be_split.rs"); }
mod covered_008 { include!("raw_host_adapter_cannot_submit_to_framework_turn.rs"); }
mod covered_010 { include!("runtime_host_authority_not_cloneable.rs"); }
mod covered_011 { include!("replacement_candidate_packets_not_cloneable.rs"); }
mod covered_012 { include!("unadmitted_candidate_cannot_enter_admitted_boundary.rs"); }
mod covered_013 { include!("unadmitted_candidate_cannot_enter_runtime_comparison.rs"); }
mod covered_014 { include!("unadmitted_candidate_cannot_enter_impact_classification.rs"); }
mod covered_015 { include!("unadmitted_candidate_cannot_enter_impact_narrowing.rs"); }
mod covered_016 { include!("active_runtime_state_not_constructible_from_app_local_parts.rs"); }
mod covered_017 { include!("forged_last_valid_receipt_not_installable.rs"); }
mod covered_018 { include!("raw_source_cannot_enter_plan_lowering.rs"); }
mod covered_019 { include!("admitted_candidate_cannot_bypass_activation_readiness.rs"); }
mod covered_020 { include!("unresolved_capability_string_cannot_enter_plan_node_input.rs"); }
mod covered_021 { include!("unregistered_component_hook_family_not_public_plan_input.rs"); }
mod covered_022 { include!("frame_path_cannot_resolve_component_or_command_by_string.rs"); }
mod covered_023 { include!("stale_runtime_handle_cannot_replace_fresh_plan_receipt.rs"); }
mod covered_024 { include!("frame_path_cannot_scan_artifact_tree_from_execution_plan.rs"); }
mod covered_025 { include!("pointer_identity_cannot_replace_plan_equivalence.rs"); }
mod covered_026 { include!("unadmitted_candidate_cannot_enter_identity_matching.rs"); }
mod covered_027 { include!("raw_domain_geometry_cannot_execute_canvas_lane.rs"); }
mod covered_028 { include!("raw_renderer_pointer_cannot_execute_canvas_lane.rs"); }
mod covered_029 { include!("private_component_lane_string_cannot_enter_lane_admission.rs"); }
mod covered_030 { include!("extension_hook_cannot_override_active_plan_truth.rs"); }
mod covered_031 { include!("ordinary_frame_target_string_constructor_missing.rs"); }
mod covered_032 { include!("raw_plan_input_cannot_execute_ordinary_lane.rs"); }
mod covered_033 { include!("local_query_explanation_record_cannot_replace_query_inspection_link.rs"); }
mod covered_034 { include!("raw_renderer_pointer_cannot_execute_realtime_lane.rs"); }
mod covered_035 { include!("ordinary_widget_fallback_cannot_execute_realtime_lane.rs"); }
mod covered_036 { include!("local_query_result_state_enum_cannot_replace_query_binding_posture.rs"); }
mod covered_037 { include!("local_subscription_recovery_path_cannot_replace_query_rebind.rs"); }
mod covered_038 { include!("visible_range_offset_pagination_constructor_missing.rs"); }
mod covered_039 { include!("raw_query_string_cannot_execute_virtualized_data_lane.rs"); }
