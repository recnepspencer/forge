use super::super::{
    retention_basis, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportRetentionBatchRequest,
    SubscriptionSupportRetentionDecision, SupportActionBreadthBudget, SupportActionId,
    SupportAllocationScope, SupportPathClass, SupportProgramDensityClass, SupportProgramPathPolicy,
    WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

fn retention_request(
    action_id: SupportActionId,
    affected_bases: Vec<crate::SubscriptionSupportOperationalBasis>,
    decision: SubscriptionSupportRetentionDecision,
) -> SubscriptionSupportRetentionBatchRequest {
    SubscriptionSupportRetentionBatchRequest {
        action_id,
        affected_bases,
        decision,
        path: SupportProgramPathPolicy {
            path_class: SupportPathClass::OperationalPlanning,
            density_class: SupportProgramDensityClass::FamilyLocalBatch,
            allocation_scope: SupportAllocationScope::FamilyLocalBatch,
            budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
            payload_header_bytes: 128,
        },
    }
}

pub(super) fn record_retention(evidence: &mut CertificationMatrixEvidence) {
    let retention_exact = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(retention_request(
                SupportActionId::new("support-retention:cert-exact").unwrap(),
                vec![retention_basis("exact-a"), retention_basis("exact-b")],
                SubscriptionSupportRetentionDecision::retain_exact(),
            ))
            .unwrap();
        let report = store
            .publish_subscription_support_retention_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportRetentionExactPreserved,
            &retention_exact.0,
            retention_exact.1.clone(),
        )
        .unwrap(),
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportFamilyLocalBatchBounded,
            &retention_exact.0,
            retention_exact.1,
        )
        .unwrap(),
    );

    let retention_compacted = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(retention_request(
                SupportActionId::new("support-retention:cert-compacted").unwrap(),
                vec![retention_basis("compact-a"), retention_basis("compact-b")],
                SubscriptionSupportRetentionDecision::compact_exact("compacted-basis:cert")
                    .unwrap(),
            ))
            .unwrap();
        let report = store
            .publish_subscription_support_retention_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportRetentionCompactedExact,
            &retention_compacted.0,
            retention_compacted.1,
        )
        .unwrap(),
    );

    let retention_reclaimed = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(retention_request(
                SupportActionId::new("support-retention:cert-reclaim").unwrap(),
                vec![retention_basis("reclaim")],
                SubscriptionSupportRetentionDecision::reclaim_with_rebuild(
                    "basis:cert-reclaim",
                    "maintenance:key:cert-reclaim",
                )
                .unwrap(),
            ))
            .unwrap();
        let report = store
            .publish_subscription_support_retention_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportRetentionReclaimedRebuildable,
            &retention_reclaimed.0,
            retention_reclaimed.1,
        )
        .unwrap(),
    );

    let retention_expired = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_retention_batch(retention_request(
                SupportActionId::new("support-retention:cert-expired").unwrap(),
                vec![retention_basis("expired")],
                SubscriptionSupportRetentionDecision::expire_by_policy("policy-expired:cert")
                    .unwrap(),
            ))
            .unwrap();
        let report = store
            .publish_subscription_support_retention_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_retention_report(
            SubscriptionSupportCertificationLaneKind::SupportRetentionExpiredByPolicy,
            &retention_expired.0,
            retention_expired.1,
        )
        .unwrap(),
    );
}
