use super::*;

#[test]
fn bridge_harness_subscription_suite_37_reference_workload_keeps_family_strategy_distinct() {
    let bridge = runtime(BridgeRuntimePolicy::development());
    let manifest = bridge
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            lane_ids(),
        )
        .expect("reference workload manifest should seal");

    let report = bridge
        .run_subscription_reference_workload(
            &manifest,
            vec![
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership,
                ),
            ],
        )
        .expect("reference workload should certify mixed families");

    let detail_lane = report
        .lane_reports()
        .iter()
        .find(|lane| {
            lane.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive
        })
        .expect("authoritative detail lane should exist");
    let collection_lane = report
        .lane_reports()
        .iter()
        .find(|lane| {
            lane.lane_kind() == BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation
        })
        .expect("collection hostile lane should exist");

    assert_eq!(
        detail_lane.family_kind(),
        BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact
    );
    assert_eq!(
        collection_lane.family_kind(),
        BridgeSubscriptionReferenceWorkloadFamilyKind::CollectionMembership
    );
    assert_ne!(
        detail_lane.certification_bundle_digest(),
        collection_lane.certification_bundle_digest()
    );
    assert_eq!(report.counters().global_history_scan_count(), 0);
    assert_eq!(report.counters().global_subscription_scan_count(), 0);
}
