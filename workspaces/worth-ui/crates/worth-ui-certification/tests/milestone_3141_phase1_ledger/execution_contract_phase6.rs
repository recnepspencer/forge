use super::TestIdentity;

const EMPTY: &[&str] = &[];
const EXECUTABLE_WORLD: &[&str] = &["executable-world"];

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

const WINDOWS_WORLD: TestIdentity = TestIdentity {
    package: "worth-ui-platform-pulse",
    target_kind: "test",
    target_name: "executable_world",
    features: EXECUTABLE_WORLD,
    test_name: "courtroom::native_phase6::windows_native_boundary_world_retains_click_time_pointer_after_cursor_moves",
};

pub(super) fn main_for(requirement: &str) -> Option<TestIdentity> {
    Some(match requirement {
        "P6-PREDECESSOR-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::predecessor_handoff::phase_six_predecessor_handoff_is_current",
        ),
        "P6-INPUT-AFFINITY-01" => library(
            "worth-ui-host-native",
            "native::input::observation::tests::completed_affinity_and_event_profile_are_carried_in_order",
        ),
        "P6-IME-01" => library(
            "worth-ui-host-native",
            "native::input::observation::tests::ime_keeps_preedit_commit_and_cancel_distinct_and_converts_bytes",
        ),
        "P6-POINTER-TIME-01" => library(
            "worth-ui-host-native",
            "native::input::observation::phase6_tests::button_event_uses_the_event_time_position_witness",
        ),
        "P6-PROFILE-ORDER-01" => library(
            "worth-ui-host-native",
            "native::input::observation::phase6_tests::resize_observation_keeps_the_resize_event_tick_after_completion",
        ),
        "P6-READINESS-01" => library(
            "worth-ui-host-native",
            "native::readiness::tests::physical_level_wake_coalesces_until_the_event_thread_consumes_it",
        ),
        "P6-SETTLEMENT-01" => library(
            "worth-ui-runtime",
            "facade::entry::native_observation_tests::native_observation_ready_path_drains_through_runtime_interaction_owner",
        ),
        "P6-PROTOCOL-WORLD-01" => integration(
            "worth-ui-certification",
            "application_contracts",
            "phase6_native_lifecycle::protocol_world::native_lifecycle_protocol_world_matches_independent_oracle_for_all_schedules",
        ),
        "P6-WINDOWS-WORLD-01" => WINDOWS_WORLD,
        "P6-CLOSE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::phase_six_closure_requires_every_predecessor_and_phase_six_row",
        ),
        _ => return None,
    })
}

pub(super) fn control_for(requirement: &str) -> Option<TestIdentity> {
    Some(match requirement {
        "P6-PREDECESSOR-01" => phase6_control(
            "predecessor_control",
            "predecessor_handoff_mutation_is_rejected",
        ),
        "P6-INPUT-AFFINITY-01" => phase6_control(
            "input_affinity_control",
            "input_affinity_mutation_is_rejected",
        ),
        "P6-IME-01" => phase6_control("ime_control", "ime_phase_mutation_is_rejected"),
        "P6-POINTER-TIME-01" => {
            phase6_control("pointer_time_control", "pointer_time_mutation_is_rejected")
        }
        "P6-PROFILE-ORDER-01" => phase6_control(
            "profile_order_control",
            "profile_order_mutation_is_rejected",
        ),
        "P6-READINESS-01" => phase6_control("readiness_control", "readiness_mutation_is_rejected"),
        "P6-SETTLEMENT-01" => {
            phase6_control("settlement_control", "settlement_mutation_is_rejected")
        }
        "P6-PROTOCOL-WORLD-01" => phase6_control(
            "protocol_control",
            "oracle_substitution_mutation_is_rejected",
        ),
        "P6-WINDOWS-WORLD-01" => phase6_control(
            "windows_control",
            "windows_pointer_source_mutation_is_rejected",
        ),
        "P6-CLOSE-01" => phase6_control("close_control", "close_requirement_mutation_is_rejected"),
        _ => return None,
    })
}

fn phase6_control(module: &'static str, function: &'static str) -> TestIdentity {
    integration(
        "worth-ui-certification",
        "application_contracts",
        match (module, function) {
            ("predecessor_control", "predecessor_handoff_mutation_is_rejected") => {
                "phase6_native_lifecycle::predecessor_control::predecessor_handoff_mutation_is_rejected"
            }
            ("input_affinity_control", "input_affinity_mutation_is_rejected") => {
                "phase6_native_lifecycle::input_affinity_control::input_affinity_mutation_is_rejected"
            }
            ("ime_control", "ime_phase_mutation_is_rejected") => {
                "phase6_native_lifecycle::ime_control::ime_phase_mutation_is_rejected"
            }
            ("pointer_time_control", "pointer_time_mutation_is_rejected") => {
                "phase6_native_lifecycle::pointer_time_control::pointer_time_mutation_is_rejected"
            }
            ("profile_order_control", "profile_order_mutation_is_rejected") => {
                "phase6_native_lifecycle::profile_order_control::profile_order_mutation_is_rejected"
            }
            ("readiness_control", "readiness_mutation_is_rejected") => {
                "phase6_native_lifecycle::readiness_control::readiness_mutation_is_rejected"
            }
            ("settlement_control", "settlement_mutation_is_rejected") => {
                "phase6_native_lifecycle::settlement_control::settlement_mutation_is_rejected"
            }
            ("protocol_control", "oracle_substitution_mutation_is_rejected") => {
                "phase6_native_lifecycle::protocol_control::oracle_substitution_mutation_is_rejected"
            }
            ("windows_control", "windows_pointer_source_mutation_is_rejected") => {
                "phase6_native_lifecycle::windows_control::windows_pointer_source_mutation_is_rejected"
            }
            ("close_control", "close_requirement_mutation_is_rejected") => {
                "phase6_native_lifecycle::close_control::close_requirement_mutation_is_rejected"
            }
            _ => "phase6_native_lifecycle::invalid_control::invalid_control",
        },
    )
}
