use worth_store_contracts::{S6BackgroundPressureDeclaration, S6BackgroundPressureKind};
use worth_store_io_scheduler::{S10CompactionIoReadinessHandoff, S6LaterReadinessReadmissionState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCompactionS6Pacing {
    Admitted {
        declaration: S6BackgroundPressureDeclaration,
        foreground_yields: u64,
        io_readmission_satisfied: bool,
    },
    Unsupported,
}

impl BlobCompactionS6Pacing {
    pub const fn admitted_compaction(foreground_yields: u64) -> Self {
        Self::Admitted {
            declaration: S6BackgroundPressureDeclaration::compaction_rewrite(),
            foreground_yields,
            io_readmission_satisfied: true,
        }
    }

    pub fn from_s10_handoff(
        handoff: &S10CompactionIoReadinessHandoff,
        foreground_yields: u64,
    ) -> Self {
        if handoff.readmission_state()
            != S6LaterReadinessReadmissionState::ReadmittedAfterPublication
        {
            return Self::Unsupported;
        }
        Self::Admitted {
            declaration: S6BackgroundPressureDeclaration::compaction_rewrite(),
            foreground_yields,
            io_readmission_satisfied: true,
        }
    }

    pub const fn supports_compaction(self) -> bool {
        match self {
            Self::Admitted {
                declaration,
                io_readmission_satisfied,
                ..
            } => {
                io_readmission_satisfied
                    && matches!(
                        declaration.kind(),
                        S6BackgroundPressureKind::CompactionRewrite
                    )
            }
            Self::Unsupported => false,
        }
    }

    pub const fn foreground_yields(self) -> u64 {
        match self {
            Self::Admitted {
                foreground_yields, ..
            } => foreground_yields,
            Self::Unsupported => 0,
        }
    }

    pub const fn io_readmission_satisfied(self) -> bool {
        match self {
            Self::Admitted {
                io_readmission_satisfied,
                ..
            } => io_readmission_satisfied,
            Self::Unsupported => false,
        }
    }
}

#[allow(dead_code)]
fn _pacing_is_part_of_the_boundary(_: BlobCompactionS6Pacing) {}
