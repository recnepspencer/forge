from __future__ import annotations

from worth_ui_3141_closure_sources import (
    CLOSURE_PROTOCOL_SOURCES,
    PREDECESSOR_EXECUTION_SOURCES,
)


CONTROL_ROOT = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "application_contracts/phase6_native_lifecycle/"
)
CONTROL_SUPPORT_SOURCE = CONTROL_ROOT + "mutation_receipt.rs"
CONTROL_SPECS = {
    "P6-PREDECESSOR-01": ("predecessor_control.rs", "predecessor_handoff_mutation_is_rejected"),
    "P6-INPUT-AFFINITY-01": ("input_affinity_control.rs", "input_affinity_mutation_is_rejected"),
    "P6-IME-01": ("ime_control.rs", "ime_phase_mutation_is_rejected"),
    "P6-POINTER-TIME-01": ("pointer_time_control.rs", "pointer_time_mutation_is_rejected"),
    "P6-PROFILE-ORDER-01": ("profile_order_control.rs", "profile_order_mutation_is_rejected"),
    "P6-READINESS-01": ("readiness_control.rs", "readiness_mutation_is_rejected"),
    "P6-SETTLEMENT-01": ("settlement_control.rs", "settlement_mutation_is_rejected"),
    "P6-PROTOCOL-WORLD-01": ("protocol_control.rs", "oracle_substitution_mutation_is_rejected"),
    "P6-WINDOWS-WORLD-01": ("windows_control.rs", "windows_pointer_source_mutation_is_rejected"),
    "P6-CLOSE-01": ("close_control.rs", "close_requirement_mutation_is_rejected"),
}
PROTOCOL_SOURCE = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "application_contracts/phase6_native_lifecycle/protocol_world.rs"
)
ORACLE_SOURCE = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "application_contracts/phase6_native_lifecycle/oracle.rs"
)
PRODUCTION_PROTOCOL_SOURCE = (
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/lifecycle_protocol.rs"
)
PRODUCTION_WORLD_SOURCE = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "application_contracts/phase6_native_lifecycle/production_world.rs"
)
CAUSAL_MUTATION_SOURCE = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "application_contracts/phase6_native_lifecycle/causal_mutation.rs"
)
PRODUCTION_PROTOCOL_SOURCES = (
    PRODUCTION_PROTOCOL_SOURCE,
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/"
    "lifecycle_protocol/phase.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/"
    "lifecycle_protocol/presentation.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/"
    "lifecycle_protocol/transition.rs",
)
LIFECYCLE_WORLD_SOURCES = (
    PROTOCOL_SOURCE,
    PRODUCTION_WORLD_SOURCE,
    *PRODUCTION_PROTOCOL_SOURCES,
)
PROTOCOL_TEST = (
    "phase6_native_lifecycle::"
    "protocol_world::native_lifecycle_protocol_world_matches_independent_oracle_for_all_schedules"
)
WINDOWS_SOURCE = (
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/"
    "courtroom/native_phase6.rs"
)
WINDOWS_TEST = (
    "courtroom::native_phase6::"
    "windows_native_boundary_world_retains_click_time_pointer_after_cursor_moves"
)


def build_p6_proofs(Proof, Control, predecessor_artifact: str) -> dict[str, object]:
    controls = {requirement: _control(Control, requirement) for requirement in CONTROL_SPECS}
    result = {
        "P6-PREDECESSOR-01": Proof(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::predecessor_handoff::phase_six_predecessor_handoff_is_current",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/"
            "milestone_3141_phase1_ledger/predecessor_artifact.rs::validate",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/"
            "milestone_3141_phase1_ledger/predecessor_handoff.rs::phase_six_predecessor_handoff_is_current",
            (
                "workspaces/worth-ui/crates/worth-ui-certification/tests/"
                "milestone_3141_phase1_ledger/predecessor_handoff.rs",
                "workspaces/worth-ui/crates/worth-ui-certification/tests/"
                "milestone_3141_phase1_ledger/predecessor_artifact.rs",
                "workspaces/worth-ui/crates/worth-ui-certification/tests/"
                "milestone_3141_phase1_ledger/predecessor_artifact/ledger_basis.rs",
                "scripts/ci/worth_ui_ledger_candidate_basis.py",
                "scripts/ci/worth_ui_predecessor_handoff_currentness.py",
                "scripts/ci/worth_ui_ledger_execution_identity.py",
                "scripts/ci/worth_ui_ledger_row_cache.py",
                "scripts/ci/worth_ui_predecessor_refresh_runtime.py",
                "scripts/ci/worth_ui_3141_p6_proofs.py",
                *PREDECESSOR_EXECUTION_SOURCES,
                predecessor_artifact,
                controls["P6-PREDECESSOR-01"].source,
                CONTROL_SUPPORT_SOURCE,
            ),
            control=controls["P6-PREDECESSOR-01"],
        ),
        "P6-INPUT-AFFINITY-01": _host(
            Proof,
            controls["P6-INPUT-AFFINITY-01"],
            "native::input::observation::tests::completed_affinity_and_event_profile_are_carried_in_order",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/tests.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation.rs::record_completed_presentation",
            (
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/admission.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/retention.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_focus.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer/capture.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/pointer.rs",
                *LIFECYCLE_WORLD_SOURCES,
                CAUSAL_MUTATION_SOURCE,
                CONTROL_SUPPORT_SOURCE,
            ),
        ),
        "P6-IME-01": _host(
            Proof,
            controls["P6-IME-01"],
            "native::input::observation::tests::ime_keeps_preedit_commit_and_cancel_distinct_and_converts_bytes",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/tests.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_ime.rs::observe_ime",
            (
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_ime.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/text_ime.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/retention.rs",
            *LIFECYCLE_WORLD_SOURCES,
            ),
        ),
        "P6-POINTER-TIME-01": _host(
            Proof,
            controls["P6-POINTER-TIME-01"],
            "native::input::observation::phase6_tests::button_event_uses_the_event_time_position_witness",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/phase6_tests.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer.rs::observe",
            (
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer/button.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer/motion.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer/capture.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/pointer.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/platform/windows.rs",
                *LIFECYCLE_WORLD_SOURCES,
                CAUSAL_MUTATION_SOURCE,
            ),
        ),
        "P6-PROFILE-ORDER-01": _host(
            Proof,
            controls["P6-PROFILE-ORDER-01"],
            "native::input::observation::phase6_tests::resize_observation_keeps_the_resize_event_tick_after_completion",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/phase6_tests.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation.rs::observe_profile_transition_at",
            (
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/profile.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/retention.rs",
                *LIFECYCLE_WORLD_SOURCES,
                CAUSAL_MUTATION_SOURCE,
            ),
        ),
        "P6-READINESS-01": _host(
            Proof,
            controls["P6-READINESS-01"],
            "native::readiness::tests::physical_level_wake_coalesces_until_the_event_thread_consumes_it",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/readiness.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/readiness.rs::signal_level_ready",
            (
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/readiness.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/application_handler.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/run.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/run_preflight.rs",
                *PRODUCTION_PROTOCOL_SOURCES,
            ),
        ),
        "P6-SETTLEMENT-01": _runtime(
            Proof,
            controls["P6-SETTLEMENT-01"],
            "facade::entry::native_observation_tests::native_observation_ready_path_drains_through_runtime_interaction_owner",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/native_observation_tests.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/native_observation_settlement.rs::from_outcomes",
            (
                "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/native_observation_settlement.rs",
                "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/interaction.rs",
                "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/native_application_shell.rs",
                "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application_driver.rs",
                *LIFECYCLE_WORLD_SOURCES,
                CAUSAL_MUTATION_SOURCE,
            ),
        ),
        "P6-PROTOCOL-WORLD-01": Proof(
            "worth-ui-certification",
            ("test", "application_contracts"),
            PROTOCOL_TEST,
            PRODUCTION_PROTOCOL_SOURCE + "::observe_window_event_at",
            PROTOCOL_SOURCE
            + "::native_lifecycle_protocol_world_matches_independent_oracle_for_all_schedules",
            (
            PROTOCOL_SOURCE,
            PRODUCTION_WORLD_SOURCE,
            "workspaces/worth-ui/crates/worth-ui-certification/tests/"
            "application_contracts/phase6_native_lifecycle/schedule_inventory.rs",
            ORACLE_SOURCE,
            *PRODUCTION_PROTOCOL_SOURCES,
            CAUSAL_MUTATION_SOURCE,
            controls["P6-PROTOCOL-WORLD-01"].source,
            CONTROL_SUPPORT_SOURCE,
            ),
            control=controls["P6-PROTOCOL-WORLD-01"],
        ),
        "P6-WINDOWS-WORLD-01": Proof(
            "worth-ui-platform-pulse",
            ("test", "executable_world"),
            WINDOWS_TEST,
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/platform/windows.rs::decode_client_position",
            WINDOWS_SOURCE
            + "::windows_native_boundary_world_retains_click_time_pointer_after_cursor_moves",
            (
                WINDOWS_SOURCE,
                "workspaces/worth-ui/apps/platform-pulse/src/main.rs",
                "workspaces/worth-ui/apps/platform-pulse/src/native_phase6_evidence.rs",
                "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/launch.rs",
                "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/native_platform/windows.rs",
                "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/native_platform/windows/input_delivery.rs",
                "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/native_platform/windows/input_environment.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/platform/windows.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/application_handler.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/event_pointer/button.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/window_port.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/prepared_host.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mechanics_adapter.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mechanics_adapter/presentation.rs",
                "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/native_observation_settlement.rs",
                "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/native_application_shell.rs",
                "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application_driver.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/profiles/worth-ui-windows-dx12-v1.toml",
                *LIFECYCLE_WORLD_SOURCES,
                CAUSAL_MUTATION_SOURCE,
                controls["P6-WINDOWS-WORLD-01"].source,
            ),
            ("executable-world",),
            control=controls["P6-WINDOWS-WORLD-01"],
        ),
        "P6-CLOSE-01": Proof(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::phase_six_closure_requires_every_predecessor_and_phase_six_row",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/"
            "milestone_3141_phase1_ledger/phase_closure.rs::validate_phase_closure",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/"
            "milestone_3141_phase1_ledger.rs::phase_six_closure_requires_every_predecessor_and_phase_six_row",
            (
                "workspaces/worth-ui/crates/worth-ui-certification/tests/"
                "milestone_3141_phase1_ledger.rs",
                "workspaces/worth-ui/crates/worth-ui-certification/tests/"
                "milestone_3141_phase1_ledger/phase_closure.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/close_request.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/finish.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/finish_capture.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/finish_cleanup.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/completion_report.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/application_handler.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/input/observation/retention.rs",
                *LIFECYCLE_WORLD_SOURCES,
                "scripts/ci/worth_ui_3141_p6_proofs.py",
                *CLOSURE_PROTOCOL_SOURCES,
                controls["P6-CLOSE-01"].source,
                CONTROL_SUPPORT_SOURCE,
            ),
            control=controls["P6-CLOSE-01"],
        ),
    }
    return result


def _host(Proof, control, test_name, test_source, production_entry, extra_sources):
    return Proof(
        "worth-ui-host-native",
        ("lib", "lib"),
        test_name,
        production_entry,
        test_source + "::" + test_name.rsplit("::", 1)[-1],
        tuple(
            dict.fromkeys((test_source, *extra_sources, control.source, CONTROL_SUPPORT_SOURCE))
        ),
        control=control,
    )


def _runtime(Proof, control, test_name, test_source, production_entry, extra_sources):
    return Proof(
        "worth-ui-runtime",
        ("lib", "lib"),
        test_name,
        production_entry,
        test_source + "::" + test_name.rsplit("::", 1)[-1],
        tuple(
            dict.fromkeys((test_source, *extra_sources, control.source, CONTROL_SUPPORT_SOURCE))
        ),
        control=control,
    )


def _control(Control, requirement: str):
    source_name, function = CONTROL_SPECS[requirement]
    source = CONTROL_ROOT + source_name
    return Control(
        "worth-ui-certification",
        ("test", "application_contracts"),
        f"phase6_native_lifecycle::{source_name[:-3]}::{function}",
        source,
    )
