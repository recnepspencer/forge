use crate::{
    backend::records::{MaintenanceDeclarationRecord, MaintenanceExecutionRecord, StoreState},
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy, ForgeStore, ForgeStoreBuilder,
    MaintenanceEscalationDecision, MaintenanceExecutionStatus, PinnedSnapshotPolicy,
    RetentionPolicyClass,
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
            .local_file(unique_test_store_path("forge-store-m11-maintenance")),
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

fn force_local_file_reserved(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    plan_family: crate::MaintenancePlanFamily,
    quantum_units: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let execution = state
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance execution record should exist");
    execution.execution_status = MaintenanceExecutionStatus::Reserved;
    execution.plan_family = Some(plan_family);
    execution.last_quantum_units = Some(quantum_units);
    execution.reservation_transition = Some(crate::MaintenanceReservationTransition::new(
        plan_family,
        quantum_units,
    ));
    execution.last_completed_phase = Some("reserved".to_string());
    if matches!(plan_family, crate::MaintenancePlanFamily::Escalated) {
        execution.foreground_impact = crate::MaintenanceForegroundImpact::escalated();
    }
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("reserved maintenance state should write");
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

fn force_sqlite_reserved(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    plan_family: crate::MaintenancePlanFamily,
    quantum_units: u64,
) {
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
    record.execution_status = MaintenanceExecutionStatus::Reserved;
    record.plan_family = Some(plan_family);
    record.last_quantum_units = Some(quantum_units);
    record.reservation_transition = Some(crate::MaintenanceReservationTransition::new(
        plan_family,
        quantum_units,
    ));
    record.last_completed_phase = Some("reserved".to_string());
    if matches!(plan_family, crate::MaintenancePlanFamily::Escalated) {
        record.foreground_impact = crate::MaintenanceForegroundImpact::escalated();
    }
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

fn force_local_file_deferred(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_escalation_decision(MaintenanceEscalationDecision::DeferWithOperatorSignal);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("deferred maintenance state should write");
}

fn force_sqlite_deferred(path: &std::path::Path, declaration_id: &crate::MaintenanceDeclarationId) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_escalation_decision(MaintenanceEscalationDecision::DeferWithOperatorSignal);
    });
}

fn force_local_file_escalated(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_escalation_decision(MaintenanceEscalationDecision::EscalateWithForegroundImpact);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("escalated maintenance state should write");
}

fn force_sqlite_escalated(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_escalation_decision(MaintenanceEscalationDecision::EscalateWithForegroundImpact);
    });
}

fn force_local_file_recovered(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_recovered_from_restart(true);
    let execution = state
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance execution record should exist");
    execution.restart_readmission_status =
        Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("recovered maintenance state should write");
}

fn force_sqlite_recovered(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_recovered_from_restart(true);
    });
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
    record.restart_readmission_status =
        Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission);
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

fn force_local_file_cancelled(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor = declaration
        .work_descriptor
        .clone()
        .with_freshness_window(crate::FreshnessWindow::new(0));
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("cancelled maintenance state should write");
}

fn force_sqlite_cancelled(path: &std::path::Path, declaration_id: &crate::MaintenanceDeclarationId) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_freshness_window(crate::FreshnessWindow::new(0));
    });
}

fn mutate_sqlite_declaration<F>(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    mut mutate: F,
) where
    F: FnMut(&mut MaintenanceDeclarationRecord),
{
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    let payload_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_declaration_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance declaration payload should exist");
    let mut record: MaintenanceDeclarationRecord =
        serde_json::from_str(&payload_json).expect("maintenance declaration payload should decode");
    mutate(&mut record);
    connection
        .execute(
            "UPDATE maintenance_declaration_records SET payload_json = ?1 WHERE artifact_id = ?2",
            [
                serde_json::to_string(&record)
                    .expect("maintenance declaration payload should encode"),
                declaration_id.as_str().to_string(),
            ],
        )
        .expect("sqlite maintenance declaration payload should update");
}


#[path = "milestone_11_maintenance/admission.rs"]
mod admission;
#[path = "milestone_11_maintenance/rebuild.rs"]
mod rebuild;
#[path = "milestone_11_maintenance/plan_transitions.rs"]
mod plan_transitions;
#[path = "milestone_11_maintenance/restart_status.rs"]
mod restart_status;
#[path = "milestone_11_maintenance/resume.rs"]
mod resume;
