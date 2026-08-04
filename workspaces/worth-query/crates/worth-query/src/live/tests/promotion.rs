use crate::live::*;
#[test]
fn detail_preflight_promotes_to_detail_live_plan() {
    let preflight = crate::harness::fixtures::execution_preflights::direct_runtime_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("detail preflight should promote");

    assert_eq!(live.descriptor().family(), &LiveQueryFamily::Detail);
    assert_eq!(live.performance_status(), "verified");
    assert_eq!(live.progress_basis().last_ordinal().value(), 0);
}

#[test]
fn collection_with_traversal_promotes_to_bounded_materialization_live_plan() {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");

    assert_eq!(
        live.descriptor().family(),
        &LiveQueryFamily::BoundedMaterialization
    );
    assert_eq!(live.performance_status(), "debt");
    assert!(live.subscription_digest().as_str().len() > 10);
}

#[test]
fn collection_without_traversal_promotes_to_ordered_collection_live_plan() {
    let preflight = crate::harness::fixtures::execution_preflights::
            ordered_collection_without_traversal_preflight();
    let live =
        promote_preflight_bundle_to_live(&preflight).expect("collection preflight should promote");

    assert_eq!(
        live.descriptor().family(),
        &LiveQueryFamily::OrderedCollection
    );
}

#[test]
fn cdc_collection_preflight_is_rejected_for_live_promotion() {
    let preflight = crate::harness::fixtures::execution_preflights::cdc_collection_preflight();
    let error = promote_preflight_bundle_to_live(&preflight)
        .expect_err("cdc-shaped collection should not admit live promotion");

    assert_eq!(error, LivePromotionError::UnsupportedLiveCollectionFamily);
}
