use crate::{
    backend::records::{MaintenanceDeclarationRecord, MaintenanceExecutionRecord, StoreState},
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, CpuBudgetUnits, DerivedFamilyRetentionPolicy,
    ForegroundLatencyGuard, WORTHStore, WORTHStoreBuilder, IoBudgetUnits, MaintenanceBatch,
    MaintenanceBatchClass, MaintenanceDeclaration, MaintenanceDeclarationId,
    MaintenanceDescriptorDemand, MaintenanceEscalationDecision, MaintenanceExecutionStatus,
    MaintenanceLocalityScope, MemoryBudgetUnits, PinnedSnapshotPolicy, PublicationSlotBudget,
    RetentionPolicyClass, SingleEntityAspectScope, SnapshotCaptureRequest,
};
use worth_relational::facade::payloads::RecordPayload;
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::transactions::{
    EntityMutationIntent, MutationIntent, TransactionOptions, UpdateEntityIntent, WorkerIntentBatch,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::{unique_test_sqlite_path, unique_test_store_path},
};

fn layout_request(
    branch_id: worth_relational::facade::history::BranchId,
    commit_id: worth_relational::facade::history::CommitId,
) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    )
}

fn update_entity_on_branch_with_commit(
    runtime: &mut RelationalRuntime,
    entity_id: worth_relational::facade::identity::EntityId,
    name: &str,
) -> worth_relational::facade::replay::CanonicalCommitEnvelope {
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
    builder: WORTHStoreBuilder,
) -> (WORTHStore, crate::MaintenanceBatch) {
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

fn build_maintenance_ready_store() -> (WORTHStore, crate::MaintenanceBatch) {
    build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(unique_test_store_path("worth-store-m11-maintenance")),
    )
}

fn stable_digest<T: serde::Serialize>(value: &T) -> String {
    let normalized =
        serde_json::to_value(value).expect("maintenance test evidence normalization should work");
    let json = serde_json::to_vec(&normalized)
        .expect("maintenance test evidence serialization should work");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}

fn stable_basis_request_for_store(
    store: &WORTHStore,
    branch_id: worth_relational::facade::history::BranchId,
    commit_id: worth_relational::facade::history::CommitId,
) -> crate::StableBasisReadRequest {
    let export = store.export_authoritative_records().into_canonicalized();
    let support_summary = export
        .commit_support_summaries
        .iter()
        .find(|summary| summary.branch_id == branch_id && summary.commit_id == commit_id)
        .expect("stable-basis maintenance fixture requires a commit support summary")
        .clone();
    let commit = export
        .commit_envelopes
        .iter()
        .find(|envelope| envelope.envelope.commit.commit_id == commit_id)
        .expect("stable-basis maintenance fixture requires a canonical commit");
    crate::StableBasisReadRequest::new(
        branch_id,
        commit_id,
        crate::StableBasisReadScope::SingleEntity(crate::SingleEntityAspectScope::new(
            "entity-alpha",
        )),
        stable_digest(&support_summary),
        "schema-support:v1",
        crate::StableBasisLayoutPosture::ProofOnly,
        commit.envelope_digest.clone(),
        crate::ContinuationRetentionStatus::Retained,
    )
}

fn duplicate_compaction_batch(batch: &MaintenanceBatch, duplicate_id: &str) -> MaintenanceBatch {
    let mut declarations = batch.declarations().to_vec();
    let duplicate = declarations
        .iter()
        .find_map(|declaration| match declaration {
            MaintenanceDeclaration::Compaction { declaration, .. } => {
                Some(MaintenanceDeclaration::compaction(
                    MaintenanceDeclarationId::new(duplicate_id.to_string()),
                    declaration.clone(),
                ))
            }
            _ => None,
        })
        .expect("compaction declaration should exist");
    declarations.push(duplicate);
    MaintenanceBatch::new(
        format!("{}-duplicate", batch.batch_id()),
        MaintenanceBatchClass::Retention,
        declarations,
    )
}

fn same_lane_distinct_compaction_batch(
    batch: &MaintenanceBatch,
    duplicate_id: &str,
) -> MaintenanceBatch {
    let mut declarations = batch.declarations().to_vec();
    let duplicate = declarations
        .iter()
        .find_map(|declaration| match declaration {
            MaintenanceDeclaration::Compaction { declaration, .. } => {
                Some(MaintenanceDeclaration::compaction(
                    MaintenanceDeclarationId::new(duplicate_id.to_string()),
                    crate::CompactionMaintenanceDeclaration::new(
                        declaration.retained_basis_label().to_string(),
                        declaration.retained_head_branch_ids().to_vec(),
                        declaration.stable_basis_labels().to_vec(),
                        declaration.closure_commit_ids().to_vec(),
                        declaration.frontier_commit_ids().to_vec(),
                        declaration.family_labels().to_vec(),
                        declaration.superseded_families().to_vec(),
                        declaration.rewritten_range_count() + 1,
                    ),
                ))
            }
            _ => None,
        })
        .expect("compaction declaration should exist");
    declarations.push(duplicate);
    MaintenanceBatch::new(
        format!("{}-same-lane-distinct", batch.batch_id()),
        MaintenanceBatchClass::Retention,
        declarations,
    )
}

fn tier_placement_batch(batch_id: &str, declaration_id: &str) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::tier_placement_proposal(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::TierPlacementMaintenanceDeclaration::new(
                "snapshot_family",
                "family:tier-local",
                "proposal:conservative-cold",
            ),
        )],
    )
}

fn snapshot_refresh_batch(batch_id: &str, declaration_id: &str) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::snapshot_refresh(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::SnapshotRefreshMaintenanceDeclaration::new(
                "snapshot_family",
                "family:snapshot-local",
                "refresh:publication-support",
            ),
        )],
    )
}

fn derived_family_rebuild_batch(batch_id: &str, declaration_id: &str) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::derived_family_rebuild(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::DerivedFamilyRebuildMaintenanceDeclaration::new(
                "basis:derived-rebuild",
                "family:derived-local",
                "rebuild:derived-index",
            ),
        )],
    )
}

fn replication_preparation_batch(batch_id: &str, declaration_id: &str) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::replication_preparation(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::ReplicationPreparationMaintenanceDeclaration::new(
                "replication_family",
                "family:replication-local",
                "prepare:capsule-handoff",
            ),
        )],
    )
}

fn maintenance_audit_batch(batch_id: &str, declaration_id: &str) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::maintenance_audit(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::MaintenanceAuditMaintenanceDeclaration::new(
                "audit_family",
                "family:audit-local",
                "audit:queue-summary-parity",
            ),
        )],
    )
}

fn tier_move_batch(
    batch_id: &str,
    declaration_id: &str,
    cross_locality_debt: bool,
) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::tier_move_execution(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::TierMoveMaintenanceDeclaration::new(
                "snapshot_family",
                "family:tier-local",
                "move:cold-placement",
                cross_locality_debt,
            ),
        )],
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
    execution
        .plan_family
        .get_or_insert(crate::MaintenancePlanFamily::BackgroundPaced);
    execution.last_quantum_units.get_or_insert(1);
    execution
        .resource_budget_grant
        .get_or_insert_with(|| fixture_budget_grant(1));
    execution.last_completed_phase = Some("started".to_string());
    execution.resume_count = 1;
    clear_state_scheduler_summaries(&mut state);
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
    execution.resource_budget_grant = Some(fixture_budget_grant(quantum_units));
    execution.reservation_transition = Some(crate::MaintenanceReservationTransition::new(
        plan_family,
        quantum_units,
    ));
    execution.last_completed_phase = Some("reserved".to_string());
    if matches!(plan_family, crate::MaintenancePlanFamily::Escalated) {
        execution.foreground_impact = crate::MaintenanceForegroundImpact::escalated();
    }
    clear_state_scheduler_summaries(&mut state);
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
    record
        .plan_family
        .get_or_insert(crate::MaintenancePlanFamily::BackgroundPaced);
    record.last_quantum_units.get_or_insert(1);
    record
        .resource_budget_grant
        .get_or_insert_with(|| fixture_budget_grant(1));
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
    clear_sqlite_scheduler_summaries(&connection);
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
    record.resource_budget_grant = Some(fixture_budget_grant(quantum_units));
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
    clear_sqlite_scheduler_summaries(&connection);
    clear_sqlite_scheduler_summaries(&connection);
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
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("deferred maintenance state should write");
}

fn force_local_file_high_demand(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    io: u64,
    cpu: u64,
    memory: u64,
    publication: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    declaration.work_descriptor =
        declaration
            .work_descriptor
            .clone()
            .with_demand(MaintenanceDescriptorDemand::new(
                IoBudgetUnits::new(io),
                CpuBudgetUnits::new(cpu),
                MemoryBudgetUnits::new(memory),
                PublicationSlotBudget::new(publication),
                ForegroundLatencyGuard::new(1),
            ));
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("high demand maintenance state should write");
}

fn force_sqlite_deferred(path: &std::path::Path, declaration_id: &crate::MaintenanceDeclarationId) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_escalation_decision(MaintenanceEscalationDecision::DeferWithOperatorSignal);
    });
}

fn force_sqlite_high_demand(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    io: u64,
    cpu: u64,
    memory: u64,
    publication: u64,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor =
            record
                .work_descriptor
                .clone()
                .with_demand(MaintenanceDescriptorDemand::new(
                    IoBudgetUnits::new(io),
                    CpuBudgetUnits::new(cpu),
                    MemoryBudgetUnits::new(memory),
                    PublicationSlotBudget::new(publication),
                    ForegroundLatencyGuard::new(1),
                ));
    });
}

fn force_local_file_high_latency_guard(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    guard_units: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let declaration = state
        .maintenance_declaration_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance declaration record should exist");
    let demand = declaration.work_descriptor.demand().clone();
    declaration.work_descriptor =
        declaration
            .work_descriptor
            .clone()
            .with_demand(MaintenanceDescriptorDemand::new(
                demand.predicted_io(),
                demand.predicted_cpu(),
                demand.predicted_memory(),
                demand.predicted_publication(),
                ForegroundLatencyGuard::new(guard_units),
            ));
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("high latency guard maintenance state should write");
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
    clear_state_scheduler_summaries(&mut state);
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
    clear_state_scheduler_summaries(&mut state);
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
    clear_sqlite_scheduler_summaries(&connection);
    clear_sqlite_scheduler_summaries(&connection);
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
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("cancelled maintenance state should write");
}

fn force_local_file_supersession_epoch(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
    epoch: u64,
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
        .with_supersession_epoch(crate::SupersessionEpoch::new(epoch));
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("supersession maintenance state should write");
}

fn force_sqlite_cancelled(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_freshness_window(crate::FreshnessWindow::new(0));
    });
}

fn force_local_file_global_scope_escalated(
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
        .with_locality_scope(MaintenanceLocalityScope::StoreGlobalLocalityScope)
        .with_escalation_decision(MaintenanceEscalationDecision::EscalateWithForegroundImpact);
    let execution = state
        .maintenance_execution_records
        .get_mut(declaration_id.as_str())
        .expect("maintenance execution record should exist");
    execution.lane_key = Some(declaration.work_descriptor.lane_key());
    clear_state_scheduler_summaries(&mut state);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("global scope escalated maintenance state should write");
}

fn force_sqlite_global_scope_escalated(
    path: &std::path::Path,
    declaration_id: &crate::MaintenanceDeclarationId,
) {
    mutate_sqlite_declaration(path, declaration_id, |record| {
        record.work_descriptor = record
            .work_descriptor
            .clone()
            .with_locality_scope(MaintenanceLocalityScope::StoreGlobalLocalityScope)
            .with_escalation_decision(MaintenanceEscalationDecision::EscalateWithForegroundImpact);
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
    let declaration_json: String = connection
        .query_row(
            "SELECT payload_json FROM maintenance_declaration_records WHERE artifact_id = ?1",
            [declaration_id.as_str()],
            |row| row.get(0),
        )
        .expect("maintenance declaration payload should exist");
    let declaration: MaintenanceDeclarationRecord = serde_json::from_str(&declaration_json)
        .expect("maintenance declaration payload should decode");
    record.lane_key = Some(declaration.work_descriptor.lane_key());
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
    clear_sqlite_scheduler_summaries(&connection);
}

fn fixture_budget_grant(quantum_units: u64) -> crate::MaintenanceResourceBudgetGrant {
    crate::MaintenanceResourceBudgetGrant::new(
        IoBudgetUnits::new(1),
        CpuBudgetUnits::new(1),
        MemoryBudgetUnits::new(1),
        PublicationSlotBudget::new(1),
        ForegroundLatencyGuard::new(1),
        crate::MaintenanceQuantum::new(quantum_units),
        crate::PacingWindow::new(quantum_units.max(1)),
    )
}

fn clear_state_scheduler_summaries(state: &mut StoreState) {
    state.maintenance_queue_summary_records.clear();
    state.maintenance_locality_summary_records.clear();
    state.maintenance_reservation_summary_records.clear();
    state.maintenance_resource_budget_summary_records.clear();
    state.maintenance_debt_summary_records.clear();
}

fn clear_sqlite_scheduler_summaries(connection: &rusqlite::Connection) {
    connection
        .execute_batch(
            "
            DELETE FROM maintenance_queue_summary_records;
            DELETE FROM maintenance_locality_summary_records;
            DELETE FROM maintenance_reservation_summary_records;
            DELETE FROM maintenance_resource_budget_summary_records;
            DELETE FROM maintenance_debt_summary_records;
            ",
        )
        .expect("sqlite maintenance summary tables should clear");
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
    clear_sqlite_scheduler_summaries(&connection);
}

#[path = "milestone_11_maintenance/admission.rs"]
mod admission;
#[path = "milestone_11_maintenance/foreground.rs"]
mod foreground;
#[path = "milestone_11_maintenance/plan_transitions.rs"]
mod plan_transitions;
#[path = "milestone_11_maintenance/rebuild.rs"]
mod rebuild;
#[path = "milestone_11_maintenance/restart_status.rs"]
mod restart_status;
#[path = "milestone_11_maintenance/resume.rs"]
mod resume;
