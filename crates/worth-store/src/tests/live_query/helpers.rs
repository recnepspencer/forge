use crate::{
    ContinuationBatchBudget, ContinuationBatchResult, ContinuationRetentionDescriptor,
    ContinuationRetentionStatus, CursorContinuationRequest, FetchWidth, WORTHStore, MaxBatchItems,
    MaxCoveredCommits, MaxMaterializedBytes, MaxSupportRowsPerBatch, StableBasisLayoutPosture,
    StableBasisReadRequest, StableBasisReadScope,
};
use worth_relational::facade::history::CommitId;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub(super) use super::super::harness::fixtures::runtime::{
    create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
};
pub(super) use super::super::harness::fixtures::stores::unique_test_sqlite_path;

pub(crate) fn demo_budget() -> ContinuationBatchBudget {
    ContinuationBatchBudget::new(
        FetchWidth::new(16),
        MaxBatchItems::new(32),
        MaxCoveredCommits::new(4),
        MaxMaterializedBytes::new(4_096),
        MaxSupportRowsPerBatch::new(24),
    )
}

pub(super) fn stable_digest<T: Serialize>(value: &T) -> String {
    let normalized = serde_json::to_value(value).expect("live-query test evidence normalization");
    let json = serde_json::to_vec(&normalized).expect("live-query test evidence serialization");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}

pub(super) fn stable_basis_request(
    branch_id: worth_relational::facade::history::BranchId,
    commit_id: CommitId,
    schema_boundary_artifact_id: impl Into<String>,
    support_context_digest: impl Into<String>,
    retention_status: ContinuationRetentionStatus,
) -> StableBasisReadRequest {
    StableBasisReadRequest::new(
        branch_id,
        commit_id,
        StableBasisReadScope::SingleEntity(crate::SingleEntityAspectScope::new("entity-alpha")),
        support_context_digest,
        schema_boundary_artifact_id,
        StableBasisLayoutPosture::ProofOnly,
        "authority:basis:v1",
        retention_status,
    )
}

pub(crate) fn stable_basis_request_for_store(
    store: &WORTHStore,
    branch_id: worth_relational::facade::history::BranchId,
    commit_id: CommitId,
    schema_boundary_artifact_id: impl Into<String>,
    retention_status: ContinuationRetentionStatus,
) -> StableBasisReadRequest {
    let export = store.export_authoritative_records().into_canonicalized();
    let support_summary = export
        .commit_support_summaries
        .iter()
        .find(|summary| summary.commit_id == commit_id)
        .expect("stable-basis test fixture requires a commit support summary")
        .clone();
    let commit = export
        .commit_envelopes
        .iter()
        .find(|envelope| envelope.envelope.commit.commit_id == commit_id)
        .expect("stable-basis test fixture requires a canonical frontier commit");
    StableBasisReadRequest::new(
        branch_id,
        commit_id,
        StableBasisReadScope::SingleEntity(crate::SingleEntityAspectScope::new("entity-alpha")),
        stable_digest(&support_summary),
        schema_boundary_artifact_id,
        StableBasisLayoutPosture::ProofOnly,
        commit.envelope_digest.clone(),
        retention_status,
    )
}

pub(super) fn uniform_scope_basis_request(
    store: &WORTHStore,
    branch_id: worth_relational::facade::history::BranchId,
    commit_id: CommitId,
    retention_status: ContinuationRetentionStatus,
) -> StableBasisReadRequest {
    let export = store.export_authoritative_records().into_canonicalized();
    let support_summary = export
        .commit_support_summaries
        .iter()
        .find(|summary| summary.commit_id == commit_id)
        .expect("uniform-scope stable-basis test fixture requires a commit support summary")
        .clone();
    let commit = export
        .commit_envelopes
        .iter()
        .find(|envelope| envelope.envelope.commit.commit_id == commit_id)
        .expect("uniform-scope stable-basis test fixture requires a canonical frontier commit");
    StableBasisReadRequest::new(
        branch_id,
        commit_id,
        StableBasisReadScope::UniformEntitySet(crate::EntitySetUniformAspectScope::new(vec![
            "entity-alpha".to_string(),
            "entity-beta".to_string(),
        ])),
        stable_digest(&support_summary),
        "schema-support:v1",
        StableBasisLayoutPosture::ProofOnly,
        commit.envelope_digest.clone(),
        retention_status,
    )
}

pub(super) fn retention_descriptor(
    stable_basis_id: crate::StableBasisId,
    commit_id: CommitId,
) -> ContinuationRetentionDescriptor {
    ContinuationRetentionDescriptor::new(
        stable_basis_id,
        commit_id,
        vec![
            "schema-support:v1".to_string(),
            "lineage-support:v1".to_string(),
        ],
        "schema-support:v1",
        "authority_replay",
        "snapshot_tail",
        1,
    )
}

pub(super) fn planned_basis_handle(
    store: &WORTHStore,
    branch_id: worth_relational::facade::history::BranchId,
    commit_id: CommitId,
    retention_status: ContinuationRetentionStatus,
) -> crate::StableBasisHandle {
    let plan = store
        .plan_stable_basis_read(stable_basis_request_for_store(
            store,
            branch_id,
            commit_id,
            "schema-support:v1",
            retention_status,
        ))
        .unwrap();
    let descriptor = retention_descriptor(plan.stable_basis_id().clone(), commit_id);
    plan.into_handle(descriptor)
}

pub(super) fn append_latest_commit(
    store: &mut WORTHStore,
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
) -> worth_relational::facade::replay::CanonicalCommitEnvelope {
    let envelope = latest_envelope(runtime);
    store.append_canonical_commit(envelope.clone()).unwrap();
    envelope
}

pub(super) fn run_admitted_continuation_session(
    store: &mut WORTHStore,
    branch_id: worth_relational::facade::history::BranchId,
    latest_commit_id: CommitId,
    basis: crate::StableBasisHandle,
    fetch_width: u32,
) -> (Vec<ContinuationBatchResult>, CommitId) {
    let mut results = Vec::new();

    loop {
        let plan = store
            .plan_cursor_continuation(CursorContinuationRequest::new(
                "cursor-main",
                "subscriber-a",
                branch_id.clone(),
                "demo-feed",
                "schema:v1",
                1,
                basis.clone(),
                ContinuationBatchBudget::new(
                    FetchWidth::new(fetch_width),
                    MaxBatchItems::new(8),
                    MaxCoveredCommits::new(8),
                    MaxMaterializedBytes::new(4_096),
                    MaxSupportRowsPerBatch::new(24),
                ),
            ))
            .unwrap();
        let result = store.execute_cursor_continuation(plan).unwrap();
        match result.clone() {
            ContinuationBatchResult::AdmittedNarrow(receipt) => {
                store
                    .acknowledge_cursor_continuation(receipt.into_advance_receipt())
                    .unwrap();
                results.push(result);
            }
            ContinuationBatchResult::CaughtUp(_) => break,
            other => panic!("unexpected continuation result in admitted session: {other:?}"),
        }
    }

    let resumed = store
        .plan_cursor_resume(crate::DurableCursorResumeRequest::new(
            "cursor-main",
            "subscriber-a",
            branch_id,
            "demo-feed",
            "schema:v1",
            1,
        ))
        .unwrap();
    assert_eq!(
        resumed.latest_checkpoint().basis_commit_id,
        latest_commit_id
    );
    (results, resumed.latest_checkpoint().basis_commit_id)
}
