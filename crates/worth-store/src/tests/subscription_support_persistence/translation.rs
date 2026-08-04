use super::{
    unique_test_store_path, RawSupportProgramAction, StoreErrorKind,
    SubscriptionSupportActionOrigin, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportOperationalVerdictTranslationRequest, SubscriptionSupportRetentionDecision,
    SubscriptionSupportRole, SupportActionBreadthBudget, SupportActionId,
    SupportActionPublicationState, SupportAllocationScope, SupportPathClass,
    SupportProgramDensityClass, WORTHStoreBuilder,
};

use super::{maintenance_basis, retention_basis};

#[test]
fn subscription_support_translation_rejects_WORTHd_report_basis() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let plan = store
        .admit_subscription_support_retention_batch(
            SupportActionId::new("support-retention:WORTHd-translation-basis").unwrap(),
            vec![retention_basis("WORTHd-translation-basis")],
            SubscriptionSupportRetentionDecision::retain_exact(),
            SupportPathClass::OperationalPlanning,
            SupportProgramDensityClass::FamilyLocalBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let report = store
        .publish_subscription_support_retention_consequence(plan)
        .unwrap();

    let WORTHd_basis = SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        SubscriptionSupportArtifactId("artifact:store-retention:WORTHd-translation-basis".into()),
        "basis:store-retention",
        "cursor:store-retention",
        "checkpoint:store-retention",
        "compatibility:WORTHd-drift",
        "portability:store-retention",
        SubscriptionSupportActionOrigin::Retention,
    )
    .unwrap();

    let error =
        SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_retention_report(
            &report,
            WORTHd_basis,
        )
        .expect_err(
            "translation must reject a report basis whose digests drift from the published proof",
        );

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}

#[test]
fn subscription_support_maintenance_debt_translation_requires_reported_basis() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let basis = maintenance_basis("delayed-exact-translation");
    let plan = store
        .admit_subscription_support_maintenance_batch(
            SupportActionId::new("support-maintenance:delayed-exact-translation").unwrap(),
            vec![basis.clone()],
            SubscriptionSupportMaintenanceDecision::refresh_descriptor_admitted(
                "maintenance refresh deferred by operator pacing",
            )
            .unwrap(),
            SupportPathClass::MaintenanceExecution,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::FamilyLocalBatch,
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();
    let report = store
        .report_delayed_subscription_support_maintenance(
            &plan,
            "maintenance refresh deferred by operator pacing",
            SupportActionBreadthBudget::new(4, 1024).unwrap(),
            128,
        )
        .unwrap();

    let WORTHd_basis = SubscriptionSupportOperationalBasis::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        SubscriptionSupportRole::ExactContinuation,
        basis.artifact_id().clone(),
        "basis:store-maintenance:delayed-exact-translation",
        "cursor:store-maintenance",
        "checkpoint:store-maintenance",
        "compatibility:WORTHd-drift",
        "portability:store-maintenance",
        SubscriptionSupportActionOrigin::Maintenance,
    )
    .unwrap();

    let error =
        SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_maintenance_debt_report(
            &report,
            WORTHd_basis,
        )
        .expect_err("maintenance debt translation must reject a basis the debt report did not prove");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );

    store
        .translate_subscription_support_operational_verdict(
            SubscriptionSupportOperationalVerdictTranslationRequest::exact_from_maintenance_debt_report(
                &report,
                basis,
            )
            .unwrap(),
        )
        .expect("exact delayed refresh should still translate when using the report-proven basis");
}

#[test]
fn local_file_interrupted_refresh_maintenance_work_kind_drift_fails_open() {
    let path = unique_test_store_path("worth-store-support-maintenance-work-kind-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let plan = store
            .admit_subscription_support_maintenance_batch(
                SupportActionId::new("support-maintenance:restart-refresh-drift").unwrap(),
                vec![maintenance_basis("restart-refresh-drift")],
                SubscriptionSupportMaintenanceDecision::interrupted_restart_recovered(
                    crate::SupportMaintenanceWorkKind::Refresh,
                    "maintenance-restart:refresh-drift",
                )
                .unwrap(),
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
        store
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_maintenance_descriptor_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support maintenance descriptor records should persist");
    let first_record = records
        .values_mut()
        .next()
        .expect("one descriptor record should persist");
    first_record["work_kind"] = serde_json::Value::String(String::from("Rebuild"));
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("interrupted refresh descriptors must not reopen as rebuild work");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn local_file_subscription_support_action_record_drift_fails_open() {
    let path = unique_test_store_path("worth-store-subscription-support-action-record-drift");
    let action_id = SupportActionId::new("support-retention:drifted-action-record").unwrap();
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let executed = RawSupportProgramAction::new(
            action_id.clone(),
            retention_basis("drifted-action"),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap()
        .plan()
        .verify()
        .execute();
        store
            .persist_subscription_support_executed_action_for_publication(executed)
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_action_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support action records should persist");
    let record = records
        .get_mut(action_id.as_str())
        .expect("staged action record should persist");
    record["publication_state"] = serde_json::Value::String(format!(
        "{:?}",
        SupportActionPublicationState::PublishedConsequence
    ));
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("drifted action record should fail reopen");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn local_file_subscription_support_action_record_rejects_illegal_tier_recall_rebuild_state() {
    let path = unique_test_store_path("worth-store-subscription-support-tier-recall-action-drift");
    let action_id = SupportActionId::new("support-tier-recall:drifted-action-record").unwrap();
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let executed = RawSupportProgramAction::new(
            action_id.clone(),
            retention_basis("tier-recall-drift"),
            SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        )
        .unwrap()
        .plan()
        .verify()
        .execute();
        store
            .persist_subscription_support_executed_action_for_publication(executed)
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_action_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support action records should persist");
    let record = records
        .get_mut(action_id.as_str())
        .expect("staged action record should persist");
    record["action_origin"] = serde_json::Value::String(String::from("TierRecall"));
    record["verdict"] = serde_json::Value::String(String::from("RebuildRequired"));
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("illegal tier-recall rebuild posture must fail reopen");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}
