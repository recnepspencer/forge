use worth_query::facade::{
    WORTHQueryDeclarationCanonicalEntry, WORTHQueryDeclarationCanonicalEntryKind,
    WORTHQueryDeclarationCanonicalValue, WORTHQueryDeclarationInput,
};
use hadwiger_research::facade::*;

#[test]
fn all_declaration_inputs_publish_exact_canonical_entry_sequences() {
    assert_entries(
        CandidateGraphDeclaration::new("candidate-a")
            .with_graph_version("v1")
            .with_source_note("smoke")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "candidate_graph"),
            text("graph_id", "candidate-a"),
            text("graph_version", "v1"),
            text("source_note", "smoke"),
        ],
    );
    assert_entries(
        EmbeddingDeclaration::new("candidate-a", "embedding-a").canonical_declaration_entries(),
        &[
            text("declaration_kind", "embedding"),
            text("graph_id", "candidate-a"),
            text("embedding_id", "embedding-a"),
        ],
    );
    assert_entries(
        ColorabilityDeclaration::new("candidate-a", 5).canonical_declaration_entries(),
        &[
            text("declaration_kind", "colorability"),
            text("graph_id", "candidate-a"),
            unsigned("color_count", 5),
        ],
    );
    assert_entries(
        LowerBoundWitnessDeclaration::new("candidate-a", "embedding-a", 5)
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "lower_bound_witness"),
            text("graph_id", "candidate-a"),
            text("embedding_id", "embedding-a"),
            unsigned("color_count", 5),
        ],
    );
    assert_entries(
        AdvisoryNoteDeclaration::new("candidate-a", "motif").canonical_declaration_entries(),
        &[
            text("declaration_kind", "advisory_note"),
            text("graph_id", "candidate-a"),
            text("note", "motif"),
        ],
    );
    assert_entries(
        RejectionExplanationDeclaration::new("candidate-a", "bad-edge")
            .canonical_declaration_entries(),
        &[
            text("declaration_kind", "rejection_explanation"),
            text("graph_id", "candidate-a"),
            text("rejection_basis", "bad-edge"),
        ],
    );
    assert_entries(
        PartialAdmissionExplanationDeclaration::new("candidate-a").canonical_declaration_entries(),
        &[
            text("declaration_kind", "partial_admission_explanation"),
            text("graph_id", "candidate-a"),
        ],
    );
}

#[test]
fn candidate_graph_optional_identity_entries_are_absent_until_declared() {
    assert_entries(
        CandidateGraphDeclaration::new("candidate-a").canonical_declaration_entries(),
        &[
            text("declaration_kind", "candidate_graph"),
            text("graph_id", "candidate-a"),
        ],
    );
}

#[test]
fn request_shape_validation_has_typed_fallible_constructors() {
    assert_eq!(
        CandidateGraphDeclaration::try_new("  "),
        Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField { field: "graph_id" })
    );
    assert_eq!(
        CandidateGraphDeclaration::new("candidate-a").try_with_graph_version(""),
        Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField {
            field: "graph_version"
        })
    );
    assert_eq!(
        ColorabilityDeclaration::try_new("candidate-a", 0),
        Err(HadwigerResearchDeclarationShapeError::ZeroColorCount {
            field: "color_count"
        })
    );
    assert_eq!(
        LowerBoundWitnessDeclaration::try_new("candidate-a", "  ", 5),
        Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField {
            field: "embedding_id"
        })
    );
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
        WORTHQueryDeclarationCanonicalEntryKind::Field,
        WORTHQueryDeclarationCanonicalValue::UnsignedInteger(value),
    )
}
