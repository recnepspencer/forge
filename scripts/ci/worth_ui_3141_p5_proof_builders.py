from __future__ import annotations

from typing import Any
from worth_ui_3141_p5_closure_proof_builders import (
    phase_five_close_proof,
    predecessor_proof,
)
from worth_ui_3141_p5_source_owners import (
    ATLAS_TRANSACTION_SOURCES,
    CERT_ROOT,
    COLOR_RASTER_SOURCES,
    DPI_REPLACEMENT_SOURCES,
    GLYPH_RASTER_SOURCES,
    NATIVE,
    P5_CASE_AUTHORITY_SOURCES,
    PHYSICAL_SIGNAL_SOURCES,
    PLATFORM_PULSE,
    PRESENTATION_ASYNC_PRODUCTION_SOURCES,
    QUERY_ASYNC_PRESENTATION_SOURCES,
    QUERY_BINDING,
    RUNTIME,
    RUNTIME_BRIDGE_ASYNC_COMPLETION_SOURCES,
    RUNTIME_PAINT_SPAN_SOURCES,
    TEXT,
    TEXT_RASTER,
    unique_sources,
)
from worth_ui_3141_p5_source_worlds import (
    LOCALITY_MATRIX_SOURCES,
    PINNING_PRODUCT_SOURCES,
    PIXEL_WORLD_SOURCES,
    RECONSTRUCTION_SOURCES,
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
        "P5-TEXT-DPI-01": dpi_replacement_proof(proof_type, control_type),
        "P5-TEXT-SPAN-PAINT-01": paint_span_proof(proof_type, control_type),
        "P5-TEXT-PIXELS-01": pixel_world_proof(proof_type, control_type),
        "P5-TEXT-RECONSTRUCTION-01": reconstruction_proof(proof_type, control_type),
        "P5-TEXT-COST-01": locality_cost_proof(proof_type, control_type),
        "P5-TEXT-ASYNC-PRESENTATION-01": async_presentation_proof(
            proof_type, control_type
        ),
        "P5-CLOSE-01": phase_five_close_proof(proof_type, control_type),
    }
    return result


def glyph_raster_proof(proof_type: Any, control_type: Any) -> Any:
    return proof_type(
        "worth-ui-text",
        ("lib", "lib"),
        "phase5_ledger_evidence::qualified_alpha_and_color_raster_cross_exact_production_authority",
        f"{TEXT_RASTER}/demand/derivation.rs::derive_glyph_raster_demand",
        f"{TEXT}/phase5_ledger_evidence.rs::qualified_alpha_and_color_raster_cross_exact_production_authority",
        GLYPH_RASTER_SOURCES
        + P5_CASE_AUTHORITY_SOURCES
        + (
            f"{CERT_ROOT}/milestone_3141_phase1_topology/phase_five_raster_authority.rs",
        ),
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
        COLOR_RASTER_SOURCES + P5_CASE_AUTHORITY_SOURCES,
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


def dpi_replacement_proof(proof_type: Any, control_type: Any) -> Any:
    evidence = f"{TEXT}/phase5_ledger_evidence.rs"
    return proof_type(
        "worth-ui-text",
        ("lib", "lib"),
        "phase5_ledger_evidence::pure_dpi_replaces_raster_identity_without_relayout",
        f"{TEXT_RASTER}/demand/derivation.rs::derive_glyph_raster_demand",
        f"{evidence}::pure_dpi_replaces_raster_identity_without_relayout",
        DPI_REPLACEMENT_SOURCES,
        control=control_type(
            "worth-ui-text",
            ("lib", "lib"),
            "phase5_ledger_evidence::stale_dpi_raster_is_rejected_by_complete_successor_keys",
            evidence,
        ),
    )


def paint_span_proof(proof_type: Any, control_type: Any) -> Any:
    evidence = "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation_tests.rs"
    return proof_type(
        "worth-ui-runtime",
        ("lib", "lib"),
        "native_platform::text_presentation::preparation::tests::mixed_bidi_native_runs_keep_logical_paint_ownership",
        "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/text_presentation/preparation/demand_join.rs::prepare_demands",
        f"{evidence}::mixed_bidi_native_runs_keep_logical_paint_ownership",
        RUNTIME_PAINT_SPAN_SOURCES,
        control=control_type(
            "worth-ui-runtime",
            ("lib", "lib"),
            "native_platform::text_presentation::preparation::tests::single_color_and_logical_order_mutants_disagree_with_native_runs",
            evidence,
        ),
    )


def pixel_world_proof(proof_type: Any, control_type: Any) -> Any:
    test = "courtroom::native_phase_f::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup"
    control = "courtroom::native_phase_f::pixels::compositor_edges_and_unrelated_bright_pixels_cannot_satisfy_the_authored_text_oracle"
    oracle = f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f.rs"
    return proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        test,
        f"{NATIVE}/presentation/port/transaction.rs::present",
        f"{oracle}::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup",
        PIXEL_WORLD_SOURCES + P5_CASE_AUTHORITY_SOURCES,
        features=("executable-world",),
        control=control_type(
            "worth-ui-platform-pulse",
            ("test", "executable_world"),
            control,
            oracle,
            features=("executable-world",),
        ),
    )


def reconstruction_proof(proof_type: Any, control_type: Any) -> Any:
    oracle = f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f_reconstruction.rs"
    return proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        "courtroom::native_phase_f_reconstruction::every_derived_state_reconstructs_in_a_fresh_product_world",
        f"{RUNTIME}/facade/entry/native_application_shell/presentation_recovery.rs::reconstruct_current_presentation",
        f"{oracle}::every_derived_state_reconstructs_in_a_fresh_product_world",
        RECONSTRUCTION_SOURCES + P5_CASE_AUTHORITY_SOURCES,
        features=("executable-world",),
        control=control_type(
            "worth-ui-host-native",
            ("lib", "lib"),
            "native::presentation::retained_draw_list::tests::reconstruction_tests::cold_reconstruction_rebuilds_every_index_then_next_delta_remains_local",
            f"{NATIVE}/presentation/retained_draw_list/reconstruction_tests.rs",
        ),
    )


def locality_cost_proof(proof_type: Any, control_type: Any) -> Any:
    oracle = "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/phase5_locality_closure.rs"
    return proof_type(
        "worth-ui-certification",
        ("test", "application_contracts"),
        "phase5_locality_closure::all_32_fresh_native_locality_worlds_retain_owner_issued_evidence",
        f"{RUNTIME}/mounting/presentation/coordinator/semantic_text_raster.rs::present",
        f"{oracle}::all_32_fresh_native_locality_worlds_retain_owner_issued_evidence",
        unique_sources(*LOCALITY_MATRIX_SOURCES, *P5_CASE_AUTHORITY_SOURCES),
        control=control_type(
            "worth-ui-certification",
            ("test", "application_contracts"),
            "phase5_locality_hostile_control::exact_owner_cost_mutants_are_convicted_by_performed_small_worlds",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/phase5_locality_hostile_control.rs",
        ),
    )


def async_presentation_proof(proof_type: Any, control_type: Any) -> Any:
    oracle = f"{PLATFORM_PULSE}/tests/executable_world/courtroom/native_phase_f.rs"
    return proof_type(
        "worth-ui-platform-pulse",
        ("test", "executable_world"),
        "courtroom::native_phase_f::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup",
        f"{QUERY_BINDING}/host_owner/settlement.rs::admit_presented",
        f"{oracle}::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup",
        unique_sources(
            *PIXEL_WORLD_SOURCES,
            *PHYSICAL_SIGNAL_SOURCES,
            *PRESENTATION_ASYNC_PRODUCTION_SOURCES,
            f"{QUERY_BINDING}/host_owner_authority_tests.rs",
            f"{QUERY_BINDING}/host_owner_hostile_control_tests.rs",
            f"{QUERY_BINDING}/host_owner_tests.rs",
            f"{QUERY_BINDING}/host_owner_tests/completion.rs",
            f"{QUERY_BINDING}/host_owner_unresolved_tests.rs",
            *QUERY_ASYNC_PRESENTATION_SOURCES,
            "crates/worth-runtime-bridge/src/conditional_execution.rs",
            "crates/worth-runtime-bridge/src/conditional_execution/contract.rs",
            "crates/worth-runtime-bridge/src/conditional_execution/owned_async.rs",
            "crates/worth-runtime-bridge/src/conditional_execution/owned_async_observation.rs",
            *RUNTIME_BRIDGE_ASYNC_COMPLETION_SOURCES,
            "workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/presentation_work/authority.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/fixtures/compile_contracts/Cargo.toml",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/suites/compile_contract_execution.csv",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/live_authority_cannot_be_reconstructed_or_substituted.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/live_authority_cannot_be_reconstructed_or_substituted.stderr",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/live_authority_flows_through_owner_issued_values.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/physical_signal_cannot_authorize_query_effect.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/physical_signal_cannot_authorize_query_effect.stderr",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/recovery_authority_is_not_serializable.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/recovery_authority_is_not_serializable.stderr",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/reporting_material_cannot_open_authority.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/ui/phase5_async_authority/reporting_material_cannot_open_authority.stderr",
            f"{RUNTIME}/mounting/presentation/coordinator/cancellation.rs",
            f"{RUNTIME}/mounting/presentation/coordinator/cancellation_settlement.rs",
            f"{RUNTIME}/mounting/presentation/coordinator/pending_completion.rs",
            f"{RUNTIME}/mounting/presentation/coordinator/settlement.rs",
            f"{RUNTIME}/mounting/presentation/outcome.rs",
            f"{RUNTIME}/mounting/presentation/terminal.rs",
            f"{RUNTIME}/native_platform/application_driver/physical_recovery_tracker.rs",
            f"{RUNTIME}/native_platform/application_driver/program_progress.rs",
            f"{RUNTIME}/native_platform/application_driver/program_progress/superseding_pair.rs",
            f"{NATIVE}/mechanics_adapter/presentation/pending_completion.rs",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/mounted_presentation/query_uncertainty.rs",
            "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json",
            "scripts/ci/run_worth_ui_compile_contracts.py",
            *P5_CASE_AUTHORITY_SOURCES,
        ),
        features=("executable-world",),
        control=control_type(
            "worth-ui-query-binding",
            ("lib", "lib"),
            "presentation_async::host_owner::hostile_control_tests::typed_async_hostile_family_matches_the_independent_transition_adjudicator",
            f"{QUERY_BINDING}/host_owner_hostile_control_tests.rs",
        ),
    )
