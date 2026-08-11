use super::super::{
    portability_basis, SubscriptionSupportCertificationLaneKind,
    SubscriptionSupportCertificationLaneOutcome, SubscriptionSupportPortabilityBatchRequest,
    SubscriptionSupportPortabilityDecision, SupportActionBreadthBudget, SupportActionId,
    SupportAllocationScope, SupportPathClass, SupportPortabilityManifestBudget,
    SupportProgramDensityClass, SupportProgramPathPolicy, WORTHStoreBuilder,
};
use super::evidence::CertificationMatrixEvidence;

pub(super) fn record_portability(evidence: &mut CertificationMatrixEvidence) {
    let portability_full = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_portability_batch(
                SubscriptionSupportPortabilityBatchRequest {
                    action_id: SupportActionId::new("support-portability:cert-full").unwrap(),
                    affected_bases: vec![
                        portability_basis(
                            crate::SubscriptionSupportActionOrigin::ReplicationExport,
                            "full-a",
                        ),
                        portability_basis(
                            crate::SubscriptionSupportActionOrigin::ReplicationExport,
                            "full-b",
                        ),
                    ],
                    included_support_count: 2,
                    omitted_support_count: 0,
                    manifest_budget: SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                    decision: SubscriptionSupportPortabilityDecision::full_scope_replication(
                        "identity-preservation:cert-full",
                        "identity-preservation:cert-full",
                    )
                    .unwrap(),
                    path: SupportProgramPathPolicy {
                        path_class: SupportPathClass::ReplicationExport,
                        density_class: SupportProgramDensityClass::PortabilityScopeBatch,
                        allocation_scope: SupportAllocationScope::PortabilityManifest,
                        budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                        payload_header_bytes: 128,
                    },
                },
            )
            .unwrap();
        let report = store
            .publish_subscription_support_portability_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityFullScopeReplicated,
            &portability_full.0,
            portability_full.1.clone(),
        )
        .unwrap(),
    );
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityScopeBatchBounded,
            &portability_full.0,
            portability_full.1,
        )
        .unwrap(),
    );

    let portability_partial = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let omitted_id = portability_basis(
            crate::SubscriptionSupportActionOrigin::ReplicationExport,
            "partial-b",
        )
        .artifact_id()
        .clone();
        let plan = store
            .admit_subscription_support_portability_batch(
                SubscriptionSupportPortabilityBatchRequest {
                    action_id: SupportActionId::new("support-portability:cert-partial").unwrap(),
                    affected_bases: vec![
                        portability_basis(
                            crate::SubscriptionSupportActionOrigin::ReplicationExport,
                            "partial-a",
                        ),
                        portability_basis(
                            crate::SubscriptionSupportActionOrigin::ReplicationExport,
                            "partial-b",
                        ),
                    ],
                    included_support_count: 1,
                    omitted_support_count: 1,
                    manifest_budget: SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                    decision: SubscriptionSupportPortabilityDecision::partial_scope_omission(
                        vec![omitted_id],
                        "partial scope export omitted one artifact",
                    )
                    .unwrap(),
                    path: SupportProgramPathPolicy {
                        path_class: SupportPathClass::ReplicationExport,
                        density_class: SupportProgramDensityClass::PortabilityScopeBatch,
                        allocation_scope: SupportAllocationScope::PortabilityManifest,
                        budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                        payload_header_bytes: 128,
                    },
                },
            )
            .unwrap();
        let report = store
            .publish_subscription_support_portability_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityPartialOmission,
            &portability_partial.0,
            portability_partial.1,
        )
        .unwrap(),
    );

    let portability_import = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let plan = store
            .admit_subscription_support_portability_batch(
                SubscriptionSupportPortabilityBatchRequest {
                    action_id: SupportActionId::new("support-portability:cert-import").unwrap(),
                    affected_bases: vec![portability_basis(
                        crate::SubscriptionSupportActionOrigin::ReplicationImport,
                        "import",
                    )],
                    included_support_count: 1,
                    omitted_support_count: 0,
                    manifest_budget: SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                    decision: SubscriptionSupportPortabilityDecision::target_import_admitted(
                        "target-import:cert",
                        "identity-preservation:cert",
                        "semantic:cert-import",
                    )
                    .unwrap(),
                    path: SupportProgramPathPolicy {
                        path_class: SupportPathClass::ImportAdmission,
                        density_class: SupportProgramDensityClass::PortabilityScopeBatch,
                        allocation_scope: SupportAllocationScope::PortabilityManifest,
                        budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                        payload_header_bytes: 128,
                    },
                },
            )
            .unwrap();
        let report = store
            .publish_subscription_support_portability_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityImportAdmitted,
            &portability_import.0,
            portability_import.1,
        )
        .unwrap(),
    );

    let portability_missing_basis = {
        let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let basis_a = portability_basis(
            crate::SubscriptionSupportActionOrigin::ReplicationImport,
            "missing-a",
        );
        let basis_b = portability_basis(
            crate::SubscriptionSupportActionOrigin::ReplicationImport,
            "missing-b",
        );
        let plan = store
            .admit_subscription_support_portability_batch(
                SubscriptionSupportPortabilityBatchRequest {
                    action_id: SupportActionId::new(
                        "support-portability:cert-missing-basis",
                    )
                    .unwrap(),
                    affected_bases: vec![basis_a.clone(), basis_b.clone()],
                    included_support_count: 2,
                    omitted_support_count: 0,
                    manifest_budget: SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
                    decision:
                        SubscriptionSupportPortabilityDecision::target_import_missing_basis_not_resumable(
                            "target-import-missing:cert",
                            vec![basis_a.artifact_id().clone()],
                            "missing exact imported basis",
                        )
                        .unwrap(),
                    path: SupportProgramPathPolicy {
                        path_class: SupportPathClass::ImportAdmission,
                        density_class: SupportProgramDensityClass::PortabilityScopeBatch,
                        allocation_scope: SupportAllocationScope::PortabilityManifest,
                        budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                        payload_header_bytes: 128,
                    },
                },
            )
            .unwrap();
        let report = store
            .publish_subscription_support_portability_consequence(plan)
            .unwrap();
        let counters = store.subscription_support_counters();
        (report, counters)
    };
    evidence.record_lane_outcome(
        SubscriptionSupportCertificationLaneOutcome::from_portability_report(
            SubscriptionSupportCertificationLaneKind::SupportPortabilityImportMissingBasisNotResumable,
            &portability_missing_basis.0,
            portability_missing_basis.1,
        )
        .unwrap(),
    );
}
