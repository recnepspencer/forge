use forge_runtime_bridge::facade::{
    BridgeCommittedPatchBody, BridgeCommittedPatchDigest, BridgeCommittedPatchEnvelope,
    BridgeCommittedPatchSummary, TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity,
    TruthSnapshotIdentity,
};

fn main() {
    let _ = (
        BridgeCommittedPatchBody::new(vec![]),
        BridgeCommittedPatchSummary::new(1, 1),
        sealed_authority_placeholder(),
        TruthCommitIdentity::new("commit"),
        TruthPatchIdentity::new("patch"),
        TruthSnapshotIdentity::new("snapshot"),
        TruthBranchIdentity::new("branch"),
    );
    let _ = BridgeCommittedPatchEnvelope::from_parts;
}

fn sealed_authority_placeholder<T>() -> T {
    panic!("compile-fail fixture never executes")
}
