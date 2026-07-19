use super::{
    contains_row, unmet_required_assertion_classes, unmet_required_rows, CanonicalCertificationRow,
    CertificationMatrix, HostileExpectation, ParityAnchor, RejectionCertificationRow,
    RequiredAssertionClass,
};

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
