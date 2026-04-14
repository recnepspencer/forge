use crate::harness::fixtures::schema_view::{
    alternate_detail_schema_view, detail_schema_view, legal_collection_bundle, legal_detail_bundle,
};
use crate::identity::CanonicalEquivalence;
use crate::validation::{validate_canonical_bundle, QueryValidationError, ValidationEvent};

#[test]
fn legal_detail_bundle_validates_deterministically() {
    let left = validate_canonical_bundle(legal_detail_bundle(), detail_schema_view())
        .expect("legal detail bundle should validate");
    let right = validate_canonical_bundle(legal_detail_bundle(), detail_schema_view())
        .expect("legal detail bundle should validate");

    assert_eq!(left.query().schema_basis(), right.query().schema_basis());
    assert_eq!(left.query().digest(), right.query().digest());
    assert_eq!(left.result_shape().digest(), right.result_shape().digest());
    assert_eq!(left.equivalence_to(&right), CanonicalEquivalence::Equivalent);
    assert_eq!(left.counters().validated_projection_entry_count(), 2);
    assert_eq!(left.counters().validated_traversal_clause_count(), 1);
    assert_eq!(left.counters().validated_result_shape_binding_count(), 2);
    assert_eq!(left.counters().schema_lookup_count(), 5);
    assert_eq!(left.counters().validation_rejection_count(), 0);
    assert_eq!(left.counters().projection_widening_denial_count(), 0);
    assert_eq!(left.report().rejection_matrix().projection_rejections(), 0);
    assert_eq!(left.report().rejection_matrix().traversal_rejections(), 0);
    assert_eq!(left.report().rejection_matrix().result_shape_rejections(), 0);
    assert_eq!(left.report().rejection_matrix().compatibility_rejections(), 0);
    assert!(left
        .report()
        .events()
        .iter()
        .any(|event| matches!(event, ValidationEvent::CompatibilityEstablished)));
    assert!(left
        .report()
        .events()
        .iter()
        .any(|event| matches!(event, ValidationEvent::IdentityFrozen { .. })));
}

#[test]
fn collection_bundle_validates_with_distinct_identity() {
    let detail = validate_canonical_bundle(legal_detail_bundle(), detail_schema_view())
        .expect("detail bundle should validate");
    let collection = validate_canonical_bundle(legal_collection_bundle(), detail_schema_view())
        .expect("collection bundle should validate");

    assert_ne!(detail.query().digest(), collection.query().digest());
}

#[test]
fn alternate_schema_basis_changes_validated_identity() {
    let left = validate_canonical_bundle(legal_detail_bundle(), detail_schema_view())
        .expect("detail bundle should validate");
    let right = validate_canonical_bundle(legal_detail_bundle(), alternate_detail_schema_view())
        .expect("detail bundle should validate");

    assert_ne!(left.query().schema_basis(), right.query().schema_basis());
    assert_ne!(left.query().digest(), right.query().digest());
    assert_ne!(left.result_shape().digest(), right.result_shape().digest());
}

#[test]
fn validation_invariants_catch_duplicate_identity_freeze() {
    let mut validated = validate_canonical_bundle(legal_detail_bundle(), detail_schema_view())
        .expect("detail bundle should validate");
    let query_digest = validated.query().digest().as_str().to_string();
    let result_shape_digest = validated.result_shape().digest().as_str().to_string();

    validated
        .report_mut_for_test()
        .events_mut_for_test()
        .push(ValidationEvent::IdentityFrozen {
            query_digest,
            result_shape_digest,
        });

    let error = validated
        .check_invariants()
        .expect_err("duplicate identity freeze should fail invariants");
    assert_eq!(
        error,
        QueryValidationError::ValidationInvariantViolation {
            message: "validated identity must be frozen exactly once",
        }
    );
}
