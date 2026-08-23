use super::{integration, library, TestIdentity};

pub(super) fn main_for(requirement: &str) -> Option<TestIdentity> {
    Some(match requirement {
        "P4-FONT-COLLECTION-01" => library(
            "worth-ui-text",
            "font_collection::phase4_evidence::application_font_collections_are_multi_family_owned_and_generation_safe",
        ),
        "P4-PREDECESSOR-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::predecessor_handoff::phase_four_predecessor_handoff_is_current",
        ),
        "P4-TEXT-PROFILE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::text_profile_qualification::global_text_profile_assets_indexes_and_dependencies_are_exact",
        ),
        "P4-COLOR-FONT-ADMISSION-01" => library(
            "worth-ui-text",
            "font_collection::phase4_evidence::admitted_color_fonts_have_complete_owned_table_semantics",
        ),
        "P4-UNICODE-SEGMENTATION-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::unicode_17_segmentation_corpora_are_exhaustive",
        ),
        "P4-EMOJI-SEQUENCE-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::every_rgi_sequence_is_atomic_through_analysis_fallback_and_layout",
        ),
        "P4-BIDI-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::unicode_17_bidi_corpora_drive_visual_order",
        ),
        "P4-FALLBACK-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::whole_cluster_fallback_is_exhaustive_and_script_safe",
        ),
        "P4-SHAPING-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::mixed_script_shaping_emits_exact_nonzero_glyphs",
        ),
        "P4-LINE-LAYOUT-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::unicode_line_fitting_preserves_clusters_and_capacity",
        ),
        "P4-CAPACITY-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::derived_capacity_is_reserved_before_analysis_and_shaping",
        ),
        "P4-ORIGINAL-RANGE-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::original_utf8_ranges_survive_mixed_script_layout",
        ),
        "P4-BIDI-INTERACTION-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::bidi_interaction_records_exact_edges_affinities_hits_and_selection",
        ),
        "P4-TEXT-RECONSTRUCTION-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::retired_layout_reconstructs_from_exact_pinned_bytes",
        ),
        "P4-MEASUREMENT-IDENTITY-01" | "P4-ACCESSIBILITY-GEOMETRY-01" => integration(
            "worth-ui-certification",
            "application_contracts",
            "projection_presentation::font_stack::authored_application_stack_and_emoji_fallback_cross_mounted_headless_consumers",
        ),
        "P4-TEXT-CONTENT-LOCALITY-01" | "P4-TEXT-COST-01" => integration(
            "worth-ui-certification",
            "application_contracts",
            "projection_presentation::collection_query::real_query_collection_snapshot_and_patch_publish_keyed_semantic_text",
        ),
        "P4-TEXT-WIDTH-LOCALITY-01" | "P4-UNCHANGED-01" => library(
            "worth-ui-runtime",
            "mounting::projection::frame_storage::mechanic_source_tests::phase4_locality::content_and_width_locality_have_exact_constant_work_at_every_qualified_size",
        ),
        "P4-CLOSE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::phase_four_closure_requires_every_predecessor_and_phase_four_row",
        ),
        _ => return None,
    })
}

pub(super) fn control_for(requirement: &str) -> Option<TestIdentity> {
    Some(match requirement {
        "P4-FONT-COLLECTION-01" => library(
            "worth-ui-text",
            "font_collection::phase4_evidence::application_font_authority_mutants_are_rejected_at_the_owning_boundaries",
        ),
        "P4-PREDECESSOR-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::predecessor_artifact::tests::phase_four_stale_source_or_missing_row_is_rejected",
        ),
        "P4-TEXT-PROFILE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::text_profile_qualification::global_text_profile_rejects_manifest_and_artifact_drift",
        ),
        "P4-COLOR-FONT-ADMISSION-01" => library(
            "worth-ui-text",
            "font_collection::phase4_evidence::malformed_or_unsupported_color_font_sources_deny_atomically",
        ),
        "P4-UNICODE-SEGMENTATION-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::zwj_flag_and_dictionary_boundary_substitutions_are_rejected",
        ),
        "P4-EMOJI-SEQUENCE-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::variation_and_zwj_decomposition_is_rejected_by_real_layout_geometry",
        ),
        "P4-BIDI-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::logical_order_rendering_is_rejected_by_visual_geometry",
        ),
        "P4-FALLBACK-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::emoji_or_indic_cluster_split_is_rejected_before_shaping",
        ),
        "P4-SHAPING-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::one_run_latin_substitution_is_rejected_by_features_and_axes",
        ),
        "P4-LINE-LAYOUT-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::mid_cluster_wrap_and_post_capacity_ellipsis_are_rejected",
        ),
        "P4-CAPACITY-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::shape_before_capacity_denial_is_rejected",
        ),
        "P4-ORIGINAL-RANGE-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::normalized_offset_substitution_is_rejected_by_cluster_hits",
        ),
        "P4-BIDI-INTERACTION-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::swapped_bidi_caret_affinity_is_rejected",
        ),
        "P4-TEXT-RECONSTRUCTION-01" => library(
            "worth-ui-text",
            "phase4_ledger_evidence::stale_layout_reuse_is_rejected_by_exact_collection_lineage",
        ),
        "P4-MEASUREMENT-IDENTITY-01" | "P4-ACCESSIBILITY-GEOMETRY-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_topology::font_authority::headless_measurement_and_accessibility_consume_only_qualified_records",
        ),
        "P4-TEXT-CONTENT-LOCALITY-01" | "P4-TEXT-COST-01" => library(
            "worth-ui-runtime",
            "mounting::projection::frame_storage::mechanic_source_tests::phase4_locality::collection::collection_patch_content_locality_is_constant_at_every_qualified_size",
        ),
        "P4-TEXT-WIDTH-LOCALITY-01" | "P4-UNCHANGED-01" => library(
            "worth-ui-runtime",
            "mounting::projection::frame_storage::mechanic_source_tests::phase4_locality::retained_document_scan_and_global_width_substitution_are_rejected",
        ),
        "P4-CLOSE-01" => integration(
            "worth-ui-certification",
            "topology_contracts",
            "milestone_3141_phase1_ledger::mutation_tests::phase_closure_mode_rejects_open_rows_at_or_before_its_gate",
        ),
        _ => return None,
    })
}
