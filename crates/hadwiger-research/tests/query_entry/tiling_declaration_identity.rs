use worth_query::facade::{
    WORTHQueryDeclarationCanonicalEntry, WORTHQueryDeclarationCanonicalValue,
    WORTHQueryDeclarationInput, WORTHQueryDeclaredFamilyChecked,
};
use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger research handle should admit")
}

#[test]
fn tiling_declaration_inputs_publish_exact_canonical_entry_sequences() {
    assert_entries(
        MotifSeedDeclaration::new("motif-a")
            .with_source_family("parts-core")
            .with_novelty_signature("wl:a")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "motif_seed"),
            text("motif_id", "motif-a"),
            text("source_family", "parts-core"),
            text("novelty_signature", "wl:a"),
        ],
    );
    assert_entries(
        TerminalForcingStudyDeclaration::new("terminal-a", "motif-a")
            .with_terminal("right")
            .expect("terminal should admit")
            .with_terminal("left")
            .expect("terminal should admit")
            .with_relation_goal("must-differ")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "terminal_forcing_study"),
            text("study_id", "terminal-a"),
            text("motif_ref", "motif-a"),
            text("terminal.0000", "left"),
            text("terminal.0001", "right"),
            text("relation_goal", "must-differ"),
        ],
    );
    assert_entries(
        PeriodicQuotientCellDeclaration::new("cell-a")
            .with_lattice_basis_ref("lattice-a")
            .with_boundary_ownership_ref("boundary-a")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "periodic_quotient_cell"),
            text("cell_id", "cell-a"),
            text("lattice_basis_ref", "lattice-a"),
            text("boundary_ownership_ref", "boundary-a"),
        ],
    );
    assert_entries(
        GeneratedPatternClosureDeclaration::new("closure-a", "cell-a")
            .with_generator("translate:e2")
            .expect("generator should admit")
            .with_generator("translate:e1")
            .expect("generator should admit")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "generated_pattern_closure"),
            text("closure_id", "closure-a"),
            text("pattern_ref", "cell-a"),
            text("generator.0000", "translate:e1"),
            text("generator.0001", "translate:e2"),
        ],
    );
}

#[test]
fn tiling_declarations_cover_contact_conflict_and_core_identity() {
    assert_entries(
        TileContactWitnessDeclaration::new("contact-a")
            .with_left_tile_ref("tile-a")
            .with_right_tile_ref("tile-b")
            .with_contact_signature("center-north")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "tile_contact_witness"),
            text("contact_id", "contact-a"),
            text("left_tile_ref", "tile-a"),
            text("right_tile_ref", "tile-b"),
            text("contact_signature", "center-north"),
        ],
    );
    assert_entries(
        ConflictGraphExtractionDeclaration::new("extract-a", "cell-a")
            .with_distance_certificate_family("unit-distance-exact")
            .with_required_color_count(6)
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "conflict_graph_extraction"),
            text("extraction_id", "extract-a"),
            text("subject_ref", "cell-a"),
            text("distance_certificate_family", "unit-distance-exact"),
            unsigned("required_color_count", 6),
        ],
    );
    assert_entries(
        CoreExtractionDeclaration::new("core-a", "conflict-graph-a")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "core_extraction"),
            text("core_extraction_id", "core-a"),
            text("conflict_graph_ref", "conflict-graph-a"),
        ],
    );
    assert_entries(
        TilingEquivalenceClassificationDeclaration::new(
            "equiv-a",
            "exact_conflict_graph",
            "left-graph",
            "right-graph",
        )
        .canonical_declaration_entries(),
        &[
            text("declaration_kind", "tiling_equivalence_classification"),
            text("equivalence_id", "equiv-a"),
            text("scope", "exact_conflict_graph"),
            text("left_ref", "left-graph"),
            text("right_ref", "right-graph"),
        ],
    );
    assert_entries(
        TilingSuppressionDeclaration::new("suppress-a", "equiv-ref", "dead-end-ref")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "tiling_suppression"),
            text("suppression_id", "suppress-a"),
            text("equivalence_ref", "equiv-ref"),
            text("suppression_ref", "dead-end-ref"),
        ],
    );
    assert_entries(
        TilingReactivationDeclaration::new("reactivate-a", "suppression-ref", "new-evidence-ref")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "tiling_reactivation"),
            text("reactivation_id", "reactivate-a"),
            text("suppression_ref", "suppression-ref"),
            text("qualifying_evidence_ref", "new-evidence-ref"),
        ],
    );
}

#[test]
fn tiling_request_shape_validation_rejects_empty_and_duplicate_identity_fields() {
    assert_eq!(
        MotifSeedDeclaration::try_new("  "),
        Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField { field: "motif_id" })
    );
    assert_eq!(
        TerminalForcingStudyDeclaration::new("terminal-a", "motif-a").with_terminal("  "),
        Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField { field: "terminal" })
    );
    assert_eq!(
        TerminalForcingStudyDeclaration::new("terminal-a", "motif-a")
            .with_terminal("left")
            .expect("first terminal should admit")
            .with_terminal("left"),
        Err(
            HadwigerResearchDeclarationShapeError::DuplicateIdentityField {
                field: "terminal",
                value: "left".to_string(),
            }
        )
    );
    assert_eq!(
        GeneratedPatternClosureDeclaration::new("closure-a", "cell-a").with_generator(""),
        Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField { field: "generator" })
    );
    assert_eq!(
        GeneratedPatternClosureDeclaration::new("closure-a", "cell-a")
            .with_generator("translate:e1")
            .expect("first generator should admit")
            .with_generator("translate:e1"),
        Err(
            HadwigerResearchDeclarationShapeError::DuplicateIdentityField {
                field: "generator",
                value: "translate:e1".to_string(),
            }
        )
    );
    assert_eq!(
        CoreExtractionDeclaration::try_new("core-a", ""),
        Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField {
            field: "conflict_graph_ref"
        })
    );
}

#[test]
fn changed_tiling_identity_changes_declaration_digest() {
    let handle = handle();
    let left = admitted_declaration(declare_research_request_checked(
        &handle,
        ConflictGraphExtractionDeclaration::new("extract-a", "cell-a")
            .with_distance_certificate_family("unit-distance-exact"),
    ));
    let changed = admitted_declaration(declare_research_request_checked(
        &handle,
        ConflictGraphExtractionDeclaration::new("extract-a", "cell-a")
            .with_distance_certificate_family("interval-exact"),
    ));

    assert_ne!(left.declaration_digest(), changed.declaration_digest());
}

#[test]
fn equivalent_tiling_repeated_fields_converge_despite_insertion_order() {
    let handle = handle();
    let left = admitted_declaration(declare_research_request_checked(
        &handle,
        GeneratedPatternClosureDeclaration::new("closure-a", "cell-a")
            .with_generator("translate:e2")
            .expect("generator should admit")
            .with_generator("translate:e1")
            .expect("generator should admit"),
    ));
    let right = admitted_declaration(declare_research_request_checked(
        &handle,
        GeneratedPatternClosureDeclaration::new("closure-a", "cell-a")
            .with_generator("translate:e1")
            .expect("generator should admit")
            .with_generator("translate:e2")
            .expect("generator should admit"),
    ));

    assert_eq!(left.declaration_digest(), right.declaration_digest());
}

#[test]
fn changed_tiling_optional_metadata_changes_declaration_digest() {
    let handle = handle();
    assert_digest_changes(
        admitted_declaration(declare_research_request_checked(
            &handle,
            MotifSeedDeclaration::new("motif-a").with_source_family("parts-core"),
        )),
        admitted_declaration(declare_research_request_checked(
            &handle,
            MotifSeedDeclaration::new("motif-a").with_source_family("kuratowski-like"),
        )),
    );
    assert_digest_changes(
        admitted_declaration(declare_research_request_checked(
            &handle,
            PeriodicQuotientCellDeclaration::new("cell-a")
                .with_boundary_ownership_ref("half-open-x"),
        )),
        admitted_declaration(declare_research_request_checked(
            &handle,
            PeriodicQuotientCellDeclaration::new("cell-a")
                .with_boundary_ownership_ref("closed-left-open-right"),
        )),
    );
    assert_digest_changes(
        admitted_declaration(declare_research_request_checked(
            &handle,
            TileContactWitnessDeclaration::new("contact-a").with_contact_signature("center-north"),
        )),
        admitted_declaration(declare_research_request_checked(
            &handle,
            TileContactWitnessDeclaration::new("contact-a").with_contact_signature("center-east"),
        )),
    );
    assert_digest_changes(
        admitted_declaration(declare_research_request_checked(
            &handle,
            ConflictGraphExtractionDeclaration::new("extract-a", "cell-a")
                .with_required_color_count(5),
        )),
        admitted_declaration(declare_research_request_checked(
            &handle,
            ConflictGraphExtractionDeclaration::new("extract-a", "cell-a")
                .with_required_color_count(6),
        )),
    );
}

#[test]
fn minimal_tiling_canonical_entries_omit_empty_identity_values() {
    assert_no_empty_identity_values(MotifSeedDeclaration::new("motif-a"));
    assert_no_empty_identity_values(TerminalForcingStudyDeclaration::new(
        "terminal-a",
        "motif-a",
    ));
    assert_no_empty_identity_values(PeriodicQuotientCellDeclaration::new("cell-a"));
    assert_no_empty_identity_values(GeneratedPatternClosureDeclaration::new(
        "closure-a",
        "cell-a",
    ));
    assert_no_empty_identity_values(TileContactWitnessDeclaration::new("contact-a"));
    assert_no_empty_identity_values(ConflictGraphExtractionDeclaration::new(
        "extract-a",
        "cell-a",
    ));
    assert_no_empty_identity_values(CoreExtractionDeclaration::new("core-a", "conflict-graph-a"));
    assert_no_empty_identity_values(TilingEquivalenceClassificationDeclaration::new(
        "equiv-a",
        "exact_conflict_graph",
        "left-graph",
        "right-graph",
    ));
    assert_no_empty_identity_values(TilingSuppressionDeclaration::new(
        "suppress-a",
        "equiv-ref",
        "dead-end-ref",
    ));
    assert_no_empty_identity_values(TilingReactivationDeclaration::new(
        "reactivate-a",
        "suppression-ref",
        "new-evidence-ref",
    ));
}

fn assert_digest_changes<I>(
    left: worth_query::facade::WORTHQueryCanonicalDeclarationArtifact<
        HadwigerResearchDomainEntry,
        I,
    >,
    right: worth_query::facade::WORTHQueryCanonicalDeclarationArtifact<
        HadwigerResearchDomainEntry,
        I,
    >,
) where
    I: HadwigerResearchDeclarationInput,
{
    assert_ne!(left.declaration_digest(), right.declaration_digest());
}

fn admitted_declaration<I>(
    checked: WORTHQueryDeclaredFamilyChecked<HadwigerResearchDomainEntry, I>,
) -> worth_query::facade::WORTHQueryCanonicalDeclarationArtifact<HadwigerResearchDomainEntry, I>
where
    I: HadwigerResearchDeclarationInput,
{
    match checked {
        WORTHQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("Hadwiger tiling declaration should admit"),
    }
}

fn assert_no_empty_identity_values<I>(request: I)
where
    I: HadwigerResearchDeclarationInput,
{
    let entries = request.canonical_declaration_entries();
    assert!(entries.iter().all(|entry| !entry.locus().is_empty()));
    assert!(entries.iter().all(|entry| match entry.value() {
        WORTHQueryDeclarationCanonicalValue::ExactText(value) => !value.trim().is_empty(),
        _ => true,
    }));
}

fn assert_entries(
    actual: Vec<WORTHQueryDeclarationCanonicalEntry>,
    expected: &[WORTHQueryDeclarationCanonicalEntry],
) {
    assert_eq!(actual, expected);
    assert_eq!(actual[0].locus(), "declaration_kind");
    assert!(actual.iter().all(|entry| !entry.locus().is_empty()));
}

fn text(locus: &str, value: &str) -> WORTHQueryDeclarationCanonicalEntry {
    WORTHQueryDeclarationCanonicalEntry::text(locus, value)
}

fn unsigned(locus: &str, value: u128) -> WORTHQueryDeclarationCanonicalEntry {
    WORTHQueryDeclarationCanonicalEntry::new(
        locus,
        worth_query::facade::WORTHQueryDeclarationCanonicalEntryKind::Field,
        WORTHQueryDeclarationCanonicalValue::UnsignedInteger(value),
    )
}
