use forge_query::facade::{
    ForgeQueryDeclarationEntryCrossingSurface, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationLegalityClass, ForgeQueryDeclarationPrimaryAuthorityFamily,
    ForgeQueryDeclarationRelationalTruthClaim, ForgeQueryDeclaredFamilyChecked,
    ForgeQueryGroupedDeclarationPosture, ForgeQueryLowerAuthorityRouteFamily,
    ForgeQueryOrdinaryOutcome, ForgeQuerySignalCompatibilityPosture,
};
use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger research handle should admit")
}

#[test]
fn tiling_declaration_families_use_distinct_relational_contracts() {
    assert_tiling_family::<MotifSeedDeclarationFamily>("hadwiger.tiling.motif_seed");
    assert_tiling_family::<TerminalForcingStudyDeclarationFamily>(
        "hadwiger.tiling.terminal_forcing_study",
    );
    assert_tiling_family::<PeriodicQuotientCellDeclarationFamily>(
        "hadwiger.tiling.periodic_quotient_cell",
    );
    assert_tiling_family::<GeneratedPatternClosureDeclarationFamily>(
        "hadwiger.tiling.generated_pattern_closure",
    );
    assert_tiling_family::<TileContactWitnessDeclarationFamily>(
        "hadwiger.tiling.tile_contact_witness",
    );
    assert_tiling_family::<ConflictGraphExtractionDeclarationFamily>(
        "hadwiger.tiling.conflict_graph_extraction",
    );
    assert_tiling_family::<CoreExtractionDeclarationFamily>("hadwiger.tiling.core_extraction");
}

#[test]
fn tiling_orchestration_helper_returns_query_ordinary_outcome() {
    let handle = handle();
    let outcome = orchestrate_research_request_entry(
        &handle,
        PeriodicQuotientCellDeclaration::new("cell-a")
            .with_lattice_basis_ref("lattice-a")
            .with_boundary_ownership_ref("boundary-a"),
    );

    match outcome {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => {
            assert_eq!(
                envelope.declaration_family_key(),
                "hadwiger.tiling.periodic_quotient_cell"
            );
        }
        _ => panic!("expected tiling declaration entry to return a Query ordinary outcome"),
    }
}

#[test]
fn all_tiling_declarations_admit_through_hadwiger_helper_path() {
    let handle = handle();

    let declarations = [
        admitted_declaration(declare_research_request_checked(
            &handle,
            MotifSeedDeclaration::new("motif-a"),
        ))
        .declaration_family_key()
        .to_string(),
        admitted_declaration(declare_research_request_checked(
            &handle,
            TerminalForcingStudyDeclaration::new("terminal-a", "motif-a"),
        ))
        .declaration_family_key()
        .to_string(),
        admitted_declaration(declare_research_request_checked(
            &handle,
            PeriodicQuotientCellDeclaration::new("cell-a"),
        ))
        .declaration_family_key()
        .to_string(),
        admitted_declaration(declare_research_request_checked(
            &handle,
            GeneratedPatternClosureDeclaration::new("closure-a", "cell-a"),
        ))
        .declaration_family_key()
        .to_string(),
        admitted_declaration(declare_research_request_checked(
            &handle,
            TileContactWitnessDeclaration::new("contact-a"),
        ))
        .declaration_family_key()
        .to_string(),
        admitted_declaration(declare_research_request_checked(
            &handle,
            ConflictGraphExtractionDeclaration::new("extract-a", "cell-a"),
        ))
        .declaration_family_key()
        .to_string(),
        admitted_declaration(declare_research_request_checked(
            &handle,
            CoreExtractionDeclaration::new("core-a", "conflict-graph-a"),
        ))
        .declaration_family_key()
        .to_string(),
    ];

    assert_eq!(
        declarations,
        [
            "hadwiger.tiling.motif_seed",
            "hadwiger.tiling.terminal_forcing_study",
            "hadwiger.tiling.periodic_quotient_cell",
            "hadwiger.tiling.generated_pattern_closure",
            "hadwiger.tiling.tile_contact_witness",
            "hadwiger.tiling.conflict_graph_extraction",
            "hadwiger.tiling.core_extraction",
        ]
    );
}

#[test]
fn tiling_helper_and_direct_query_declarations_converge() {
    let handle = handle();
    let request = MotifSeedDeclaration::new("motif-a")
        .with_source_family("parts-core")
        .with_novelty_signature("wl:a");
    let helper = admitted_declaration(declare_research_request_checked(&handle, request.clone()));
    let direct = admitted_declaration(handle.declare_checked(request));

    assert_eq!(helper.declaration_digest(), direct.declaration_digest());
}

#[test]
fn tiling_readiness_and_inventory_are_query_owned_seam_rows() {
    let handle = handle();
    assert_family_inventory::<MotifSeedDeclaration>(&handle);
    assert_family_inventory::<TerminalForcingStudyDeclaration>(&handle);
    assert_family_inventory::<PeriodicQuotientCellDeclaration>(&handle);
    assert_family_inventory::<GeneratedPatternClosureDeclaration>(&handle);
    assert_family_inventory::<TileContactWitnessDeclaration>(&handle);
    assert_family_inventory::<ConflictGraphExtractionDeclaration>(&handle);
    assert_family_inventory::<CoreExtractionDeclaration>(&handle);
}

fn assert_family_inventory<I>(handle: &HadwigerResearchHandle)
where
    I: HadwigerResearchDeclarationInput,
{
    let inventory = research_declaration_entry_inventory::<I>(handle);
    let readiness = research_declaration_entry_readiness::<I>(handle);
    assert!(!inventory.rows().is_empty());
    assert!(!readiness.rows().is_empty());
    assert!(inventory.rows().iter().any(|row| {
        row.surface() == ForgeQueryDeclarationEntryCrossingSurface::RelationalTruthRouting
            && row.relational_truth_claim().is_some()
    }));
}

fn assert_tiling_family<F>(family_key: &'static str)
where
    F: ForgeQueryDeclarationFamilyMarker<HadwigerResearchDomainEntry>,
{
    let taxonomy = F::taxonomy();
    assert_eq!(F::semantic_family_key(), family_key);
    assert_eq!(
        taxonomy.primary_authority_family(),
        ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth
    );
    assert_eq!(
        taxonomy.signal_compatibility(),
        ForgeQuerySignalCompatibilityPosture::NotCompatible
    );
    assert_eq!(
        taxonomy.grouped_posture(),
        ForgeQueryGroupedDeclarationPosture::NeighborhoodCapable
    );
    assert_eq!(
        F::legality_contract().legality_class(),
        ForgeQueryDeclarationLegalityClass::AuthoritativeHotArtifact
    );
    assert_eq!(
        F::route_contract().allowed_route_families(),
        &[ForgeQueryLowerAuthorityRouteFamily::Relational]
    );
    assert!(!F::route_contract().can_defer());
    assert_eq!(
        F::relational_truth_contract()
            .expect("tiling family should declare relational truth contract")
            .truth_claim(),
        ForgeQueryDeclarationRelationalTruthClaim::AuthoritativeCurrentTruth
    );
}

fn admitted_declaration<I>(
    checked: ForgeQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>,
) -> forge_query::facade::ForgeQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    match checked {
        ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("Hadwiger tiling declaration should admit"),
    }
}
