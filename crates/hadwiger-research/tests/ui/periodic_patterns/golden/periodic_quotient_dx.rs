use hadwiger_research::facade::{
    BoundaryOwnershipPolicy, ExactRational, GeneratedPatternReplaySuite, PeriodicQuotientCell,
    PeriodicTranslationRule, RectangularTileRegion, TilingCell, TilingColorId,
    certify_periodic_quotient_replay_checked, admit_hadwiger_research_handle,
    HadwigerCanonicalArtifact, HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");
    let cell = TilingCell::builder("cell-a")
        .with_rectangular_tile(
            RectangularTileRegion::new(
                "tile-a",
                TilingColorId::new("red").expect("color admits"),
                ExactRational::integer(0),
                ExactRational::integer(1),
                ExactRational::integer(0),
                ExactRational::integer(1),
            )
            .expect("region admits")
            .with_boundary_ownership(
                BoundaryOwnershipPolicy::owned_half_open("left,bottom").expect("policy admits"),
            ),
        )
        .expect("tile admits")
        .finish()
        .expect("cell admits");
    let quotient = PeriodicQuotientCell::builder("quotient-a", cell.reference())
        .with_source_cell(cell)
        .with_lattice_basis_vector("u", ExactRational::integer(2), ExactRational::integer(0))
        .expect("basis admits")
        .with_translation_rule(
            PeriodicTranslationRule::new("wrap", "tile-a", "tile-a")
                .with_translation("u")
                .expect("translation admits")
                .with_color_preserved()
                .expect("rule admits"),
        )
        .expect("rule attaches")
        .finish()
        .expect("quotient admits");
    let suite = GeneratedPatternReplaySuite::builder("suite-a", quotient.reference())
        .with_periodic_quotient_cell(quotient)
        .expect("quotient attaches")
        .finish()
        .expect("suite admits");
    let checked = certify_periodic_quotient_replay_checked(&handle, suite).expect("replays");

    assert!(!checked.admits_theorem_authority());
}
