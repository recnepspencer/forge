use crate::{
    evidence::StoreCounterSnapshot,
    snapshot::{stable_snapshot_digest, SnapshotImageBundle},
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone4CertificationBundle {
    pub truth_digest: String,
    pub restore_digest: String,
    pub rebuild_digest: String,
    pub counter_snapshot: StoreCounterSnapshot,
}

impl Milestone4CertificationBundle {
    pub fn new(
        truth_image: &SnapshotImageBundle,
        restored_image: &SnapshotImageBundle,
        rebuilt_image: &SnapshotImageBundle,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        Self {
            truth_digest: stable_snapshot_digest(truth_image),
            restore_digest: stable_snapshot_digest(restored_image),
            rebuild_digest: stable_snapshot_digest(rebuilt_image),
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 4 certification serialization")
    }
}
