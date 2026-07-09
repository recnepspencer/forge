use worth_store_physical_format::RootPublicationValidationWitness;

use super::{
    CrashStableFreeReusePosture, OldReachabilityPreservation, PhysicalIdentityReuse,
    PhysicalPublicationDenial, PublicationEpochPair, ValidatedPhysicalPublicationIntent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewRootPublicationProof {
    validation: RootPublicationValidationWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationEpochReadiness {
    epochs: PublicationEpochPair,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationLatchReadiness {
    declared_publish_latches_released_before_blocking_io: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalPublicationReadiness {
    old_reachability: OldReachabilityPreservation,
    new_root: NewRootPublicationProof,
    epochs: PublicationEpochReadiness,
    latches: PublicationLatchReadiness,
    free_reuse: Option<CrashStableFreeReusePosture>,
}

impl NewRootPublicationProof {
    pub const fn from_root_validation(validation: RootPublicationValidationWitness) -> Self {
        Self { validation }
    }

    pub const fn validation(self) -> RootPublicationValidationWitness {
        self.validation
    }
}

impl PublicationEpochReadiness {
    pub const fn from_epoch_pair(epochs: PublicationEpochPair) -> Self {
        Self { epochs }
    }

    pub const fn epochs(self) -> PublicationEpochPair {
        self.epochs
    }
}

impl PublicationLatchReadiness {
    pub const fn declared_publish_latches_released_before_blocking_io() -> Self {
        Self {
            declared_publish_latches_released_before_blocking_io: true,
        }
    }

    pub const fn exposes_blocking_io_cost(self) -> bool {
        self.declared_publish_latches_released_before_blocking_io
    }
}

impl PhysicalPublicationReadiness {
    pub fn from_validated_intent(
        validated: &ValidatedPhysicalPublicationIntent,
        new_root: NewRootPublicationProof,
        latches: PublicationLatchReadiness,
    ) -> Self {
        Self {
            old_reachability: validated.old_reachability(),
            new_root,
            epochs: PublicationEpochReadiness::from_epoch_pair(validated.epochs()),
            latches,
            free_reuse: None,
        }
    }

    pub fn with_free_reuse_posture(
        mut self,
        free_reuse: CrashStableFreeReusePosture,
    ) -> Result<Self, PhysicalPublicationDenial> {
        self.free_reuse = Some(free_reuse);
        Ok(self)
    }

    pub(crate) fn validate_for_intent(
        self,
        intent: &ValidatedPhysicalPublicationIntent,
    ) -> Result<Self, PhysicalPublicationDenial> {
        if self.old_reachability != intent.old_reachability() {
            return Err(PhysicalPublicationDenial::MissingReachabilityEvidence);
        }
        if self.epochs.epochs() != intent.epochs() {
            return Err(PhysicalPublicationDenial::StaleRootPublicationEpoch);
        }
        if self.new_root.validation() != intent.new_root_validation() {
            return Err(PhysicalPublicationDenial::NewRootPublicationProofMismatch);
        }
        if intent.identity_reuse() == PhysicalIdentityReuse::Requested && self.free_reuse.is_none()
        {
            return Err(PhysicalPublicationDenial::IdentityReuseWithoutCrashStableFence);
        }
        Ok(self)
    }

    pub const fn old_reachability(self) -> OldReachabilityPreservation {
        self.old_reachability
    }

    pub const fn new_root(self) -> NewRootPublicationProof {
        self.new_root
    }

    pub const fn epochs(self) -> PublicationEpochReadiness {
        self.epochs
    }

    pub const fn latches(self) -> PublicationLatchReadiness {
        self.latches
    }

    pub const fn free_reuse(self) -> Option<CrashStableFreeReusePosture> {
        self.free_reuse
    }
}
