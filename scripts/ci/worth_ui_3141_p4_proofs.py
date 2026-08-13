from __future__ import annotations

from typing import Any

from worth_ui_3141_p4_predecessor_proof import predecessor_proof
from worth_ui_3141_p4_proof_builder import (
    CERT_ROOT,
    HEADLESS_ROOT,
    RUNTIME_ROOT,
    TEXT_ROOT,
    proof,
    text_proof,
)


def build_p4_proofs(
    proof_type: Any,
    control_type: Any,
    predecessor_artifact: str,
) -> dict[str, Any]:
    result = font_and_profile_proofs(proof_type, control_type)
    result.update(text_mechanics_proofs(proof_type, control_type))
    result.update(consumer_and_locality_proofs(proof_type, control_type))
    result["P4-PREDECESSOR-01"] = predecessor_proof(
        proof_type, control_type, predecessor_artifact
    )
    return result


def font_and_profile_proofs(proof_type: Any, control_type: Any) -> dict[str, Any]:
    font_evidence = f"{TEXT_ROOT}/font_collection/phase4_evidence.rs"
    qualification = (
        f"{CERT_ROOT}/milestone_3141_phase1_ledger/text_profile_qualification.rs"
    )
    result = {
        "P4-FONT-COLLECTION-01": proof(
            proof_type,
            control_type,
            "P4-FONT-COLLECTION-01",
            "worth-ui-text",
            ("lib", "lib"),
            "font_collection::phase4_evidence::application_font_collections_are_multi_family_owned_and_generation_safe",
            f"{TEXT_ROOT}/font_collection/lifecycle.rs::register_application_pack",
            font_evidence,
            "font_collection::phase4_evidence::application_font_authority_mutants_are_rejected_at_the_owning_boundaries",
            font_evidence,
            "scripts/ci/worth_ui_3141_phase4_case_contracts.py",
            f"{TEXT_ROOT}/font_collection/selection.rs",
            f"{TEXT_ROOT}/font_collection/application_pack.rs",
            f"{TEXT_ROOT}/font_collection/application_test_world.rs",
            f"{TEXT_ROOT}/font_collection/application_format_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_family_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_variation_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_feature_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_fallback_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_selection_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_pack_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_reconstruction_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_lifecycle_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_metadata_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_capacity_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_byte_capacity_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_pack/metadata.rs",
            f"{TEXT_ROOT}/font_collection/application_pack/metadata_inventory.rs",
            f"{TEXT_ROOT}/font_collection/coverage.rs",
            "workspaces/worth-ui/crates/worth-ui/examples/text_platform.rs",
            "workspaces/worth-ui/docs/text-platform.md",
        ),
        "P4-TEXT-PROFILE-01": proof(
            proof_type,
            control_type,
            "P4-TEXT-PROFILE-01",
            "worth-ui-certification",
            ("test", "topology_contracts"),
            "milestone_3141_phase1_ledger::text_profile_qualification::global_text_profile_assets_indexes_and_dependencies_are_exact",
            f"{qualification}::validate_profile",
            qualification,
            "milestone_3141_phase1_ledger::text_profile_qualification::global_text_profile_rejects_manifest_and_artifact_drift",
            qualification,
            "workspaces/worth-ui/profiles/worth-ui-global-text-v2/manifest.toml",
            "workspaces/worth-ui/profiles/worth-ui-global-text-v2/artifact-inventory.toml",
            "workspaces/worth-ui/Cargo.toml",
            "workspaces/worth-ui/crates/worth-ui-text/Cargo.toml",
            "tools/worth-ui-text-profile-qualification/Cargo.toml",
            "tools/worth-ui-text-profile-qualification/Cargo.lock",
        ),
        "P4-COLOR-FONT-ADMISSION-01": proof(
            proof_type,
            control_type,
            "P4-COLOR-FONT-ADMISSION-01",
            "worth-ui-text",
            ("lib", "lib"),
            "font_collection::phase4_evidence::admitted_color_fonts_have_complete_owned_table_semantics",
            f"{TEXT_ROOT}/font_collection/application_pack/color_tables.rs::validate",
            font_evidence,
            "font_collection::phase4_evidence::malformed_or_unsupported_color_font_sources_deny_atomically",
            font_evidence,
            f"{TEXT_ROOT}/font_collection/application_color_tests.rs",
            f"{TEXT_ROOT}/font_collection/application_pack/color_tables/bitmap.rs",
            f"{TEXT_ROOT}/font_collection/application_pack/color_tables/bitmap/cbdt.rs",
            f"{TEXT_ROOT}/font_collection/application_pack/color_tables/bitmap/sbix.rs",
            f"{TEXT_ROOT}/font_collection/application_pack/color_tables/colr.rs",
            f"{TEXT_ROOT}/font_collection/application_pack/color_tables/png.rs",
            f"{TEXT_ROOT}/font_collection/application_pack/color_tables/traversal.rs",
        ),
    }
    return result


def text_mechanics_proofs(proof_type: Any, control_type: Any) -> dict[str, Any]:
    specifications = {
        "P4-UNICODE-SEGMENTATION-01": (
            "unicode_17_segmentation_corpora_are_exhaustive",
            f"{TEXT_ROOT}/analysis.rs::analyze",
            "zwj_flag_and_dictionary_boundary_substitutions_are_rejected",
            (
                f"{TEXT_ROOT}/dictionary_segmentation.rs",
                f"{TEXT_ROOT}/analysis_conformance_tests.rs",
            ),
        ),
        "P4-EMOJI-SEQUENCE-01": (
            "every_rgi_sequence_is_atomic_through_analysis_fallback_and_layout",
            f"{TEXT_ROOT}/analysis.rs::analyze",
            "variation_and_zwj_decomposition_is_rejected_by_real_layout_geometry",
            (f"{TEXT_ROOT}/fallback.rs", f"{TEXT_ROOT}/layout.rs"),
        ),
        "P4-BIDI-01": (
            "unicode_17_bidi_corpora_drive_visual_order",
            f"{TEXT_ROOT}/analysis.rs::analyze",
            "logical_order_rendering_is_rejected_by_visual_geometry",
            (
                f"{TEXT_ROOT}/bidi_data.rs",
                f"{TEXT_ROOT}/layout/visual_order.rs",
                f"{TEXT_ROOT}/layout/tests.rs",
                f"{TEXT_ROOT}/layout/paragraph_alignment_tests.rs",
            ),
        ),
        "P4-FALLBACK-01": (
            "whole_cluster_fallback_is_exhaustive_and_script_safe",
            f"{TEXT_ROOT}/fallback.rs::select_with_posture",
            "emoji_or_indic_cluster_split_is_rejected_before_shaping",
            (f"{TEXT_ROOT}/font_collection/selection.rs",),
        ),
        "P4-SHAPING-01": (
            "mixed_script_shaping_emits_exact_nonzero_glyphs",
            f"{TEXT_ROOT}/shaping.rs::shape",
            "one_run_latin_substitution_is_rejected_by_features_and_axes",
            (
                f"{TEXT_ROOT}/shaping/records.rs",
                f"{TEXT_ROOT}/shaping/reference_fixture_tests.rs",
                f"{TEXT_ROOT}/font_collection/face.rs",
                f"{TEXT_ROOT}/language.rs",
                "workspaces/worth-ui/profiles/worth-ui-global-text-v2/fixtures/hb-shape-13.0.0.json",
            ),
        ),
        "P4-LINE-LAYOUT-01": (
            "unicode_line_fitting_preserves_clusters_and_capacity",
            f"{TEXT_ROOT}/layout.rs::layout_with_posture",
            "mid_cluster_wrap_and_post_capacity_ellipsis_are_rejected",
            (
                f"{TEXT_ROOT}/layout/line_fitting.rs",
                f"{TEXT_ROOT}/layout/visual_order.rs",
                f"{TEXT_ROOT}/layout/tests.rs",
                f"{TEXT_ROOT}/layout/paragraph_alignment_tests.rs",
                f"{TEXT_ROOT}/layout/line_anchor_tests.rs",
                f"{TEXT_ROOT}/layout/contextual_line_shaping.rs",
                f"{TEXT_ROOT}/layout/contextual_shaping_tests.rs",
                f"{TEXT_ROOT}/layout/ellipsis.rs",
                f"{TEXT_ROOT}/font_collection/ink_bounds.rs",
                f"{TEXT_ROOT}/font_collection/ink_bounds/bitmap.rs",
                f"{TEXT_ROOT}/font_collection/ink_bounds/color.rs",
                f"{TEXT_ROOT}/font_collection/ink_bounds/color_path.rs",
                f"{TEXT_ROOT}/font_collection/ink_bounds/color_region.rs",
                f"{TEXT_ROOT}/font_collection/ink_bounds/color_tests.rs",
                f"{TEXT_ROOT}/font_collection/ink_bounds_tests.rs",
            ),
        ),
        "P4-CAPACITY-01": (
            "derived_capacity_is_reserved_before_analysis_and_shaping",
            f"{TEXT_ROOT}/admission.rs::admit_with_identity",
            "shape_before_capacity_denial_is_rejected",
            (f"{TEXT_ROOT}/font_collection/admission.rs", f"{TEXT_ROOT}/language.rs"),
        ),
        "P4-ORIGINAL-RANGE-01": (
            "original_utf8_ranges_survive_mixed_script_layout",
            f"{TEXT_ROOT}/layout.rs::layout_with_posture",
            "normalized_offset_substitution_is_rejected_by_cluster_hits",
            (f"{TEXT_ROOT}/layout/interaction.rs",),
        ),
        "P4-BIDI-INTERACTION-01": (
            "bidi_interaction_records_exact_edges_affinities_hits_and_selection",
            f"{TEXT_ROOT}/layout/interaction.rs::hit_test",
            "swapped_bidi_caret_affinity_is_rejected",
            (f"{TEXT_ROOT}/layout/interaction.rs", f"{TEXT_ROOT}/layout/visual_order.rs"),
        ),
        "P4-TEXT-RECONSTRUCTION-01": (
            "retired_layout_reconstructs_from_exact_pinned_bytes",
            f"{TEXT_ROOT}/reconstruction.rs::reconstruct",
            "stale_layout_reuse_is_rejected_by_exact_collection_lineage",
            (f"{TEXT_ROOT}/layout.rs", f"{TEXT_ROOT}/font_collection/lifecycle.rs"),
        ),
    }
    return {
        requirement: text_proof(
            proof_type, control_type, requirement, main, production, hostile, *sources
        )
        for requirement, (main, production, hostile, sources) in specifications.items()
    }


def consumer_and_locality_proofs(proof_type: Any, control_type: Any) -> dict[str, Any]:
    font_world = f"{CERT_ROOT}/application_contracts/projection_presentation/font_stack.rs"
    accessibility_oracle = (
        f"{CERT_ROOT}/application_contracts/projection_presentation/"
        "font_stack/accessibility_geometry.rs"
    )
    collection_world = (
        f"{CERT_ROOT}/application_contracts/projection_presentation/collection_query.rs"
    )
    collection_control = (
        f"{CERT_ROOT}/application_contracts/projection_presentation/collection_query/locality.rs"
    )
    width_world = (
        f"{RUNTIME_ROOT}/mounting/projection/frame_storage/mechanic_source_tests/phase4_locality.rs"
    )
    width_fixture = (
        f"{RUNTIME_ROOT}/mounting/projection/frame_storage/"
        "mechanic_source_tests/phase4_locality/world.rs"
    )
    collection_locality_source = (
        f"{RUNTIME_ROOT}/mounting/projection/frame_storage/"
        "mechanic_source_tests/phase4_locality/collection.rs"
    )
    locality_main = (
        "mounting::projection::frame_storage::mechanic_source_tests::phase4_locality::"
        "content_and_width_locality_have_exact_constant_work_at_every_qualified_size"
    )
    locality_control = (
        "mounting::projection::frame_storage::mechanic_source_tests::phase4_locality::"
        "retained_document_scan_and_global_width_substitution_are_rejected"
    )
    collection_locality_control = (
        "mounting::projection::frame_storage::mechanic_source_tests::phase4_locality::"
        "collection::collection_patch_content_locality_is_constant_at_every_qualified_size"
    )
    result: dict[str, Any] = {}
    for requirement, entry in {
        "P4-MEASUREMENT-IDENTITY-01": f"{HEADLESS_ROOT}/headless_transcript/text_measurement.rs::qualified_measurement",
        "P4-ACCESSIBILITY-GEOMETRY-01": f"{HEADLESS_ROOT}/headless_transcript/text_accessibility.rs::accessibility_geometry",
    }.items():
        authority = f"{CERT_ROOT}/milestone_3141_phase1_topology/font_authority.rs"
        result[requirement] = proof_type(
            "worth-ui-certification",
            ("test", "application_contracts"),
            "projection_presentation::font_stack::authored_application_stack_and_emoji_fallback_cross_mounted_headless_consumers",
            entry,
            f"{font_world}::authored_application_stack_and_emoji_fallback_cross_mounted_headless_consumers",
            (
                entry.rsplit("::", 1)[0],
                font_world,
                accessibility_oracle,
                f"{HEADLESS_ROOT}/headless_transcript/semantic_text.rs",
                f"{RUNTIME_ROOT}/mounting/projection/semantic_text/qualified.rs",
                f"{RUNTIME_ROOT}/mounting/presentation/consumption_view.rs",
                authority,
            ),
            control=control_type(
                "worth-ui-certification",
                ("test", "topology_contracts"),
                "milestone_3141_phase1_topology::font_authority::headless_measurement_and_accessibility_consume_only_qualified_records",
                authority,
            ),
        )
    for requirement, entry in {
        "P4-TEXT-CONTENT-LOCALITY-01": f"{RUNTIME_ROOT}/mounting/projection/frame_storage/semantic_mechanics.rs::apply_collection_updates",
        "P4-TEXT-COST-01": f"{RUNTIME_ROOT}/mounting/projection/frame_storage/semantic_mechanics.rs::apply_collection_updates",
    }.items():
        result[requirement] = proof_type(
            "worth-ui-certification",
            ("test", "application_contracts"),
            "projection_presentation::collection_query::real_query_collection_snapshot_and_patch_publish_keyed_semantic_text",
            entry,
            f"{collection_world}::real_query_collection_snapshot_and_patch_publish_keyed_semantic_text",
            (
                entry.rsplit("::", 1)[0],
                f"{RUNTIME_ROOT}/mounting/projection/frame_storage/semantic_mechanics/capacity.rs",
                f"{RUNTIME_ROOT}/mounting/projection/frame_storage/semantic_mechanics/diff.rs",
                f"{RUNTIME_ROOT}/mounting/projection/frame_storage/semantic_mechanics/layout_index.rs",
                f"{RUNTIME_ROOT}/runtime/persistent_index/ordered_map.rs",
                f"{RUNTIME_ROOT}/runtime/persistent_index/ranked_sequence.rs",
                f"{RUNTIME_ROOT}/runtime/persistent_index/test_observation.rs",
                collection_world,
                collection_control,
                width_world,
                width_fixture,
                collection_locality_source,
            ),
            control=control_type(
                "worth-ui-runtime",
                ("lib", "lib"),
                collection_locality_control,
                collection_locality_source,
            ),
        )
    for requirement in ("P4-TEXT-WIDTH-LOCALITY-01", "P4-UNCHANGED-01"):
        result[requirement] = proof(
            proof_type,
            control_type,
            requirement,
            "worth-ui-runtime",
            ("lib", "lib"),
            locality_main,
            f"{RUNTIME_ROOT}/mounting/projection/frame_storage/mechanic_source.rs::apply",
            width_world,
            locality_control,
            width_world,
            width_fixture,
            f"{RUNTIME_ROOT}/runtime/persistent_index/ordered_map.rs",
            f"{RUNTIME_ROOT}/runtime/persistent_index/ranked_sequence.rs",
            f"{RUNTIME_ROOT}/runtime/persistent_index/test_observation.rs",
        )
    close_source = f"{CERT_ROOT}/milestone_3141_phase1_ledger.rs"
    result["P4-CLOSE-01"] = proof(
        proof_type,
        control_type,
        "P4-CLOSE-01",
        "worth-ui-certification",
        ("test", "topology_contracts"),
        "milestone_3141_phase1_ledger::phase_four_closure_requires_every_predecessor_and_phase_four_row",
        f"{close_source}::validate_phase_closure",
        close_source,
        "milestone_3141_phase1_ledger::mutation_tests::phase_closure_mode_rejects_open_rows_at_or_before_its_gate",
        f"{CERT_ROOT}/milestone_3141_phase1_ledger/mutation_tests.rs",
    )
    return result
