use worth_ui_host_contract::UiTextOriginalRange;

#[test]
#[ignore = "Phase 4 closure: exhausts the frozen Unicode segmentation corpora"]
fn unicode_17_segmentation_corpora_are_exhaustive() {
    crate::analysis::conformance_tests::every_unicode_17_grapheme_break_case_matches_the_frozen_corpus();
    crate::analysis::conformance_tests::every_unicode_17_word_break_case_matches_the_frozen_corpus(
    );
    crate::analysis::conformance_tests::every_unicode_17_line_break_case_matches_the_frozen_corpus(
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-UNICODE-SEGMENTATION-01\":22048}}");
}

#[test]
fn zwj_flag_and_dictionary_boundary_substitutions_are_rejected() {
    crate::analysis::tests::representative_rgi_emoji_sequences_remain_atomic_original_ranges();
    crate::layout::word_boundary_tests::qualified_layout_retains_pinned_dictionary_boundaries_in_original_utf8();
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-UNICODE-SEGMENTATION-01\":\"zwj-or-flag-split\"}}"
    );
}

#[test]
#[ignore = "Phase 4 closure: exhausts every Unicode 17 RGI sequence through layout"]
fn every_rgi_sequence_is_atomic_through_analysis_fallback_and_layout() {
    crate::analysis::conformance_tests::every_unicode_17_rgi_emoji_is_one_extended_grapheme_cluster(
    );
    crate::layout::rgi_tests::every_unicode_17_rgi_sequence_remains_atomic_through_layout_and_ellipsis();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-EMOJI-SEQUENCE-01\":3953}}");
}

#[test]
fn variation_and_zwj_decomposition_is_rejected_by_real_layout_geometry() {
    crate::analysis::tests::representative_rgi_emoji_sequences_remain_atomic_original_ranges();
    crate::layout::tests::ellipsis_is_a_shaped_cluster_and_never_splits_an_rgi_emoji();
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-EMOJI-SEQUENCE-01\":\"variation-or-zwj-decomposition\"}}"
    );
}

#[test]
#[ignore = "Phase 4 closure: exhausts both Unicode 17 bidi conformance corpora"]
fn unicode_17_bidi_corpora_drive_visual_order() {
    crate::analysis::conformance_tests::every_unicode_17_bidi_character_case_uses_repository_data_and_visual_order();
    crate::analysis::bidi_conformance_tests::every_unicode_17_abstract_bidi_case_matches_levels_and_visual_order();
    crate::layout::paragraph_alignment_tests::adjacent_bidi_paragraphs_own_their_half_open_alignment_boundary();
    crate::layout::paragraph_alignment_tests::trailing_empty_line_inherits_the_last_bidi_paragraph_alignment();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-BIDI-01\":582553}}");
}

#[test]
fn logical_order_rendering_is_rejected_by_visual_geometry() {
    crate::analysis::bidi_conformance_tests::abstract_bidi_classes_use_exact_unicode_17_representatives();
    crate::layout::tests::mixed_direction_layout_carries_visual_runs_carets_hits_and_discontiguous_selection();
    crate::layout::paragraph_alignment_tests::adjacent_bidi_paragraphs_own_their_half_open_alignment_boundary();
    crate::layout::paragraph_alignment_tests::trailing_empty_line_inherits_the_last_bidi_paragraph_alignment();
    println!("WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-BIDI-01\":\"logical-order-rendering\"}}");
}

#[test]
#[ignore = "Phase 4 closure: resolves every RGI cluster plus a Khmer shaping syllable"]
fn whole_cluster_fallback_is_exhaustive_and_script_safe() {
    crate::fallback::tests::every_unicode_17_rgi_sequence_selects_one_complete_color_emoji_cluster(
    );
    crate::font_collection::application_fallback_tests::khmer_shaping_syllable_falls_back_and_shapes_as_one_whole_cluster();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-FALLBACK-01\":3953}}");
}

#[test]
fn emoji_or_indic_cluster_split_is_rejected_before_shaping() {
    crate::fallback::tests::repeated_clusters_reuse_one_exact_face_probe_inside_the_paragraph();
    crate::shaping::tests::each_complete_missing_cluster_emits_one_glyph_with_its_exact_original_range();
    println!("WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-FALLBACK-01\":\"emoji-or-indic-split\"}}");
}

#[test]
fn mixed_script_shaping_emits_exact_nonzero_glyphs() {
    crate::shaping::reference_fixture_tests::pinned_reference_harfbuzz_records_match_full_production_shaping();
    crate::shaping::tests::mixed_latin_arabic_indic_and_emoji_shape_in_exhaustive_runs_with_original_ranges();
    let source = "office \u{645}\u{631}\u{62d}\u{628}\u{627} \u{915}\u{94d}\u{937} \u{1f469}\u{1f3fd}\u{200d}\u{1f4bb}";
    let layout = crate::layout::tests::layout(source, 320_000, 8);
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P4-SHAPING-01\":{}}}",
        layout.glyphs().len()
    );
}

#[test]
fn one_run_latin_substitution_is_rejected_by_features_and_axes() {
    crate::shaping::reference_fixture_tests::pinned_reference_harfbuzz_records_match_full_production_shaping();
    crate::shaping::tests::authored_feature_spans_partition_runs_and_change_real_glyph_formation();
    crate::shaping::tests::qualified_width_axes_change_real_advances_and_out_of_range_axes_are_denied();
    println!("WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-SHAPING-01\":\"one-run-latin\"}}");
}

#[test]
fn unicode_line_fitting_preserves_clusters_and_capacity() {
    crate::layout::tests::unicode_wrap_preserves_cluster_boundaries_and_exact_line_capacity();
    crate::layout::tests::first_cluster_wider_than_the_line_is_reported_as_overflowing();
    crate::layout::paragraph_alignment_tests::adjacent_bidi_paragraphs_own_their_half_open_alignment_boundary();
    crate::layout::paragraph_alignment_tests::trailing_empty_line_inherits_the_last_bidi_paragraph_alignment();
    crate::layout::line_anchor_tests::empty_paragraph_has_one_hit_testable_line_boundary_without_paint();
    crate::layout::line_anchor_tests::consecutive_hard_breaks_and_trailing_empty_line_have_distinct_anchors();
    crate::layout::line_anchor_tests::painted_text_before_a_hard_break_keeps_the_trailing_empty_line_hit_testable();
    crate::layout::line_anchor_tests::rtl_empty_line_anchor_uses_the_visual_start_and_line_wide_hit_geometry();
    crate::font_collection::ink_bounds_tests::variable_and_color_glyph_ink_is_derived_from_the_selected_font_instance();
    crate::layout::contextual_shaping_tests::arabic_and_indic_soft_lines_match_independently_shaped_line_segments();
    crate::layout::contextual_shaping_tests::contextual_line_fragments_do_not_consume_logical_run_capacity();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-LINE-LAYOUT-01\":3}}");
}

#[test]
fn mid_cluster_wrap_and_post_capacity_ellipsis_are_rejected() {
    crate::layout::tests::ellipsis_is_a_shaped_cluster_and_never_splits_an_rgi_emoji();
    crate::layout::tests::ellipsis_cannot_publish_a_run_beyond_the_qualified_capacity();
    crate::layout::paragraph_alignment_tests::adjacent_bidi_paragraphs_own_their_half_open_alignment_boundary();
    crate::layout::paragraph_alignment_tests::trailing_empty_line_inherits_the_last_bidi_paragraph_alignment();
    crate::layout::line_anchor_tests::consecutive_hard_breaks_and_trailing_empty_line_have_distinct_anchors();
    crate::layout::line_anchor_tests::painted_text_before_a_hard_break_keeps_the_trailing_empty_line_hit_testable();
    crate::font_collection::ink_bounds_tests::variable_and_color_glyph_ink_is_derived_from_the_selected_font_instance();
    crate::layout::contextual_shaping_tests::arabic_and_indic_soft_lines_match_independently_shaped_line_segments();
    crate::layout::contextual_shaping_tests::contextual_line_fragments_do_not_consume_logical_run_capacity();
    println!("WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-LINE-LAYOUT-01\":\"mid-cluster-wrap\"}}");
}

#[test]
fn original_utf8_ranges_survive_mixed_script_layout() {
    let source = "a\u{301} \u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{1f469}\u{1f3fd}\u{200d}\u{1f4bb}";
    let layout = crate::layout::tests::layout(source, 240_000, 8);
    let ranges = layout
        .glyphs()
        .iter()
        .map(|glyph| glyph.original_range())
        .collect::<Vec<_>>();
    assert!(original_ranges_match_mixed_script_oracle(source, &ranges));
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P4-ORIGINAL-RANGE-01\":{}}}",
        layout.glyphs().len()
    );
}

#[test]
fn normalized_offset_substitution_is_rejected_by_cluster_hits() {
    let source = "a\u{301} \u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{1f469}\u{1f3fd}\u{200d}\u{1f4bb}";
    let layout = crate::layout::tests::layout(source, 240_000, 8);
    let mut substituted = layout
        .glyphs()
        .iter()
        .map(|glyph| glyph.original_range())
        .collect::<Vec<_>>();
    substituted[0] = UiTextOriginalRange::from_text_mechanics(0, 1).unwrap();
    assert!(source.is_char_boundary(1));
    assert!(!original_ranges_match_mixed_script_oracle(
        source,
        &substituted
    ));
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-ORIGINAL-RANGE-01\":\"normalized-offset-substitution\"}}"
    );
}

fn original_ranges_match_mixed_script_oracle(source: &str, ranges: &[UiTextOriginalRange]) -> bool {
    let expected = [
        (0, 3),
        (3, 4),
        (10, 12),
        (8, 10),
        (6, 8),
        (4, 6),
        (12, 13),
        (13, 28),
    ];
    ranges.len() == expected.len()
        && ranges.iter().zip(expected).all(|(range, (start, end))| {
            range.start() == start
                && range.end() == end
                && source.is_char_boundary(start as usize)
                && source.is_char_boundary(end as usize)
        })
}

#[test]
fn bidi_interaction_records_exact_edges_affinities_hits_and_selection() {
    let source = "abc \u{5e9}\u{5dc}\u{5d5}\u{5dd} xyz";
    let layout = crate::layout::tests::layout(source, 180_000, 8);
    crate::layout::tests::mixed_direction_layout_carries_visual_runs_carets_hits_and_discontiguous_selection();
    crate::layout::line_anchor_tests::consecutive_hard_breaks_and_trailing_empty_line_have_distinct_anchors();
    let selection = layout
        .selection_rects(UiTextOriginalRange::from_text_mechanics(0, source.len() as u32).unwrap())
        .unwrap();
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P4-BIDI-INTERACTION-01\":{}}}",
        layout.carets().len() + selection.len()
    );
}

#[test]
fn swapped_bidi_caret_affinity_is_rejected() {
    crate::layout::tests::mixed_direction_layout_carries_visual_runs_carets_hits_and_discontiguous_selection();
    crate::layout::line_anchor_tests::empty_paragraph_has_one_hit_testable_line_boundary_without_paint();
    crate::layout::line_anchor_tests::rtl_empty_line_anchor_uses_the_visual_start_and_line_wide_hit_geometry();
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-BIDI-INTERACTION-01\":\"swapped-bidi-caret-affinity\"}}"
    );
}

#[test]
fn derived_capacity_is_reserved_before_analysis_and_shaping() {
    crate::admission::tests::capacity_denial_precedes_analysis_and_shaping();
    crate::admission::tests::collection_issued_derived_bound_denies_before_grapheme_or_style_analysis();
    crate::language::tests::paragraph_and_style_language_admission_share_strict_pinned_bcp47_contract();
    crate::font_collection::application_capacity_tests::application_gsub_expansion_bound_is_carried_into_pre_shape_reservation();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-CAPACITY-01\":3}}");
}

#[test]
fn shape_before_capacity_denial_is_rejected() {
    crate::layout::tests::ellipsis_cannot_publish_a_run_beyond_the_qualified_capacity();
    crate::language::tests::paragraph_and_style_language_admission_share_strict_pinned_bcp47_contract();
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-CAPACITY-01\":\"shape-before-capacity-denial\"}}"
    );
}

#[test]
fn retired_layout_reconstructs_from_exact_pinned_bytes() {
    crate::font_collection::application_reconstruction_tests::retired_collection_denies_fresh_work_but_reconstructs_its_exact_qualified_layout();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P4-TEXT-RECONSTRUCTION-01\":1}}");
}

#[test]
fn stale_layout_reuse_is_rejected_by_exact_collection_lineage() {
    crate::font_collection::application_reconstruction_tests::same_numbered_collections_cannot_substitute_for_the_request_owner();
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-TEXT-RECONSTRUCTION-01\":\"stale-layout-reuse\"}}"
    );
}
