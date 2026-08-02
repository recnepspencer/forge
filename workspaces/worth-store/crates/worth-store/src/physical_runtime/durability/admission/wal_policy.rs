use std::num::{NonZeroU32, NonZeroU64};

/// Maximum encoded bytes admitted in one Store-owned WAL segment.
///
/// The limit is independent of checkpoint memory and retained-tail budgets:
/// it determines append fit and rotation, not capture allocation or retention.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WalSegmentByteLimit(NonZeroU64);

impl WalSegmentByteLimit {
    pub const fn new(bytes: NonZeroU64) -> Self {
        Self(bytes)
    }

    pub const fn get(self) -> NonZeroU64 {
        self.0
    }
}

/// Maximum WAL segment artifacts admitted in the Store-owned inventory.
///
/// This bound exists independently of segment bytes and retained-tail bytes:
/// it bounds directory enumeration before any artifact can be trusted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WalSegmentInventoryLimit(NonZeroU32);

impl WalSegmentInventoryLimit {
    pub const fn new(segments: NonZeroU32) -> Self {
        Self(segments)
    }

    pub const fn get(self) -> NonZeroU32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalWalPolicy {
    segment_bytes: WalSegmentByteLimit,
    segment_inventory: WalSegmentInventoryLimit,
}

impl PhysicalWalPolicy {
    pub const fn segmented(
        segment_bytes: WalSegmentByteLimit,
        segment_inventory: WalSegmentInventoryLimit,
    ) -> Self {
        Self {
            segment_bytes,
            segment_inventory,
        }
    }

    pub const fn segment_byte_limit(self) -> WalSegmentByteLimit {
        self.segment_bytes
    }

    pub const fn segment_inventory_limit(self) -> WalSegmentInventoryLimit {
        self.segment_inventory
    }
}
