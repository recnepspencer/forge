use crate::{
    StoreAuthenticityPhysicalIdentity, StoreCurrentAuthenticityScopeWitness,
    StoreSecurityScopeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreAuthenticityWitnessInput {
    posture: StoreAuthenticityWitnessPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StoreAuthenticityWitnessPosture {
    Absent,
    Verified(StoreAuthenticityWitnessBinding),
    Stale(StoreAuthenticityWitnessBinding),
    Unavailable,
    Unsupported,
    Failed(StoreAuthenticityWitnessBinding),
}

impl StoreAuthenticityWitnessInput {
    pub const fn absent() -> Self {
        Self {
            posture: StoreAuthenticityWitnessPosture::Absent,
        }
    }

    pub const fn unavailable() -> Self {
        Self {
            posture: StoreAuthenticityWitnessPosture::Unavailable,
        }
    }

    pub const fn unsupported() -> Self {
        Self {
            posture: StoreAuthenticityWitnessPosture::Unsupported,
        }
    }

    pub(crate) const fn from_posture(posture: StoreAuthenticityWitnessPosture) -> Self {
        Self { posture }
    }

    pub(crate) const fn posture(self) -> StoreAuthenticityWitnessPosture {
        self.posture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreAuthenticityWitnessObservationDeclaration {
    Verified,
    Stale,
    Failed,
}

impl StoreAuthenticityWitnessObservationDeclaration {
    pub const fn verified() -> Self {
        Self::Verified
    }

    pub const fn stale() -> Self {
        Self::Stale
    }

    pub const fn failed() -> Self {
        Self::Failed
    }
}

pub const fn admit_store_authenticity_witness_observation(
    scope: &StoreCurrentAuthenticityScopeWitness,
    physical_identity: StoreAuthenticityPhysicalIdentity,
    declaration: StoreAuthenticityWitnessObservationDeclaration,
) -> StoreAuthenticityWitnessInput {
    let binding =
        StoreAuthenticityWitnessBinding::from_admitted_observation(scope, physical_identity);
    match declaration {
        StoreAuthenticityWitnessObservationDeclaration::Verified => {
            StoreAuthenticityWitnessInput::from_posture(StoreAuthenticityWitnessPosture::Verified(
                binding,
            ))
        }
        StoreAuthenticityWitnessObservationDeclaration::Stale => {
            StoreAuthenticityWitnessInput::from_posture(StoreAuthenticityWitnessPosture::Stale(
                binding,
            ))
        }
        StoreAuthenticityWitnessObservationDeclaration::Failed => {
            StoreAuthenticityWitnessInput::from_posture(StoreAuthenticityWitnessPosture::Failed(
                binding,
            ))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreAuthenticityWitnessBinding {
    scope_identity: StoreSecurityScopeIdentity,
    physical_identity: StoreAuthenticityPhysicalIdentity,
}

impl StoreAuthenticityWitnessBinding {
    const fn from_admitted_observation(
        scope: &StoreCurrentAuthenticityScopeWitness,
        physical_identity: StoreAuthenticityPhysicalIdentity,
    ) -> Self {
        Self {
            scope_identity: scope.identity(),
            physical_identity,
        }
    }

    pub const fn scope_identity(&self) -> StoreSecurityScopeIdentity {
        self.scope_identity
    }

    pub const fn physical_identity(&self) -> StoreAuthenticityPhysicalIdentity {
        self.physical_identity
    }
}
