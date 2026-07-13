use crate::blob_basis::BlobGenerationBasis;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_recovery_physics::{CheckpointCoveredLsnRange, LogSequenceNumber};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageBasisKind {
    WalLsn,
    RootEpoch,
    BlobGeneration,
    CheckpointFrontier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalCoverageBasis {
    WalLsn(LogSequenceNumber),
    RootEpoch(PhysicalEpoch),
    BlobGeneration(BlobGenerationBasis),
    CheckpointFrontier(CheckpointCoveredLsnRange),
}

impl PhysicalCoverageBasis {
    pub const fn wal_lsn(lsn: LogSequenceNumber) -> Self {
        Self::WalLsn(lsn)
    }

    pub const fn root_epoch(epoch: PhysicalEpoch) -> Self {
        Self::RootEpoch(epoch)
    }

    pub const fn blob_generation(generation: BlobGenerationBasis) -> Self {
        Self::BlobGeneration(generation)
    }

    pub const fn checkpoint_frontier(range: CheckpointCoveredLsnRange) -> Self {
        Self::CheckpointFrontier(range)
    }

    pub const fn basis_kind(&self) -> CoverageBasisKind {
        match self {
            Self::WalLsn(_) => CoverageBasisKind::WalLsn,
            Self::RootEpoch(_) => CoverageBasisKind::RootEpoch,
            Self::BlobGeneration(_) => CoverageBasisKind::BlobGeneration,
            Self::CheckpointFrontier(_) => CoverageBasisKind::CheckpointFrontier,
        }
    }

    pub fn watermark(&self) -> LayoutWatermark {
        LayoutWatermark::from_physical_basis(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutWatermark {
    basis_kind: CoverageBasisKind,
    start_inclusive: u64,
    end_exclusive: u64,
}

impl LayoutWatermark {
    pub(crate) const fn exact(basis_kind: CoverageBasisKind, value: u64) -> Self {
        Self {
            basis_kind,
            start_inclusive: value,
            end_exclusive: value,
        }
    }

    pub(crate) const fn ranged(
        basis_kind: CoverageBasisKind,
        start_inclusive: u64,
        end_exclusive: u64,
    ) -> Self {
        Self {
            basis_kind,
            start_inclusive,
            end_exclusive,
        }
    }

    pub fn from_physical_basis(basis: &PhysicalCoverageBasis) -> Self {
        match basis {
            PhysicalCoverageBasis::WalLsn(lsn) => Self::exact(CoverageBasisKind::WalLsn, lsn.get()),
            PhysicalCoverageBasis::RootEpoch(epoch) => {
                Self::exact(CoverageBasisKind::RootEpoch, epoch.get())
            }
            PhysicalCoverageBasis::BlobGeneration(generation) => {
                Self::exact(CoverageBasisKind::BlobGeneration, generation.sequence())
            }
            PhysicalCoverageBasis::CheckpointFrontier(range) => Self::ranged(
                CoverageBasisKind::CheckpointFrontier,
                range.range().start().get(),
                range.range().end_exclusive().get(),
            ),
        }
    }

    pub const fn basis_kind(self) -> CoverageBasisKind {
        self.basis_kind
    }

    pub const fn start_inclusive(self) -> u64 {
        self.start_inclusive
    }

    pub const fn value(self) -> u64 {
        self.end_exclusive
    }
}
