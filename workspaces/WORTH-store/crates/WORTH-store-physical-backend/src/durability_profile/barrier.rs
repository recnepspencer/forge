#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WalDurabilityBarrier {
    SimulatedDurableCommit,
    WalFileFsync,
    WalDirectoryFsync,
    WindowsFlushFileBuffers,
    WindowsDirectorySync,
    OrderedPersistenceFence,
}

impl WalDurabilityBarrier {
    pub const fn bit(self) -> u16 {
        match self {
            Self::SimulatedDurableCommit => 1 << 0,
            Self::WalFileFsync => 1 << 1,
            Self::WalDirectoryFsync => 1 << 2,
            Self::WindowsFlushFileBuffers => 1 << 3,
            Self::WindowsDirectorySync => 1 << 4,
            Self::OrderedPersistenceFence => 1 << 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WalDurabilityBarrierSet {
    bits: u16,
}

impl WalDurabilityBarrierSet {
    pub const EMPTY: Self = Self { bits: 0 };

    pub const fn of(barrier: WalDurabilityBarrier) -> Self {
        Self {
            bits: barrier.bit(),
        }
    }

    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }

    pub const fn bits(self) -> u16 {
        self.bits
    }

    pub const fn contains(self, barrier: WalDurabilityBarrier) -> bool {
        self.bits & barrier.bit() == barrier.bit()
    }

    pub const fn insert(self, barrier: WalDurabilityBarrier) -> Self {
        Self {
            bits: self.bits | barrier.bit(),
        }
    }

    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    pub const fn satisfies(self, required: Self) -> bool {
        self.bits & required.bits == required.bits
    }

    pub fn first_missing_from(self, required: Self) -> Option<WalDurabilityBarrier> {
        for barrier in ALL_BARRIERS {
            if required.contains(barrier) && !self.contains(barrier) {
                return Some(barrier);
            }
        }
        None
    }
}

const ALL_BARRIERS: [WalDurabilityBarrier; 6] = [
    WalDurabilityBarrier::SimulatedDurableCommit,
    WalDurabilityBarrier::WalFileFsync,
    WalDurabilityBarrier::WalDirectoryFsync,
    WalDurabilityBarrier::WindowsFlushFileBuffers,
    WalDurabilityBarrier::WindowsDirectorySync,
    WalDurabilityBarrier::OrderedPersistenceFence,
];
