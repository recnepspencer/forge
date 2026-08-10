use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy, MaintenanceBatch,
    PinnedSnapshotPolicy, RetentionPolicyClass, SingleEntityAspectScope, SnapshotCaptureRequest,
    WORTHStore, WORTHStoreBuilder,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use worth_relational::facade::payloads::RecordPayload;
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::transactions::{
    EntityMutationIntent, MutationIntent, TransactionOptions, UpdateEntityIntent, WorkerIntentBatch,
};

use super::super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::unique_test_store_path,
};

pub(super) fn layout_request(
    branch_id: worth_relational::facade::history::BranchId,
    commit_id: worth_relational::facade::history::CommitId,
) -> AspectLayoutReadRequest {
    AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(branch_id, commit_id),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    )
}

pub(super) fn update_entity_on_branch_with_commit(
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

pub(super) fn build_maintenance_ready_store_with_builder(
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

pub(super) fn build_maintenance_ready_store() -> (WORTHStore, crate::MaintenanceBatch) {
    build_maintenance_ready_store_with_builder(
        WORTHStoreBuilder::new().local_file(unique_test_store_path("worth-store-m11-maintenance")),
    )
}

pub(super) fn stable_digest<T: serde::Serialize>(value: &T) -> String {
    let normalized =
        serde_json::to_value(value).expect("maintenance test evidence normalization should work");
    let json = serde_json::to_vec(&normalized)
        .expect("maintenance test evidence serialization should work");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}

pub(super) fn stable_basis_request_for_store(
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
