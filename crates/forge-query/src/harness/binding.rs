use crate::authoring::{RawAuthoredQuery, RawAuthoredResultShape};
use crate::facade::{
    derive_binding_requirements, resolve_bindings, AspectFieldSelector, AuthoredResultShapeField,
    BindingFailureClass, BindingResolutionError, BoundBinding, BoundBindings,
    IdentityBindingDescriptor, NonIdentityBindingMetadata, QueryBindingDescriptor,
    QueryBindingSlot, QueryBindingSubject, RootEntityKey,
};

#[test]
fn equivalent_binding_order_does_not_change_query_digest() {
    let query = RawAuthoredQuery::collection_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let bindings_a = QueryBindingDescriptor::new()
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ))
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap());

    let bindings_b = QueryBindingDescriptor::new()
        .with_non_identity(NonIdentityBindingMetadata::new("route", "tasks.index").unwrap())
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ));

    let request_a = crate::facade::GuidedAuthoringPath::pair_collection_with_bindings(
        query.clone(),
        shape.clone(),
        bindings_a,
    )
    .unwrap();
    let request_b =
        crate::facade::GuidedAuthoringPath::pair_collection_with_bindings(query, shape, bindings_b)
            .unwrap();

    let bundle_a = crate::facade::canonicalize_request(request_a).unwrap();
    let bundle_b = crate::facade::canonicalize_request(request_b).unwrap();

    assert_eq!(bundle_a.query().digest(), bundle_b.query().digest());
    assert_eq!(
        bundle_a.equivalence_to(&bundle_b),
        crate::facade::CanonicalEquivalence::Equivalent
    );
    assert_eq!(bundle_a.counters().binding_descriptor_count, 2);
    assert_eq!(bundle_b.counters().binding_descriptor_count, 2);
    assert_eq!(bundle_a.counters().canonicalization_warning_count, 1);
    assert_eq!(bundle_b.counters().canonicalization_warning_count, 1);
    assert_eq!(bundle_a.counters().canonicalization_fallback_count, 0);
    assert_eq!(bundle_b.counters().canonicalization_fallback_count, 0);
    assert!(bundle_a.report().events().iter().any(|event| matches!(
        event,
        crate::facade::NormalizationEvent::NonIdentityBindingIgnored { .. }
    )));
    bundle_a.check_invariants().unwrap();
    bundle_b.check_invariants().unwrap();
}

#[test]
fn forbidden_binding_metadata_is_rejected_at_binding_boundary() {
    let error = NonIdentityBindingMetadata::new("policy", "internal").unwrap_err();
    assert_eq!(
        error.failure_class(),
        BindingFailureClass::ForbiddenMetadata
    );
    assert!(matches!(
        error,
        crate::facade::BindingError::ForbiddenMetadataKey { .. }
    ));
}

#[test]
fn unsupported_binding_metadata_is_rejected_at_binding_boundary() {
    let error = NonIdentityBindingMetadata::new("future_magic", "value").unwrap_err();
    assert_eq!(
        error.failure_class(),
        BindingFailureClass::UnsupportedMetadata
    );
    assert!(matches!(
        error,
        crate::facade::BindingError::UnsupportedMetadataKey { .. }
    ));
}

#[test]
fn duplicate_binding_descriptor_same_subject_deduplicates() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();
    let bindings = QueryBindingDescriptor::new()
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ))
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ));

    let request =
        crate::facade::GuidedAuthoringPath::pair_detail_with_bindings(query, shape, bindings)
            .unwrap();
    let bundle = crate::facade::canonicalize_request(request).unwrap();

    assert_eq!(bundle.query().identity_bindings().len(), 1);
    assert_eq!(bundle.counters().binding_descriptor_count, 2);
    assert_eq!(bundle.counters().canonicalization_fallback_count, 0);
    assert!(bundle.report().events().iter().any(|event| matches!(
        event,
        crate::facade::NormalizationEvent::IdentityBindingCollapsedDuplicate { .. }
    )));
}

#[test]
fn conflicting_binding_descriptor_subject_fails_explicitly() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();
    let bindings = QueryBindingDescriptor::new()
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
        ))
        .with_identity(IdentityBindingDescriptor::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::TraversalRoot,
        ));

    let error = crate::canonicalization::pipeline::QueryCanonicalizer::canonicalize_bundle(
        query.clone().into_raw(),
        shape.clone().into_raw(),
        bindings,
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        crate::facade::CanonicalizationFailureClass::BindingRejection
    );
    assert!(matches!(
        error,
        crate::facade::QueryCanonicalizationError::DuplicateBindingDescriptorConflict { .. }
    ));
}

#[test]
fn validated_bundle_derives_binding_requirements() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    let bindings = QueryBindingDescriptor::new().with_identity(IdentityBindingDescriptor::new(
        QueryBindingSlot::new("root").unwrap(),
        QueryBindingSubject::RootEntity,
    ));

    let request =
        crate::facade::GuidedAuthoringPath::pair_detail_with_bindings(query, shape, bindings)
            .unwrap();
    let canonical = crate::facade::canonicalize_request(request).unwrap();
    let schema = crate::harness::fixtures::schema_view::detail_schema_view();
    let validated = crate::facade::validate_canonical_bundle(canonical, schema).unwrap();

    let requirements = derive_binding_requirements(&validated);
    assert_eq!(requirements.requirements().len(), 1);
    assert_eq!(requirements.requirements()[0].slot().as_str(), "root");
    assert_eq!(
        requirements.requirements()[0].subject(),
        &QueryBindingSubject::RootEntity
    );
    assert!(requirements.requirements()[0].identity_bearing());
}

#[test]
fn binding_resolution_requires_exact_slot_match() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    let bindings = QueryBindingDescriptor::new().with_identity(IdentityBindingDescriptor::new(
        QueryBindingSlot::new("root").unwrap(),
        QueryBindingSubject::RootEntity,
    ));

    let request =
        crate::facade::GuidedAuthoringPath::pair_detail_with_bindings(query, shape, bindings)
            .unwrap();
    let canonical = crate::facade::canonicalize_request(request).unwrap();
    let schema = crate::harness::fixtures::schema_view::detail_schema_view();
    let validated = crate::facade::validate_canonical_bundle(canonical, schema).unwrap();
    let requirements = derive_binding_requirements(&validated);

    let ok = resolve_bindings(
        requirements.clone(),
        BoundBindings::new(vec![BoundBinding::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::RootEntity,
            "task-1",
        )]),
    )
    .unwrap();
    assert_eq!(ok.bindings().bindings().len(), 1);

    let missing = resolve_bindings(requirements.clone(), BoundBindings::new(vec![])).unwrap_err();
    assert!(matches!(
        missing,
        BindingResolutionError::MissingBindingSlot { .. }
    ));

    let extra = resolve_bindings(
        requirements.clone(),
        BoundBindings::new(vec![
            BoundBinding::new(
                QueryBindingSlot::new("root").unwrap(),
                QueryBindingSubject::RootEntity,
                "task-1",
            ),
            BoundBinding::new(
                QueryBindingSlot::new("extra").unwrap(),
                QueryBindingSubject::RootEntity,
                "task-2",
            ),
        ]),
    )
    .unwrap_err();
    assert!(matches!(extra, BindingResolutionError::ExtraBindingSlot { .. }));

    let wrong_subject = resolve_bindings(
        requirements,
        BoundBindings::new(vec![BoundBinding::new(
            QueryBindingSlot::new("root").unwrap(),
            QueryBindingSubject::TraversalRoot,
            "task-1",
        )]),
    )
    .unwrap_err();
    assert!(matches!(
        wrong_subject,
        BindingResolutionError::ConflictingBindingSubjects { .. }
    ));
}
