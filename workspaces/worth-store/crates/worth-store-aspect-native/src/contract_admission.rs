use worth_foundational::{
    AspectContract, AspectKey, AspectMask, ContractValidatedAspectValue, DiagnosticMask,
    FieldLevelAspectPatch, MutationMask, ProjectionMask,
};

use crate::{StoreAspectIdentity, StoreAspectNativeDenial, StorePhysicalBoundaryWitness};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectContractAdmission {
    identity: StoreAspectIdentity,
    contract: AspectContract,
    projection_mask: Option<AspectMask<ProjectionMask>>,
    mutation_mask: Option<AspectMask<MutationMask>>,
    diagnostic_mask: Option<AspectMask<DiagnosticMask>>,
    physical_witness: StorePhysicalBoundaryWitness,
}

/// Compact, copyable identity of one exact admitted aspect contract revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreAspectContractStamp {
    identity: u64,
    revision: u64,
    canonical_fingerprint: [u8; 32],
}

/// Compact identity of the exact contract, selected Store masks, and physical
/// boundary installed as one Signal aspect binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreAspectBindingStamp([u8; 32]);

impl StoreAspectBindingStamp {
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl StoreAspectContractStamp {
    pub const fn identity(self) -> u64 {
        self.identity
    }

    pub const fn revision(self) -> u64 {
        self.revision
    }

    pub const fn canonical_fingerprint(self) -> [u8; 32] {
        self.canonical_fingerprint
    }

    pub(crate) fn from_contract(contract: &AspectContract) -> Self {
        Self {
            identity: contract.identity().0,
            revision: contract.revision().0,
            canonical_fingerprint: crate::contract_fingerprint::canonical_contract_fingerprint(
                contract,
            ),
        }
    }

    pub(crate) fn from_validated_value(value: &ContractValidatedAspectValue) -> Self {
        Self::from_contract(value.contract())
    }

    pub(crate) fn from_field_patch(patch: &FieldLevelAspectPatch) -> Self {
        Self::from_contract(patch.contract())
    }
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
            projection_mask: None,
            mutation_mask: None,
            diagnostic_mask: None,
            physical_witness,
        })
    }

    pub fn admit_projection_mask(
        mut self,
        mask: AspectMask<ProjectionMask>,
    ) -> Result<Self, StoreAspectNativeDenial> {
        self.contract
            .admits_projection_mask(&mask)
            .map_err(|_| StoreAspectNativeDenial::ProjectionMaskNotAdmitted)?;
        self.projection_mask = Some(mask);
        Ok(self)
    }

    pub fn with_mutation_mask(
        mut self,
        mask: AspectMask<MutationMask>,
    ) -> Result<Self, StoreAspectNativeDenial> {
        self.contract
            .admits_mutation_mask(&mask)
            .map_err(|_| StoreAspectNativeDenial::MutationMaskNotAdmitted)?;
        self.mutation_mask = Some(mask);
        Ok(self)
    }

    pub fn admit_mutation_mask(
        self,
        mask: AspectMask<MutationMask>,
    ) -> Result<Self, StoreAspectNativeDenial> {
        self.with_mutation_mask(mask)
    }

    pub fn with_diagnostic_mask(
        mut self,
        mask: AspectMask<DiagnosticMask>,
    ) -> Result<Self, StoreAspectNativeDenial> {
        self.contract
            .admits_diagnostic_mask(&mask)
            .map_err(|_| StoreAspectNativeDenial::DiagnosticMaskNotAdmitted)?;
        self.diagnostic_mask = Some(mask);
        Ok(self)
    }

    pub fn admit_diagnostic_mask(
        self,
        mask: AspectMask<DiagnosticMask>,
    ) -> Result<Self, StoreAspectNativeDenial> {
        self.with_diagnostic_mask(mask)
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

    pub const fn projection_mask(&self) -> Option<&AspectMask<ProjectionMask>> {
        self.projection_mask.as_ref()
    }

    pub const fn diagnostic_mask(&self) -> Option<&AspectMask<DiagnosticMask>> {
        self.diagnostic_mask.as_ref()
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }

    pub fn contract_stamp(&self) -> StoreAspectContractStamp {
        StoreAspectContractStamp::from_contract(&self.contract)
    }

    pub fn binding_stamp(&self) -> StoreAspectBindingStamp {
        StoreAspectBindingStamp(crate::contract_fingerprint::canonical_binding_fingerprint(
            self.contract_stamp(),
            self.physical_witness,
            self.projection_mask.as_ref(),
            self.mutation_mask.as_ref(),
            self.diagnostic_mask.as_ref(),
        ))
    }
}
