use forge_query::facade::runtime::{
    ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};

use super::admission_test_support::split_request_subject;
use super::SpatialGeometryEvidenceTouchRequest;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::LoopFixtureEntryOrder;

#[test]
fn query_selector_precision_matches_only_spatial_product_declared_descriptor_surface() {
    let subject = split_request_subject(LoopFixtureEntryOrder::Canonical);
    let authority = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&subject.receipt)
        .with_complete_ledger(&subject.complete)
        .admit()
        .expect("split receipt should admit");
    let lookup = authority
        .spatial_evidence_lookup(&subject.complete)
        .expect("lookup should derive from admitted authority");
    let lowered = authority
        .query_touch_descriptor(&lookup)
        .expect("authority should lower to Query descriptor product");
    let descriptor = lowered.touch_descriptor();
    let first_aspect = lowered
        .aspect_paths()
        .first()
        .expect("lowering should declare aspect paths")
        .clone();

    assert!(
        ForgeQueryGraphTouchSelector::collection(lowered.collection())
            .expect("collection selector should admit")
            .matches_descriptor(descriptor)
    );
    assert!(
        ForgeQueryGraphTouchSelector::relation_kind(lowered.relation_kind())
            .expect("relation-kind selector should admit")
            .matches_descriptor(descriptor)
    );
    assert!(ForgeQueryGraphTouchSelector::aspect_path(first_aspect)
        .expect("aspect-path selector should admit")
        .matches_descriptor(descriptor));
    assert!(ForgeQueryGraphTouchSelector::read_verb(
        ForgeQueryGraphTouchReadVerb::ObservesAspectPath
    )
    .matches_descriptor(descriptor));

    assert!(
        !ForgeQueryGraphTouchSelector::collection("worth.spatial.unrelated")
            .expect("unrelated collection selector should admit")
            .matches_descriptor(descriptor)
    );
    assert!(
        !ForgeQueryGraphTouchSelector::aspect_path("unrelated.aspect")
            .expect("unrelated aspect selector should admit")
            .matches_descriptor(descriptor)
    );
    assert!(!ForgeQueryGraphTouchSelector::read_verb(
        ForgeQueryGraphTouchReadVerb::RetainsLiveSubscription
    )
    .matches_descriptor(descriptor));
    assert!(!ForgeQueryGraphTouchSelector::declared_mutation_collection(
        lowered.collection(),
        ForgeQueryMutationFamily::Update,
        ["spatial-touch:update"],
        ["spatial_touch.digest"],
    )
    .expect("declared mutation selector should construct as Query vocabulary")
    .matches_descriptor(descriptor));
}
