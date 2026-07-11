use crate::blob_basis::S8BlobGenerationBasis;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_recovery_physics::{CheckpointCoveredLsnRange, LogSequenceNumber};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8CoverageBasisKind {
    WalLsn,
    RootEpoch,
    BlobGeneration,
    CheckpointFrontier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S8PhysicalCoverageBasis {
    WalLsn(LogSequenceNumber),
    RootEpoch(PhysicalEpoch),
    BlobGeneration(S8BlobGenerationBasis),
    CheckpointFrontier(CheckpointCoveredLsnRange),
}

impl S8PhysicalCoverageBasis {
    pub const fn wal_lsn(lsn: LogSequenceNumber) -> Self {
        Self::WalLsn(lsn)
    }

    pub const fn root_epoch(epoch: PhysicalEpoch) -> Self {
        Self::RootEpoch(epoch)
    }

    pub const fn blob_generation(generation: S8BlobGenerationBasis) -> Self {
        Self::BlobGeneration(generation)
    }

    pub const fn checkpoint_frontier(range: CheckpointCoveredLsnRange) -> Self {
        Self::CheckpointFrontier(range)
    }

    pub const fn basis_kind(&self) -> S8CoverageBasisKind {
        match self {
            Self::WalLsn(_) => S8CoverageBasisKind::WalLsn,
            Self::RootEpoch(_) => S8CoverageBasisKind::RootEpoch,
            Self::BlobGeneration(_) => S8CoverageBasisKind::BlobGeneration,
            Self::CheckpointFrontier(_) => S8CoverageBasisKind::CheckpointFrontier,
        }
    }

    pub fn watermark(&self) -> S8LayoutWatermark {
        S8LayoutWatermark::from_physical_basis(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutWatermark {
    basis_kind: S8CoverageBasisKind,
    start_inclusive: u64,
    end_exclusive: u64,
}

impl S8LayoutWatermark {
    pub(crate) const fn exact(basis_kind: S8CoverageBasisKind, value: u64) -> Self {
        Self {
            basis_kind,
            start_inclusive: value,
            end_exclusive: value,
        }
    }

    pub(crate) const fn ranged(
        basis_kind: S8CoverageBasisKind,
        start_inclusive: u64,
        end_exclusive: u64,
    ) -> Self {
        Self {
            basis_kind,
            start_inclusive,
            end_exclusive,
        }
    }

    pub fn from_physical_basis(basis: &S8PhysicalCoverageBasis) -> Self {
        match basis {
            S8PhysicalCoverageBasis::WalLsn(lsn) => {
                Self::exact(S8CoverageBasisKind::WalLsn, lsn.get())
            }
            S8PhysicalCoverageBasis::RootEpoch(epoch) => {
                Self::exact(S8CoverageBasisKind::RootEpoch, epoch.get())
            }
            S8PhysicalCoverageBasis::BlobGeneration(generation) => {
                Self::exact(S8CoverageBasisKind::BlobGeneration, generation.sequence())
            }
            S8PhysicalCoverageBasis::CheckpointFrontier(range) => Self::ranged(
                S8CoverageBasisKind::CheckpointFrontier,
                range.range().start().get(),
                range.range().end_exclusive().get(),
            ),
        }
    }

    pub const fn basis_kind(self) -> S8CoverageBasisKind {
        self.basis_kind
    }

    pub const fn start_inclusive(self) -> u64 {
        self.start_inclusive
    }

    pub const fn value(self) -> u64 {
        self.end_exclusive
    }
}
