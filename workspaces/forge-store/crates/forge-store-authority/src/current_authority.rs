use forge_store_aspect_native::{
    StoreAspectBoundaryFact, StoreAspectIdentity, StorePhysicalBoundaryWitness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreCurrentAuthorityWitness {
    boundary_fact: StoreAspectBoundaryFact,
}

impl StoreCurrentAuthorityWitness {
    pub(crate) const fn from_boundary_fact(boundary_fact: StoreAspectBoundaryFact) -> Self {
        Self { boundary_fact }
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        self.boundary_fact.identity()
    }

    pub const fn boundary_fact(&self) -> &StoreAspectBoundaryFact {
        &self.boundary_fact
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.boundary_fact.authority_input().physical_witness()
    }

    pub const fn current_physical_authority(&self) -> StoreCurrentPhysicalAuthorityWitness<'_> {
        StoreCurrentPhysicalAuthorityWitness {
            identity: self.boundary_fact.identity(),
            physical_witness: self.boundary_fact.authority_input().physical_witness(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreCurrentPhysicalAuthorityWitness<'a> {
    identity: &'a StoreAspectIdentity,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreCurrentPhysicalAuthorityWitness<'_> {
    pub const fn identity(&self) -> &StoreAspectIdentity {
        self.identity
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}

pub fn require_current_store_authority(
    boundary_fact: StoreAspectBoundaryFact,
) -> StoreCurrentAuthorityWitness {
    StoreCurrentAuthorityWitness::from_boundary_fact(boundary_fact)
}

pub fn require_current_physical_authority(
    current_authority: &StoreCurrentAuthorityWitness,
) -> StoreCurrentPhysicalAuthorityWitness<'_> {
    current_authority.current_physical_authority()
}
