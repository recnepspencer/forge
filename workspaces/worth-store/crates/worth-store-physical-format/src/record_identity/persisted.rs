#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersistedRecordIdentity {
    allocation_epoch: [u8; 16],
    ordinal: u64,
}

impl PersistedRecordIdentity {
    pub fn new(allocation_epoch: [u8; 16], ordinal: u64) -> Option<Self> {
        if allocation_epoch == [0; 16] || ordinal == 0 {
            None
        } else {
            Some(Self {
                allocation_epoch,
                ordinal,
            })
        }
    }

    pub const fn allocation_epoch(self) -> [u8; 16] {
        self.allocation_epoch
    }

    pub const fn ordinal(self) -> u64 {
        self.ordinal
    }
}
