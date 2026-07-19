use sha2::{Digest, Sha256};
use worth_runtime_bridge::facade::{
    RelationalBridgeSnapshotIdentityParts, TruthBranchIdentity, TruthCommitIdentity,
    TruthSnapshotIdentity,
};

pub(crate) fn query_truth_branch_identity(
    namespace: &str,
    evidence: impl AsRef<str>,
) -> TruthBranchIdentity {
    TruthBranchIdentity::from_relational_branch_id(
        stable_bridge_position(namespace, evidence).to_string(),
    )
}

pub(crate) fn query_truth_commit_identity(
    namespace: &str,
    evidence: impl AsRef<str>,
) -> TruthCommitIdentity {
    TruthCommitIdentity::from_relational_commit_id(stable_bridge_position(namespace, evidence))
}

pub(crate) fn query_truth_snapshot_identity(
    namespace: &str,
    evidence: impl AsRef<str>,
) -> TruthSnapshotIdentity {
    let snapshot_id = stable_bridge_position(namespace, evidence.as_ref());
    let version_id = stable_bridge_position(format!("{namespace}:version"), evidence);
    TruthSnapshotIdentity::from_relational_snapshot(RelationalBridgeSnapshotIdentityParts::new(
        snapshot_id,
        version_id,
    ))
}

fn stable_bridge_position(namespace: impl AsRef<str>, evidence: impl AsRef<str>) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(namespace.as_ref().as_bytes());
    hasher.update([0]);
    hasher.update(evidence.as_ref().as_bytes());
    let digest = hasher.finalize();
    u64::from_be_bytes(
        digest[0..8]
            .try_into()
            .expect("sha256 digest always has at least eight bytes"),
    )
}
