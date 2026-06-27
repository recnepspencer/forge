use super::*;

#[path = "activation_staging_boundary_tests.rs"]
mod activation_staging_boundary_tests;
#[path = "activation_staging_test_support.rs"]
mod activation_staging_test_support;
#[path = "active_runtime_authority_tests.rs"]
mod active_runtime_authority_tests;
#[path = "artifact_equivalence_boundary_tests.rs"]
mod artifact_equivalence_boundary_tests;
#[path = "atomic_plan_swap_boundary_tests.rs"]
mod atomic_plan_swap_boundary_tests;
#[path = "candidate_admission_boundary_tests.rs"]
mod candidate_admission_boundary_tests;
#[path = "canvas_spatial_lane_boundary_tests.rs"]
mod canvas_spatial_lane_boundary_tests;
#[path = "canvas_spatial_lane_test_support.rs"]
mod canvas_spatial_lane_test_support;
#[path = "dependency_impact_narrowing_boundary_tests.rs"]
mod dependency_impact_narrowing_boundary_tests;
#[path = "dependency_impact_narrowing_test_support.rs"]
pub(crate) mod dependency_impact_narrowing_test_support;
#[path = "durable_state_inventory_boundary_tests.rs"]
mod durable_state_inventory_boundary_tests;
#[path = "durable_state_inventory_custom_hook_boundary_tests.rs"]
mod durable_state_inventory_custom_hook_boundary_tests;
#[path = "durable_state_inventory_test_support.rs"]
mod durable_state_inventory_test_support;
#[path = "durable_state_reconciliation_boundary_tests.rs"]
mod durable_state_reconciliation_boundary_tests;
#[path = "durable_state_reconciliation_test_support.rs"]
mod durable_state_reconciliation_test_support;
#[path = "execution_plan_input_boundary_tests.rs"]
mod execution_plan_input_boundary_tests;
#[path = "file_rust_replacement_parity_boundary_tests.rs"]
mod file_rust_replacement_parity_boundary_tests;
#[path = "file_rust_replacement_parity_test_support.rs"]
mod file_rust_replacement_parity_test_support;
#[path = "frame_activation_gate_boundary_tests.rs"]
mod frame_activation_gate_boundary_tests;
#[path = "frame_activation_gate_test_support.rs"]
mod frame_activation_gate_test_support;
#[path = "handle_allocation_boundary_tests.rs"]
mod handle_allocation_boundary_tests;
#[path = "identity_match_graph_boundary_tests.rs"]
mod identity_match_graph_boundary_tests;
#[path = "identity_match_graph_report_boundary_tests.rs"]
mod identity_match_graph_report_boundary_tests;
#[path = "identity_match_graph_test_support.rs"]
mod identity_match_graph_test_support;
#[path = "identity_state_query_certification_boundary_tests.rs"]
mod identity_state_query_certification_boundary_tests;
#[path = "identity_state_query_certification_test_support.rs"]
mod identity_state_query_certification_test_support;
#[path = "lane_admission_boundary_tests.rs"]
mod lane_admission_boundary_tests;
#[path = "lane_frame_cost_certification_boundary_tests.rs"]
mod lane_frame_cost_certification_boundary_tests;
#[path = "lane_frame_cost_certification_scale_fixture.rs"]
mod lane_frame_cost_certification_scale_fixture;
#[path = "lane_frame_cost_certification_test_support.rs"]
mod lane_frame_cost_certification_test_support;
#[path = "lane_meaning_parity_boundary_tests.rs"]
mod lane_meaning_parity_boundary_tests;
#[path = "lane_meaning_parity_test_support.rs"]
mod lane_meaning_parity_test_support;
#[path = "measurement_boundary_tests.rs"]
mod measurement_boundary_tests;
#[path = "node_replacement_classification_boundary_tests.rs"]
mod node_replacement_classification_boundary_tests;
#[path = "node_replacement_classification_test_support.rs"]
mod node_replacement_classification_test_support;
#[path = "ordinary_lane_boundary_tests.rs"]
mod ordinary_lane_boundary_tests;
#[path = "ordinary_lane_test_support.rs"]
mod ordinary_lane_test_support;
#[path = "plan_equivalence_boundary_tests.rs"]
mod plan_equivalence_boundary_tests;
#[path = "plan_inspection_boundary_tests.rs"]
mod plan_inspection_boundary_tests;
#[path = "plan_inspection_expected_provenance.rs"]
mod plan_inspection_expected_provenance;
#[path = "plan_topology_boundary_tests.rs"]
mod plan_topology_boundary_tests;
#[path = "query_binding_comparison_boundary_tests.rs"]
mod query_binding_comparison_boundary_tests;
#[path = "query_binding_comparison_test_support.rs"]
mod query_binding_comparison_test_support;
#[path = "query_binding_posture_drift_boundary_tests.rs"]
mod query_binding_posture_drift_boundary_tests;
#[path = "query_live_rebind_boundary_tests.rs"]
mod query_live_rebind_boundary_tests;
#[path = "realtime_overlay_lane_boundary_tests.rs"]
mod realtime_overlay_lane_boundary_tests;
#[path = "realtime_overlay_lane_pending_activation_fixture.rs"]
mod realtime_overlay_lane_pending_activation_fixture;
#[path = "realtime_overlay_lane_test_support.rs"]
mod realtime_overlay_lane_test_support;
#[path = "reload_counter_boundary_tests.rs"]
mod reload_counter_boundary_tests;
#[path = "reload_failure_boundary_tests.rs"]
mod reload_failure_boundary_tests;
#[path = "reload_failure_test_support.rs"]
mod reload_failure_test_support;
#[path = "reload_storm_certification_boundary_tests.rs"]
mod reload_storm_certification_boundary_tests;
#[path = "reload_storm_certification_test_support.rs"]
mod reload_storm_certification_test_support;
#[path = "replacement_candidate_boundary_tests.rs"]
mod replacement_candidate_boundary_tests;
#[path = "replacement_impact_boundary_tests.rs"]
mod replacement_impact_boundary_tests;
#[path = "replacement_impact_test_support.rs"]
pub(crate) mod replacement_impact_test_support;
#[path = "runtime_diagnostics_boundary_tests.rs"]
mod runtime_diagnostics_boundary_tests;
#[path = "runtime_diagnostics_family_coverage_tests.rs"]
mod runtime_diagnostics_family_coverage_tests;
#[path = "runtime_diagnostics_projection_boundary_tests.rs"]
mod runtime_diagnostics_projection_boundary_tests;
#[path = "runtime_diagnostics_projection_test_support.rs"]
mod runtime_diagnostics_projection_test_support;
#[path = "source_ingress_boundary_tests.rs"]
mod source_ingress_boundary_tests;
#[path = "source_ingress_test_support.rs"]
pub(crate) mod source_ingress_test_support;
#[path = "steady_frame_counter_boundary_tests.rs"]
mod steady_frame_counter_boundary_tests;
#[path = "steady_frame_counter_forbidden_work_tests.rs"]
mod steady_frame_counter_forbidden_work_tests;
#[path = "steady_frame_counter_schema_tests.rs"]
mod steady_frame_counter_schema_tests;
#[path = "steady_frame_report_planning_tests.rs"]
mod steady_frame_report_planning_tests;
#[path = "virtualized_data_lane_boundary_tests.rs"]
mod virtualized_data_lane_boundary_tests;
#[path = "virtualized_data_lane_test_support.rs"]
mod virtualized_data_lane_test_support;
