use worth_store_aspect_native::{
    StoreAspectBindingStamp, StoreAspectBoundaryFact, StoreAspectContractAdmission,
    StoreAspectContractStamp, StoreAspectIdentity, StoreAspectPatchBoundaryFact,
    StoreCanonicalBasisFamily, StoreDigestEquivalenceBasis, StoreEquivalenceBasisIdentity,
    StorePhysicalBoundaryWitness,
};

/// Admitted Store-native meaning carried by one physical work declaration.
///
/// The variants preserve whether the operation projects authoritative state
/// or applies an authoritative patch. Raw Foundational values and Signal masks
/// have no construction path into this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkSemanticBasis {
    kind: PhysicalWorkSemanticBasisKind,
    retained_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PhysicalWorkSemanticBasisKind {
    Projection {
        fact: StoreAspectBoundaryFact,
        aspect: StoreAspectIdentity,
        contract: StoreAspectContractStamp,
        binding: StoreAspectBindingStamp,
        canonical_basis: StoreEquivalenceBasisIdentity,
    },
    Mutation {
        patch: StoreAspectPatchBoundaryFact,
        aspect: StoreAspectIdentity,
        contract: StoreAspectContractStamp,
        binding: StoreAspectBindingStamp,
        canonical_basis: StoreEquivalenceBasisIdentity,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkSemanticBasisDenial {
    AspectIdentityMismatch,
    ContractRevisionMismatch,
    ContractCanonicalMismatch,
    InconsistentPatchContracts,
    MutationMaskMismatch,
    PhysicalWitnessMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkSemanticPosture {
    Projection,
    Mutation,
}

impl PhysicalWorkSemanticBasis {
    pub fn projection(
        fact: StoreAspectBoundaryFact,
        contract: StoreAspectContractAdmission,
    ) -> Result<Self, PhysicalWorkSemanticBasisDenial> {
        require_matching_projection(&fact, &contract)?;
        let aspect = contract.identity().clone();
        let binding = contract.binding_stamp();
        let contract = contract.contract_stamp();
        let canonical_basis = StoreDigestEquivalenceBasis::exact_native_basis(
            StoreCanonicalBasisFamily::AspectBoundaryFact,
        )
        .identity();
        let retained_bytes = fact.semantic_byte_width();
        Ok(Self {
            kind: PhysicalWorkSemanticBasisKind::Projection {
                fact,
                aspect,
                contract,
                binding,
                canonical_basis,
            },
            retained_bytes,
        })
    }

    pub fn mutation(
        patch: StoreAspectPatchBoundaryFact,
        contract: StoreAspectContractAdmission,
    ) -> Result<Self, PhysicalWorkSemanticBasisDenial> {
        require_matching_mutation(&patch, &contract)?;
        let aspect = contract.identity().clone();
        let binding = contract.binding_stamp();
        let contract = contract.contract_stamp();
        let canonical_basis = StoreDigestEquivalenceBasis::exact_native_basis(
            StoreCanonicalBasisFamily::AspectPatchBoundaryFact,
        )
        .identity();
        let retained_bytes = patch.semantic_byte_width();
        Ok(Self {
            kind: PhysicalWorkSemanticBasisKind::Mutation {
                patch,
                aspect,
                contract,
                binding,
                canonical_basis,
            },
            retained_bytes,
        })
    }

    pub const fn aspect_identity(&self) -> &StoreAspectIdentity {
        match &self.kind {
            PhysicalWorkSemanticBasisKind::Projection { aspect, .. }
            | PhysicalWorkSemanticBasisKind::Mutation { aspect, .. } => aspect,
        }
    }

    pub const fn canonical_basis(&self) -> StoreEquivalenceBasisIdentity {
        match &self.kind {
            PhysicalWorkSemanticBasisKind::Projection {
                canonical_basis, ..
            }
            | PhysicalWorkSemanticBasisKind::Mutation {
                canonical_basis, ..
            } => *canonical_basis,
        }
    }

    pub const fn posture(&self) -> PhysicalWorkSemanticPosture {
        match &self.kind {
            PhysicalWorkSemanticBasisKind::Projection { .. } => {
                PhysicalWorkSemanticPosture::Projection
            }
            PhysicalWorkSemanticBasisKind::Mutation { .. } => PhysicalWorkSemanticPosture::Mutation,
        }
    }

    pub const fn contract_stamp(&self) -> StoreAspectContractStamp {
        match &self.kind {
            PhysicalWorkSemanticBasisKind::Projection { contract, .. }
            | PhysicalWorkSemanticBasisKind::Mutation { contract, .. } => *contract,
        }
    }

    pub const fn binding_stamp(&self) -> StoreAspectBindingStamp {
        match &self.kind {
            PhysicalWorkSemanticBasisKind::Projection { binding, .. }
            | PhysicalWorkSemanticBasisKind::Mutation { binding, .. } => *binding,
        }
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        match &self.kind {
            PhysicalWorkSemanticBasisKind::Projection { fact, .. } => {
                fact.authority_input().physical_witness()
            }
            PhysicalWorkSemanticBasisKind::Mutation { patch, .. } => {
                patch.patch_input().physical_witness()
            }
        }
    }

    pub const fn projection_fact(&self) -> Option<&StoreAspectBoundaryFact> {
        match &self.kind {
            PhysicalWorkSemanticBasisKind::Projection { fact, .. } => Some(fact),
            PhysicalWorkSemanticBasisKind::Mutation { .. } => None,
        }
    }

    pub const fn mutation_patch(&self) -> Option<&StoreAspectPatchBoundaryFact> {
        match &self.kind {
            PhysicalWorkSemanticBasisKind::Projection { .. } => None,
            PhysicalWorkSemanticBasisKind::Mutation { patch, .. } => Some(patch),
        }
    }

    pub const fn semantic_byte_width(&self) -> usize {
        self.retained_bytes
    }
}

fn require_matching_projection(
    fact: &StoreAspectBoundaryFact,
    contract: &StoreAspectContractAdmission,
) -> Result<(), PhysicalWorkSemanticBasisDenial> {
    if fact.identity() != contract.identity() {
        return Err(PhysicalWorkSemanticBasisDenial::AspectIdentityMismatch);
    }
    if fact.authority_input().physical_witness() != contract.physical_witness() {
        return Err(PhysicalWorkSemanticBasisDenial::PhysicalWitnessMismatch);
    }
    require_matching_contract(fact.contract_stamp(), contract.contract_stamp())
}

fn require_matching_mutation(
    patch: &StoreAspectPatchBoundaryFact,
    contract: &StoreAspectContractAdmission,
) -> Result<(), PhysicalWorkSemanticBasisDenial> {
    if patch.identity() != contract.identity() {
        return Err(PhysicalWorkSemanticBasisDenial::AspectIdentityMismatch);
    }
    if patch.patch_input().physical_witness() != contract.physical_witness() {
        return Err(PhysicalWorkSemanticBasisDenial::PhysicalWitnessMismatch);
    }
    let Some(mutation_mask) = contract.mutation_mask() else {
        return Err(PhysicalWorkSemanticBasisDenial::MutationMaskMismatch);
    };
    if !patch.is_within_mutation_mask(mutation_mask) {
        return Err(PhysicalWorkSemanticBasisDenial::MutationMaskMismatch);
    }
    let patch_stamp = patch
        .contract_stamp()
        .ok_or(PhysicalWorkSemanticBasisDenial::InconsistentPatchContracts)?;
    require_matching_contract(patch_stamp, contract.contract_stamp())
}

fn require_matching_contract(
    fact: StoreAspectContractStamp,
    admitted: StoreAspectContractStamp,
) -> Result<(), PhysicalWorkSemanticBasisDenial> {
    if fact.identity() != admitted.identity() || fact.revision() != admitted.revision() {
        return Err(PhysicalWorkSemanticBasisDenial::ContractRevisionMismatch);
    }
    if fact.canonical_fingerprint() != admitted.canonical_fingerprint() {
        return Err(PhysicalWorkSemanticBasisDenial::ContractCanonicalMismatch);
    }
    Ok(())
}
