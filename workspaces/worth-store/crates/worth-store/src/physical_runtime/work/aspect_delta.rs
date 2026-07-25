use std::sync::Arc;
use worth_store_aspect_native::{StoreAspectBoundaryFact, StoreAspectPatchBoundaryFact};

use super::{PhysicalSignalAspectBinding, PhysicalSignalAspectBindingDigest, PhysicalWorkScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkAspectDeltaDenial {
    MutationMaskAbsent,
    MutationMaskMismatch,
    AspectIdentityMismatch,
    ContractRevisionMismatch,
    ContractCanonicalMismatch,
    BindingWitnessMismatch,
}

#[derive(Debug, Clone)]
pub struct PhysicalWorkAspectDelta {
    binding: PhysicalSignalAspectBindingDigest,
    scope: PhysicalWorkScope,
    partitioned: bool,
    binding_capability: Arc<()>,
}

impl PhysicalWorkAspectDelta {
    pub fn from_boundary_fact(
        binding: &PhysicalSignalAspectBinding,
        fact: &StoreAspectBoundaryFact,
        scope: PhysicalWorkScope,
    ) -> Result<Self, PhysicalWorkAspectDeltaDenial> {
        require_mutation_authority(binding)?;
        require_fact_binding(binding, fact)?;
        Ok(Self::from_binding(binding, scope))
    }

    pub fn from_patch_boundary_fact(
        binding: &PhysicalSignalAspectBinding,
        patch: &StoreAspectPatchBoundaryFact,
        scope: PhysicalWorkScope,
    ) -> Result<Self, PhysicalWorkAspectDeltaDenial> {
        let mutation_mask = binding
            .contract()
            .mutation_mask()
            .ok_or(PhysicalWorkAspectDeltaDenial::MutationMaskAbsent)?;
        if binding.identity() != patch.identity() {
            return Err(PhysicalWorkAspectDeltaDenial::AspectIdentityMismatch);
        }
        if binding.contract().physical_witness() != patch.patch_input().physical_witness() {
            return Err(PhysicalWorkAspectDeltaDenial::BindingWitnessMismatch);
        }
        let patch_contract = patch
            .contract_stamp()
            .ok_or(PhysicalWorkAspectDeltaDenial::ContractRevisionMismatch)?;
        require_contract(binding, patch_contract)?;
        if !patch.is_within_mutation_mask(mutation_mask) {
            return Err(PhysicalWorkAspectDeltaDenial::MutationMaskMismatch);
        }
        Ok(Self::from_binding(binding, scope))
    }

    pub const fn binding(&self) -> PhysicalSignalAspectBindingDigest {
        self.binding
    }

    pub const fn is_partitioned(&self) -> bool {
        self.partitioned
    }

    pub(in crate::physical_runtime) const fn scope(&self) -> &PhysicalWorkScope {
        &self.scope
    }

    pub(in crate::physical_runtime) fn is_installed_by(
        &self,
        binding: &PhysicalSignalAspectBinding,
    ) -> bool {
        binding.digest() == self.binding && binding.installs(&self.binding_capability)
    }

    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime) fn rebind_for_certification(
        mut self,
        binding: &PhysicalSignalAspectBinding,
    ) -> Result<Self, PhysicalWorkAspectDeltaDenial> {
        if self.binding != binding.digest() {
            return Err(PhysicalWorkAspectDeltaDenial::AspectIdentityMismatch);
        }
        self.binding_capability = binding.capability();
        self.partitioned = binding.partition().is_some();
        Ok(self)
    }

    fn from_binding(binding: &PhysicalSignalAspectBinding, scope: PhysicalWorkScope) -> Self {
        Self {
            binding: binding.digest(),
            scope,
            partitioned: binding.partition().is_some(),
            binding_capability: binding.capability(),
        }
    }
}

impl PartialEq for PhysicalWorkAspectDelta {
    fn eq(&self, other: &Self) -> bool {
        self.binding == other.binding
            && self.scope == other.scope
            && self.partitioned == other.partitioned
            && Arc::ptr_eq(&self.binding_capability, &other.binding_capability)
    }
}

impl Eq for PhysicalWorkAspectDelta {}

fn require_mutation_authority(
    binding: &PhysicalSignalAspectBinding,
) -> Result<(), PhysicalWorkAspectDeltaDenial> {
    binding
        .contract()
        .mutation_mask()
        .ok_or(PhysicalWorkAspectDeltaDenial::MutationMaskAbsent)
        .map(|_| ())
}

fn require_fact_binding(
    binding: &PhysicalSignalAspectBinding,
    fact: &StoreAspectBoundaryFact,
) -> Result<(), PhysicalWorkAspectDeltaDenial> {
    if binding.identity() != fact.identity() {
        return Err(PhysicalWorkAspectDeltaDenial::AspectIdentityMismatch);
    }
    if binding.contract().physical_witness() != fact.authority_input().physical_witness() {
        return Err(PhysicalWorkAspectDeltaDenial::BindingWitnessMismatch);
    }
    require_contract(binding, fact.contract_stamp())
}

fn require_contract(
    binding: &PhysicalSignalAspectBinding,
    fact: worth_store_aspect_native::StoreAspectContractStamp,
) -> Result<(), PhysicalWorkAspectDeltaDenial> {
    let admitted = binding.contract().contract_stamp();
    if fact.identity() != admitted.identity() || fact.revision() != admitted.revision() {
        return Err(PhysicalWorkAspectDeltaDenial::ContractRevisionMismatch);
    }
    if fact.canonical_fingerprint() != admitted.canonical_fingerprint() {
        return Err(PhysicalWorkAspectDeltaDenial::ContractCanonicalMismatch);
    }
    Ok(())
}
