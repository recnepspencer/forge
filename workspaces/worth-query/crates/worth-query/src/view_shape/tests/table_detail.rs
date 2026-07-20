use super::*;
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, plan_admitted_view_shape_from_execution_plan,
    validate_canonical_bundle_for_admitted_view_shape, ViewShapeComplexityStatus,
    ViewShapeDescriptor, ViewShapeFailureClass, ViewShapeInvalidationPosture,
    ViewShapePatchPosture,
};

#[test]
fn table_denies_detail_queries() {
    let error = admit_view_shape(&direct_detail(), ViewShapeDescriptor::table()).unwrap_err();
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::IncompatibleCanonicalFamily
    );
}

#[test]
fn table_plan_is_verified_runtime_backed_product_lane() {
    let canonical = direct_collection();
    let planned = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            collection_schema_view(),
            admit_view_shape(&canonical, ViewShapeDescriptor::table()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    assert!(planned.admitted().compatibility().admitted());
    assert_eq!(planned.family().as_str(), "table");
    assert_eq!(
        planned.complexity().status(),
        ViewShapeComplexityStatus::Verified
    );
    assert_eq!(
        planned.invalidation_posture(),
        &ViewShapeInvalidationPosture::OrderedCollectionMembershipAndOrdering
    );
    assert_eq!(
        planned.patch_posture(),
        &ViewShapePatchPosture::TableRowPatch
    );
    assert_eq!(
        planned.validated_view().canonical_query_digest(),
        canonical.query().digest()
    );
    assert_eq!(
        planned.validated_view().canonical_result_shape_digest(),
        canonical.result_shape().digest()
    );
    assert_eq!(
        planned.execution_plan().query().canonical_query_digest(),
        canonical.query().digest()
    );
    assert_eq!(
        planned
            .execution_plan()
            .result_shape()
            .canonical_result_shape_digest(),
        canonical.result_shape().digest()
    );
}

#[test]
fn detail_plan_is_verified_runtime_backed_product_lane() {
    let canonical = direct_detail();
    let planned = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(&canonical, ViewShapeDescriptor::detail()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    assert!(planned.admitted().compatibility().admitted());
    assert_eq!(planned.family().as_str(), "detail");
    assert_eq!(
        planned.complexity().status(),
        ViewShapeComplexityStatus::Verified
    );
    assert_eq!(
        planned.invalidation_posture(),
        &ViewShapeInvalidationPosture::DetailProjectionFields
    );
    assert_eq!(
        planned.patch_posture(),
        &ViewShapePatchPosture::DetailFieldPatch
    );
    assert!(planned
        .delivery_metadata()
        .projection_legality_matches_detail());
    assert_eq!(
        planned.validated_view().canonical_query_digest(),
        canonical.query().digest()
    );
    assert_eq!(
        planned.validated_view().canonical_result_shape_digest(),
        canonical.result_shape().digest()
    );
    assert_eq!(
        planned.execution_plan().query().canonical_query_digest(),
        canonical.query().digest()
    );
    assert_eq!(
        planned
            .execution_plan()
            .result_shape()
            .canonical_result_shape_digest(),
        canonical.result_shape().digest()
    );
}

#[test]
fn live_promotion_consumes_the_exact_admitted_execution_plan() {
    let canonical = direct_collection();
    let planned = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            collection_schema_view(),
            admit_view_shape(&canonical, ViewShapeDescriptor::table()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let promoted = plan_admitted_view_shape_from_execution_plan(
        planned.validated_view().clone(),
        planned.execution_plan().clone(),
    )
    .expect("exact admitted execution plan should promote to live");

    assert_eq!(
        promoted.execution_plan().query().plan_digest(),
        planned.execution_plan().query().plan_digest()
    );
}

#[test]
fn live_promotion_rejects_an_execution_plan_from_another_query() {
    let collection = direct_collection();
    let collection_plan = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &collection,
            collection_schema_view(),
            admit_view_shape(&collection, ViewShapeDescriptor::table()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let detail = direct_detail();
    let detail_plan = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &detail,
            detail_schema_view(),
            admit_view_shape(&detail, ViewShapeDescriptor::detail()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    let error = plan_admitted_view_shape_from_execution_plan(
        collection_plan.validated_view().clone(),
        detail_plan.execution_plan().clone(),
    )
    .expect_err("live promotion must reject unrelated admitted evidence");
    assert_eq!(
        error.failure_class(),
        &ViewShapeFailureClass::PlanningInvariantRejected
    );
}
