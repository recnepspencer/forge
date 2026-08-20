use super::{integration, library, TestIdentity, EXECUTABLE_WORLD};

pub(super) fn main_for(requirement: &str) -> Option<TestIdentity> {
    Some(match requirement {
        "P5-PREDECESSOR-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::predecessor_handoff::phase_five_predecessor_handoff_is_current",
        ),
        "P5-GLYPH-RASTER-01" => library(
            "worth-ui-text",
            "phase5_ledger_evidence::qualified_alpha_and_color_raster_cross_exact_production_authority",
        ),
        "P5-COLOR-EMOJI-01" => library(
            "worth-ui-text",
            "phase5_ledger_evidence::every_qualified_color_source_and_rgi_sequence_crosses_production_raster",
        ),
        "P5-ATLAS-01" => library(
            "worth-ui-host-native",
            "native::mechanics_adapter::text_atlas::tests::gate_d_evidence::real_dx12_signal_transaction_matches_the_independent_atlas_model_and_closes_exactly",
        ),
        "P5-ATLAS-PINNING-01" => native_world(
            "courtroom::native_gate_d_pin::live_layout_pins_cross_runtime_native_signal_and_release_at_last_owner",
        ),
        "P5-TEXT-DPI-01" => library(
            "worth-ui-text",
            "phase5_ledger_evidence::pure_dpi_replaces_raster_identity_without_relayout",
        ),
        "P5-TEXT-SPAN-PAINT-01" => library(
            "worth-ui-runtime",
            "native_platform::text_presentation::preparation::tests::mixed_bidi_native_runs_keep_logical_paint_ownership",
        ),
        "P5-TEXT-PIXELS-01" | "P5-TEXT-ASYNC-PRESENTATION-01" => native_world(
            "courtroom::native_phase_f::query_async_reconstruction_joins_exact_transitions_to_external_pixels_and_cleanup",
        ),
        "P5-TEXT-RECONSTRUCTION-01" => native_world(
            "courtroom::native_phase_f_reconstruction::every_derived_state_reconstructs_in_a_fresh_product_world",
        ),
        "P5-TEXT-COST-01" => integration(
            "worth-ui-certification",
            "phase5_closure",
            "phase5_locality_closure::all_32_fresh_native_locality_worlds_retain_owner_issued_evidence",
        ),
        "P5-CLOSE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::phase_five_closure_requires_every_predecessor_and_phase_five_row",
        ),
        _ => return None,
    })
}

pub(super) fn control_for(requirement: &str) -> Option<TestIdentity> {
    Some(match requirement {
        "P5-PREDECESSOR-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::phase_five_stale_source_or_missing_row_is_rejected",
        ),
        "P5-GLYPH-RASTER-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_topology::phase_five_raster_authority::consumer_raster_authority_mutants_are_rejected",
        ),
        "P5-COLOR-EMOJI-01" => library(
            "worth-ui-text",
            "phase5_ledger_evidence::emoji_tint_split_and_unqualified_color_sources_are_rejected",
        ),
        "P5-ATLAS-01" => library(
            "worth-ui-host-native",
            "native::mechanics_adapter::text_atlas::tests::gate_d_evidence::host_atlas_escape_and_lifecycle_faults_are_causally_rejected",
        ),
        "P5-ATLAS-PINNING-01" => library(
            "worth-ui-runtime",
            "mounting::presentation::coordinator::text_pins::tests::shared_pins_release_only_after_the_last_binding_is_deregistered",
        ),
        "P5-TEXT-DPI-01" => library(
            "worth-ui-text",
            "phase5_ledger_evidence::stale_dpi_raster_is_rejected_by_complete_successor_keys",
        ),
        "P5-TEXT-SPAN-PAINT-01" => library(
            "worth-ui-runtime",
            "native_platform::text_presentation::preparation::tests::single_color_and_logical_order_mutants_disagree_with_native_runs",
        ),
        "P5-TEXT-PIXELS-01" => native_world(
            "courtroom::native_phase_f::pixels::compositor_edges_and_unrelated_bright_pixels_cannot_satisfy_the_authored_text_oracle",
        ),
        "P5-TEXT-RECONSTRUCTION-01" => library(
            "worth-ui-host-native",
            "native::presentation::retained_draw_list::tests::reconstruction_tests::cold_reconstruction_rebuilds_every_index_then_next_delta_remains_local",
        ),
        "P5-TEXT-COST-01" => integration(
            "worth-ui-certification",
            "phase5_closure",
            "phase5_locality_hostile_control::exact_owner_cost_mutants_are_convicted_by_performed_small_worlds",
        ),
        "P5-TEXT-ASYNC-PRESENTATION-01" => library(
            "worth-ui-query-binding",
            "presentation_async::host_owner::hostile_control_tests::typed_async_hostile_family_matches_the_independent_transition_adjudicator",
        ),
        "P5-CLOSE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::mutation_tests::phase_closure_mode_rejects_open_rows_at_or_before_its_gate",
        ),
        _ => return None,
    })
}

const fn native_world(test_name: &'static str) -> TestIdentity {
    TestIdentity {
        package: "worth-ui-platform-pulse",
        target_kind: "test",
        target_name: "executable_world",
        features: EXECUTABLE_WORLD,
        test_name,
    }
}
