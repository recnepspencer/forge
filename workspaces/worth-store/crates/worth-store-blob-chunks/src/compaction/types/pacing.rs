use worth_store_contracts::{BackgroundPressureDeclaration, BackgroundPressureKind};
use worth_store_io_scheduler::{BackgroundIoPressureClass, BackgroundPacingCapability};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCompactionPacingAdmission {
    Admitted {
        declaration: BackgroundPressureDeclaration,
        foreground_yields: u64,
        io_readmission_satisfied: bool,
    },
    Unsupported,
}

impl BlobCompactionPacingAdmission {
    pub const fn admitted_compaction(foreground_yields: u64) -> Self {
        Self::Admitted {
            declaration: BackgroundPressureDeclaration::compaction_rewrite(),
            foreground_yields,
            io_readmission_satisfied: true,
        }
    }

    pub fn from_scheduler_capability(
        capability: BackgroundPacingCapability,
        foreground_yields: u64,
    ) -> Self {
        if capability.class() != BackgroundIoPressureClass::CompactionRewrite {
            return Self::Unsupported;
        }
        Self::Admitted {
            declaration: BackgroundPressureDeclaration::compaction_rewrite(),
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
                        BackgroundPressureKind::CompactionRewrite
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
fn _pacing_is_part_of_the_boundary(_: BlobCompactionPacingAdmission) {}
