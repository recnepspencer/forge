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
    let rgi_sequences = std::thread::scope(|scope| {
        let font_sources =
            scope.spawn(|| crate::font_collection::phase5_raster_evidence::prove_color_sources());
        let intrinsic_raster = scope.spawn(|| {
            crate::raster::phase5_evidence::prove_every_intrinsic_color_raster();
        });
        intrinsic_raster
            .join()
            .expect("intrinsic-color raster evidence passed");
        font_sources
            .join()
            .expect("font-backed color-source evidence passed")
    });
    assert_eq!(rgi_sequences, 3_953);
    println!(
        "WORTH_UI_LEDGER_CASES={{\"P5-COLOR-EMOJI-01\":[\"colrv0-cpal\",\"colrv1-cpal\",\"cbdt-cblc\",\"sbix-png-dupe\",\"selector-lane\",\"exhaustive-rgi\",\"gradient-composite\",\"nonseparable-composite\",\"bitmap-composite\"]}}"
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P5-COLOR-EMOJI-01\":{rgi_sequences}}}");
}

#[test]
fn emoji_tint_split_and_unqualified_color_sources_are_rejected() {
    std::thread::scope(|scope| {
        let raster_mutants =
            scope.spawn(crate::raster::phase5_evidence::reject_intrinsic_color_mutants);
        let source_mutants = scope
            .spawn(crate::font_collection::phase5_raster_evidence::reject_color_source_mutants);
        raster_mutants
            .join()
            .expect("intrinsic-color raster mutants rejected");
        source_mutants
            .join()
            .expect("font-backed color-source mutants rejected");
    });
    println!(
        "WORTH_UI_LEDGER_MUTATION_CASES={{\"P5-COLOR-EMOJI-01\":[\"foreground-tint\",\"cluster-split\",\"source-substitution\",\"malformed-graph\",\"unsupported-bitmap\",\"unbounded-current-color\"]}}"
    );
    println!("WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P5-COLOR-EMOJI-01\":\"emoji-tint-or-split\"}}");
}

#[test]
fn pure_dpi_replaces_raster_identity_without_relayout() {
    crate::raster::demand_identity_tests::dpi_is_a_raster_identity_boundary_without_relayout();
    println!(
        "WORTH_UI_LEDGER_CASES={{\"P5-TEXT-DPI-01\":[\"same-layout-identity\",\"same-logical-attribution\",\"new-dpi-raster-keys\"]}}"
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P5-TEXT-DPI-01\":1}}");
}

#[test]
fn stale_dpi_raster_is_rejected_by_complete_successor_keys() {
    crate::raster::demand_identity_tests::stale_dpi_raster_key_cannot_satisfy_the_successor_demand(
    );
    println!("WORTH_UI_LEDGER_MUTATION_CASES={{\"P5-TEXT-DPI-01\":[\"stale-dpi-raster-key\"]}}");
    println!("WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P5-TEXT-DPI-01\":\"stale-dpi-raster\"}}");
}
