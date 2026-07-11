use super::{LayoutPlanFingerprint, LayoutRollbackRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutInterruptionPolicy {
    ResumeDeclaredMigration,
    RollbackDeclaredMigration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutInterruptionState {
    fingerprint: LayoutPlanFingerprint,
}

impl LayoutInterruptionState {
    pub(crate) const fn new(fingerprint: LayoutPlanFingerprint) -> Self {
        Self { fingerprint }
    }

    pub const fn fingerprint(self) -> LayoutPlanFingerprint {
        self.fingerprint
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutInterruptedMigrationDisposition {
    Resume(LayoutInterruptionState),
    Rollback(LayoutRollbackRequest),
}
