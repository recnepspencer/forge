//! Governed intrinsic-color evidence assembled from production boundaries.

pub(in crate::raster) fn prove_color_batch_family() -> usize {
    super::tests::qualified_color_transaction_preserves_cluster_and_rgba_identity();

    let source = "👩\u{200d}💻";
    let layout = super::tests::layout_for(source);
    let demand = super::tests::demand_for(&layout, source);
    let raster = super::rasterize_intrinsic_color(&layout, &demand).unwrap();
    usize::from(!raster.batch().records().is_empty())
}

pub(in crate::raster) fn prove_every_intrinsic_color_raster() {
    super::tests::text_and_emoji_variation_selectors_choose_distinct_raster_lanes();
    super::tests::every_unicode_17_rgi_sequence_crosses_intrinsic_color_owner();
    super::transaction_tests::distinct_layout_attributions_share_one_color_raster_key();
    assert_eq!(prove_color_batch_family(), 1);
}

pub(in crate::raster) fn reject_intrinsic_color_mutants() {
    super::tests::intrinsic_color_ignores_mounted_foreground_tint();
    super::tests::variation_selector_stays_in_one_intrinsic_color_cluster();
    super::tests::source_substitution_is_rejected_by_positioned_glyph_provenance();
}
