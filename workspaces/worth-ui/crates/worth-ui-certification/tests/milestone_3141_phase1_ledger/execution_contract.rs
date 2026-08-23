#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TestIdentity {
    pub(super) package: &'static str,
    pub(super) target_kind: &'static str,
    pub(super) target_name: &'static str,
    pub(super) features: &'static [&'static str],
    pub(super) test_name: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CompileCase {
    pub(super) owner: &'static str,
    pub(super) kind: &'static str,
    pub(super) target: &'static str,
}

const EMPTY: &[&str] = &[];
const EXECUTABLE_WORLD: &[&str] = &["executable-world"];
const COMPILE_MAIN: TestIdentity = integration(
    "worth-ui-certification",
    "topology_contracts",
    "milestone_3141_phase1_topology::compile_contract_artifact::phase_one_compile_contract_artifact_matches_every_executed_case",
);
const P2_MAIN: TestIdentity = TestIdentity {
    package: "worth-ui-platform-pulse",
    target_kind: "test",
    target_name: "executable_world",
    features: EXECUTABLE_WORLD,
    test_name: "courtroom::native_phase2::windows_native_boundary_world_presents_quiesces_and_closes_without_residue",
};
const P3_NATIVE_MAIN: TestIdentity = TestIdentity {
    package: "worth-ui-platform-pulse",
    target_kind: "test",
    target_name: "executable_world",
    features: EXECUTABLE_WORLD,
    test_name: "courtroom::native_phase3::maximum_overlap_deltas_cross_public_runtime_native_pixels_and_exact_costs",
};
const P3_MIXED_MAIN: TestIdentity = integration(
    "worth-ui-certification",
    "application_contracts",
    "host_platform::mixed_carrier_successors_are_local_at_the_4096_command_ceiling",
);

#[path = "execution_compile_cases.rs"]
mod compile_cases;
#[path = "execution_contract_phase4.rs"]
mod phase4;
#[path = "execution_contract_phase5.rs"]
mod phase5;
#[path = "execution_contract_phase6.rs"]
mod phase6;

const fn library(package: &'static str, test_name: &'static str) -> TestIdentity {
    TestIdentity {
        package,
        target_kind: "lib",
        target_name: "lib",
        features: EMPTY,
        test_name,
    }
}

const fn integration(
    package: &'static str,
    target_name: &'static str,
    test_name: &'static str,
) -> TestIdentity {
    TestIdentity {
        package,
        target_kind: "test",
        target_name,
        features: EMPTY,
        test_name,
    }
}

pub(super) fn main_for(requirement: &str) -> Option<TestIdentity> {
    if requirement.starts_with("P4-") {
        return phase4::main_for(requirement);
    }
    let test = match requirement {
        "P1-AFFINITY-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::one_replacement_carries_one_change_and_exact_predecessor_successor_damage",
        ),
        "P1-AUTHORITY-01"
        | "P1-ORDER-SOURCE-01"
        | "P1-PLATFORM-AUTHORITY-01"
        | "P1-PRESENTATION-AUTHORITY-01"
        | "P1-PROTOCOL-01" => COMPILE_MAIN,
        "P1-BACKEND-FEATURES-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_topology::resolved_graphs::default_all_feature_and_windows_resolved_graphs_are_exact_and_mutation_sensitive",
        ),
        "P1-BASELINE-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::coordinator::admission::tests::actual_baseline_registration_gates_the_presentation_admission_transition",
        ),
        "P1-CLOSE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::phase_one_closure_prerequisites_are_final_source",
        ),
        "P1-CONSUMERS-01" => library(
            "worth-ui-host-headless",
            "headless_static_paint_tests::validated_agreement_static_paint_consumes_and_mixed_contract_stops_before_consumer",
        ),
        "P1-DAMAGE-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::replacement_damage_is_clipped_to_predecessor_and_successor_visibility",
        ),
        "P1-HEADLESS-01" => integration(
            "worth-ui-certification",
            "application_contracts",
            "mounted_headless_recorder::real_cross_lane_recording_preserves_exact_unperformed_external_mechanics",
        ),
        "P1-HEADLESS-COST-01" | "P1-WORLDS-01" => integration(
            "worth-ui-certification",
            "application_contracts",
            "host_platform::maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work",
        ),
        "P1-ORDER-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::equal_layer_total_order_follows_authored_node_order_not_command_identity",
        ),
        "P1-PRODUCER-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::removal_and_insert_carry_exact_identities_vacated_damage_and_total_order",
        ),
        "P1-PRODUCER-COST-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::unchanged_successor_carries_zero_command_order_and_damage_work",
        ),
        "P1-PREPARATION-LIFECYCLE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_topology::phase_one_product_preparation_is_effect_free_and_host_neutral",
        ),
        "P1-PROFILE-01" => library(
            "worth-ui-host-native",
            "qualification_tests::every_qualified_semantic_and_dependency_pin_matches_the_closed_record",
        ),
        "P1-TOPOLOGY-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_topology::phase_one_host_platform_topology_verdict_covers_every_workspace_manifest",
        ),
        requirement if requirement.starts_with("P2-") => P2_MAIN,
        "P3-PREDECESSOR-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::predecessor_handoff::phase_three_predecessor_handoff_is_current",
        ),
        "P3-BASELINE-REPLAY-01"
        | "P3-DAMAGE-REPLAY-01"
        | "P3-DRAW-LIST-01"
        | "P3-HP02-WORLD-01"
        | "P3-PHYSICAL-AMPLIFICATION-01"
        | "P3-TRANSACTION-01"
        | "P3-UNCHANGED-01" => P3_NATIVE_MAIN,
        "P3-DELTA-SOURCE-01" | "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01" => P3_MIXED_MAIN,
        "P3-CLOSE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::phase_three_closure_requires_every_predecessor_and_phase_three_row",
        ),
        "P3-CLIPPED-DELTA-01" => integration(
            "worth-ui-certification",
            "application_contracts",
            "platform_pulse::clipped_to_zero_native_delta_advances_without_a_new_physical_epoch",
        ),
        "P3-RECONSTRUCTION-01" => integration(
            "worth-ui-certification",
            "application_contracts",
            "mounted_headless_recorder::reconstruction::missing_surface_state_reconstructs_from_mounted_authority_then_returns_to_local_delta",
        ),
        "P3-DAMAGE-INDEX-01" => library(
            "worth-ui-host-native",
            "native::presentation::damage_index::tests::maximum_overlap_stores_and_probes_each_command_once",
        ),
        "P3-STALE-DELTA-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::delta_source::stale_successor_affinity_is_denied_before_work_issuance",
        ),
        "P3-TOTAL-ORDER-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::equal_layer_successor_reorder_remains_authored_when_identity_order_opposes_it",
        ),
        requirement if requirement.starts_with("P5-") => return phase5::main_for(requirement),
        requirement if requirement.starts_with("P6-") => return phase6::main_for(requirement),
        _ => return None,
    };
    Some(test)
}

pub(super) fn current_predecessor_main_for(requirement: &str) -> Option<TestIdentity> {
    match requirement {
        "P1-DAMAGE-01" => Some(library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::damage_bounds::replacement_damage_is_clipped_to_predecessor_and_successor_visibility",
        )),
        "P3-DELTA-SOURCE-01" | "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01" => {
            Some(P3_MIXED_MAIN)
        }
        _ => main_for(requirement),
    }
}

pub(super) fn control_for(requirement: &str) -> Option<TestIdentity> {
    if requirement.starts_with("P4-") {
        return phase4::control_for(requirement);
    }
    Some(match requirement {
        "P1-CONSUMERS-01" => library(
            "worth-ui-host-egui",
            "adapter::semantic_text::tests::validated_agreement_semantic_text_consumes_and_mixed_contract_stops_before_consumer",
        ),
        "P2-APPLICATION-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_topology::compile_contract_artifact::product_native_driver_substitution_is_compiler_rejected",
        ),
        "P2-CLOSE-01" => library(
            "worth-ui-host-native",
            "native::event_loop::tests::indeterminate_external_work_moves_into_retryable_cleanup_authority",
        ),
        "P2-EVENT-LOOP-01" => library(
            "worth-ui-host-native",
            "native::event_loop::tests::callback_thread_transition_rejects_off_thread_run",
        ),
        "P2-GRAPHICS-01" => library(
            "worth-ui-host-native",
            "native::graphics::tests::adapter_selection_returns_the_exact_qualified_candidate_and_rejects_substitutes",
        ),
        "P2-PIXELS-01" => TestIdentity {
            package: "worth-ui-platform-pulse",
            target_kind: "test",
            target_name: "executable_world",
            features: EXECUTABLE_WORLD,
            test_name: "native_platform::windows::independent_window_capture_rejects_monitor_pixel_substitution",
        },
        "P2-PORTS-01" => library(
            "worth-ui-host-native",
            "native::presentation::tests::external_port_failures_cross_the_real_framework_settlement_transition",
        ),
        "P2-PRESENT-01" => library(
            "worth-ui-host-native",
            "native::presentation::raster::tests::geometry_and_color_are_derived_from_the_admitted_command",
        ),
        "P2-WORLD-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::result_artifact::mutation_tests::phase_two_boundary_observation_rejects_each_causal_mutation",
        ),
        "P2-READINESS-01" => library(
            "worth-ui-host-native",
            "native::readiness::tests::committed_readiness_requests_exactly_one_redraw_and_preserves_the_latest_generation",
        ),
        "P2-WINDOW-01" => library(
            "worth-ui-host-native",
            "native::graphics::tests::window_basis_classifier_rearms_only_for_new_scale_or_nonzero_extent",
        ),
        "P3-PREDECESSOR-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::stale_source_or_missing_row_is_rejected",
        ),
        "P3-BASELINE-REPLAY-01" => library(
            "worth-ui-host-native",
            "native::presentation::delta::tests::opaque_replay_baseline_is_rejected_before_raster_work",
        ),
        "P3-DAMAGE-INDEX-01" => library(
            "worth-ui-host-native",
            "native::presentation::damage_index::tests::sparse_and_same_center_adversaries_use_exact_two_dimensional_pruning",
        ),
        "P3-DAMAGE-REPLAY-01" => library(
            "worth-ui-host-native",
            "native::presentation::retained_draw_list::tests::replay_tests::removing_the_top_command_replays_the_vacated_underlying_command",
        ),
        "P3-DRAW-LIST-01" => library(
            "worth-ui-host-native",
            "native::presentation::retained_draw_list::tests::delta_transaction_tests::exact_delta_updates_draw_order_damage_and_replay_without_retained_scans",
        ),
        "P3-DELTA-SOURCE-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::producer_slope::admitted_sources_leave_only_local_work_inside_delta_issuance",
        ),
        "P3-PRODUCER-SLOPE-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::producer_slope::complete_retained_scan_and_clone_mutant_is_rejected_by_the_local_oracle",
        ),
        "P3-HEADLESS-COST-01" => library(
            "worth-ui-host-headless",
            "headless_recorder::presentation::tests::ordinary_delta_returns_one_delta_record_without_parallel_retained_history",
        ),
        "P3-HP02-WORLD-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_topology::phase_three_application::phase_three_world_accepts_only_semantic_program_input_through_the_ordinary_driver",
        ),
        "P3-PHYSICAL-AMPLIFICATION-01" => library(
            "worth-ui-host-native",
            "native::presentation::delta::tests::physical_delta_cost_exposes_the_full_surface_amplification_boundary",
        ),
        "P3-RECONSTRUCTION-01" => library(
            "worth-ui-host-native",
            "native::mechanics_adapter::presentation::tests::derived_state_loss_rejects_without_effects_until_owner_reconstruction_arrives",
        ),
        "P3-STALE-DELTA-01" => library(
            "worth-ui-host-native",
            "native::presentation::retained_draw_list::tests::stale_delta_denies_without_mutating_retained_commands",
        ),
        "P3-TOTAL-ORDER-01" => library(
            "worth-ui-host-native",
            "native::presentation::retained_order::tests::repeated_insertions_into_one_gap_keep_a_bounded_balanced_index",
        ),
        "P3-TRANSACTION-01" => library(
            "worth-ui-host-native",
            "native::presentation::retained_draw_list::tests::superseded_transaction_tests::superseded_then_rejected_pending_deltas_restore_host_truth_and_resources",
        ),
        "P3-UNCHANGED-01" => library(
            "worth-ui-host-native",
            "native::mechanics_adapter::presentation::tests::unchanged_reuses_the_last_physical_presentation_epoch",
        ),
        "P3-CLOSE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::mutation_tests::phase_closure_mode_rejects_open_rows_at_or_before_its_gate",
        ),
        "P3-CLIPPED-DELTA-01" => library(
            "worth-ui-host-native",
            "native::presentation::delta::tests::offscreen_delta_advances_retained_truth_without_physical_work",
        ),
        requirement if requirement.starts_with("P5-") => return phase5::control_for(requirement),
        requirement if requirement.starts_with("P6-") => return phase6::control_for(requirement),
        _ => return None,
    })
}

pub(super) use compile_cases::compile_cases_for;

#[path = "execution_posture.rs"]
mod posture;
pub(super) use posture::{
    control_budget_ms, counter_amount, current_predecessor_counter_amount,
    expected_declared_ignored, fault_boundary, is_shared_main, main_budget_ms,
};
