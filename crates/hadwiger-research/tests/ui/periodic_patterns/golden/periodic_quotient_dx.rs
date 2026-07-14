use hadwiger_research::facade::{
    BoundaryOwnershipPolicy, ExactRational, GeneratedPatternReplaySuite, PeriodicQuotientCell,
    PeriodicTranslationRule, RectangularTileRegion, TilingCell, TilingColorId,
    certify_periodic_quotient_replay_checked, hadwiger_research_domain_package,
    HadwigerCanonicalArtifact, HadwigerResearchDomainEntry, HadwigerResearchHandle,
    HadwigerResearchOperatingContext, HadwigerResearchQueryExt,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

fn main() {
    let handle = installed_declarations().expect("handle admits");
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

fn installed_declarations() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id").map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime().with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-periodic-quotient-dx").map_err(|error| error.to_string())?;
    let installed = workspace.domain(HadwigerResearchDomainEntry).map_err(|error| error.to_string())?;
    installed.research_declarations(&workspace, HadwigerResearchOperatingContext::default())
        .map_err(|error| error.to_string())
}
