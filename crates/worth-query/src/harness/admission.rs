use crate::authoring::{AuthoredQueryBundleRequest, RawAuthoredQuery, RawAuthoredResultShape};
use crate::facade::foundation::{
    AspectFieldSelector, AuthoredResultShapeField, QueryBindingDescriptor, RootEntityKey,
    TraversalSelector,
};

#[test]
fn unprojected_shape_field_fails_compatibility() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("status", "kind", "status").unwrap())
        .build()
        .unwrap();

    let error = AuthoredQueryBundleRequest::new(
        query.clone().into_raw(),
        shape.clone().into_raw(),
        QueryBindingDescriptor::default(),
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::AuthoredBundleFailureClass::ProjectionShapeMismatch
    );
    assert!(matches!(
        error,
        crate::facade::foundation::AuthoredBundleError::UnprojectedShapeField { .. }
    ));
}

#[test]
fn family_mismatch_fails_explicitly() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::collection_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let error = AuthoredQueryBundleRequest::new(
        query.clone().into_raw(),
        shape.clone().into_raw(),
        QueryBindingDescriptor::default(),
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::AuthoredBundleFailureClass::FamilyMismatch
    );
    assert!(matches!(
        error,
        crate::facade::foundation::AuthoredBundleError::QueryShapeFamilyMismatch { .. }
    ));
}

#[test]
fn empty_root_is_rejected_at_authoring_boundary() {
    let error = RootEntityKey::new("").unwrap_err();
    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::AuthoringFailureClass::InvalidAtom
    );
    assert!(matches!(
        error,
        crate::facade::foundation::AuthoringError::EmptyRootEntityKey
    ));
}

#[test]
fn zero_depth_traversal_is_rejected_at_authoring_boundary() {
    let error = TraversalSelector::bounded("owner", 0).unwrap_err();
    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::AuthoringFailureClass::InvalidAtom
    );
    assert!(matches!(
        error,
        crate::facade::foundation::AuthoringError::UnsupportedTraversalDepth { .. }
    ));
}

#[test]
fn empty_delivered_field_name_is_rejected_at_authoring_boundary() {
    let error = AuthoredResultShapeField::new("title", "text", "").unwrap_err();
    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::AuthoringFailureClass::InvalidAtom
    );
    assert!(matches!(
        error,
        crate::facade::foundation::AuthoringError::EmptyDeliveredFieldName
    ));
}

#[test]
fn empty_projection_selector_is_rejected_at_authoring_boundary() {
    let error = AspectFieldSelector::new("", "text").unwrap_err();
    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::AuthoringFailureClass::InvalidAtom
    );
    assert!(matches!(
        error,
        crate::facade::foundation::AuthoringError::EmptyProjectionSelector
    ));
}

#[test]
fn empty_projection_set_is_rejected_at_authoring_boundary() {
    let error = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .build()
        .unwrap_err();
    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::AuthoringFailureClass::InvalidAssembly
    );
    assert!(matches!(
        error,
        crate::facade::foundation::AuthoringError::EmptyProjectionSet
    ));
}

#[test]
fn empty_result_shape_field_set_is_rejected_at_authoring_boundary() {
    let error = RawAuthoredResultShape::detail_builder()
        .build()
        .unwrap_err();
    assert_eq!(
        error.failure_class(),
        crate::facade::foundation::AuthoringFailureClass::InvalidAssembly
    );
    assert!(matches!(
        error,
        crate::facade::foundation::AuthoringError::EmptyResultShapeFieldSet
    ));
}

#[test]
fn unsupported_authored_query_family_is_rejected_explicitly() {
    let query =
        RawAuthoredQuery::unsupported_for_test(RootEntityKey::new("task").unwrap(), "grouped")
            .with_projection(AspectFieldSelector::new("title", "text").unwrap());
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let error = crate::canonicalization::pipeline::QueryCanonicalizer::canonicalize_bundle(
        query,
        shape.into_raw(),
        QueryBindingDescriptor::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::facade::foundation::QueryCanonicalizationError::UnsupportedAuthoredQueryFamily {
            family: "grouped"
        }
    ));
}

#[test]
fn unsupported_authored_result_shape_family_is_rejected_explicitly() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::unsupported_for_test("inspector")
        .with_field(AuthoredResultShapeField::new("title", "text", "title").unwrap());

    let error = crate::canonicalization::pipeline::QueryCanonicalizer::canonicalize_bundle(
        query.into_raw(),
        shape,
        QueryBindingDescriptor::default(),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        crate::facade::foundation::QueryCanonicalizationError::UnsupportedAuthoredResultShapeFamily {
            family: "inspector"
        }
    ));
}

#[test]
fn non_canonical_helper_residue_is_rejected_during_bundle_assembly() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "text").unwrap())
        .build()
        .unwrap();
    let shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "text", "title").unwrap())
        .build()
        .unwrap();

    let request = AuthoredQueryBundleRequest::new(
        query.into_raw(),
        shape.into_raw(),
        QueryBindingDescriptor::default(),
    )
    .unwrap()
    .with_helper_residue_for_test("builder_history");

    let error = crate::facade::foundation::canonicalize_request(request).unwrap_err();
    assert!(matches!(
        error,
        crate::facade::foundation::QueryCanonicalizationError::NonCanonicalHelperResidueDetected {
            residue: "builder_history"
        }
    ));
}
