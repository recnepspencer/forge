from __future__ import annotations

from typing import Any

from worth_ui_3141_p5_raster_sources import (
    COLOR_RASTER_SOURCES,
    GLYPH_RASTER_SOURCES,
)


CERT_ROOT = "workspaces/worth-ui/crates/worth-ui-certification/tests"
LEDGER = f"{CERT_ROOT}/milestone_3141_phase1_ledger"
NATIVE = "workspaces/worth-ui/crates/worth-ui-host-native/src/native"
PHYSICAL = f"{NATIVE}/physical_work_signal"
ATLAS = f"{NATIVE}/text_atlas"
TEXT = "workspaces/worth-ui/crates/worth-ui-text/src"
TEXT_RASTER = f"{TEXT}/raster"

PHYSICAL_SIGNAL_SOURCES = (
    f"{PHYSICAL}/mod.rs",
    f"{PHYSICAL}/completion_reconciliation.rs",
    f"{PHYSICAL}/construction.rs",
    f"{PHYSICAL}/counters.rs",
    f"{PHYSICAL}/declarations/mod.rs",
    f"{PHYSICAL}/declarations/aspects.rs",
    f"{PHYSICAL}/declarations/resources.rs",
    f"{PHYSICAL}/identity.rs",
    f"{PHYSICAL}/locality.rs",
    f"{PHYSICAL}/observation.rs",
    f"{PHYSICAL}/routing/mod.rs",
    f"{PHYSICAL}/routing/progression.rs",
    f"{PHYSICAL}/routing/request.rs",
    f"{PHYSICAL}/routing/external_observation.rs",
    f"{PHYSICAL}/shutdown.rs",
    f"{PHYSICAL}/temporal_progression.rs",
    f"{PHYSICAL}/wake_delivery.rs",
    f"{PHYSICAL}/worker.rs",
    f"{PHYSICAL}/worker_graph.rs",
    f"{PHYSICAL}/tests.rs",
    f"{PHYSICAL}/tests/request_locality.rs",
)

ATLAS_TRANSACTION_SOURCES = PHYSICAL_SIGNAL_SOURCES + (
    "scripts/ci/worth_ui_3141_phase4_case_contracts.py",
    f"{NATIVE}/host_state.rs",
    f"{NATIVE}/host_state/text_atlas_lifecycle.rs",
    f"{NATIVE}/resource_census.rs",
    f"{NATIVE}/resource_ownership.rs",
    f"{NATIVE}/resource_registry.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_admission.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_gate_d_evidence.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_rasterization.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_settlement.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_transaction.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_upload.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_upload_sink.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_signal_failure_tests.rs",
    f"{NATIVE}/mechanics_adapter/text_atlas_tests.rs",
    f"{NATIVE}/graphics/adapter_selection.rs",
    f"{ATLAS}/mod.rs",
    f"{ATLAS}/admission.rs",
    f"{ATLAS}/alpha.rs",
    f"{ATLAS}/boundary_tests.rs",
    f"{ATLAS}/candidate_store.rs",
    f"{ATLAS}/capacity.rs",
    f"{ATLAS}/census.rs",
    f"{ATLAS}/cleanup.rs",
    f"{ATLAS}/color.rs",
    f"{ATLAS}/demand_admission.rs",
    f"{ATLAS}/entry.rs",
    f"{ATLAS}/eviction.rs",
    f"{ATLAS}/gate_d_model_evidence.rs",
    f"{ATLAS}/in_flight.rs",
    f"{ATLAS}/key.rs",
    f"{ATLAS}/model_key.rs",
    f"{ATLAS}/model_oracle.rs",
    f"{ATLAS}/model_placement.rs",
    f"{ATLAS}/model_records.rs",
    f"{ATLAS}/ownership.rs",
    f"{ATLAS}/ownership_tests.rs",
    f"{ATLAS}/pinning.rs",
    f"{ATLAS}/placement.rs",
    f"{ATLAS}/placement_model_tests.rs",
    f"{ATLAS}/pinning_capacity_tests.rs",
    f"{ATLAS}/planning.rs",
    f"{ATLAS}/recovery.rs",
    f"{ATLAS}/recovery_identity_tests.rs",
    f"{ATLAS}/settlement.rs",
    f"{ATLAS}/settling.rs",
    f"{ATLAS}/test_device_tests.rs",
    f"{ATLAS}/transaction.rs",
    f"{ATLAS}/transaction_plan_snapshot.rs",
    f"{ATLAS}/upload.rs",
    f"{ATLAS}/upload_staging.rs",
)

RUNTIME_PIN_SOURCES = (
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/semantic_text_raster.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/text_pins.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/text_pins_tests.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/rasterization.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/recovery.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/transaction.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/qualified_text/raster_key.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/qualified_text/raster_batch_view.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/qualified_text/raster_transaction.rs",
)

PINNING_PRODUCT_SOURCES = ATLAS_TRANSACTION_SOURCES + RUNTIME_PIN_SOURCES + (
    "scripts/ci/worth_ui_3141_supporting_world.py",
    "scripts/ci/worth_ui_ledger_dependency.py",
    "scripts/ci/worth_ui_ledger_hostile_control_evidence.py",
    "scripts/ci/worth_ui_ledger_observation.py",
    "scripts/ci/worth_ui_ledger_phase_five_portfolio.py",
    "scripts/ci/worth_ui_ledger_portfolio_row.py",
    "scripts/ci/worth_ui_ledger_row_evidence.py",
    "scripts/ci/worth_ui_ledger_verifier_rebinding.py",
    f"{LEDGER}/dependency_row.rs",
    f"{LEDGER}/phase_four_case_contract.rs",
    f"{LEDGER}/runner_artifact_authentication.rs",
    f"{LEDGER}/supporting_world_artifact.rs",
    "workspaces/worth-ui/crates/worth-ui-host-contract/src/operational_adapter.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/cleanup.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/contract.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/finish.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/physical_clock.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/physical_progression.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/run.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/terminal_cleanup.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/window_port.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mechanics_adapter.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics/port.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/readiness.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/lib.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/native/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-host-native/src/prepared_host.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/certification_support/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/certification_support/presentation_mechanics.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/certification_support/semantic_text_projection.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/lib.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/host_session_authority.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/prepared_application_authority/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/prepared_application_authority/host_session_plan.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/host/adapter/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/host/adapter/operational_contract.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/host/adapter/session_authority.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/mod.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/authorized_native_host.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/mounted_coordinator.rs",
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/qualified_text_test_support.rs",
    "workspaces/worth-ui/apps/platform-pulse/src/main.rs",
    "workspaces/worth-ui/apps/platform-pulse/Cargo.toml",
    "workspaces/worth-ui/crates/worth-ui/src/facade/certification.rs",
    "workspaces/worth-ui/crates/worth-ui/src/facade/mod.rs",
    "workspaces/worth-ui/crates/worth-ui/src/lib.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_gate_d_pin.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/kill_on_close_job.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/launch.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/mod.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/native_desktop_lease.rs",
    "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/shutdown.rs",
    "_docs/worth-ui/milestone-3.14.1-evidence/p5-atlas-01.json",
)


def build_p5_proofs(
    proof_type: Any,
    control_type: Any,
    predecessor_artifact: str,
) -> dict[str, Any]:
    result = {
        "P5-PREDECESSOR-01": predecessor_proof(
            proof_type, control_type, predecessor_artifact
        ),
        "P5-GLYPH-RASTER-01": glyph_raster_proof(proof_type, control_type),
        "P5-COLOR-EMOJI-01": color_raster_proof(proof_type, control_type),
        "P5-ATLAS-01": atlas_proof(proof_type, control_type),
        "P5-ATLAS-PINNING-01": pinning_proof(proof_type, control_type),
    }
    return result


def glyph_raster_proof(proof_type: Any, control_type: Any) -> Any:
    return proof_type(
        "worth-ui-text",
        ("lib", "lib"),
        "phase5_ledger_evidence::qualified_alpha_and_color_raster_cross_exact_production_authority",
        f"{TEXT_RASTER}/demand.rs::derive_glyph_raster_demand",
        f"{TEXT}/phase5_ledger_evidence.rs::qualified_alpha_and_color_raster_cross_exact_production_authority",
        GLYPH_RASTER_SOURCES,
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_topology::phase_five_raster_authority::consumer_raster_authority_mutants_are_rejected",
            f"{CERT_ROOT}/milestone_3141_phase1_topology/phase_five_raster_authority.rs",
        ),
    )


def color_raster_proof(proof_type: Any, control_type: Any) -> Any:
    return proof_type(
        "worth-ui-text",
        ("lib", "lib"),
        "phase5_ledger_evidence::every_qualified_color_source_and_rgi_sequence_crosses_production_raster",
        f"{TEXT_RASTER}/color/mod.rs::rasterize_intrinsic_color",
        f"{TEXT}/phase5_ledger_evidence.rs::every_qualified_color_source_and_rgi_sequence_crosses_production_raster",
        COLOR_RASTER_SOURCES,
        control=control_type(
            "worth-ui-text",
            ("lib", "lib"),
            "phase5_ledger_evidence::emoji_tint_split_and_unqualified_color_sources_are_rejected",
            f"{TEXT}/phase5_ledger_evidence.rs",
        ),
    )


def atlas_proof(proof_type: Any, control_type: Any) -> Any:
    main = "native::mechanics_adapter::text_atlas::tests::gate_d_evidence::real_dx12_signal_transaction_matches_the_independent_atlas_model_and_closes_exactly"
    control = "native::mechanics_adapter::text_atlas::tests::gate_d_evidence::host_atlas_escape_and_lifecycle_faults_are_causally_rejected"
    return proof_type(
        "worth-ui-host-native",
        ("lib", "lib"),
        main,
        f"{NATIVE}/mechanics_adapter/text_atlas_transaction.rs::perform",
        f"{NATIVE}/mechanics_adapter/text_atlas_gate_d_evidence.rs::real_dx12_signal_transaction_matches_the_independent_atlas_model_and_closes_exactly",
        ATLAS_TRANSACTION_SOURCES,
        control=control_type(
            "worth-ui-host-native",
            ("lib", "lib"),
            control,
            f"{NATIVE}/mechanics_adapter/text_atlas_gate_d_evidence.rs",
        ),
    )


def pinning_proof(proof_type: Any, control_type: Any) -> Any:
    tests = "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/text_pins_tests.rs"
    return proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        "courtroom::native_gate_d_pin::live_layout_pins_cross_runtime_native_signal_and_release_at_last_owner",
        "workspaces/worth-ui/apps/platform-pulse/src/main.rs::run_native_gate_d_pin_world",
        "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_gate_d_pin.rs::live_layout_pins_cross_runtime_native_signal_and_release_at_last_owner",
        PINNING_PRODUCT_SOURCES,
        features=("executable-world",),
        control=control_type(
            "worth-ui-runtime",
            ("lib", "lib"),
            "mounting::presentation::coordinator::text_pins::tests::shared_pins_release_only_after_the_last_binding_is_deregistered",
            tests,
        ),
    )


def predecessor_proof(
    proof_type: Any, control_type: Any, predecessor_artifact: str
) -> Any:
    validator = f"{LEDGER}/predecessor_artifact.rs"
    handoff = f"{LEDGER}/predecessor_handoff.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "topology_contracts"),
        "milestone_3141_phase1_ledger::predecessor_handoff::phase_five_predecessor_handoff_is_current",
        f"{validator}::validate",
        f"{handoff}::phase_five_predecessor_handoff_is_current",
        (
            validator,
            handoff,
            "scripts/ci/worth_ui_3141_p5_proofs.py",
            "scripts/ci/verify_worth_ui_3141_ledger.py",
            "scripts/ci/worth_ui_ledger_phase_five_portfolio.py",
            predecessor_artifact,
        ),
        control=control_type(
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::phase_five_stale_source_or_missing_row_is_rejected",
            validator,
        ),
    )
