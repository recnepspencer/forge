use forge_foundational::AuthoritativeRecordAspectStateArtifact;

use crate::{StoreAspectIdentity, StoreAspectNativeDenial, StorePhysicalBoundaryWitness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectAuthorityInput {
    admitted_state: AuthoritativeRecordAspectStateArtifact,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreAspectAuthorityInput {
    pub const fn new(
        admitted_state: AuthoritativeRecordAspectStateArtifact,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Self {
        Self {
            admitted_state,
            physical_witness,
        }
    }

    pub const fn admitted_state(&self) -> &AuthoritativeRecordAspectStateArtifact {
        &self.admitted_state
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectBoundaryFact {
    identity: StoreAspectIdentity,
    authority_input: StoreAspectAuthorityInput,
}

impl StoreAspectBoundaryFact {
    pub fn from_admitted_state(
        identity: StoreAspectIdentity,
        authority_input: StoreAspectAuthorityInput,
    ) -> Result<Self, StoreAspectNativeDenial> {
        if !admitted_state_contains_only_identity(authority_input.admitted_state(), &identity) {
            return Err(StoreAspectNativeDenial::IdentityMismatch);
        }

        Ok(Self {
            identity,
            authority_input,
        })
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn authority_input(&self) -> &StoreAspectAuthorityInput {
        &self.authority_input
    }
}

fn admitted_state_contains_only_identity(
    admitted_state: &AuthoritativeRecordAspectStateArtifact,
    identity: &StoreAspectIdentity,
) -> bool {
    admitted_state.payload().aspects().len() == 1
        && admitted_state
            .payload()
            .get(identity.aspect_key())
            .is_some()
}
