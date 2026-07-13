use crate::authoring::{RawAuthoredQuery, RawAuthoredResultShape};
use crate::canonicalization::pipeline::QueryCanonicalizer;
use crate::facade::foundation::{AspectFieldSelector, AuthoredResultShapeField, RootEntityKey};

#[test]
fn conflicting_alias_identity_fails_explicitly() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "label").unwrap())
        .field(AuthoredResultShapeField::new("status", "kind", "label").unwrap())
        .build()
        .unwrap();

    let error = QueryCanonicalizer::canonicalize_bundle(
        query.clone().into_raw(),
        shape.clone().into_raw(),
        crate::facade::foundation::QueryBindingDescriptor::default(),
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::CanonicalizationFailureClass::CompatibilityRejection
    );
    assert!(matches!(
        error,
        crate::facade::foundation::QueryCanonicalizationError::AmbiguousShapeAliasIdentity { .. }
    ));
}

#[test]
fn semantically_distinct_queries_are_not_equivalent() {
    let query_a = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let query_b = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .build()
        .unwrap();
    let shape_a = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();
    let shape_b = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .build()
        .unwrap();

    let bundle_a = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_detail(query_a, shape_a).unwrap(),
    )
    .unwrap();
    let bundle_b = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_detail(query_b, shape_b).unwrap(),
    )
    .unwrap();

    assert_eq!(
        bundle_a.query().equivalence_to(bundle_b.query()),
        crate::facade::foundation::CanonicalEquivalence::Distinct
    );
    assert_eq!(
        bundle_a
            .result_shape()
            .equivalence_to(bundle_b.result_shape()),
        crate::facade::foundation::CanonicalEquivalence::Distinct
    );
    assert_eq!(
        bundle_a.equivalence_to(&bundle_b),
        crate::facade::foundation::CanonicalEquivalence::Distinct
    );
}

#[test]
fn result_shape_omission_changes_shape_identity_but_not_query_identity() {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .build()
        .unwrap();
    let full_shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .build()
        .unwrap();
    let omitted_shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let full_bundle = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_collection(query.clone(), full_shape)
            .unwrap(),
    )
    .unwrap();
    let omitted_bundle = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_collection(query, omitted_shape)
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        full_bundle.query().digest(),
        omitted_bundle.query().digest()
    );
    assert_eq!(
        full_bundle.query().equivalence_to(omitted_bundle.query()),
        crate::facade::foundation::CanonicalEquivalence::Equivalent
    );
    assert_ne!(
        full_bundle.result_shape().digest(),
        omitted_bundle.result_shape().digest()
    );
    assert_eq!(
        full_bundle
            .result_shape()
            .equivalence_to(omitted_bundle.result_shape()),
        crate::facade::foundation::CanonicalEquivalence::Distinct
    );
    assert_eq!(
        full_bundle.equivalence_to(&omitted_bundle),
        crate::facade::foundation::CanonicalEquivalence::Distinct
    );
}

#[test]
fn alias_identity_changes_result_shape_digest_without_changing_query_digest() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let title_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();
    let label_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "label").unwrap())
        .build()
        .unwrap();

    let title_bundle = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_detail(query.clone(), title_shape)
            .unwrap(),
    )
    .unwrap();
    let label_bundle = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_detail(query, label_shape).unwrap(),
    )
    .unwrap();

    assert_eq!(title_bundle.query().digest(), label_bundle.query().digest());
    assert_eq!(
        title_bundle.query().equivalence_to(label_bundle.query()),
        crate::facade::foundation::CanonicalEquivalence::Equivalent
    );
    assert_ne!(
        title_bundle.result_shape().digest(),
        label_bundle.result_shape().digest()
    );
    assert_eq!(
        title_bundle
            .result_shape()
            .equivalence_to(label_bundle.result_shape()),
        crate::facade::foundation::CanonicalEquivalence::Distinct
    );
}

#[test]
fn invalid_canonical_ordering_basis_is_detected_explicitly() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("a", "alpha").unwrap())
        .project(AspectFieldSelector::new("b", "beta").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("a", "alpha", "alpha").unwrap())
        .field(AuthoredResultShapeField::new("b", "beta", "beta").unwrap())
        .build()
        .unwrap();

    let mut bundle = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_detail(query, shape).unwrap(),
    )
    .unwrap();
    bundle.query_mut_for_test().reverse_projection_for_test();

    let error = crate::canonicalization::pipeline::validate_canonical_ordering_for_test(
        bundle.query(),
        bundle.result_shape(),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::facade::foundation::QueryCanonicalizationError::InvalidCanonicalOrderingBasis {
            artifact: "query_projection"
        }
    ));
}

#[test]
fn digest_basis_inconsistency_is_detected_explicitly() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let mut bundle = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_detail(query, shape).unwrap(),
    )
    .unwrap();
    bundle
        .query_mut_for_test()
        .corrupt_digest_for_test("query_corrupt");

    let error = crate::canonicalization::pipeline::validate_digest_basis_consistency_for_test(
        bundle.query(),
        bundle.result_shape(),
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::facade::foundation::QueryCanonicalizationError::DigestBasisInconsistency {
            artifact: "query"
        }
    ));
}

#[test]
fn result_shape_ordering_and_digest_inconsistency_are_detected_explicitly() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("a", "alpha").unwrap())
        .project(AspectFieldSelector::new("b", "beta").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("a", "alpha", "alpha").unwrap())
        .field(AuthoredResultShapeField::new("b", "beta", "beta").unwrap())
        .build()
        .unwrap();

    let mut bundle = crate::facade::foundation::canonicalize_request(
        crate::facade::foundation::GuidedAuthoringPath::pair_detail(query, shape).unwrap(),
    )
    .unwrap();
    bundle.result_shape_mut_for_test().reverse_fields_for_test();

    let ordering_error = crate::canonicalization::pipeline::validate_canonical_ordering_for_test(
        bundle.query(),
        bundle.result_shape(),
    )
    .unwrap_err();
    assert!(matches!(
        ordering_error,
        crate::facade::foundation::QueryCanonicalizationError::InvalidCanonicalOrderingBasis {
            artifact: "result_shape_fields"
        }
    ));

    bundle
        .result_shape_mut_for_test()
        .corrupt_digest_for_test("result_shape_corrupt");
    let digest_error =
        crate::canonicalization::pipeline::validate_digest_basis_consistency_for_test(
            bundle.query(),
            bundle.result_shape(),
        )
        .unwrap_err();
    assert!(matches!(
        digest_error,
        crate::facade::foundation::QueryCanonicalizationError::DigestBasisInconsistency {
            artifact: "result_shape"
        }
    ));
}
