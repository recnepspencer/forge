use super::{
    OldReachabilityPreservation, PhysicalPublicationDenial, PublicationEpochPair,
    PublicationRootCandidate,
};
use crate::CurrentPhysicalRoot;
use forge_store_physical_format::RootPublicationValidationWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPublicationIntentKind {
    CopyOnWriteRootManifest,
    InPlaceReachableOverwrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIdentityReuse {
    None,
    Requested,
}

#[derive(Debug, Clone)]
pub struct PhysicalPublicationIntent {
    kind: PhysicalPublicationIntentKind,
    old_root: PublicationRootCandidate,
    new_root: PublicationRootCandidate,
    old_reachability: Option<OldReachabilityPreservation>,
    identity_reuse: PhysicalIdentityReuse,
}

#[derive(Debug, Clone)]
pub struct ValidatedPhysicalPublicationIntent {
    kind: PhysicalPublicationIntentKind,
    old_root: PublicationRootCandidate,
    new_root: PublicationRootCandidate,
    old_reachability: OldReachabilityPreservation,
    epochs: PublicationEpochPair,
    identity_reuse: PhysicalIdentityReuse,
}

impl PhysicalPublicationIntent {
    pub fn copy_on_write_root_manifest(
        old_root: PublicationRootCandidate,
        new_root: PublicationRootCandidate,
        old_reachability: OldReachabilityPreservation,
    ) -> Self {
        Self {
            kind: PhysicalPublicationIntentKind::CopyOnWriteRootManifest,
            old_root,
            new_root,
            old_reachability: Some(old_reachability),
            identity_reuse: PhysicalIdentityReuse::None,
        }
    }

    pub fn copy_on_write_root_manifest_with_identity_reuse(
        old_root: PublicationRootCandidate,
        new_root: PublicationRootCandidate,
        old_reachability: OldReachabilityPreservation,
    ) -> Self {
        Self {
            identity_reuse: PhysicalIdentityReuse::Requested,
            ..Self::copy_on_write_root_manifest(old_root, new_root, old_reachability)
        }
    }

    pub fn in_place_reachable_overwrite_attempt(
        old_root: PublicationRootCandidate,
        new_root: PublicationRootCandidate,
        old_reachability: OldReachabilityPreservation,
    ) -> Self {
        Self {
            kind: PhysicalPublicationIntentKind::InPlaceReachableOverwrite,
            old_root,
            new_root,
            old_reachability: Some(old_reachability),
            identity_reuse: PhysicalIdentityReuse::None,
        }
    }

    pub fn missing_old_reachability_attempt(
        old_root: PublicationRootCandidate,
        new_root: PublicationRootCandidate,
    ) -> Self {
        Self {
            kind: PhysicalPublicationIntentKind::CopyOnWriteRootManifest,
            old_root,
            new_root,
            old_reachability: None,
            identity_reuse: PhysicalIdentityReuse::None,
        }
    }

    pub fn validate_copy_on_write_inputs(
        self,
    ) -> Result<ValidatedPhysicalPublicationIntent, PhysicalPublicationDenial> {
        if self.kind != PhysicalPublicationIntentKind::CopyOnWriteRootManifest {
            return Err(PhysicalPublicationDenial::InPlaceReachableOverwrite);
        }
        let old_reachability = self
            .old_reachability
            .ok_or(PhysicalPublicationDenial::MissingOldReachability)?;
        let epochs =
            PublicationEpochPair::advance_from(self.old_root.root(), self.new_root.root())?;
        Ok(ValidatedPhysicalPublicationIntent {
            kind: self.kind,
            old_root: self.old_root,
            new_root: self.new_root,
            old_reachability,
            epochs,
            identity_reuse: self.identity_reuse,
        })
    }

    pub const fn kind(&self) -> PhysicalPublicationIntentKind {
        self.kind
    }
}

impl ValidatedPhysicalPublicationIntent {
    pub const fn old_root(&self) -> CurrentPhysicalRoot {
        self.old_root.root()
    }

    pub const fn new_root(&self) -> CurrentPhysicalRoot {
        self.new_root.root()
    }

    pub const fn old_reachability(&self) -> OldReachabilityPreservation {
        self.old_reachability
    }

    pub const fn epochs(&self) -> PublicationEpochPair {
        self.epochs
    }

    pub const fn identity_reuse(&self) -> PhysicalIdentityReuse {
        self.identity_reuse
    }

    pub const fn old_root_validation(&self) -> RootPublicationValidationWitness {
        self.old_root.validation()
    }

    pub const fn new_root_validation(&self) -> RootPublicationValidationWitness {
        self.new_root.validation()
    }

    pub const fn kind(&self) -> PhysicalPublicationIntentKind {
        self.kind
    }
}
