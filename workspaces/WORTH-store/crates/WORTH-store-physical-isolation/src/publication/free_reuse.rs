use super::{PhysicalPublicationDenial, ReleasedOldReachability};
use crate::{CurrentGenerationPhysicalReference, PhysicalOrderingContract, PhysicalOrderingSite};
use worth_store_physical_format::PhysicalGenerationOwner;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocatorPublicationFence {
    ordering: PhysicalOrderingContract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashStableFreeReusePosture {
    fence: AllocatorPublicationFence,
    old_identity: CurrentGenerationPhysicalReference,
    reused_identity: CurrentGenerationPhysicalReference,
    released_old_reachability: ReleasedOldReachability,
}

impl AllocatorPublicationFence {
    pub fn from_ordering(
        ordering: PhysicalOrderingContract,
    ) -> Result<Self, PhysicalPublicationDenial> {
        Ok(Self {
            ordering: ordering.require_site(PhysicalOrderingSite::AllocatorPublication)?,
        })
    }

    pub const fn ordering(self) -> PhysicalOrderingContract {
        self.ordering
    }
}

impl CrashStableFreeReusePosture {
    pub fn admit(
        fence: AllocatorPublicationFence,
        old_identity: CurrentGenerationPhysicalReference,
        reused_identity: CurrentGenerationPhysicalReference,
        released_old_reachability: ReleasedOldReachability,
    ) -> Result<Self, PhysicalPublicationDenial> {
        if !same_physical_identity_ignoring_generation(
            old_identity.owner(),
            reused_identity.owner(),
        ) {
            return Err(PhysicalPublicationDenial::IdentityReuseOwnerMismatch);
        }
        if reused_identity.generation().get() <= old_identity.generation().get() {
            return Err(PhysicalPublicationDenial::IdentityReuseWithoutGenerationAdvance);
        }
        Ok(Self {
            fence,
            old_identity,
            reused_identity,
            released_old_reachability,
        })
    }

    pub const fn fence(self) -> AllocatorPublicationFence {
        self.fence
    }

    pub const fn old_identity(self) -> CurrentGenerationPhysicalReference {
        self.old_identity
    }

    pub const fn reused_identity(self) -> CurrentGenerationPhysicalReference {
        self.reused_identity
    }

    pub const fn generation_advanced(self) -> bool {
        self.reused_identity.generation().get() > self.old_identity.generation().get()
    }

    pub const fn released_old_reachability(self) -> ReleasedOldReachability {
        self.released_old_reachability
    }

    pub const fn reclaim_eligible(self) -> bool {
        self.released_old_reachability
            .footprint_basis()
            .protected_references()
            > 0
    }
}

fn same_physical_identity_ignoring_generation(
    old: PhysicalGenerationOwner,
    reused: PhysicalGenerationOwner,
) -> bool {
    old.domain() == reused.domain()
        && old.segment_id() == reused.segment_id()
        && old.page_id() == reused.page_id()
        && old.extent_id() == reused.extent_id()
        && old.slot() == reused.slot()
        && old.root_reference() == reused.root_reference()
        && old.allocation_class() == reused.allocation_class()
}
