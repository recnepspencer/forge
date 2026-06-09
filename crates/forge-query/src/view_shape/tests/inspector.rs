use super::*;
use crate::identity_evolution::InspectorIdentityClassification;
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeComplexityStatus, ViewShapeDescriptor, ViewShapeFailureClass,
    ViewShapeInvalidationPosture, ViewShapePatchPosture,
};

#[test]
fn inspector_denies_collection_queries() {
    let error = admit_view_shape(
        &direct_collection(),
        ViewShapeDescriptor::inspector_detail_observed(),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::IncompatibleCanonicalFamily
    );
}

#[test]
fn focused_inspector_requires_focus_contract() {
    let error = admit_view_shape(
        &direct_detail(),
        ViewShapeDescriptor::inspector_detail_focused_missing_for_test(),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::FocusAspectRequired
    );
}

#[test]
fn observed_and_focused_inspector_produce_distinct_plan_metadata() {
    let canonical = direct_detail();
    let observed = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(&canonical, ViewShapeDescriptor::inspector_detail_observed()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let focused = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(
                &canonical,
                ViewShapeDescriptor::inspector_detail_focused("profile"),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    assert_ne!(observed.view_shape_digest(), focused.view_shape_digest());
    assert_ne!(
        observed.view_plan_digest().as_str(),
        focused.view_plan_digest().as_str()
    );
    assert_eq!(
        observed.invalidation_posture(),
        &ViewShapeInvalidationPosture::InspectorObservedNarrowDetail
    );
    assert_eq!(
        focused.invalidation_posture(),
        &ViewShapeInvalidationPosture::InspectorFocusedAspect
    );
    assert_eq!(
        observed.patch_posture(),
        &ViewShapePatchPosture::ObservedInspectorPatch
    );
    assert_eq!(
        observed.complexity().status(),
        ViewShapeComplexityStatus::Verified
    );
    assert_eq!(
        focused.patch_posture(),
        &ViewShapePatchPosture::FocusedInspectorAspectPatch
    );
    assert_eq!(
        focused.complexity().status(),
        ViewShapeComplexityStatus::Verified
    );
}

#[test]
fn identity_aware_focused_inspector_mints_distinct_digest_and_binding() {
    let canonical = direct_detail();
    let ordinary = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(
                &canonical,
                ViewShapeDescriptor::inspector_detail_focused("profile"),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let identity_aware = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(
                &canonical,
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    "profile",
                    InspectorIdentityClassification::AuthoritativeContinuity,
                ),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    assert_ne!(
        ordinary.view_shape_digest(),
        identity_aware.view_shape_digest()
    );
    assert_ne!(
        ordinary.delivery_metadata().identity_consumption().digest(),
        identity_aware
            .delivery_metadata()
            .identity_consumption()
            .digest()
    );
    assert_eq!(
        identity_aware
            .delivery_metadata()
            .identity_consumption()
            .classification(),
        Some(InspectorIdentityClassification::AuthoritativeContinuity)
    );
    assert_eq!(
        identity_aware.complexity().status(),
        ViewShapeComplexityStatus::Verified
    );
}

#[test]
fn identity_aware_observed_inspector_mints_summary_consumption_without_focus() {
    let canonical = direct_detail();
    let ordinary = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(&canonical, ViewShapeDescriptor::inspector_detail_observed()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let identity_aware = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(
                &canonical,
                ViewShapeDescriptor::identity_aware_inspector_detail_observed(),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    assert_ne!(
        ordinary.view_shape_digest(),
        identity_aware.view_shape_digest()
    );
    assert_ne!(
        ordinary.delivery_metadata().identity_consumption().digest(),
        identity_aware
            .delivery_metadata()
            .identity_consumption()
            .digest()
    );
    assert_eq!(
        identity_aware.delivery_metadata().focus_aspect(),
        None,
        "observed inspector summary should not silently become focused delivery"
    );
    assert_eq!(
        identity_aware.complexity().status(),
        ViewShapeComplexityStatus::Verified
    );
}

#[test]
fn identity_consumption_is_rejected_for_non_inspector_views() {
    let error = admit_view_shape(
        &direct_collection(),
        ViewShapeDescriptor::identity_aware_inspector_detail_observed(),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::IncompatibleCanonicalFamily
    );
}
