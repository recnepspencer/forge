use super::{
    unique_test_sqlite_path, unique_test_store_path, StoreErrorKind,
    SubscriptionSupportMaintenanceBatchRequest, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportOperationalVerdict, SupportActionBreadthBudget, SupportActionId,
    SupportAllocationScope, SupportPathClass, SupportProgramDensityClass, SupportProgramPathPolicy,
    WORTHStoreBuilder,
};

use super::maintenance_basis;

fn maintenance_request(
    action_id: SupportActionId,
    affected_bases: Vec<crate::SubscriptionSupportOperationalBasis>,
    decision: SubscriptionSupportMaintenanceDecision,
) -> SubscriptionSupportMaintenanceBatchRequest {
    SubscriptionSupportMaintenanceBatchRequest {
        action_id,
        affected_bases,
        decision,
        path: SupportProgramPathPolicy {
            path_class: SupportPathClass::MaintenanceExecution,
            density_class: SupportProgramDensityClass::MaintenanceKeyBatch,
            allocation_scope: SupportAllocationScope::FamilyLocalBatch,
            budget: SupportActionBreadthBudget::new(4, 1024).unwrap(),
            payload_header_bytes: 128,
        },
    }
}

#[test]
fn subscription_support_maintenance_delay_report_persists_without_publishing_action() {
    let path = unique_test_store_path("worth-store-support-maintenance-delay-local-reopen");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("delayed-local");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(maintenance_request(
                SupportActionId::new("support-maintenance:delayed-local").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
            ))
            .unwrap();

        let report = store
            .report_delayed_subscription_support_maintenance(
                &plan,
                "maintenance pacing deferred support rebuild",
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();

        assert_eq!(
            report.debt_summary().verdict(),
            SubscriptionSupportOperationalVerdict::RebuildRequired
        );
        assert_eq!(
            store
                .subscription_support_counters()
                .support_maintenance_delay_count(),
            1
        );
        assert_eq!(
            store
                .subscription_support_counters()
                .support_action_envelope_publications(),
            0
        );
    }

    let reopened = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    assert_eq!(
        reopened
            .subscription_support_counters()
            .support_maintenance_delay_count(),
        1
    );

    let raw = std::fs::read_to_string(&path).unwrap();
    let payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get("subscription_support_maintenance_debt_records")
        .and_then(serde_json::Value::as_object)
        .expect("maintenance debt records should persist");
    assert_eq!(records.len(), 1);
}

#[test]
fn subscription_support_maintenance_descriptor_records_survive_local_file_reopen() {
    let path = unique_test_store_path("worth-store-support-maintenance-local-reopen");
    let declaration_id = {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("local-reopen");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(maintenance_request(
                SupportActionId::new("support-maintenance:local-reopen").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
            ))
            .unwrap();
        let declaration_id = plan.maintenance_receipt().admitted_declarations()[0]
            .declaration()
            .id()
            .clone();
        let report = store
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        assert_eq!(report.descriptor_records().len(), 1);
        declaration_id
    };

    let reopened = WORTHStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    let status = reopened.maintenance_status(&declaration_id).unwrap();
    assert_eq!(
        status.execution_status(),
        crate::MaintenanceExecutionStatus::Admitted
    );
    let raw = std::fs::read_to_string(&path).unwrap();
    let state: crate::backend::records::StoreState = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        state
            .subscription_support_maintenance_descriptor_records
            .len(),
        1
    );
}

#[test]
fn subscription_support_maintenance_descriptor_records_survive_sqlite_reopen() {
    let path = unique_test_sqlite_path("worth-store-support-maintenance-sqlite-reopen");
    let declaration_id = {
        let mut store = WORTHStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("sqlite-reopen");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(maintenance_request(
                SupportActionId::new("support-maintenance:sqlite-reopen").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
            ))
            .unwrap();
        let declaration_id = plan.maintenance_receipt().admitted_declarations()[0]
            .declaration()
            .id()
            .clone();
        store
            .publish_subscription_support_maintenance_consequence(plan)
            .unwrap();
        declaration_id
    };

    let reopened = WORTHStoreBuilder::new()
        .sqlite_file(path.clone())
        .build()
        .unwrap();
    let status = reopened.maintenance_status(&declaration_id).unwrap();
    assert_eq!(
        status.execution_status(),
        crate::MaintenanceExecutionStatus::Admitted
    );
    let connection = rusqlite::Connection::open(path).unwrap();
    let row_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM subscription_support_maintenance_descriptor_records",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(row_count, 1);
}

#[test]
fn local_file_subscription_support_maintenance_descriptor_drift_fails_open() {
    let path = unique_test_store_path("worth-store-support-maintenance-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("drift");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(maintenance_request(
                SupportActionId::new("support-maintenance:drift").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
            ))
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
        .expect("one record should persist");
    first_record["descriptor_digest"] = serde_json::Value::String("drifted-digest".into());
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("descriptor drift should fail reopen");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}

#[test]
fn local_file_subscription_support_maintenance_debt_drift_fails_open() {
    let path = unique_test_store_path("worth-store-support-maintenance-debt-drift");
    {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        let basis = maintenance_basis("debt-drift");
        let retained_basis_digest = basis.basis_digest().to_string();
        let plan = store
            .admit_subscription_support_maintenance_batch(maintenance_request(
                SupportActionId::new("support-maintenance:debt-drift").unwrap(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    retained_basis_digest,
                )
                .unwrap(),
            ))
            .unwrap();
        store
            .report_delayed_subscription_support_maintenance(
                &plan,
                "maintenance debt drift fixture",
                SupportActionBreadthBudget::new(4, 1024).unwrap(),
                128,
            )
            .unwrap();
    }

    let raw = std::fs::read_to_string(&path).unwrap();
    let mut payload: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let records = payload
        .get_mut("subscription_support_maintenance_debt_records")
        .and_then(serde_json::Value::as_object_mut)
        .expect("support maintenance debt records should persist");
    let first_record = records
        .values_mut()
        .next()
        .expect("one debt record should persist");
    first_record["verdict"] = serde_json::Value::String(String::from("ExactResumePreserved"));
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let error = WORTHStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("drifted maintenance debt report should fail reopen");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportPublicationViolation
    );
}
