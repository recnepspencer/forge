#[test]
#[ignore = "Phase 4 closure: exercises the complete application-font owner contract"]
fn application_font_collections_are_multi_family_owned_and_generation_safe() {
    let cases: [fn(); 12] = [
        super::application_format_tests::owned_ttf_otf_ttc_and_otc_bytes_cross_one_public_pack_transition,
        super::application_family_tests::ordered_application_families_and_face_attributes_select_owned_bytes,
        super::application_family_tests::static_regular_bold_italic_and_oblique_faces_match_exact_requests,
        super::application_family_tests::same_named_families_in_distinct_packs_never_merge_authority,
        super::application_variation_tests::application_variable_axes_slant_features_and_metadata_drive_real_shaping,
        super::application_variation_tests::variable_slant_axis_is_selected_and_changes_real_shaping,
        super::application_feature_tests::authored_latin_features_do_not_split_or_block_color_emoji_fallback,
        super::application_fallback_tests::authored_and_profile_fallback_use_exact_owned_faces_for_whole_clusters,
        super::application_fallback_tests::khmer_shaping_syllable_falls_back_and_shapes_as_one_whole_cluster,
        super::application_selection_tests::authored_application_family_stacks_are_selected_independently_per_span,
        super::application_pack_tests::collection_indices_and_pack_generations_are_exact_and_old_layouts_pin_bytes,
        super::application_reconstruction_tests::retired_collection_denies_fresh_work_but_reconstructs_its_exact_qualified_layout,
    ];
    std::thread::scope(|scope| {
        let executions = cases.map(|case| scope.spawn(case));
        for execution in executions {
            execution
                .join()
                .expect("application-font evidence case passed");
        }
    });
    println!(
        "WORTH_UI_LEDGER_CASES={}",
        concat!(
            "{\"P4-FONT-COLLECTION-01\":[",
            "\"owned-ttf\",",
            "\"owned-otf\",",
            "\"owned-ttc-multi-index\",",
            "\"owned-otc-multi-index\",",
            "\"ordered-multi-family-stack\",",
            "\"static-regular-bold-italic-oblique\",",
            "\"pack-scoped-family-name-collision\",",
            "\"variable-weight\",",
            "\"variable-width\",",
            "\"variable-slant\",",
            "\"explicit-opentype-feature\",",
            "\"whole-cluster-default-emoji-last-resort-fallback\",",
            "\"whole-cluster-khmer-shaping-syllable\",",
            "\"independent-per-span-stack\",",
            "\"generation-replace-remove-pins-predecessor-bytes\",",
            "\"exact-generation-reconstruction\"]}"
        )
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-FONT-COLLECTION-01\":16}}");
}

#[test]
fn application_font_authority_mutants_are_rejected_at_the_owning_boundaries() {
    let cases: [fn(); 13] = [
        super::application_format_tests::woff_and_woff2_are_typed_unsupported_containers_before_font_parsing,
        super::application_format_tests::aat_substitution_tables_are_typed_unsupported_before_metadata_admission,
        super::application_feature_tests::unsupported_explicit_feature_is_denied_before_shaping,
        super::application_lifecycle_tests::multiple_packs_never_make_registration_order_a_selector,
        super::application_metadata_tests::malformed_localized_name_record_is_denied_before_pack_publication,
        super::application_pack_tests::malformed_ambiguous_unsupported_and_over_capacity_packs_deny_atomically,
        super::application_pack_tests::exhausted_collection_generation_cannot_alias_its_successor,
        super::application_reconstruction_tests::same_numbered_collections_cannot_substitute_for_the_request_owner,
        super::application_selection_tests::pack_and_face_selection_ignore_registration_and_definition_order,
        super::application_selection_tests::family_fallback_never_uses_a_worse_face_to_skip_a_later_family,
        super::application_variation_tests::variable_face_matching_uses_the_requested_value_against_each_axis_range,
        super::coverage_tests::application_pack_without_a_unicode_coverage_map_is_denied_atomically,
        super::application_pack::qualification::tests::pack_identity_frames_variable_fields_at_real_definition_boundaries,
    ];
    std::thread::scope(|scope| {
        let executions = cases.map(|case| scope.spawn(case));
        for execution in executions {
            execution.join().expect("application-font mutant rejected");
        }
    });
    println!(
        "WORTH_UI_LEDGER_MUTATION_CASES={}",
        concat!(
            "{\"P4-FONT-COLLECTION-01\":[",
            "\"unsupported-web-container\",",
            "\"unsupported-aat-shaping-table\",",
            "\"unsupported-explicit-feature\",",
            "\"registration-order-substitution\",",
            "\"malformed-localized-name\",",
            "\"malformed-ambiguous-unsupported-over-capacity-pack\",",
            "\"generation-exhaustion-alias\",",
            "\"same-number-different-lineage\",",
            "\"face-definition-order-substitution\",",
            "\"worse-face-skips-later-family\",",
            "\"variable-axis-range-substitution\",",
            "\"missing-unicode-coverage\",",
            "\"pack-family-boundary-alias\"]}"
        )
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-FONT-COLLECTION-01\":\"ambient-or-single-family-or-stale-generation-or-registration-order-substitution\"}}"
    );
}

#[test]
#[ignore = "Phase 4 closure: validates every admitted color-font source"]
fn admitted_color_fonts_have_complete_owned_table_semantics() {
    super::application_color_tests::every_qualified_color_table_format_crosses_public_pack_admission();
    super::application_color_tests::qualified_colrv1_composite_and_gradient_enums_cross_public_admission();
    super::application_color_tests::owned_color_emoji_bytes_are_selected_as_one_complete_application_cluster();
    super::application_color_tests::repository_color_emoji_requires_resolvable_locations_and_intact_png_chunks();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-COLOR-FONT-ADMISSION-01\":4}}");
}

#[test]
fn malformed_or_unsupported_color_font_sources_deny_atomically() {
    super::application_color_tests::malformed_colr_and_sbix_tables_cannot_hide_behind_a_parseable_outline_font();
    super::application_color_tests::unknown_colrv1_composite_and_gradient_enums_deny_before_publication();
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-COLOR-FONT-ADMISSION-01\":\"unsupported-svg-or-layer-drop\"}}"
    );
}
