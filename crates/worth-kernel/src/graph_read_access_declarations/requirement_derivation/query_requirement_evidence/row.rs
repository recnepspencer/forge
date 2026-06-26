use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessMemoryEstimateBasis, ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessRequirementKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadQueryRequirementRowEvidence {
    kind: ForgeQueryGraphReadAccessRequirementKind,
    semantic_slot_key: String,
    digest_part: String,
    rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis,
    invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis,
    complexity_contract: ForgeQueryGraphReadAccessComplexityContract,
    memory_estimate_basis: ForgeQueryGraphReadAccessMemoryEstimateBasis,
}

impl WorthGraphReadQueryRequirementRowEvidence {
    pub fn kind(&self) -> &ForgeQueryGraphReadAccessRequirementKind {
        &self.kind
    }

    pub fn semantic_slot_key(&self) -> &str {
        &self.semantic_slot_key
    }

    pub fn digest_part(&self) -> &str {
        &self.digest_part
    }

    pub fn rebuild_basis(&self) -> &ForgeQueryGraphReadAccessRebuildBasis {
        &self.rebuild_basis
    }

    pub fn invalidation_basis(&self) -> &ForgeQueryGraphReadAccessInvalidationBasis {
        &self.invalidation_basis
    }

    pub fn complexity_contract(&self) -> &ForgeQueryGraphReadAccessComplexityContract {
        &self.complexity_contract
    }

    pub fn memory_estimate_basis(&self) -> &ForgeQueryGraphReadAccessMemoryEstimateBasis {
        &self.memory_estimate_basis
    }
}
