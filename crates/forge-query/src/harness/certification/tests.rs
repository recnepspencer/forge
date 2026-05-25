use super::{
    contains_row, milestone_five_point_four_requirements, milestone_five_point_one_requirements,
    milestone_five_point_two_requirements, milestone_nine_one_requirements,
    milestone_one_requirements, milestone_three_requirements, milestone_two_requirements,
    unmet_required_assertion_classes, unmet_required_rows, CanonicalCertificationRow,
    CertificationMatrix, HostileExpectation, ParityAnchor, RejectionCertificationRow,
    RequiredAssertionClass,
};

#[test]
fn requirements_registry_exposes_milestone_rows() {
    let milestone_one = milestone_one_requirements();
    let milestone_two = milestone_two_requirements();
    let milestone_three = milestone_three_requirements();
    let milestone_five_point_one = milestone_five_point_one_requirements();
    let milestone_five_point_two = milestone_five_point_two_requirements();
    let milestone_five_point_four = milestone_five_point_four_requirements();
    let milestone_nine_one = milestone_nine_one_requirements();

    assert_eq!(
        milestone_one.suite_name,
        "Canonical Query Normalization Parity Test"
    );
    assert!(milestone_one
        .required_canonical_rows
        .contains(&"detail-query-parity"));
    assert_eq!(
        milestone_two.suite_name,
        "Schema-Aware Rejection And Projection Legality Test"
    );
    assert!(milestone_two
        .required_rejection_rows
        .contains(&"forbidden-widening-case"));
    assert!(milestone_three
        .required_canonical_rows
        .contains(&"route-semantic-difference"));
    assert_eq!(
        milestone_five_point_one.suite_name,
        "Region-Scoped Live Narrowing And Stream Contract Test"
    );
    assert!(milestone_five_point_one
        .required_rejection_rows
        .contains(&"forbidden-locality-widening"));
    assert_eq!(
        milestone_five_point_two.suite_name,
        "Preview Session Basis And Promotion Parity Test"
    );
    assert!(milestone_five_point_two
        .required_canonical_rows
        .contains(&"preview-promotion-comparison-parity"));
    assert!(milestone_five_point_two
        .required_canonical_rows
        .contains(&"preview-live-admission-parity"));
    assert!(milestone_five_point_two
        .required_canonical_rows
        .contains(&"preview-live-drift-explicitness"));
    assert!(milestone_five_point_two
        .required_rejection_rows
        .contains(&"invalid-preview-basis"));
    assert!(milestone_five_point_two
        .required_rejection_rows
        .contains(&"discarded-preview-execution-denied"));
    assert!(milestone_five_point_two
        .required_rejection_rows
        .contains(&"preview-shape-mismatch-denied"));
    assert!(milestone_five_point_two
        .required_rejection_rows
        .contains(&"read-only-preview-denies-promotion-comparison"));
    assert!(milestone_five_point_two
        .required_rejection_rows
        .contains(&"preview-broad-fallback-forbidden"));
    assert!(milestone_five_point_two
        .required_rejection_rows
        .contains(&"preview-live-drift-denied"));
    assert!(milestone_five_point_two
        .required_rejection_rows
        .contains(&"preview-live-broad-fallback-forbidden"));
    assert!(milestone_five_point_two
        .required_assertion_classes
        .contains(&RequiredAssertionClass::Inequality));
    assert_eq!(
        milestone_five_point_four.suite_name,
        "Structural Correspondence And Historical Materialization Path Test"
    );
    assert!(milestone_five_point_four
        .required_canonical_rows
        .contains(&"lineage-correspondence-authoritative"));
    assert!(milestone_five_point_four
        .required_canonical_rows
        .contains(&"prediction-drift-explicit"));
    assert!(milestone_five_point_four
        .required_rejection_rows
        .contains(&"hidden-materialization-path-substitution-forbidden"));
    assert!(milestone_five_point_four
        .required_rejection_rows
        .contains(&"broad-candidate-scan-success-forbidden"));
    assert_eq!(
        milestone_nine_one.suite_name,
        "Query Subscription Declaration And Lowering Parity Test"
    );
    assert!(milestone_nine_one
        .required_canonical_rows
        .contains(&"detail-direct-scope-template-saved-facade-parity"));
    assert!(milestone_nine_one
        .required_canonical_rows
        .contains(&"scale-slope-row-count-only-honesty"));
    assert!(milestone_nine_one
        .required_rejection_rows
        .contains(&"durable-reload-overclaim-denies-before-activation"));
    assert!(milestone_nine_one
        .required_rejection_rows
        .contains(&"scale-report-source-mismatch-denies-certification"));
    assert!(milestone_nine_one
        .required_assertion_classes
        .contains(&RequiredAssertionClass::ZeroResidue));
}

#[test]
fn shared_certification_core_reports_unmet_rows() {
    let matrix = CertificationMatrix {
        suite_name: "test",
        rows: vec![CanonicalCertificationRow {
            row_name: "control-row",
            perturbation_class: 0_u8,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: "control",
            hostile_lane: "hostile",
            parity_lane: "parity",
        }],
        rejection_rows: vec![RejectionCertificationRow {
            row_name: "rejection-row",
            perturbation_class: 1_u8,
            control_lane: "control",
            hostile_lane: "rejection",
            parity_lane: "parity",
        }],
    };

    assert!(contains_row(&matrix, "control-row"));
    assert!(contains_row(&matrix, "rejection-row"));
    assert_eq!(
        unmet_required_rows(
            &matrix,
            &["control-row", "missing-canonical"],
            &["rejection-row", "missing-rejection"]
        ),
        vec!["missing-canonical", "missing-rejection"]
    );
    assert_eq!(
        unmet_required_assertion_classes(
            &[
                RequiredAssertionClass::Equality,
                RequiredAssertionClass::TypedFailure
            ],
            &[
                RequiredAssertionClass::Equality,
                RequiredAssertionClass::Inequality,
                RequiredAssertionClass::TypedFailure,
            ]
        ),
        vec![RequiredAssertionClass::Inequality]
    );
}
