use super::super::{
    compatibility_basis, compatibility_batch_request, read_receipt_witness,
    rejected_read_outcome_witness, CompatibilityRejectionKind, CompatibilityRelation,
    SubscriptionSupportCertificationLaneKind, SubscriptionSupportCertificationLaneOutcome,
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportFamilyId, SupportActionId,
    SupportAllocationScope, SupportPathClass, SupportProgramDensityClass, SupportProgramPathPolicy,
    WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_compatibility(evidence: &mut CertificationMatrixEvidence) {
    let compatibility_exact = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_compatibility_batch(compatibility_batch_request(
                SupportActionId::new("support-compatibility:cert-exact").unwrap(),
                vec![
                    compatibility_basis("exact-a"),
                    compatibility_basis("exact-b"),
                ],
                read_receipt_witness(CompatibilityRelation::Native),
                "semantic:cert-compatibility:exact",
                SubscriptionSupportCompatibilityDecision::exact_compatible_migration(
                    "classifier-equivalence:cert-v1-v2",
                )
                .unwrap(),
            ))
            .unwrap();
        let report = store
            .publish_subscription_support_compatibility_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_compatibility_report(
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityExactMigration,
            &compatibility_exact.0,
            compatibility_exact.1,
        )
        .unwrap(),
    );

    let compatibility_basis_local = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_compatibility_batch(
                crate::SubscriptionSupportCompatibilityBatchRequest {
                    action_id: SupportActionId::new("support-compatibility:cert-basis-local")
                        .unwrap(),
                    affected_bases: vec![
                        compatibility_basis("basis-local-a"),
                        compatibility_basis("basis-local-b"),
                    ],
                    compatibility_receipt: read_receipt_witness(CompatibilityRelation::Native),
                    semantic_digest: "semantic:cert-compatibility:basis-local".to_string(),
                    decision: SubscriptionSupportCompatibilityDecision::exact_compatible_migration(
                        "classifier-equivalence:cert-basis-local",
                    )
                    .unwrap(),
                    path: crate::SupportProgramPathPolicy {
                        path_class: SupportPathClass::OperationalPlanning,
                        density_class: SupportProgramDensityClass::BasisLocalBatch,
                        allocation_scope: SupportAllocationScope::ActionLocal,
                        budget: crate::SupportActionBreadthBudget::new(4, 1024).unwrap(),
                        payload_header_bytes: 128,
                    },
                },
            )
            .unwrap();
        let report = store
            .publish_subscription_support_compatibility_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_compatibility_report(
            SubscriptionSupportCertificationLaneKind::SupportBasisLocalBatchBounded,
            &compatibility_basis_local.0,
            compatibility_basis_local.1,
        )
        .unwrap(),
    );

    let compatibility_degraded = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_compatibility_batch(compatibility_batch_request(
                SupportActionId::new("support-compatibility:cert-degraded").unwrap(),
                vec![compatibility_basis("degraded")],
                read_receipt_witness(CompatibilityRelation::AdapterRequired),
                "semantic:cert-compatibility:degraded",
                SubscriptionSupportCompatibilityDecision::degraded_compatibility(
                    "classifier equivalence weakened by removed cursor hint",
                )
                .unwrap(),
            ))
            .unwrap();
        let report = store
            .publish_subscription_support_compatibility_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_compatibility_report(
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityDegraded,
            &compatibility_degraded.0,
            compatibility_degraded.1,
        )
        .unwrap(),
    );

    for (lane, decision) in [
        (
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityOldReaderRejected,
            SubscriptionSupportCompatibilityDecision::old_reader_rejected(1, 2).unwrap(),
        ),
        (
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityUnknownFamilyRejected,
            SubscriptionSupportCompatibilityDecision::unknown_family_rejected(
                SubscriptionSupportFamilyId::new("unknown-support-family").unwrap(),
            ),
        ),
        (
            SubscriptionSupportCertificationLaneKind::SupportCompatibilityVersionSkewRejected,
            SubscriptionSupportCompatibilityDecision::version_skew_rejected(
                "payload version has no admitted support reader",
            )
            .unwrap(),
        ),
    ] {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_compatibility_batch(compatibility_batch_request(
                SupportActionId::new(format!("support-compatibility:{lane:?}")).unwrap(),
                vec![compatibility_basis("reject")],
                rejected_read_outcome_witness(
                    CompatibilityRejectionKind::ReaderCapabilityUnsupported,
                ),
                "semantic:cert-compatibility:reject",
                decision,
            ))
            .unwrap();
        let report = store
            .publish_subscription_support_compatibility_consequence(plan)
            .unwrap();
        evidence.record_lane_outcome(
            SubscriptionSupportCertificationLaneOutcome::from_compatibility_report(
                lane,
                &report,
                store.subscription_support_counters(),
            )
            .unwrap(),
        );
    }
}
