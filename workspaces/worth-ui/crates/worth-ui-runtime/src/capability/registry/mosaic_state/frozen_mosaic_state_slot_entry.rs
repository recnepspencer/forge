use super::{MosaicStateReconciliationKey, MosaicStateSlotDescriptor};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenMosaicStateSlotEntry {
    descriptor: MosaicStateSlotDescriptor,
    reconciliation_key: MosaicStateReconciliationKey,
}

impl FrozenMosaicStateSlotEntry {
    pub(crate) fn new(
        descriptor: MosaicStateSlotDescriptor,
        reconciliation_key: MosaicStateReconciliationKey,
    ) -> Self {
        Self {
            descriptor,
            reconciliation_key,
        }
    }

    pub fn descriptor(&self) -> &MosaicStateSlotDescriptor {
        &self.descriptor
    }

    pub fn reconciliation_key(&self) -> &MosaicStateReconciliationKey {
        &self.reconciliation_key
    }
}
