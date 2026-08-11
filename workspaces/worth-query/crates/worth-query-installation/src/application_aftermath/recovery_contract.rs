//! Installed recovery contract describing legal recovery next actions.
//!
//! Gate 8.1 installs the contract shape. Recovery-handle progression is Gate 8.3.

use super::published_posture::PublishedAftermathPosture;

/// Whether the installed aftermath admits recovery work later.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstalledAftermathRecoveryContract {
    /// Recovery work is not admitted for this posture.
    NotAdmitted,
    /// Recovery work may be admitted by a later gate for this posture.
    Admissible { posture: PublishedAftermathPosture },
}

impl InstalledAftermathRecoveryContract {
    pub(crate) const fn for_posture(posture: PublishedAftermathPosture) -> Self {
        match posture {
            PublishedAftermathPosture::Irreversible => Self::NotAdmitted,
            PublishedAftermathPosture::Reversible
            | PublishedAftermathPosture::Compensatable
            | PublishedAftermathPosture::Reconcilable => Self::Admissible { posture },
        }
    }
}
