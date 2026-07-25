use crate::BlobWalRecordIdentity;

use super::LsmCompactionRecordSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmCompactionRecordIdentitySet {
    value: BlobWalRecordIdentity,
    generation: BlobWalRecordIdentity,
    tombstone: BlobWalRecordIdentity,
}

impl LsmCompactionRecordIdentitySet {
    pub(super) fn from_records(records: &LsmCompactionRecordSet) -> Self {
        Self {
            value: records.value().identity(),
            generation: records.generation().identity(),
            tombstone: records.tombstone().identity(),
        }
    }

    pub const fn value(self) -> BlobWalRecordIdentity {
        self.value
    }

    pub const fn generation(self) -> BlobWalRecordIdentity {
        self.generation
    }

    pub const fn tombstone(self) -> BlobWalRecordIdentity {
        self.tombstone
    }

    pub const fn in_replay_order(self) -> [BlobWalRecordIdentity; 3] {
        [self.value, self.generation, self.tombstone]
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::membership) const fn issued_for_certification(
        value: BlobWalRecordIdentity,
        generation: BlobWalRecordIdentity,
        tombstone: BlobWalRecordIdentity,
    ) -> Self {
        Self {
            value,
            generation,
            tombstone,
        }
    }
}
