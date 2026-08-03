use worth_store_wal::{LogSequenceNumber, WalLsnRange, WalSegmentArtifactIdentity};

use super::super::wal::inventory::PhysicalWalSegmentInventoryEntry;

pub enum PhysicalRecoveryWalTail {
    GenerationZero,
    Contiguous {
        durable_lsn_end: LogSequenceNumber,
        segments: Box<[PhysicalRecoveryWalSegment]>,
    },
    InspectionRequired {
        durable_lsn_end: Option<LogSequenceNumber>,
        segments: Box<[PhysicalRecoveryWalSegment]>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecoveryWalSegment {
    identity: WalSegmentArtifactIdentity,
    lsn_range: WalLsnRange,
    byte_count: u64,
}

impl PhysicalRecoveryWalTail {
    pub(in crate::physical_runtime::durability) fn from_inventory(
        durable_lsn_end: Option<LogSequenceNumber>,
        entries: &[PhysicalWalSegmentInventoryEntry],
        sealed: bool,
    ) -> Self {
        let segments = entries
            .iter()
            .copied()
            .map(PhysicalRecoveryWalSegment::from_inventory)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        if sealed {
            return Self::InspectionRequired {
                durable_lsn_end,
                segments,
            };
        }
        match durable_lsn_end {
            Some(durable_lsn_end) => Self::Contiguous {
                durable_lsn_end,
                segments,
            },
            None => Self::GenerationZero,
        }
    }

    pub const fn requires_inspection(&self) -> bool {
        matches!(self, Self::InspectionRequired { .. })
    }

    pub fn segments(&self) -> &[PhysicalRecoveryWalSegment] {
        match self {
            Self::GenerationZero => &[],
            Self::Contiguous { segments, .. } | Self::InspectionRequired { segments, .. } => {
                segments
            }
        }
    }

    pub const fn durable_lsn_end(&self) -> Option<LogSequenceNumber> {
        match self {
            Self::GenerationZero => None,
            Self::Contiguous {
                durable_lsn_end, ..
            } => Some(*durable_lsn_end),
            Self::InspectionRequired {
                durable_lsn_end, ..
            } => *durable_lsn_end,
        }
    }
}

impl PhysicalRecoveryWalSegment {
    const fn from_inventory(entry: PhysicalWalSegmentInventoryEntry) -> Self {
        Self {
            identity: entry.identity(),
            lsn_range: entry.lsn_range(),
            byte_count: entry.byte_count(),
        }
    }

    pub const fn identity(self) -> WalSegmentArtifactIdentity {
        self.identity
    }

    pub const fn lsn_range(self) -> WalLsnRange {
        self.lsn_range
    }

    pub const fn byte_count(self) -> u64 {
        self.byte_count
    }
}
