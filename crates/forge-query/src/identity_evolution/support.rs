use super::{
    contracts::IdentityEvolutionComplexityContract, families::LineageTraversalFamily,
};
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionDeferredScopeMarker {
    RecursiveTraversal,
    BroadCollectionDiscovery,
    StoreBackedParity,
}

impl IdentityEvolutionDeferredScopeMarker {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RecursiveTraversal => "recursive_traversal",
            Self::BroadCollectionDiscovery => "broad_collection_discovery",
            Self::StoreBackedParity => "store_backed_parity",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionSupportProfile {
    admitted_traversal_families: Vec<LineageTraversalFamily>,
    deferred_scope_markers: Vec<IdentityEvolutionDeferredScopeMarker>,
    complexity_contracts: Vec<IdentityEvolutionComplexityContract>,
    profile_digest: String,
}

impl IdentityEvolutionSupportProfile {
    pub fn admitted_traversal_families(&self) -> &[LineageTraversalFamily] {
        &self.admitted_traversal_families
    }

    pub fn deferred_scope_markers(&self) -> &[IdentityEvolutionDeferredScopeMarker] {
        &self.deferred_scope_markers
    }

    pub fn complexity_contracts(&self) -> &[IdentityEvolutionComplexityContract] {
        &self.complexity_contracts
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }
}

pub fn runtime_backed_direct_identity_evolution_support_profile(
) -> IdentityEvolutionSupportProfile {
    let admitted_traversal_families = vec![
        LineageTraversalFamily::DirectPredecessor,
        LineageTraversalFamily::DirectSuccessor,
        LineageTraversalFamily::DirectReplacement,
        LineageTraversalFamily::DirectSplitSuccessors,
        LineageTraversalFamily::DirectMergeSuccessor,
        LineageTraversalFamily::BranchLocalDirectEvolution,
    ];
    let complexity_contracts = admitted_traversal_families
        .iter()
        .copied()
        .map(IdentityEvolutionComplexityContract::direct_lineage)
        .collect::<Vec<_>>();
    let profile_digest = hash_parts(&[
        format!(
            "traversal:{}",
            admitted_traversal_families
                .iter()
                .map(LineageTraversalFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "deferred:{}",
            [
                IdentityEvolutionDeferredScopeMarker::RecursiveTraversal,
                IdentityEvolutionDeferredScopeMarker::BroadCollectionDiscovery,
                IdentityEvolutionDeferredScopeMarker::StoreBackedParity,
            ]
            .iter()
            .map(IdentityEvolutionDeferredScopeMarker::as_str)
            .collect::<Vec<_>>()
            .join(",")
        ),
        format!(
            "contracts:{}",
            complexity_contracts
                .iter()
                .map(|contract| contract.digest().as_str().to_string())
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]);
    IdentityEvolutionSupportProfile {
        admitted_traversal_families,
        deferred_scope_markers: vec![
            IdentityEvolutionDeferredScopeMarker::RecursiveTraversal,
            IdentityEvolutionDeferredScopeMarker::BroadCollectionDiscovery,
            IdentityEvolutionDeferredScopeMarker::StoreBackedParity,
        ],
        complexity_contracts,
        profile_digest,
    }
}
