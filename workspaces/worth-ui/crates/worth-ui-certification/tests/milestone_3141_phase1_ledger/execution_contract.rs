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
    let test = match requirement {
        "P1-AFFINITY-01" => library("worth-ui-runtime", "mounting::presentation::work_producer_tests::one_replacement_carries_one_change_and_exact_predecessor_successor_damage"),
        "P1-AUTHORITY-01" | "P1-ORDER-SOURCE-01" | "P1-PLATFORM-AUTHORITY-01"
        | "P1-PRESENTATION-AUTHORITY-01" | "P1-PROTOCOL-01" => COMPILE_MAIN,
        "P1-BACKEND-FEATURES-01" => integration("worth-ui-certification", "topology_contracts", "milestone_3141_phase1_topology::resolved_graphs::default_all_feature_and_windows_resolved_graphs_are_exact_and_mutation_sensitive"),
        "P1-BASELINE-01" => library("worth-ui-runtime", "mounting::presentation::coordinator::admission::tests::actual_baseline_registration_gates_the_presentation_admission_transition"),
        "P1-CLOSE-01" => integration("worth-ui-certification", "topology_contracts", "milestone_3141_phase1_ledger::phase_one_closure_prerequisites_are_final_source"),
        "P1-CONSUMERS-01" => library("worth-ui-host-headless", "headless_static_paint_tests::validated_agreement_static_paint_consumes_and_mixed_contract_stops_before_consumer"),
        "P1-DAMAGE-01" => library("worth-ui-runtime", "mounting::presentation::work_producer_tests::replacement_damage_is_clipped_to_predecessor_and_successor_visibility"),
        "P1-HEADLESS-01" => integration("worth-ui-certification", "application_contracts", "mounted_headless_recorder::real_cross_lane_recording_preserves_exact_unperformed_external_mechanics"),
        "P1-HEADLESS-COST-01" | "P1-WORLDS-01" => integration("worth-ui-certification", "application_contracts", "host_platform::maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work"),
        "P1-ORDER-01" => library("worth-ui-runtime", "mounting::presentation::work_producer_tests::equal_layer_total_order_follows_authored_node_order_not_command_identity"),
        "P1-PRODUCER-01" => library("worth-ui-runtime", "mounting::presentation::work_producer_tests::removal_and_insert_carry_exact_identities_vacated_damage_and_total_order"),
        "P1-PRODUCER-COST-01" => library("worth-ui-runtime", "mounting::presentation::work_producer_tests::unchanged_successor_carries_zero_command_order_and_damage_work"),
        "P1-PREPARATION-LIFECYCLE-01" => integration("worth-ui-certification", "topology_contracts", "milestone_3141_phase1_topology::phase_one_product_preparation_is_effect_free_and_host_neutral"),
        "P1-PROFILE-01" => library("worth-ui-host-native", "qualification_tests::every_qualified_semantic_and_dependency_pin_matches_the_closed_record"),
        "P1-TOPOLOGY-01" => integration("worth-ui-certification", "topology_contracts", "milestone_3141_phase1_topology::phase_one_host_platform_topology_verdict_covers_every_workspace_manifest"),
        requirement if requirement.starts_with("P2-") => P2_MAIN,
        _ => return None,
    };
    Some(test)
}

pub(super) fn control_for(requirement: &str) -> Option<TestIdentity> {
    Some(match requirement {
        "P1-CONSUMERS-01" => library("worth-ui-host-egui", "adapter::semantic_text::tests::validated_agreement_semantic_text_consumes_and_mixed_contract_stops_before_consumer"),
        "P2-APPLICATION-01" => integration("worth-ui-certification", "topology_contracts", "milestone_3141_phase1_topology::compile_contract_artifact::product_native_driver_substitution_is_compiler_rejected"),
        "P2-CLOSE-01" => library("worth-ui-host-native", "native::event_loop::tests::indeterminate_external_work_moves_into_retryable_cleanup_authority"),
        "P2-EVENT-LOOP-01" => library("worth-ui-host-native", "native::event_loop::tests::callback_thread_transition_rejects_off_thread_run"),
        "P2-GRAPHICS-01" => library("worth-ui-host-native", "native::graphics::tests::adapter_selection_returns_the_exact_qualified_candidate_and_rejects_substitutes"),
        "P2-PIXELS-01" => TestIdentity {
            package: "worth-ui-platform-pulse",
            target_kind: "test",
            target_name: "executable_world",
            features: EXECUTABLE_WORLD,
            test_name: "native_platform::windows::independent_window_capture_rejects_monitor_pixel_substitution",
        },
        "P2-PORTS-01" => library("worth-ui-host-native", "native::presentation::tests::external_port_failures_cross_the_real_framework_settlement_transition"),
        "P2-PRESENT-01" => library("worth-ui-host-native", "native::presentation::raster::tests::geometry_and_color_are_derived_from_the_admitted_command"),
        "P2-WORLD-01" => integration("worth-ui-certification", "topology_contracts", "milestone_3141_phase1_ledger::result_artifact::mutation_tests::phase_two_boundary_observation_rejects_each_causal_mutation"),
        "P2-READINESS-01" => library("worth-ui-host-native", "native::readiness::tests::committed_readiness_requests_exactly_one_redraw_and_preserves_the_latest_generation"),
        "P2-WINDOW-01" => library("worth-ui-host-native", "native::graphics::tests::window_basis_classifier_rearms_only_for_new_scale_or_nonzero_extent"),
        _ => return None,
    })
}

const AUTHORITY_CASES: &[CompileCase] = &[
    case(
        "product",
        "fail",
        "product-native-preparation-no-builder-extraction",
    ),
    case("product", "pass", "product-native-preparation-valid"),
];
const ORDER_CASES: &[CompileCase] = &[
    case("product", "fail", "product-paint-identities-non-orderable"),
    case(
        "certification",
        "pass",
        "product-paint-identities-lawful-correlation",
    ),
];
const PLATFORM_CASES: &[CompileCase] = &[
    case("product", "fail", "product-cannot-bind-native-host"),
    case("product", "pass", "product-native-preparation-valid"),
];
const PRESENTATION_CASES: &[CompileCase] = &[
    case("host", "fail", "host-presentation-work-authority"),
    case("host", "pass", "host-presentation-mechanics-consumer"),
];
const PROTOCOL_CASES: &[CompileCase] = &[
    case("host", "pass", "host-presentation-mechanics-consumer"),
    case(
        "product",
        "fail",
        "product-raw-protocol-consumer-substitution",
    ),
];

const fn case(owner: &'static str, kind: &'static str, target: &'static str) -> CompileCase {
    CompileCase {
        owner,
        kind,
        target,
    }
}

pub(super) fn compile_cases_for(requirement: &str) -> &'static [CompileCase] {
    match requirement {
        "P1-AUTHORITY-01" => AUTHORITY_CASES,
        "P1-ORDER-SOURCE-01" => ORDER_CASES,
        "P1-PLATFORM-AUTHORITY-01" => PLATFORM_CASES,
        "P1-PRESENTATION-AUTHORITY-01" => PRESENTATION_CASES,
        "P1-PROTOCOL-01" => PROTOCOL_CASES,
        _ => &[],
    }
}

pub(super) fn counter_amount(requirement: &str) -> Option<u64> {
    Some(match requirement {
        "P1-AFFINITY-01" => 3,
        "P1-CLOSE-01" => 20,
        "P1-CONSUMERS-01" => 2,
        "P1-AUTHORITY-01"
        | "P1-DAMAGE-01"
        | "P1-ORDER-01"
        | "P1-PLATFORM-AUTHORITY-01"
        | "P1-PRESENTATION-AUTHORITY-01"
        | "P1-PROFILE-01" => 2,
        "P1-ORDER-SOURCE-01" => 2,
        "P1-PROTOCOL-01" => 4,
        "P1-HEADLESS-COST-01"
        | "P1-PRODUCER-COST-01"
        | "P1-PREPARATION-LIFECYCLE-01"
        | "P2-CLOSE-01" => 0,
        "P1-TOPOLOGY-01" => 25,
        "P1-WORLDS-01" => 2_048,
        "P2-PIXELS-01" => 3,
        "P2-PORTS-01" => 4,
        "P1-PRODUCER-01" => 2,
        _ if main_for(requirement).is_some() => 1,
        _ => return None,
    })
}

pub(super) fn fault_boundary(requirement: &str) -> Option<&'static str> {
    Some(match requirement {
        requirement if requirement.starts_with("P1-") => "not-applicable",
        "P2-APPLICATION-01" | "P2-EVENT-LOOP-01" | "P2-GRAPHICS-01" | "P2-READINESS-01"
        | "P2-WINDOW-01" => "before-effects",
        requirement if requirement.starts_with("P2-") => "after-effects-may-have-begun",
        _ => return None,
    })
}

pub(super) fn main_budget_ms(requirement: &str) -> u64 {
    if requirement.starts_with("P2-") {
        30_000
    } else {
        60_000
    }
}

pub(super) fn expected_declared_ignored(requirement: &str) -> bool {
    matches!(
        requirement,
        "P1-CLOSE-01" | "P1-HEADLESS-COST-01" | "P1-WORLDS-01"
    ) || requirement.starts_with("P2-")
}
