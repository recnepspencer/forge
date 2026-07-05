use core::num::NonZeroU64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoResourceUnitKind {
    QueueSlot,
    BandwidthToken,
    FlushPermit,
    SyncDebt,
    ReadAheadWindow,
    WriteBackWindow,
    DirtyPageBudget,
    WorkerPermit,
    CacheResidencyHint,
    ReclaimPermit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoResourceUnitDenial {
    ZeroQueueSlot,
    ZeroBandwidthToken,
    ZeroFlushPermit,
    ZeroSyncDebt,
    ZeroReadAheadWindow,
    ZeroWriteBackWindow,
    ZeroDirtyPageBudget,
    ZeroWorkerPermit,
    ZeroCacheResidencyHint,
    ZeroReclaimPermit,
}

macro_rules! resource_unit {
    ($name:ident, $denial:ident, $constructor:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $name(NonZeroU64);

        impl $name {
            pub fn $constructor(value: u64) -> Result<Self, IoResourceUnitDenial> {
                NonZeroU64::new(value)
                    .map(Self)
                    .ok_or(IoResourceUnitDenial::$denial)
            }

            pub const fn get(self) -> u64 {
                self.0.get()
            }
        }
    };
}

resource_unit!(QueueSlot, ZeroQueueSlot, new);
resource_unit!(BandwidthToken, ZeroBandwidthToken, bytes);
resource_unit!(FlushPermit, ZeroFlushPermit, new);
resource_unit!(SyncDebt, ZeroSyncDebt, units);
resource_unit!(ReadAheadWindow, ZeroReadAheadWindow, pages);
resource_unit!(WriteBackWindow, ZeroWriteBackWindow, pages);
resource_unit!(DirtyPageBudget, ZeroDirtyPageBudget, pages);
resource_unit!(WorkerPermit, ZeroWorkerPermit, new);
resource_unit!(CacheResidencyHint, ZeroCacheResidencyHint, frames);
resource_unit!(ReclaimPermit, ZeroReclaimPermit, new);
