use forge_query::facade::{
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessMemoryEstimateBasis, ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessRequirementKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementVocabulary {
    requirement_kinds: Vec<ForgeQueryGraphReadAccessRequirementKind>,
    rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis,
    invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis,
    complexity_contract: ForgeQueryGraphReadAccessComplexityContract,
    memory_estimate_basis: ForgeQueryGraphReadAccessMemoryEstimateBasis,
}

impl WorthGraphReadRequirementVocabulary {
    pub fn relation_frontier() -> Self {
        Self::new(
            vec![
                ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
                ForgeQueryGraphReadAccessRequirementKind::TraversalWorkset,
                ForgeQueryGraphReadAccessRequirementKind::VisitedSet,
            ],
            ForgeQueryGraphReadAccessRebuildBasis::AuthoritativeRelationTruth,
            ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeRelationDelta,
            ForgeQueryGraphReadAccessComplexityContract::BoundedTraversalWorkset,
            ForgeQueryGraphReadAccessMemoryEstimateBasis::FrontierDepthBound,
        )
    }

    pub fn predicate_filtered_relation() -> Self {
        Self::new(
            vec![
                ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
                ForgeQueryGraphReadAccessRequirementKind::PredicateSupport,
                ForgeQueryGraphReadAccessRequirementKind::ResultBuffer,
            ],
            ForgeQueryGraphReadAccessRebuildBasis::SelectivityProof,
            ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
            ForgeQueryGraphReadAccessComplexityContract::CandidatePredicateSupport,
            ForgeQueryGraphReadAccessMemoryEstimateBasis::PredicateCandidateSet,
        )
    }

    fn new(
        mut requirement_kinds: Vec<ForgeQueryGraphReadAccessRequirementKind>,
        rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis,
        invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis,
        complexity_contract: ForgeQueryGraphReadAccessComplexityContract,
        memory_estimate_basis: ForgeQueryGraphReadAccessMemoryEstimateBasis,
    ) -> Self {
        requirement_kinds.sort();
        requirement_kinds.dedup();
        Self {
            requirement_kinds,
            rebuild_basis,
            invalidation_basis,
            complexity_contract,
            memory_estimate_basis,
        }
    }

    pub fn requirement_kinds(&self) -> &[ForgeQueryGraphReadAccessRequirementKind] {
        &self.requirement_kinds
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
