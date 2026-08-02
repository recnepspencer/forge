use worth_store_physical_format::PhysicalCheckpointIdentity;
use worth_store_wal::WalSegmentArtifactIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWalReclamationReport {
    checkpoint: PhysicalCheckpointIdentity,
    planned_segments: u32,
    reclaimed_segments: u32,
    reclaimed_bytes: u64,
    first_unreclaimed: Option<WalSegmentArtifactIdentity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWalReclamationObservation {
    NotRequired {
        checkpoint: PhysicalCheckpointIdentity,
    },
    Reclaimed(PhysicalWalReclamationReport),
    DeferredBeforeEffect(PhysicalWalReclamationReport),
    InspectionRequired(PhysicalWalReclamationReport),
}

impl PhysicalWalReclamationReport {
    pub(super) const fn new(
        checkpoint: PhysicalCheckpointIdentity,
        planned_segments: u32,
        reclaimed_segments: u32,
        reclaimed_bytes: u64,
        first_unreclaimed: Option<WalSegmentArtifactIdentity>,
    ) -> Self {
        Self {
            checkpoint,
            planned_segments,
            reclaimed_segments,
            reclaimed_bytes,
            first_unreclaimed,
        }
    }

    pub const fn checkpoint(self) -> PhysicalCheckpointIdentity {
        self.checkpoint
    }

    pub const fn planned_segments(self) -> u32 {
        self.planned_segments
    }

    pub const fn reclaimed_segments(self) -> u32 {
        self.reclaimed_segments
    }

    pub const fn reclaimed_bytes(self) -> u64 {
        self.reclaimed_bytes
    }

    pub const fn first_unreclaimed(self) -> Option<WalSegmentArtifactIdentity> {
        self.first_unreclaimed
    }
}

impl PhysicalWalReclamationObservation {
    pub const fn checkpoint(self) -> PhysicalCheckpointIdentity {
        match self {
            Self::NotRequired { checkpoint } => checkpoint,
            Self::Reclaimed(report)
            | Self::DeferredBeforeEffect(report)
            | Self::InspectionRequired(report) => report.checkpoint(),
        }
    }
}
