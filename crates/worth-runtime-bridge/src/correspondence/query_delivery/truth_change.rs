use super::super::BridgeDeliveredCorrespondenceChangeSet;

#[derive(Debug, Clone)]
pub struct BridgeDeliveredTruthChange {
    change_set: BridgeDeliveredCorrespondenceChangeSet,
}

impl BridgeDeliveredTruthChange {
    pub(crate) const fn new(change_set: BridgeDeliveredCorrespondenceChangeSet) -> Self {
        Self { change_set }
    }

    pub const fn change_set(&self) -> &BridgeDeliveredCorrespondenceChangeSet {
        &self.change_set
    }
}
