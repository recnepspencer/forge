//! Governed raster evidence assembled without creating a second raster owner.

pub(crate) fn prove_exact_raster_authority() -> usize {
    super::demand_alpha_tests::demand_uses_layout_owned_lineage_and_alpha_raster_reuses_exact_misses();
    super::alpha_transaction_tests::distinct_layout_attributions_share_one_alpha_raster_key();

    super::demand_alpha_tests::qualified_alpha_batch_family_count()
        + super::color::phase5_evidence::prove_color_batch_family()
}

pub(crate) fn prove_every_intrinsic_color_raster() {
    super::color::phase5_evidence::prove_every_intrinsic_color_raster();
}

pub(crate) fn reject_intrinsic_color_mutants() {
    super::demand_alpha_tests::same_generation_foreign_lineage_is_denied_before_outline_work();
    super::color::phase5_evidence::reject_intrinsic_color_mutants();
}
