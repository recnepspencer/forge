use forge_store_physical_isolation::StablePhysicalReadReceipt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCompactionReadHold {
    Released(StablePhysicalReadReceipt),
    Active(StablePhysicalReadReceipt),
}

impl BlobCompactionReadHold {
    pub const fn released(receipt: StablePhysicalReadReceipt) -> Self {
        Self::Released(receipt)
    }

    pub const fn active(receipt: StablePhysicalReadReceipt) -> Self {
        Self::Active(receipt)
    }

    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active(_))
    }

    pub(crate) const fn released_receipt(self) -> Option<StablePhysicalReadReceipt> {
        match self {
            Self::Released(receipt) => Some(receipt),
            Self::Active(_) => None,
        }
    }
}

#[allow(dead_code)]
fn _read_hold_is_part_of_the_boundary(_: BlobCompactionReadHold) {}