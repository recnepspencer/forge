#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeDeliveryReceipt {
    delivered_target_count: usize,
    snapshot_identity: crate::snapshot::TruthSnapshotIdentity,
}

impl BridgeDeliveryReceipt {
    pub fn new(
        delivered_target_count: usize,
        snapshot_identity: crate::snapshot::TruthSnapshotIdentity,
    ) -> Self {
        Self {
            delivered_target_count,
            snapshot_identity,
        }
    }

    pub fn delivered_target_count(&self) -> usize {
        self.delivered_target_count
    }

    pub fn snapshot_identity(&self) -> &crate::snapshot::TruthSnapshotIdentity {
        &self.snapshot_identity
    }
}
