use super::BranchDeltaLayerId;
use worth_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn stable_branch_delta_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("branch delta digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn stable_shared_base_authority_digest(
    source_branch_id: &BranchId,
    source_frontier_commit_id: Option<CommitId>,
    canonicalization_version: u32,
) -> String {
    stable_branch_delta_digest(&(
        source_branch_id,
        source_frontier_commit_id,
        canonicalization_version,
    ))
}

pub fn stable_branch_delta_layer_authority_digest(
    branch_id: &BranchId,
    base_frontier_commit_id: Option<CommitId>,
    target_frontier_commit_id: CommitId,
    commit_ids: &[CommitId],
    canonicalization_version: u32,
) -> String {
    let _: Option<BranchDeltaLayerId> = None;
    stable_branch_delta_digest(&(
        branch_id,
        base_frontier_commit_id,
        target_frontier_commit_id,
        commit_ids,
        canonicalization_version,
    ))
}
