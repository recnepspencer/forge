use forge_store_physical_format::PhysicalGenerationOwner;

use crate::{CurrentGenerationPhysicalReference, PhysicalOrderingContract, PhysicalOrderingSite};

use super::FreeReuseFenceDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenerationAdvanceReceipt {
    old_identity: CurrentGenerationPhysicalReference,
    reused_identity: CurrentGenerationPhysicalReference,
    ordering: PhysicalOrderingContract,
}

impl GenerationAdvanceReceipt {
    pub fn from_identity_reuse(
        old_identity: CurrentGenerationPhysicalReference,
        reused_identity: CurrentGenerationPhysicalReference,
        ordering: PhysicalOrderingContract,
    ) -> Result<Self, FreeReuseFenceDenial> {
        let ordering = ordering
            .require_site(PhysicalOrderingSite::GenerationAdvancement)
            .map_err(|_| FreeReuseFenceDenial::GenerationAdvancementOrderingNotCrashStable)?;
        if !same_physical_identity_ignoring_generation(
            old_identity.owner(),
            reused_identity.owner(),
        ) {
            return Err(FreeReuseFenceDenial::IdentityReuseOwnerMismatch);
        }
        if reused_identity.generation().get() <= old_identity.generation().get() {
            return Err(FreeReuseFenceDenial::IdentityReuseWithoutGenerationAdvance);
        }
        Ok(Self {
            old_identity,
            reused_identity,
            ordering,
        })
    }

    pub const fn old_identity(self) -> CurrentGenerationPhysicalReference {
        self.old_identity
    }

    pub const fn reused_identity(self) -> CurrentGenerationPhysicalReference {
        self.reused_identity
    }

    pub const fn ordering(self) -> PhysicalOrderingContract {
        self.ordering
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
