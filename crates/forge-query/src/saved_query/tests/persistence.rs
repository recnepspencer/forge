use super::*;
use crate::saved_query::{
    freeze_direct_saved_query, SavedQueryFailureClass, SavedQueryFreezeContext,
    SavedQueryPersistenceClaim,
};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor,
};

#[test]
fn durable_claims_are_explicitly_denied() {
    let direct = direct_collection();
    let view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            collection_schema_view(),
            admit_view_shape(&direct, ViewShapeDescriptor::table()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let saved = freeze_direct_saved_query(
        &direct,
        &view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap();

    let durable_reload = saved
        .admit_persistence_claim(SavedQueryPersistenceClaim::DurableReload)
        .unwrap_err();
    assert_eq!(
        durable_reload.failure_class(),
        &SavedQueryFailureClass::DurableClaimDenied
    );
    let import_export = saved
        .admit_persistence_claim(SavedQueryPersistenceClaim::ImportExport)
        .unwrap_err();
    assert_eq!(
        import_export.failure_class(),
        &SavedQueryFailureClass::DurableClaimDenied
    );
    let restart = saved
        .admit_persistence_claim(SavedQueryPersistenceClaim::RestartStableContinuation)
        .unwrap_err();
    assert_eq!(
        restart.failure_class(),
        &SavedQueryFailureClass::DurableClaimDenied
    );
}

#[test]
fn saved_query_freeze_denies_mismatched_canonical_and_view_plan() {
    let detail = direct_detail();
    let collection = direct_collection();
    let collection_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &collection,
            collection_schema_view(),
            admit_view_shape(&collection, ViewShapeDescriptor::table()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    let error = freeze_direct_saved_query(
        &detail,
        &collection_view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &SavedQueryFailureClass::FreezeInvariantRejected
    );
}
