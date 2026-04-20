use crate::{
    backend::records::StoreState, BranchDeltaAutoCompactDisposition, BranchDeltaFallbackClass,
    BranchDeltaReadRequest, BranchDeltaReadStrategy, BranchDeltaRewritePolicyDecision,
    BranchDeltaRewriteRequest, BranchDeltaRewriteStrategy, ComplexityStatus, ForgeStoreBuilder,
    SharedBaseBranchCreationRequest, SnapshotCaptureRequest, SnapshotReadRequest, StoreErrorKind,
    MAX_DIRECT_LAYER_READ_DEPTH, MAX_REWRITE_LAYER_WIDTH, RECOMMENDED_REWRITE_LAYER_WIDTH,
};
use forge_relational::facade::history::{BranchId, CommitId};

use super::harness::{
    corruption::local_file::{
        force_branch_delta_artifact_commit_mismatch, force_branch_delta_replacement_gap,
        force_branch_delta_replacement_proof_length_drift,
        force_branch_delta_replacement_proof_mismatch,
        force_branch_delta_replacement_self_reference, force_clear_branch_delta_layer_artifacts,
        force_remove_first_branch_delta_layer,
    },
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};

#[test]
fn append_delta_publication_admits_first_layer_for_empty_base_branch() {
    let mut state = StoreState::default();
    let layer_id = state.publish_branch_delta_layer_for_append(
        BranchId("feature-empty".to_string()),
        None,
        CommitId(41),
        vec![CommitId(41)],
    );

    assert_eq!(layer_id, Some(1));
    let record = state
        .branch_delta_layer_records
        .get(&1)
        .expect("first empty-base delta layer should publish");
    assert_eq!(record.base_frontier_commit_id, None);
    assert_eq!(record.commit_ids, vec![CommitId(41)]);
    assert!(record.replacement_lineage_proof.is_empty());
}

fn admitted_branch_delta_read(
    store: &crate::ForgeStore,
    branch_id: BranchId,
    target_commit_id: CommitId,
) -> crate::BranchDeltaReadResult {
    let witness = store
        .admit_same_branch_descendant(BranchDeltaReadRequest::new(branch_id, target_commit_id))
        .unwrap();
    store.read_branch_delta(witness).unwrap()
}

#[path = "delta/shared_base.rs"]
mod shared_base;
#[path = "delta/direct_read.rs"]
mod direct_read;
#[path = "delta/budget_and_rewrite.rs"]
mod budget_and_rewrite;
#[path = "delta/auto_compact.rs"]
mod auto_compact;
#[path = "delta/rewrite_execution.rs"]
mod rewrite_execution;
#[path = "delta/persistence.rs"]
mod persistence;
#[path = "delta/local_reopen.rs"]
mod local_reopen;
#[path = "delta/sqlite_reopen.rs"]
mod sqlite_reopen;
