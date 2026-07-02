#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryOutcomeKind {
    RecoveredOldRoot,
    RecoveredNewRoot,
    MixedRoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryOutcomeObservation {
    kind: RecoveryOutcomeKind,
}

impl RecoveryOutcomeObservation {
    pub const fn recovered_old_root() -> Self {
        Self {
            kind: RecoveryOutcomeKind::RecoveredOldRoot,
        }
    }

    pub const fn recovered_new_root() -> Self {
        Self {
            kind: RecoveryOutcomeKind::RecoveredNewRoot,
        }
    }

    pub const fn mixed_root() -> Self {
        Self {
            kind: RecoveryOutcomeKind::MixedRoot,
        }
    }

    pub const fn kind(&self) -> RecoveryOutcomeKind {
        self.kind
    }
}
