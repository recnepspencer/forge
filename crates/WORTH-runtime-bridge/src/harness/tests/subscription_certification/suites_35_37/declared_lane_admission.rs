use super::*;

#[test]
fn bridge_harness_subscription_suite_36_reference_workload_requires_declared_lanes() {
    let bridge = runtime(BridgeRuntimePolicy::development());
    let manifest = bridge
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            BridgeSubscriptionReferenceWorkloadLaneIdSet::from_declared_lane_labels([
                "authoritative-live",
            ]),
        )
        .expect("single-lane manifest should seal before workload admission");

    let insufficient = bridge
        .run_subscription_reference_workload(
            &manifest,
            vec![BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
            )],
        )
        .expect_err("reference workload certification requires at least two lanes");
    assert_eq!(
        insufficient.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::InsufficientLaneSet
    );

    let duplicate_only = bridge
        .run_subscription_reference_workload(
            &manifest,
            vec![
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::AuthoritativeLive,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
            ],
        )
        .expect_err("duplicate lane requests must not satisfy cross-lane certification");
    assert_eq!(
        duplicate_only.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::InsufficientLaneSet
    );

    let undeclared = bridge
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
            ],
        )
        .expect_err("requested lanes must be present in the sealed manifest");
    assert_eq!(
        undeclared.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::LaneNotDeclaredByManifest
    );
    assert_eq!(
        undeclared.lane_kind(),
        Some(BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation)
    );

    let no_control_manifest = bridge
        .declare_subscription_reference_workload_manifest(
            product_ids(),
            component_ids(),
            BridgeSubscriptionReferenceWorkloadLaneIdSet::from_declared_lane_labels([
                "diagnostics-tier-variation",
                "hostile-adapter-variation",
            ]),
        )
        .expect("non-control lanes can be declared, but cannot certify alone");
    let no_control = bridge
        .run_subscription_reference_workload(
            &no_control_manifest,
            vec![
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::DiagnosticsTierVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
                BridgeSubscriptionReferenceWorkloadLaneRequest::new(
                    BridgeSubscriptionReferenceWorkloadLaneKind::HostileAdapterVariation,
                    BridgeSubscriptionReferenceWorkloadFamilyKind::DetailExact,
                ),
            ],
        )
        .expect_err("reference workload certification requires authoritative control");
    assert_eq!(
        no_control.rejection_kind(),
        BridgeSubscriptionReferenceWorkloadRejectionKind::MissingAuthoritativeControlLane
    );
}
