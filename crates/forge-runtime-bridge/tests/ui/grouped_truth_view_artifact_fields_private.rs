use forge_runtime_bridge::facade::{
    BridgeGroupedTruthViewArtifact, BridgeGroupedTruthViewDigest, TruthSnapshotIdentity,
};

fn main() {
    let _ = BridgeGroupedTruthViewArtifact {
        truth_view_digest: "truth-view".into(),
        basis_snapshot_identity: TruthSnapshotIdentity::new("snapshot-a"),
        contract: todo!(),
        members: vec![],
        digest: BridgeGroupedTruthViewDigest::new(&[]),
    };
}
