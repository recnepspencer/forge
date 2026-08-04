use super::{
    StoreErrorKind, SubscriptionResumeClassification, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportMissingSupportMaintenanceAdmission,
    SubscriptionSupportMissingSupportRecoveryRequest,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportRestartShard,
    SubscriptionSupportRole, SupportActionBreadthBudget, SupportActionId, WORTHStoreBuilder,
};

use super::{raw_exact, raw_materialized};

#[test]
fn subscription_support_restart_reconstruction_rejects_unbounded_shard_work() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    for basis in ["basis:1", "basis:2"] {
        let admitted = store
            .admit_subscription_support_declaration(raw_exact())
            .unwrap();
        let publishable = store
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                basis,
                format!("cursor:{basis}"),
                format!("checkpoint:{basis}"),
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        store.publish_subscription_support(publishable).unwrap();
    }

    let error = store
        .reconstruct_subscription_support_restart_shard(
            SubscriptionSupportRestartReconstructionRequest::new(
                SubscriptionSupportRestartShard::for_family(
                    SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                    SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
                ),
                1,
            )
            .unwrap(),
        )
        .expect_err("restart reconstruction must reject shards over the admitted row bound");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .restart_reconstruction_count(),
        0
    );
}

#[test]
fn subscription_support_restart_reconstruction_rejects_family_kind_mismatch() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let admitted = store
        .admit_subscription_support_declaration(raw_exact())
        .unwrap();
    let publishable = store
        .subscription_support_pipeline()
        .prepare_exact(
            admitted,
            "basis:1",
            "cursor:1",
            "checkpoint:1",
            "schema:1",
            "compatibility:1",
        )
        .unwrap();
    store.publish_subscription_support(publishable).unwrap();

    let error = store
        .reconstruct_subscription_support_restart_shard(
            SubscriptionSupportRestartReconstructionRequest::new(
                SubscriptionSupportRestartShard::for_family(
                    SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
                    SubscriptionSupportFamilyKind::DegradedContinuationSupport,
                ),
                8,
            )
            .unwrap(),
        )
        .expect_err("restart shard proof must include the admitted family kind");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn subscription_support_missing_materialized_support_requires_retained_rebuild_basis() {
    let artifact_id = {
        let mut source = WORTHStoreBuilder::new().in_memory().build().unwrap();
        let admitted = source
            .admit_subscription_support_declaration(raw_materialized())
            .unwrap();
        let publishable = source
            .subscription_support_pipeline()
            .prepare_exact(
                admitted,
                "basis:retained",
                "cursor:1",
                "checkpoint:1",
                "schema:1",
                "compatibility:1",
            )
            .unwrap();
        source
            .publish_subscription_support(publishable)
            .unwrap()
            .artifact_id()
            .clone()
    };
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let rebuild = store
        .classify_missing_subscription_support(
            SubscriptionSupportMissingSupportRecoveryRequest::new(
                SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                SubscriptionSupportRole::NarrowingMaterialization,
                artifact_id.clone(),
                "basis:retained",
                "cursor:1",
                "checkpoint:1",
                "compatibility:1",
                "portability:1",
            )
            .unwrap()
            .with_rebuild_maintenance_admission(
                "basis:retained",
                SubscriptionSupportMissingSupportMaintenanceAdmission::new(
                    SupportActionId::new("support-maintenance:missing-recovery").unwrap(),
                    SupportActionBreadthBudget::new(1, 1024).unwrap(),
                    128,
                )
                .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        rebuild.classification(),
        SubscriptionResumeClassification::RebuildRequired
    );
    let maintenance_report = rebuild
        .maintenance_report()
        .expect("rebuildable missing support must admit maintenance work");
    assert_eq!(
        maintenance_report.participation_record().decision_kind(),
        crate::SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .rebuild_basis_plan_count(),
        1
    );
    assert_eq!(
        store
            .subscription_support_counters()
            .support_maintenance_rebuild_debt_count(),
        1
    );

    let denied = store
        .classify_missing_subscription_support(
            SubscriptionSupportMissingSupportRecoveryRequest::new(
                SubscriptionSupportFamilyId::new("materialized-narrowing-support").unwrap(),
                SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
                SubscriptionSupportRole::NarrowingMaterialization,
                artifact_id,
                "basis:retained",
                "cursor:1",
                "checkpoint:1",
                "compatibility:1",
                "portability:1",
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        denied.classification(),
        SubscriptionResumeClassification::NotResumable
    );
}
