use crate::{
    backend::records::{MaintenanceExecutionRecord, StoreState},
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy, ForgeStore, ForgeStoreBuilder,
    MaintenanceExecutionStatus, PinnedSnapshotPolicy, RetentionPolicyClass,
    SingleEntityAspectScope, SnapshotCaptureRequest,
};
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{
    EntityMutationIntent, MutationIntent, TransactionOptions, UpdateEntityIntent, WorkerIntentBatch,
};
use serde_json::json;

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::{unique_test_sqlite_path, unique_test_store_path},
};

fn layout_request(
    branch_id: forge_relational::facade::history::BranchId,
    commit_id: forge_relational::facade::history::CommitId,
) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    )
}

fn update_entity_on_branch_with_commit(
    runtime: &mut RelationalRuntime,
    entity_id: forge_relational::facade::identity::EntityId,
    name: &str,
) -> forge_relational::facade::replay::CanonicalCommitEnvelope {
    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(
        WorkerIntentBatch::new(format!("update-{name}")).push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id,
                payload: RecordPayload::StructuredJson(json!({ "name": name })),
            }),
        )),
    );
    let outcome = tx.commit().expect("update commit");
    runtime
        .replay()
        .canonical_commit_envelope(outcome.commit.commit_id)
        .unwrap()
        .clone()
}

fn build_maintenance_ready_store_with_builder(
    builder: ForgeStoreBuilder,
) -> (ForgeStore, crate::MaintenanceBatch) {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = builder.build().unwrap();
    store.append_canonical_commit(initial).unwrap();

    let head = update_entity_on_branch_with_commit(&mut runtime, entity_id, "main-v2");
    store.append_canonical_commit(head.clone()).unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(policy))
        .unwrap();
    (store, batch)
}

fn build_maintenance_ready_store() -> (ForgeStore, crate::MaintenanceBatch) {
    build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new()
            .local_file(unique_test_store_path("forge-store-m10-5-maintenance")),
    )
}

fn force_local_file_started(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let execution = state
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance execution record should exist");
    execution.execution_status = MaintenanceExecutionStatus::Started;
    execution.last_completed_phase = Some("started".to_string());
    execution.resume_count = 1;
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("started maintenance state should write");
}

fn force_sqlite_started(path: &std::path::Path, declaration_id: &crate::MaintenanceDeclarationId) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    let payload_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_execution_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance execution payload should exist");
    let mut record: MaintenanceExecutionRecord =
        serde_json::from_str(&payload_json).expect("maintenance execution payload should decode");
    record.execution_status = MaintenanceExecutionStatus::Started;
    record.last_completed_phase = Some("started".to_string());
    record.resume_count = 1;
    connection
        .execute(
            "UPDATE maintenance_execution_records SET payload_json = ?1 WHERE artifact_id = ?2",
            [
                serde_json::to_string(&record)
                    .expect("maintenance execution payload should encode"),
                declaration_id.as_str().to_string(),
            ],
        )
        .expect("sqlite maintenance execution payload should update");
}

#[test]
fn retention_maintenance_batch_lowers_and_admits_durably() {
    let (mut store, batch) = build_maintenance_ready_store();

    assert!(!batch.declarations().is_empty());
    assert_eq!(batch.batch_class(), crate::MaintenanceBatchClass::Retention);
    let receipt = store.admit_maintenance_batch(batch.clone()).unwrap();
    assert!(!receipt.admitted_declarations().is_empty());
    assert_eq!(
        receipt.batch_summary().declaration_count(),
        receipt.admitted_declarations().len() as u64
    );

    let status = store
        .maintenance_status(receipt.admitted_declarations()[0].declaration().id())
        .unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Admitted
    );
    let report = store.milestone_10_5_maintenance_report();
    assert_eq!(report.declared_batch_count, 1);
    assert_eq!(
        report.persisted_declaration_count,
        receipt.admitted_declarations().len() as u64
    );
    let counters = store.milestone_10_5_counter_contract();
    assert_eq!(
        counters.maintenance_admission_count,
        receipt.admitted_declarations().len() as u64
    );
}

#[test]
fn admitted_maintenance_declarations_execute_and_persist_status() {
    let (mut store, batch) = build_maintenance_ready_store();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let compaction = receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("compaction declaration")
        .clone();

    let completed = store.start_maintenance_declaration(&compaction).unwrap();
    assert_eq!(completed.last_completed_phase(), "compaction_cutover");
    let status = store
        .maintenance_status(compaction.declaration().id())
        .unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    let counters = store.milestone_10_5_counter_contract();
    assert_eq!(counters.maintenance_resume_count, 0);
    assert_eq!(counters.maintenance_completion_count, 1);
    assert_eq!(counters.maintenance_checkpoint_count, 2);
}

#[test]
fn rebuild_declaration_executes_against_target_specific_debt() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(envelope.clone()).unwrap();
    let request = layout_request(envelope.branch_context.clone(), envelope.commit.commit_id);
    let materialization = store
        .materialize_milestone_6_layout_support(request.clone())
        .unwrap();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        Vec::new(),
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );

    let initial_batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(policy.clone()))
        .unwrap();
    let initial_receipt = store.admit_maintenance_batch(initial_batch).unwrap();
    let reclaim = initial_receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Reclaim { .. }
            )
        })
        .expect("reclaim declaration")
        .clone();
    let rebuild_declaration = initial_receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Rebuild { .. }
            )
        })
        .expect("rebuild declaration")
        .declaration();
    let rebuild_id = rebuild_declaration.id().clone();
    let expected_debt_link = format!(
        "rebuild-debt:{}:{}:{}",
        "milestone_6_layout_materialization",
        format!(
            "branch:{}@{}",
            envelope.branch_context.0, envelope.commit.commit_id.0
        ),
        materialization.artifact_id()
    );

    match rebuild_declaration {
        crate::MaintenanceDeclaration::Rebuild { declaration, .. } => {
            assert_eq!(
                declaration.rebuild_target_id(),
                materialization.artifact_id()
            );
            assert_eq!(
                declaration.debt_link_artifact_id(),
                Some(expected_debt_link.as_str())
            );
        }
        _ => unreachable!("selected declaration should be a rebuild"),
    }

    let deferred = store
        .start_maintenance_declaration(
            initial_receipt
                .admitted_declarations()
                .iter()
                .find(|declaration| declaration.declaration().id() == &rebuild_id)
                .expect("rebuild admitted declaration"),
        )
        .unwrap_err();
    assert_eq!(deferred.error_kind(), "ReclaimEligibilityViolation");
    assert_eq!(
        store
            .maintenance_status(&rebuild_id)
            .unwrap()
            .execution_status(),
        MaintenanceExecutionStatus::Admitted
    );

    let reclaim_completed = store.start_maintenance_declaration(&reclaim).unwrap();
    assert_eq!(reclaim_completed.last_completed_phase(), "derived_reclaim");

    let rebuild = store
        .start_maintenance_declaration(
            initial_receipt
                .admitted_declarations()
                .iter()
                .find(|declaration| declaration.declaration().id() == &rebuild_id)
                .expect("rebuild admitted declaration"),
        )
        .unwrap();
    assert_eq!(rebuild.last_completed_phase(), "rebuild");
    assert_eq!(
        store
            .maintenance_status(&rebuild_id)
            .unwrap()
            .execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    let counters = store.milestone_10_5_counter_contract();
    assert!(counters.maintenance_debt_link_count >= 1);
    assert_eq!(
        store
            .fetch_milestone_6_layout_support(request)
            .unwrap()
            .artifact_id(),
        materialization.artifact_id()
    );
}

#[test]
fn maintenance_status_survives_restart() {
    let path = unique_test_store_path("forge-store-m10-5-maintenance-restart");
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let initial = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new()
        .local_file(path.clone())
        .build()
        .unwrap();
    store.append_canonical_commit(initial).unwrap();
    let head = update_entity_on_branch_with_commit(&mut runtime, entity_id, "main-v2");
    store.append_canonical_commit(head.clone()).unwrap();
    store
        .materialize_milestone_6_layout_support(layout_request(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            head.branch_context.clone(),
            head.commit.commit_id,
        ))
        .unwrap();
    let policy = ConservativeRetentionPolicy::new(
        Vec::new(),
        vec![PinnedSnapshotPolicy::new(snapshot.snapshot_id)],
        vec![DerivedFamilyRetentionPolicy::Milestone6LayoutMaterialization],
    );
    let batch = store
        .plan_retention_maintenance_batch(RetentionPolicyClass::Conservative(policy))
        .unwrap();
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let root_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let status = reopened.maintenance_status(&root_id).unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Admitted
    );
    assert_eq!(
        reopened
            .milestone_10_5_maintenance_report()
            .declared_batch_count,
        1
    );
}

#[test]
fn maintenance_status_survives_sqlite_restart() {
    let path = unique_test_sqlite_path("forge-store-m10-5-maintenance-sqlite-restart");
    let (mut store, batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(path.clone()),
    );
    let receipt = store.admit_maintenance_batch(batch).unwrap();
    let root_id = receipt.admitted_declarations()[0]
        .declaration()
        .id()
        .clone();
    drop(store);

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let status = reopened.maintenance_status(&root_id).unwrap();
    assert_eq!(
        status.execution_status(),
        MaintenanceExecutionStatus::Admitted
    );
    let report = reopened.milestone_10_5_maintenance_report();
    assert_eq!(report.declared_batch_count, 1);
    assert_eq!(
        report.persisted_declaration_count,
        receipt.admitted_declarations().len() as u64
    );
}

#[test]
fn started_maintenance_can_resume_after_restart_in_both_durable_lanes() {
    let local_path = unique_test_store_path("forge-store-m10-5-maintenance-resume-local");
    let (mut local_store, local_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().local_file(local_path.clone()),
    );
    let local_receipt = local_store.admit_maintenance_batch(local_batch).unwrap();
    let local_compaction = local_receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("local compaction declaration")
        .declaration()
        .id()
        .clone();
    drop(local_store);
    force_local_file_started(&local_path, &local_compaction);

    let mut reopened_local = ForgeStoreBuilder::new()
        .local_file(local_path)
        .build()
        .unwrap();
    let completed_local = reopened_local
        .resume_maintenance_declaration(&local_compaction)
        .unwrap();
    assert_eq!(completed_local.last_completed_phase(), "compaction_cutover");
    assert_eq!(
        reopened_local
            .maintenance_status(&local_compaction)
            .unwrap()
            .execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        reopened_local
            .milestone_10_5_counter_contract()
            .maintenance_resume_count,
        1
    );

    let sqlite_path = unique_test_sqlite_path("forge-store-m10-5-maintenance-resume-sqlite");
    let (mut sqlite_store, sqlite_batch) = build_maintenance_ready_store_with_builder(
        ForgeStoreBuilder::new().sqlite_file(sqlite_path.clone()),
    );
    let sqlite_receipt = sqlite_store.admit_maintenance_batch(sqlite_batch).unwrap();
    let sqlite_compaction = sqlite_receipt
        .admitted_declarations()
        .iter()
        .find(|declaration| {
            matches!(
                declaration.declaration(),
                crate::MaintenanceDeclaration::Compaction { .. }
            )
        })
        .expect("sqlite compaction declaration")
        .declaration()
        .id()
        .clone();
    drop(sqlite_store);
    force_sqlite_started(&sqlite_path, &sqlite_compaction);

    let mut reopened_sqlite = ForgeStoreBuilder::new()
        .sqlite_file(sqlite_path)
        .build()
        .unwrap();
    let completed_sqlite = reopened_sqlite
        .resume_maintenance_declaration(&sqlite_compaction)
        .unwrap();
    assert_eq!(
        completed_sqlite.last_completed_phase(),
        "compaction_cutover"
    );
    assert_eq!(
        reopened_sqlite
            .maintenance_status(&sqlite_compaction)
            .unwrap()
            .execution_status(),
        MaintenanceExecutionStatus::Completed
    );
    assert_eq!(
        reopened_sqlite
            .milestone_10_5_counter_contract()
            .maintenance_resume_count,
        1
    );
}
