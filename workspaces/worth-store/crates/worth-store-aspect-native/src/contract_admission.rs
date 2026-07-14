use worth_foundational::{AspectContract, AspectKey, AspectMask, DiagnosticMask, MutationMask};

use crate::{StoreAspectIdentity, StoreAspectNativeDenial, StorePhysicalBoundaryWitness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectContractAdmission {
    identity: StoreAspectIdentity,
    contract: AspectContract,
    mutation_mask: Option<AspectMask<MutationMask>>,
    diagnostic_mask: Option<AspectMask<DiagnosticMask>>,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreAspectContractAdmission {
    pub fn new(
        identity: StoreAspectIdentity,
        contract: AspectContract,
        physical_witness: StorePhysicalBoundaryWitness,
    ) -> Result<Self, StoreAspectNativeDenial> {
        if identity.aspect_key() != contract.key() {
            return Err(StoreAspectNativeDenial::IdentityMismatch);
        }

        Ok(Self {
            identity,
            contract,
            mutation_mask: None,
            diagnostic_mask: None,
            physical_witness,
        })
    }

    pub fn with_mutation_mask(mut self, mask: AspectMask<MutationMask>) -> Self {
        self.mutation_mask = Some(mask);
        self
    }

    pub fn with_diagnostic_mask(mut self, mask: AspectMask<DiagnosticMask>) -> Self {
        self.diagnostic_mask = Some(mask);
        self
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn contract(&self) -> &AspectContract {
        &self.contract
    }

    pub fn aspect_key(&self) -> &AspectKey {
        self.contract.key()
    }

    pub const fn mutation_mask(&self) -> Option<&AspectMask<MutationMask>> {
        self.mutation_mask.as_ref()
    }

    pub const fn diagnostic_mask(&self) -> Option<&AspectMask<DiagnosticMask>> {
        self.diagnostic_mask.as_ref()
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}
