from __future__ import annotations

from typing import Any


PREDECESSOR_VALIDATOR = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "milestone_3141_phase1_ledger/predecessor_artifact.rs"
)
PREDECESSOR_ORACLE = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "milestone_3141_phase1_ledger/predecessor_handoff.rs"
)
PREDECESSOR_TEST = (
    "milestone_3141_phase1_ledger::predecessor_handoff::"
    "phase_three_predecessor_handoff_is_current"
)
PREDECESSOR_CONTROL = (
    "milestone_3141_phase1_ledger::predecessor_artifact::tests::"
    "stale_source_or_missing_row_is_rejected"
)
NATIVE_WORLD_SOURCE = (
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/"
    "courtroom/native_phase3.rs"
)
NATIVE_WORLD_TEST = (
    "courtroom::native_phase3::"
    "maximum_overlap_deltas_cross_public_runtime_native_pixels_and_exact_costs"
)
MIXED_WORLD_SOURCE = (
    "workspaces/worth-ui/crates/worth-ui-certification/tests/"
    "application_contracts/host_platform/mod.rs"
)
MIXED_WORLD_TEST = "host_platform::mixed_carrier_successors_are_local_at_the_4096_command_ceiling"
MIXED_WORLD_ARTIFACT = "_docs/worth-ui/milestone-3.14.1-evidence/p3-delta-source-01.json"


def unique_sources(*sources: str) -> tuple[str, ...]:
    return tuple(dict.fromkeys(sources))


def build_p3_proofs(
    proof_type: Any, control_type: Any, predecessor_artifact: str
) -> dict[str, Any]:
    control = control_type(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        PREDECESSOR_CONTROL,
        PREDECESSOR_VALIDATOR,
    )
    result = {
        "P3-PREDECESSOR-01": proof_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            PREDECESSOR_TEST,
            f"{PREDECESSOR_VALIDATOR}::validate",
            f"{PREDECESSOR_ORACLE}::phase_three_predecessor_handoff_is_current",
            (
                PREDECESSOR_VALIDATOR,
                PREDECESSOR_ORACLE,
                "scripts/ci/verify_worth_ui_3141_ledger.py",
                "scripts/ci/worth_ui_ledger_phase_two_portfolio.py",
                "scripts/ci/worth_ui_ledger_phase_three_portfolio.py",
                "scripts/ci/worth_ui_ledger_operational_successors.py",
                "scripts/ci/worth_ui_predecessor_handoff.py",
                "scripts/ci/worth_ui_ledger_source_state.py",
                predecessor_artifact,
            ),
            control=control,
        )
    }
    native_sources = (
        NATIVE_WORLD_SOURCE,
        "workspaces/worth-ui/apps/platform-pulse/src/native_phase3_application.rs",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/native_application_program.rs",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/native_application_shell/component_presence.rs",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application_driver.rs",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs",
        "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mechanics_adapter/presentation.rs",
        "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/delta.rs",
    )
    native_controls = {
        "P3-BASELINE-REPLAY-01": native_control(
            control_type,
            "native::presentation::delta::tests::opaque_replay_baseline_is_rejected_before_raster_work",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/delta.rs",
        ),
        "P3-DAMAGE-REPLAY-01": native_control(
            control_type,
            "native::presentation::retained_draw_list::tests::replay_tests::removing_the_top_command_replays_the_vacated_underlying_command",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/retained_draw_list/replay_tests.rs",
        ),
        "P3-DRAW-LIST-01": native_control(
            control_type,
            "native::presentation::retained_draw_list::tests::delta_transaction_tests::exact_delta_updates_draw_order_damage_and_replay_without_retained_scans",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/retained_draw_list/delta_transaction_tests.rs",
        ),
        "P3-PHYSICAL-AMPLIFICATION-01": native_control(
            control_type,
            "native::presentation::delta::tests::physical_delta_cost_exposes_the_full_surface_amplification_boundary",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/delta.rs",
        ),
        "P3-TRANSACTION-01": native_control(
            control_type,
            "native::presentation::retained_draw_list::tests::delta_transaction_tests::exact_delta_updates_draw_order_damage_and_replay_without_retained_scans",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/retained_draw_list/delta_transaction_tests.rs",
        ),
        "P3-UNCHANGED-01": native_control(
            control_type,
            "native::mechanics_adapter::presentation::tests::unchanged_reuses_the_last_physical_presentation_epoch",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mechanics_adapter/presentation_tests.rs",
        ),
    }
    native_entries = {
        "P3-BASELINE-REPLAY-01": "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/pipeline.rs::draw_presentation_operations",
        "P3-DAMAGE-REPLAY-01": "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/retained_draw_list/replay.rs::replay_plan",
        "P3-DRAW-LIST-01": "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/retained_draw_list/delta_transaction.rs::stage_delta",
        "P3-PHYSICAL-AMPLIFICATION-01": "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/delta.rs::delta_cost",
        "P3-TRANSACTION-01": "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/delta.rs::settle_staged_delta",
        "P3-UNCHANGED-01": "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mechanics_adapter/presentation/retained_frame.rs::retain_unchanged",
    }
    for requirement, entry in native_entries.items():
        result[requirement] = proof_type(
            "worth-ui-platform-pulse",
            ("test", "executable_world"),
            NATIVE_WORLD_TEST,
            entry,
            f"{NATIVE_WORLD_SOURCE}::maximum_overlap_deltas_cross_public_runtime_native_pixels_and_exact_costs",
            unique_sources(
                *native_sources,
                entry.rsplit("::", 1)[0],
                native_controls[requirement].source,
            ),
            features=("executable-world",),
            control=native_controls[requirement],
        )
    result.update(non_native_proofs(proof_type, control_type))
    result["P3-RECONSTRUCTION-01"] = proof_type(
        "worth-ui-certification",
        ("test", "application_contracts"),
        "mounted_headless_recorder::reconstruction::missing_surface_state_reconstructs_from_mounted_authority_then_returns_to_local_delta",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_reconstruction",
        "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/mounted_headless_recorder/reconstruction.rs::missing_surface_state_reconstructs_from_mounted_authority_then_returns_to_local_delta",
        (
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/mounted_headless_recorder/reconstruction.rs",
            "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/retained_draw_list/reconstruction_tests.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mechanics_adapter/presentation_tests.rs",
        ),
        control=native_control(
            control_type,
            "native::mechanics_adapter::presentation::tests::derived_state_loss_rejects_without_effects_until_owner_reconstruction_arrives",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mechanics_adapter/presentation_tests.rs",
        ),
    )
    result["P3-CLIPPED-DELTA-01"] = proof_type(
        "worth-ui-certification",
        ("test", "application_contracts"),
        "platform_pulse::clipped_to_zero_native_delta_advances_without_a_new_physical_epoch",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/terminal.rs::completion_satisfies",
        "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/platform_pulse.rs::clipped_to_zero_native_delta_advances_without_a_new_physical_epoch",
        (
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/platform_pulse.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/terminal.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/certification_support/scripted_presentation_host/mod.rs",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/certification_support/scripted_presentation_host/adapter.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/delta.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/delta_tests.rs",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/raster.rs",
        ),
        control=native_control(
            control_type,
            "native::presentation::delta::tests::offscreen_delta_advances_retained_truth_without_physical_work",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/delta_tests.rs",
        ),
    )
    result["P3-HP02-WORLD-01"] = proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        NATIVE_WORLD_TEST,
        "workspaces/worth-ui/apps/platform-pulse/src/main.rs::run_native_phase3_world",
        f"{NATIVE_WORLD_SOURCE}::maximum_overlap_deltas_cross_public_runtime_native_pixels_and_exact_costs",
        native_sources
        + (
            "workspaces/worth-ui/apps/platform-pulse/src/main.rs",
            MIXED_WORLD_SOURCE,
            MIXED_WORLD_ARTIFACT,
            "scripts/ci/worth_ui_3141_supporting_world.py",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/phase_three_application.rs",
        ),
        features=("executable-world",),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_topology::phase_three_application::phase_three_world_accepts_only_semantic_program_input_through_the_ordinary_driver",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/phase_three_application.rs",
        ),
    )
    return result


def native_control(control_type: Any, test: str, source: str) -> Any:
    return control_type("worth-ui-host-native", ("lib", "lib"), test, source)


def non_native_proofs(proof_type: Any, control_type: Any) -> dict[str, Any]:
    return {
        "P3-CLOSE-01": proof_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::phase_three_closure_requires_every_predecessor_and_phase_three_row",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs::validate_phase_closure",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs::phase_three_closure_requires_every_predecessor_and_phase_three_row",
            ("workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs",),
            control=control_type(
                "worth-ui-certification",
                ("test", "topology_contracts"),
                "milestone_3141_phase1_ledger::mutation_tests::phase_closure_mode_rejects_open_rows_at_or_before_its_gate",
                "workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger/mutation_tests.rs",
            ),
        ),
        "P3-DAMAGE-INDEX-01": library_proof(
            proof_type, control_type, "worth-ui-host-native",
            "native::presentation::damage_index::tests::maximum_overlap_stores_and_probes_each_command_once",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/damage_index.rs::intersecting",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/damage_index/tests.rs",
            "native::presentation::damage_index::tests::sparse_and_same_center_adversaries_use_exact_two_dimensional_pruning",
            extra_sources=(
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/damage_index/aabb.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/damage_index/arena.rs",
                "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/damage_index/hierarchy.rs",
            ),
        ),
        "P3-DELTA-SOURCE-01": mixed_world_proof(
            proof_type,
            control_type,
            "mounting::presentation::work_producer_tests::producer_slope::admitted_sources_leave_only_local_work_inside_delta_issuance",
            extra_sources=(
                "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests/world.rs",
            ),
        ),
        "P3-HEADLESS-COST-01": mixed_world_proof(
            proof_type, control_type,
            "headless_recorder::presentation::tests::ordinary_delta_returns_one_delta_record_without_parallel_retained_history",
            entry="workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation.rs::work_cost",
            extra_sources=(
                "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation/delta.rs",
                "workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation/node_delta.rs",
            ),
            control_package="worth-ui-host-headless",
            control_source="workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation_tests.rs",
        ),
        "P3-PRODUCER-SLOPE-01": mixed_world_proof(
            proof_type,
            control_type,
            "mounting::presentation::work_producer_tests::producer_slope::admitted_sources_leave_only_local_work_inside_delta_issuance",
            extra_sources=(
                "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests/world.rs",
            ),
        ),
        "P3-STALE-DELTA-01": library_proof(
            proof_type, control_type, "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::delta_source::stale_successor_affinity_is_denied_before_work_issuance",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_successor",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests/delta_source.rs",
            "native::presentation::retained_draw_list::tests::stale_delta_denies_without_mutating_retained_commands",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/retained_draw_list_tests.rs",
            control_package="worth-ui-host-native",
        ),
        "P3-TOTAL-ORDER-01": library_proof(
            proof_type, control_type, "worth-ui-runtime",
            "mounting::presentation::work_producer_tests::equal_layer_successor_reorder_remains_authored_when_identity_order_opposes_it",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer/state.rs::from_projection",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs",
            "native::presentation::retained_order::tests::repeated_insertions_into_one_gap_keep_a_bounded_balanced_index",
            "workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/retained_order.rs",
            control_package="worth-ui-host-native",
        ),
    }


def library_proof(
    proof_type: Any, control_type: Any, package: str, test: str,
    entry: str, source: str, control_test: str, control_source: str | None = None,
    extra_sources: tuple[str, ...] = (), control_package: str | None = None,
) -> Any:
    return proof_type(
        package, ("lib", "lib"), test, entry, f"{source}::{test.rsplit('::', 1)[1]}",
        unique_sources(entry.rsplit("::", 1)[0], source, *extra_sources),
        control=control_type(
            control_package or package, ("lib", "lib"), control_test, control_source or source
        ),
    )


def mixed_world_proof(
    proof_type: Any,
    control_type: Any,
    control_test: str,
    entry: str = "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_successor",
    extra_sources: tuple[str, ...] = (),
    control_package: str = "worth-ui-runtime",
    control_source: str = "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests/producer_slope.rs",
) -> Any:
    return proof_type(
        "worth-ui-certification", ("test", "application_contracts"), MIXED_WORLD_TEST,
        entry,
        f"{MIXED_WORLD_SOURCE}::mixed_carrier_successors_are_local_at_the_4096_command_ceiling",
        (
            MIXED_WORLD_SOURCE,
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/mixed_carrier.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/mixed_carrier/application.rs",
            entry.rsplit("::", 1)[0],
            *extra_sources,
        ),
        control=control_type(
            control_package, ("lib", "lib"), control_test, control_source,
        ),
    )
