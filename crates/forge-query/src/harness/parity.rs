use crate::authoring::{RawAuthoredQuery, RawAuthoredResultShape};
use crate::facade::{
    AspectFieldSelector, AuthoredResultShapeField, IdentityBindingDescriptor,
    NonIdentityBindingMetadata, QueryBindingDescriptor, QueryBindingSlot, QueryBindingSubject,
    RootEntityKey, TraversalSelector,
};

#[test]
fn equivalent_detail_queries_canonicalize_to_identical_query_digests() {
    let query_a = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .build()
        .unwrap();
    let query_b = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .build()
        .unwrap();

    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .build()
        .unwrap();

    let request_a =
        crate::facade::GuidedAuthoringPath::pair_detail(query_a, shape.clone()).unwrap();
    let bundle_a = crate::facade::canonicalize_request(request_a).unwrap();
    let request_b = crate::facade::GuidedAuthoringPath::pair_detail(query_b, shape).unwrap();
    let bundle_b = crate::facade::canonicalize_request(request_b).unwrap();

    assert_eq!(bundle_a.query().digest(), bundle_b.query().digest());
    assert_eq!(
        bundle_a.result_shape().digest(),
        bundle_b.result_shape().digest()
    );
    assert_eq!(
        bundle_a.query().equivalence_to(bundle_b.query()),
        crate::facade::CanonicalEquivalence::Equivalent
    );
    assert_eq!(
        bundle_a
            .result_shape()
            .equivalence_to(bundle_b.result_shape()),
        crate::facade::CanonicalEquivalence::Equivalent
    );
    assert_eq!(
        bundle_a.equivalence_to(&bundle_b),
        crate::facade::CanonicalEquivalence::Equivalent
    );
    assert_eq!(bundle_a.counters().raw_clause_count, 3);
    assert_eq!(bundle_a.counters().normalized_clause_count, 3);
    assert_eq!(bundle_a.counters().projection_entry_count, 2);
    assert_eq!(bundle_a.counters().traversal_clause_count, 1);
    assert_eq!(bundle_a.counters().result_shape_field_count, 2);
    assert_eq!(bundle_a.counters().canonicalization_warning_count, 0);
    assert_eq!(bundle_a.counters().canonicalization_fallback_count, 0);
    assert_eq!(
        bundle_a.report().compatibility(),
        &crate::facade::CompatibilityEvidence::Compatible
    );
    assert_eq!(bundle_a.report().normalized_projection_entries(), 2);
    assert_eq!(bundle_a.report().normalized_traversal_entries(), 1);
    assert_eq!(bundle_a.report().normalized_result_fields(), 2);
    assert_eq!(
        bundle_a.report().identity_freeze().query_digest,
        bundle_a.query().digest().as_str()
    );
    assert_eq!(
        bundle_a.report().identity_freeze().result_shape_digest,
        bundle_a.result_shape().digest().as_str()
    );
    assert!(bundle_a.report().events().iter().any(|event| matches!(
        event,
        crate::facade::NormalizationEvent::CompatibilityEstablished
    )));
    assert!(bundle_a.report().events().iter().any(|event| matches!(
        event,
        crate::facade::NormalizationEvent::IdentityFrozen { .. }
    )));
    bundle_a.check_invariants().unwrap();
    bundle_b.check_invariants().unwrap();
}

#[test]
fn repeated_guided_canonicalization_is_deterministic() {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request_a =
        crate::facade::GuidedAuthoringPath::pair_collection(query.clone(), shape.clone()).unwrap();
    let request_b = crate::facade::GuidedAuthoringPath::pair_collection(query, shape).unwrap();

    let bundle_a = crate::facade::canonicalize_request(request_a).unwrap();
    let bundle_b = crate::facade::canonicalize_request(request_b).unwrap();

    assert_eq!(bundle_a.query().digest(), bundle_b.query().digest());
    assert_eq!(
        bundle_a.result_shape().digest(),
        bundle_b.result_shape().digest()
    );
    assert_eq!(bundle_a.report().events(), bundle_b.report().events());
    assert_eq!(
        bundle_a.equivalence_to(&bundle_b),
        crate::facade::CanonicalEquivalence::Equivalent
    );
    bundle_a.check_invariants().unwrap();
    bundle_b.check_invariants().unwrap();
}

#[test]
fn event_order_is_deterministic_under_metadata_and_traversal_noise() {
    let query_a = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let query_b = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .project(AspectFieldSelector::new("status", "kind").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .build()
        .unwrap();
    let shape_a = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .build()
        .unwrap();
    let shape_b = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let bindings_a = QueryBindingDescriptor::new()
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap())
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ));
    let bindings_b = QueryBindingDescriptor::new()
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ))
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap());

    let request_a = crate::facade::GuidedAuthoringPath::pair_collection_with_bindings(
        query_a, shape_a, bindings_a,
    )
    .unwrap();
    let request_b = crate::facade::GuidedAuthoringPath::pair_collection_with_bindings(
        query_b, shape_b, bindings_b,
    )
    .unwrap();

    let bundle_a = crate::facade::canonicalize_request(request_a).unwrap();
    let bundle_b = crate::facade::canonicalize_request(request_b).unwrap();

    assert_eq!(bundle_a.report().events(), bundle_b.report().events());
    bundle_a.check_invariants().unwrap();
    bundle_b.check_invariants().unwrap();
}

#[test]
fn convenience_detail_canonicalization_uses_hardened_path() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .traverse(TraversalSelector::bounded("owner", 1).unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let bundle = crate::facade::GuidedAuthoringPath::canonicalize_detail(query, shape).unwrap();

    assert_eq!(bundle.query().projection().len(), 1);
    assert_eq!(bundle.query().traversal().len(), 1);
    assert_eq!(bundle.counters().canonicalization_fallback_count, 0);
    bundle.check_invariants().unwrap();
}

#[test]
fn convenience_collection_canonicalization_with_bindings_preserves_metadata_rules() {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();
    let bindings = QueryBindingDescriptor::new()
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap())
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ));

    let bundle = crate::facade::GuidedAuthoringPath::canonicalize_collection_with_bindings(
        query, shape, bindings,
    )
    .unwrap();

    assert!(bundle.report().events().iter().any(|event| matches!(
        event,
        crate::facade::NormalizationEvent::NonIdentityBindingIgnored { .. }
    )));
    assert_eq!(bundle.counters().canonicalization_fallback_count, 0);
    bundle.check_invariants().unwrap();
}
