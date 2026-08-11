use super::{
    CompatibilityRejectionKind, SubscriptionSupportActionOrigin,
    SubscriptionSupportCompatibilityBatchRequest, SubscriptionSupportCompatibilityDecision,
    SubscriptionSupportCompatibilityDecisionKind, SubscriptionSupportCompatibilityOutcome,
    SubscriptionSupportMaintenanceBatchRequest, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportOperationalVerdictTranslationRequest,
    SubscriptionSupportPortabilityBatchRequest, SubscriptionSupportPortabilityDecision,
    SubscriptionSupportPortabilityDecisionKind, SubscriptionSupportPortabilityOutcome,
    SubscriptionSupportRetentionBatchRequest, SubscriptionSupportRetentionDecision,
    SubscriptionSupportRetentionDecisionKind, SubscriptionSupportRetentionMaterialization,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope, SupportPathClass,
    SupportPortabilityManifestBudget, SupportProgramDensityClass,
    SupportProgramPathAdmissionRequest, SupportProgramPathPolicy, WORTHStoreBuilder,
};

use super::{
    compatibility_basis, maintenance_basis, portability_basis, rejected_read_outcome_witness,
    retention_basis,
};

#[test]
fn subscription_support_operational_facade_helpers_record_backend_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let basis = retention_basis("facade");
    let retention_plan = store
        .admit_subscription_support_retention_batch(SubscriptionSupportRetentionBatchRequest {
            action_id: SupportActionId::new("support-retention:facade-translation").unwrap(),
            affected_bases: vec![basis.clone()],
            decision: SubscriptionSupportRetentionDecision::retain_exact(),
            path: SupportProgramPathPolicy {
                path_class: SupportPathClass::OperationalPlanning,
                density_class: SupportProgramDensityClass::FamilyLocalBatch,
                allocation_scope: SupportAllocationScope::FamilyLocalBatch,
                budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                payload_header_bytes: 128,
            },
        })
        .unwrap();
    let retention_report = store
        .publish_subscription_support_retention_consequence(retention_plan)
        .unwrap();

    store
        .translate_subscription_support_operational_verdict(
            SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_retention_report(
                &retention_report,
                basis,
            )
            .unwrap(),
        )
        .unwrap();

    let plan = store
        .admit_subscription_support_program_path(SupportProgramPathAdmissionRequest {
            policy: SupportProgramPathPolicy {
                path_class: SupportPathClass::OperationalPlanning,
                density_class: SupportProgramDensityClass::FamilyLocalBatch,
                allocation_scope: SupportAllocationScope::FamilyLocalBatch,
                budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                payload_header_bytes: 128,
            },
            affected_entries: 2,
        })
        .unwrap();

    assert_eq!(
        store
            .subscription_support_counters()
            .operational_verdict_translation_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_batch_receipt_reuse_count(),
        0
    );

    store
        .reuse_subscription_support_batch_receipt(&plan)
        .unwrap();

    assert_eq!(
        store
            .subscription_support_counters()
            .support_batch_receipt_reuse_count(),
        1
    );
}

#[test]
fn subscription_support_retention_facade_publishes_consequence_and_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_retention_batch(SubscriptionSupportRetentionBatchRequest {
            action_id: SupportActionId::new("support-retention:facade").unwrap(),
            affected_bases: vec![retention_basis("facade-1"), retention_basis("facade-2")],
            decision: SubscriptionSupportRetentionDecision::expire_by_policy(
                "support retention window expired",
            )
            .unwrap(),
            path: SupportProgramPathPolicy {
                path_class: SupportPathClass::OperationalPlanning,
                density_class: SupportProgramDensityClass::FamilyLocalBatch,
                allocation_scope: SupportAllocationScope::FamilyLocalBatch,
                budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                payload_header_bytes: 256,
            },
        })
        .unwrap();

    assert_eq!(
        store
            .subscription_support_counters()
            .support_retention_plan_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_policy_expiration_count(),
        0
    );

    let report = store
        .publish_subscription_support_retention_consequence(plan)
        .unwrap();

    assert_eq!(
        report.survival_witness().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );
    assert_eq!(
        report.retention_record().decision_kind(),
        SubscriptionSupportRetentionDecisionKind::ExpireByPolicy
    );
    assert_eq!(
        report.retention_record().affected_set_digest(),
        report.survival_witness().affected_set_digest()
    );
    assert!(matches!(
        report.materialization(),
        SubscriptionSupportRetentionMaterialization::Expired(_)
    ));
    assert_eq!(
        store
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_expired_family_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_policy_expiration_count(),
        1
    );
}

#[test]
fn subscription_support_compatibility_facade_publishes_version_skew_consequence_and_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_compatibility_batch(
            SubscriptionSupportCompatibilityBatchRequest {
                action_id: SupportActionId::new("support-compatibility:facade").unwrap(),
                affected_bases: vec![compatibility_basis("facade")],
                compatibility_receipt: rejected_read_outcome_witness(
                    CompatibilityRejectionKind::MissingCompatibilityEdge,
                ),
                semantic_digest: "semantic:store-compatibility".to_string(),
                decision: SubscriptionSupportCompatibilityDecision::version_skew_rejected(
                    "payload version has no admitted support reader",
                )
                .unwrap(),
                path: SupportProgramPathPolicy {
                    path_class: SupportPathClass::OperationalPlanning,
                    density_class: SupportProgramDensityClass::FamilyLocalBatch,
                    allocation_scope: SupportAllocationScope::FamilyLocalBatch,
                    budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                    payload_header_bytes: 128,
                },
            },
        )
        .unwrap();

    let report = store
        .publish_subscription_support_compatibility_consequence(plan)
        .unwrap();
    let SubscriptionSupportCompatibilityOutcome::Rejected(rejection) = report.outcome() else {
        panic!("version skew must publish a typed rejection outcome");
    };
    assert_eq!(
        rejection.rejection_kind(),
        SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected
    );
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RejectedByPolicy
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_compatibility_plan_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_version_skew_rejection_count(),
        1
    );
}

#[test]
fn subscription_support_portability_facade_publishes_import_bundle_and_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_portability_batch(SubscriptionSupportPortabilityBatchRequest {
            action_id: SupportActionId::new("support-portability:facade-import").unwrap(),
            affected_bases: vec![portability_basis(
                SubscriptionSupportActionOrigin::ReplicationImport,
                "facade-import",
            )],
            included_support_count: 1,
            omitted_support_count: 0,
            manifest_budget: SupportPortabilityManifestBudget::new(4, 1024).unwrap(),
            decision: SubscriptionSupportPortabilityDecision::target_import_admitted(
                "target-store-import-admission",
                "source-identity-preservation:store-import",
                "semantic:store-portability-import",
            )
            .unwrap(),
            path: SupportProgramPathPolicy {
                path_class: SupportPathClass::ImportAdmission,
                density_class: SupportProgramDensityClass::PortabilityScopeBatch,
                allocation_scope: SupportAllocationScope::PortabilityManifest,
                budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                payload_header_bytes: 128,
            },
        })
        .unwrap();

    assert_eq!(
        store
            .subscription_support_counters()
            .support_portability_plan_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_import_admission_count(),
        0
    );

    let report = store
        .publish_subscription_support_portability_consequence(plan)
        .unwrap();
    let SubscriptionSupportPortabilityOutcome::Imported(access) = report.outcome() else {
        panic!("facade import must produce admitted semantic access");
    };

    assert_eq!(
        report.participation_record().decision_kind(),
        SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted
    );
    assert_eq!(
        access.import_admission().manifest_digest(),
        report.manifest().manifest_digest()
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_import_admission_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_portability_required_basis_count(),
        1
    );
}

#[test]
fn subscription_support_maintenance_facade_admits_descriptor_and_counters() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let basis = maintenance_basis("facade-rebuild");
    let retained_basis_digest = basis.basis_digest().to_string();
    let plan = store
        .admit_subscription_support_maintenance_batch(SubscriptionSupportMaintenanceBatchRequest {
            action_id: SupportActionId::new("support-maintenance:facade-rebuild").unwrap(),
            affected_bases: vec![basis],
            decision: SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                retained_basis_digest,
            )
            .unwrap(),
            path: SupportProgramPathPolicy {
                path_class: SupportPathClass::MaintenanceExecution,
                density_class: SupportProgramDensityClass::MaintenanceKeyBatch,
                allocation_scope: SupportAllocationScope::FamilyLocalBatch,
                budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
                payload_header_bytes: 128,
            },
        })
        .unwrap();

    assert_eq!(
        plan.maintenance_receipt().batch_summary().batch_class(),
        crate::MaintenanceBatchClass::SubscriptionSupport
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_maintenance_descriptor_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_maintenance_rebuild_debt_count(),
        0
    );

    let report = store
        .publish_subscription_support_maintenance_consequence(plan)
        .unwrap();

    assert_eq!(report.admissions().len(), 1);
    assert_eq!(report.descriptor_records().len(), 1);
    assert_eq!(
        report.completed_action().envelope().verdict(),
        SubscriptionSupportOperationalVerdict::RebuildRequired
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_maintenance_rebuild_debt_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_action_envelope_publications(),
        1
    );
}
