use std::path::PathBuf;

use crate::{
    AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet, AspectScopeClass,
    ConservativeRetentionPolicy, ContinuationRetentionStatus, DerivedFamilyRetentionPolicy,
    ForgeStore, ForgeStoreBuilder, PinnedSnapshotPolicy, RetentionPolicyClass,
    SingleEntityAspectScope, SnapshotCaptureRequest, SnapshotReadRequest, StableBasisLayoutPosture,
    StableBasisReadRequest, StableBasisReadScope, StoreErrorKind,
};
use forge_relational::facade::history::{BranchId, CommitId};
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::{
    EntityMutationIntent, MutationIntent, TransactionOptions, UpdateEntityIntent, WorkerIntentBatch,
};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::{unique_test_sqlite_path, unique_test_store_path},
};

#[derive(Clone)]
enum DurableLaneCase {
    LocalFile(PathBuf),
    Sqlite(PathBuf),
}

impl DurableLaneCase {
    fn label(&self) -> &'static str {
        match self {
            Self::LocalFile(_) => "local_file",
            Self::Sqlite(_) => "sqlite",
        }
    }

    fn build(&self) -> ForgeStore {
        match self {
            Self::LocalFile(path) => ForgeStoreBuilder::new()
                .local_file(path.clone())
                .build()
                .unwrap(),
            Self::Sqlite(path) => ForgeStoreBuilder::new()
                .sqlite_file(path.clone())
                .build()
                .unwrap(),
        }
    }
}

fn stable_digest<T: Serialize>(value: &T) -> String {
    let normalized = serde_json::to_value(value).expect("retention phase 11 normalization");
    let json = serde_json::to_vec(&normalized).expect("retention phase 11 serialization");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}

fn stable_basis_request(
    store: &ForgeStore,
    branch_id: BranchId,
    commit_id: CommitId,
) -> StableBasisReadRequest {
    let export = store.export_authoritative_records().into_canonicalized();
    let support_summary = export
        .commit_support_summaries
        .iter()
        .find(|summary| summary.commit_id == commit_id)
        .expect("retention stress stable-basis fixture requires a commit support summary")
        .clone();
    let commit = export
        .commit_envelopes
        .iter()
        .find(|envelope| envelope.envelope.commit.commit_id == commit_id)
        .expect("retention stress stable-basis fixture requires a frontier commit");
    StableBasisReadRequest::new(
        branch_id,
        commit_id,
        StableBasisReadScope::SingleEntity(SingleEntityAspectScope::new("entity-alpha")),
        stable_digest(&support_summary),
        support_summary
            .schema_support_artifact_id
            .clone()
            .unwrap_or_else(|| "schema-support:stress:v1".to_string()),
        StableBasisLayoutPosture::ProofOnly,
        commit.envelope_digest.clone(),
        ContinuationRetentionStatus::Retained,
    )
}

fn layout_request(branch_id: BranchId, commit_id: CommitId) -> AspectLayoutReadRequest {
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
    target_branch: Option<BranchId>,
) -> forge_relational::facade::replay::CanonicalCommitEnvelope {
    let mut tx = runtime.begin_transaction(TransactionOptions {
        target_branch,
        ..TransactionOptions::default()
    });
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

#[path = "retention_phase11/matrix.rs"]
mod matrix;
