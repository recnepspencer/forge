use forge_store_contracts::StableArtifactId;
use forge_store_snapshots::{PublishedSnapshotHandle, SnapshotId};

fn main() {
    let snapshot_id =
        SnapshotId::from_artifact_id(StableArtifactId::new("phase23-snapshot").unwrap());
    let _ = PublishedSnapshotHandle::new(snapshot_id, "sha256:image", 8);
}
