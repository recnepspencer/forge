use crate::authoring::{
    AspectFieldSelector, AuthoredQueryBundleRequest, AuthoredResultShapeField, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey, TraversalSelector,
};
use crate::authoring::{AuthoredBundleError, AuthoredBundleFailureClass};
use crate::authoring::{AuthoringError, AuthoringFailureClass};
use crate::binding::QueryBindingDescriptor;

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
        AuthoredBundleFailureClass::ProjectionShapeMismatch
    );
    assert!(matches!(
        error,
        AuthoredBundleError::UnprojectedShapeField { .. }
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
        AuthoredBundleFailureClass::FamilyMismatch
    );
    assert!(matches!(
        error,
        AuthoredBundleError::QueryShapeFamilyMismatch { .. }
    ));
}

#[test]
fn empty_root_is_rejected_at_authoring_boundary() {
    let error = RootEntityKey::new("").unwrap_err();
    assert_eq!(error.failure_class(), AuthoringFailureClass::InvalidAtom);
    assert!(matches!(error, AuthoringError::EmptyRootEntityKey));
}

#[test]
fn zero_depth_traversal_is_rejected_at_authoring_boundary() {
    let error = TraversalSelector::bounded("owner", 0).unwrap_err();
    assert_eq!(error.failure_class(), AuthoringFailureClass::InvalidAtom);
    assert!(matches!(
        error,
        AuthoringError::UnsupportedTraversalDepth { .. }
    ));
}

#[test]
fn empty_delivered_field_name_is_rejected_at_authoring_boundary() {
    let error = AuthoredResultShapeField::new("title", "text", "").unwrap_err();
    assert_eq!(error.failure_class(), AuthoringFailureClass::InvalidAtom);
    assert!(matches!(error, AuthoringError::EmptyDeliveredFieldName));
}

#[test]
fn empty_projection_selector_is_rejected_at_authoring_boundary() {
    let error = AspectFieldSelector::new("", "text").unwrap_err();
    assert_eq!(error.failure_class(), AuthoringFailureClass::InvalidAtom);
    assert!(matches!(error, AuthoringError::EmptyProjectionSelector));
}

#[test]
fn empty_projection_set_is_rejected_at_authoring_boundary() {
    let error = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .build()
        .unwrap_err();
    assert_eq!(
        error.failure_class(),
        AuthoringFailureClass::InvalidAssembly
    );
    assert!(matches!(error, AuthoringError::EmptyProjectionSet));
}

#[test]
fn empty_result_shape_field_set_is_rejected_at_authoring_boundary() {
    let error = RawAuthoredResultShape::detail_builder()
        .build()
        .unwrap_err();
    assert_eq!(
        error.failure_class(),
        AuthoringFailureClass::InvalidAssembly
    );
    assert!(matches!(error, AuthoringError::EmptyResultShapeFieldSet));
}
