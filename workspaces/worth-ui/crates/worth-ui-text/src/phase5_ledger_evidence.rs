//! Exact Phase 5 raster and color evidence over the production owners.

#[test]
fn qualified_alpha_and_color_raster_cross_exact_production_authority() {
    crate::font_collection::phase5_raster_evidence::prove_alpha_oracles();
    let raster_batches = crate::raster::phase5_evidence::prove_exact_raster_authority();
    assert_eq!(raster_batches, 2);
    println!(
        "WORTH_UI_LEDGER_CASES={{\"P5-GLYPH-RASTER-01\":[\"exact-demand-identity\",\"fractional-origin\",\"variable-outline\",\"last-resort-outline\",\"cross-layout-raster-reuse\",\"qualified-alpha-color-batches\"]}}"
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P5-GLYPH-RASTER-01\":{raster_batches}}}");
}

#[test]
fn every_qualified_color_source_and_rgi_sequence_crosses_production_raster() {
    let rgi_sequences = crate::font_collection::phase5_raster_evidence::prove_color_sources();
    assert_eq!(rgi_sequences, 3_953);
    crate::raster::phase5_evidence::prove_every_intrinsic_color_raster();
    println!(
        "WORTH_UI_LEDGER_CASES={{\"P5-COLOR-EMOJI-01\":[\"colrv0-cpal\",\"colrv1-cpal\",\"cbdt-cblc\",\"sbix-png-dupe\",\"selector-lane\",\"exhaustive-rgi\",\"gradient-composite\",\"nonseparable-composite\",\"bitmap-composite\"]}}"
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P5-COLOR-EMOJI-01\":{rgi_sequences}}}");
}

#[test]
fn emoji_tint_split_and_unqualified_color_sources_are_rejected() {
    crate::raster::phase5_evidence::reject_intrinsic_color_mutants();
    crate::font_collection::phase5_raster_evidence::reject_color_source_mutants();
    println!(
        "WORTH_UI_LEDGER_MUTATION_CASES={{\"P5-COLOR-EMOJI-01\":[\"foreground-tint\",\"cluster-split\",\"source-substitution\",\"malformed-graph\",\"unsupported-bitmap\",\"unbounded-current-color\"]}}"
    );
    println!("WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P5-COLOR-EMOJI-01\":\"emoji-tint-or-split\"}}");
}
