use worth_store_physical_backend::OwnershipReleaseOutcome;

/// Inert terminal result preserving OS-lease release certainty separately
/// from runtime close/abort classification.
pub enum MediaShutdownOutcome<Terminal> {
    Released(Terminal),
    ReleaseUnconfirmed {
        terminal: Terminal,
        release: OwnershipReleaseOutcome,
    },
}

impl<Terminal> MediaShutdownOutcome<Terminal> {
    pub(super) fn new(terminal: Terminal, release: OwnershipReleaseOutcome) -> Self {
        match release {
            OwnershipReleaseOutcome::Released => Self::Released(terminal),
            release @ OwnershipReleaseOutcome::ReleaseUnconfirmed { .. } => {
                Self::ReleaseUnconfirmed { terminal, release }
            }
        }
    }

    pub const fn terminal(&self) -> &Terminal {
        match self {
            Self::Released(terminal) | Self::ReleaseUnconfirmed { terminal, .. } => terminal,
        }
    }

    pub const fn release(&self) -> OwnershipReleaseOutcome {
        match self {
            Self::Released(_) => OwnershipReleaseOutcome::Released,
            Self::ReleaseUnconfirmed { release, .. } => *release,
        }
    }
}
