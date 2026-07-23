use worth_signal::facade::{Aspect, ChangedRegion};
use worth_store_aspect_native::{StoreAspectBoundaryFact, StoreAspectPatchBoundaryFact};

use super::{PhysicalSignalAspectBinding, PhysicalSignalAspectBindingDigest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkAspectDeltaDenial {
    MutationMaskAbsent,
    MutationMaskMismatch,
    AspectIdentityMismatch,
    ContractRevisionMismatch,
    ContractCanonicalMismatch,
    BindingWitnessMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkAspectDelta {
    aspect: Aspect,
    regions: Box<[ChangedRegion]>,
    binding: PhysicalSignalAspectBindingDigest,
}

impl PhysicalWorkAspectDelta {
    pub fn from_boundary_fact(
        binding: &PhysicalSignalAspectBinding,
        fact: &StoreAspectBoundaryFact,
    ) -> Result<Self, PhysicalWorkAspectDeltaDenial> {
        require_mutation_authority(binding)?;
        require_fact_binding(binding, fact)?;
        Ok(Self::from_binding(binding))
    }

    pub fn from_patch_boundary_fact(
        binding: &PhysicalSignalAspectBinding,
        patch: &StoreAspectPatchBoundaryFact,
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
        Ok(Self::from_binding(binding))
    }

    pub const fn binding(&self) -> PhysicalSignalAspectBindingDigest {
        self.binding
    }

    pub const fn is_partitioned(&self) -> bool {
        !self.regions.is_empty()
    }

    pub(in crate::physical_runtime) const fn signal_aspect(&self) -> Aspect {
        self.aspect
    }

    pub(in crate::physical_runtime) const fn regions(&self) -> &[ChangedRegion] {
        &self.regions
    }

    fn from_binding(binding: &PhysicalSignalAspectBinding) -> Self {
        let regions = binding
            .partition()
            .map(|partition| ChangedRegion {
                partition: partition.partition.clone(),
                detail: partition.detail.clone(),
            })
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            aspect: binding.signal_aspect(),
            regions,
            binding: binding.digest(),
        }
    }
}

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
