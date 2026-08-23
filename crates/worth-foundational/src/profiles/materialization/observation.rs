use crate::identities::{BoundaryEpoch, BoundaryHandle};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalObservationActivationScope {
    Operation,
    Batch,
    ManagedSession,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalObservationDisposition {
    Inactive,
    Continuous,
    ExplicitlyActivated {
        scope: FoundationalObservationActivationScope,
        session: BoundaryHandle,
        observed_epoch: BoundaryEpoch,
    },
}

impl FoundationalObservationDisposition {
    pub const fn is_active(self) -> bool {
        !matches!(self, Self::Inactive)
    }

    pub const fn scope(self) -> Option<FoundationalObservationActivationScope> {
        match self {
            Self::ExplicitlyActivated { scope, .. } => Some(scope),
            Self::Inactive | Self::Continuous => None,
        }
    }
}
