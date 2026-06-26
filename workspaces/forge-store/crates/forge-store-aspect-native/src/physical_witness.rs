use forge_store_contracts::{PhysicalAuthorityScope, StorePhysicalAuthorityWitness};

use crate::StoreAspectNativeDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorePhysicalBoundaryWitness {
    authority: StorePhysicalAuthorityWitness,
}

impl StorePhysicalBoundaryWitness {
    pub fn from_physical_authority(
        authority: StorePhysicalAuthorityWitness,
    ) -> Result<Self, StoreAspectNativeDenial> {
        if authority.authority_scope() != PhysicalAuthorityScope::AspectNativeBoundaryVocabulary {
            return Err(StoreAspectNativeDenial::PhysicalAuthorityScopeMismatch(
                authority.authority_scope(),
            ));
        }

        Ok(Self { authority })
    }

    pub const fn authority(&self) -> StorePhysicalAuthorityWitness {
        self.authority
    }
}
