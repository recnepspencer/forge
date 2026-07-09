use super::{BackgroundIoPressureClass, BackgroundResourceBudget};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundDebtKind {
    CompactionDebt,
    CheckpointFlushDebt,
    ScrubPressure,
    ReplicationPrepPressure,
    BlobContention,
    BackupPressure,
    RepairPressure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundIoDebt {
    kind: BackgroundDebtKind,
    class: BackgroundIoPressureClass,
    units: BackgroundResourceBudget,
}

impl BackgroundIoDebt {
    pub(crate) const fn new(
        class: BackgroundIoPressureClass,
        units: BackgroundResourceBudget,
    ) -> Self {
        Self {
            kind: class.debt_kind(),
            class,
            units,
        }
    }

    pub const fn kind(self) -> BackgroundDebtKind {
        self.kind
    }

    pub const fn class(self) -> BackgroundIoPressureClass {
        self.class
    }

    pub const fn units(self) -> BackgroundResourceBudget {
        self.units
    }
}
