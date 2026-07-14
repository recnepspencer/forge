use worth_foundational::ContractValidatedAspectArtifact;

use crate::{StoreAspectIdentity, StoreAspectNativeDenial, StorePhysicalBoundaryWitness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreValidatedAspectValueAdmission {
    identity: StoreAspectIdentity,
    validated_value: ContractValidatedAspectArtifact,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreValidatedAspectValueAdmission {
    pub fn new(
        identity: StoreAspectIdentity,
        validated_value: ContractValidatedAspectArtifact,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Result<Self, StoreAspectNativeDenial> {
        if identity.aspect_key() != validated_value.payload().key() {
            return Err(StoreAspectNativeDenial::IdentityMismatch);
        }

        Ok(Self {
            identity,
            validated_value,
            physical_witness,
        })
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn validated_value(&self) -> &ContractValidatedAspectArtifact {
        &self.validated_value
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}
