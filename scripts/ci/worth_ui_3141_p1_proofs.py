from __future__ import annotations


RUNTIME_TESTS = (
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/"
    "work_producer_tests.rs"
)
HOST_WORLD = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "application_contracts/host_platform/mod.rs"
)
WORLD_TEST = (
    "host_platform::maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work"
)
TOPOLOGY = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "milestone_3141_phase1_topology.rs"
)


def retained_work_proofs(rust_lib, rust_test):
    return {
        "P1-AFFINITY-01": rust_lib(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::one_replacement_carries_one_change_and_exact_predecessor_successor_damage",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_successor",
            f"{RUNTIME_TESTS}::one_replacement_carries_one_change_and_exact_predecessor_successor_damage",
        ),
        "P1-DAMAGE-01": rust_lib(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::replacement_damage_is_clipped_to_predecessor_and_successor_visibility",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer/delta_diff.rs::append_damage",
            f"{RUNTIME_TESTS}::replacement_damage_is_clipped_to_predecessor_and_successor_visibility",
        ),
        "P1-HEADLESS-01": rust_test(
            "worth-ui-certification",
            "application_contracts",
            "mounted_headless_recorder::real_cross_lane_recording_preserves_exact_unperformed_external_mechanics",
            "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation.rs::prepare_candidate",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/mounted_headless_recorder.rs::real_cross_lane_recording_preserves_exact_unperformed_external_mechanics",
        ),
        "P1-HEADLESS-COST-01": rust_test(
            "worth-ui-certification", "application_contracts", WORLD_TEST,
            "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation.rs::work_cost",
            f"{HOST_WORLD}::maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work",
        ),
    }


def ordering_proofs(compile_proof, rust_lib):
    return {
        "P1-ORDER-01": rust_lib(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::equal_layer_total_order_follows_authored_node_order_not_command_identity",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer/order_source.rs::commands_and_total_order",
            f"{RUNTIME_TESTS}::equal_layer_total_order_follows_authored_node_order_not_command_identity",
        ),
        "P1-ORDER-SOURCE-01": compile_proof(
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer/order_source.rs::commands_and_total_order"
        ),
        "P1-PRODUCER-01": rust_lib(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::removal_and_insert_carry_exact_identities_vacated_damage_and_total_order",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_successor",
            f"{RUNTIME_TESTS}::removal_and_insert_carry_exact_identities_vacated_damage_and_total_order",
        ),
        "P1-PRODUCER-COST-01": rust_lib(
            "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::unchanged_successor_carries_zero_command_order_and_damage_work",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_successor",
            f"{RUNTIME_TESTS}::unchanged_successor_carries_zero_command_order_and_damage_work",
        ),
    }


def authority_proofs(compile_proof, rust_lib, rust_test):
    return {
        "P1-AUTHORITY-01": compile_proof(
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application.rs::complete"
        ),
        "P1-BASELINE-01": rust_lib(
            "worth-ui-runtime",
            "mounting::presentation::coordinator::admission::tests::actual_baseline_registration_gates_the_presentation_admission_transition",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/admission.rs::baseline_requirement_denial",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/admission.rs::actual_baseline_registration_gates_the_presentation_admission_transition",
        ),
        "P1-PLATFORM-AUTHORITY-01": compile_proof(
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/native_platform_binding.rs::issue"
        ),
        "P1-PREPARATION-LIFECYCLE-01": rust_test(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_topology::phase_one_product_preparation_is_effect_free_and_host_neutral",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/platform/preparation.rs::prepare",
            f"{TOPOLOGY}::phase_one_product_preparation_is_effect_free_and_host_neutral",
        ),
        "P1-PRESENTATION-AUTHORITY-01": compile_proof(
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/authority/work.rs::issue_delta"
        ),
        "P1-PROTOCOL-01": compile_proof(
            "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/presentation_work/delta.rs::from_inert_mechanics"
        ),
    }


def topology_profile_proofs(rust_lib, rust_test):
    return {
        "P1-BACKEND-FEATURES-01": rust_test(
            "worth-ui-certification", "topology_contracts",
            "milestone_3141_phase1_topology::resolved_graphs::default_all_feature_and_windows_resolved_graphs_are_exact_and_mutation_sensitive",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests/qualified_dependencies.rs::assert_qualified_dependencies",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/resolved_graphs.rs::default_all_feature_and_windows_resolved_graphs_are_exact_and_mutation_sensitive",
        ),
        "P1-CONSUMERS-01": rust_test(
            "worth-ui-certification", "topology_contracts",
            "milestone_3141_phase1_topology::phase_one_consumer_inventory_rejects_legacy_protocol_branches",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/consumption_view.rs::bind",
            f"{TOPOLOGY}::phase_one_consumer_inventory_rejects_legacy_protocol_branches",
        ),
        "P1-PROFILE-01": rust_lib(
            "worth-ui-host-native",
            "qualification_tests::every_qualified_semantic_and_dependency_pin_matches_the_closed_record",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests/qualified_dependencies.rs::assert_qualified_dependencies",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests.rs::every_qualified_semantic_and_dependency_pin_matches_the_closed_record",
        ),
        "P1-TOPOLOGY-01": rust_test(
            "worth-ui-certification", "topology_contracts",
            "milestone_3141_phase1_topology::phase_one_host_platform_topology_verdict_covers_every_workspace_manifest",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/topology_verdict.rs::validate_topology",
            f"{TOPOLOGY}::phase_one_host_platform_topology_verdict_covers_every_workspace_manifest",
        ),
        "P1-WORLDS-01": rust_test(
            "worth-ui-certification", "application_contracts", WORLD_TEST,
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/world/production.rs::produce_maximum_overlap",
            f"{HOST_WORLD}::maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work",
        ),
    }


def build_p1_proofs(compile_proof, rust_lib, rust_test):
    result = retained_work_proofs(rust_lib, rust_test)
    result.update(ordering_proofs(compile_proof, rust_lib))
    result.update(authority_proofs(compile_proof, rust_lib, rust_test))
    result.update(topology_profile_proofs(rust_lib, rust_test))
    return result
