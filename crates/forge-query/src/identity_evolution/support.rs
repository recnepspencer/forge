use super::{
    contracts::IdentityEvolutionComplexityContract, families::LineageTraversalFamily,
    inspector::InspectorIdentityClassification, request::IdentityEvolutionComparisonBasisFamily,
};
use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionDeferredScopeMarker {
    RecursiveTraversal,
    BroadCollectionDiscovery,
    StoreBackedParity,
    IdentityAwareNonInspectorViews,
}

impl IdentityEvolutionDeferredScopeMarker {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RecursiveTraversal => "recursive_traversal",
            Self::BroadCollectionDiscovery => "broad_collection_discovery",
            Self::StoreBackedParity => "store_backed_parity",
            Self::IdentityAwareNonInspectorViews => "identity_aware_non_inspector_views",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionSupportProfile {
    admitted_traversal_families: Vec<LineageTraversalFamily>,
    admitted_comparison_basis_families: Vec<IdentityEvolutionComparisonBasisFamily>,
    deferred_scope_markers: Vec<IdentityEvolutionDeferredScopeMarker>,
    lineage_complexity_contracts: Vec<IdentityEvolutionComplexityContract>,
    comparison_complexity_contracts: Vec<IdentityEvolutionComplexityContract>,
    admitted_inspector_consumable_identity_classifications: Vec<InspectorIdentityClassification>,
    profile_digest: String,
}

impl IdentityEvolutionSupportProfile {
    pub fn admitted_traversal_families(&self) -> &[LineageTraversalFamily] {
        &self.admitted_traversal_families
    }

    pub fn admitted_comparison_basis_families(&self) -> &[IdentityEvolutionComparisonBasisFamily] {
        &self.admitted_comparison_basis_families
    }

    pub fn deferred_scope_markers(&self) -> &[IdentityEvolutionDeferredScopeMarker] {
        &self.deferred_scope_markers
    }

    pub fn lineage_complexity_contracts(&self) -> &[IdentityEvolutionComplexityContract] {
        &self.lineage_complexity_contracts
    }

    pub fn comparison_complexity_contracts(&self) -> &[IdentityEvolutionComplexityContract] {
        &self.comparison_complexity_contracts
    }

    pub fn admitted_inspector_consumable_identity_classifications(
        &self,
    ) -> &[InspectorIdentityClassification] {
        &self.admitted_inspector_consumable_identity_classifications
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }
}

pub fn runtime_backed_direct_identity_evolution_support_profile() -> IdentityEvolutionSupportProfile
{
    let admitted_traversal_families = vec![
        LineageTraversalFamily::DirectPredecessor,
        LineageTraversalFamily::DirectSuccessor,
        LineageTraversalFamily::DirectReplacement,
        LineageTraversalFamily::DirectSplitSuccessors,
        LineageTraversalFamily::DirectMergeSuccessor,
        LineageTraversalFamily::BranchLocalDirectEvolution,
    ];
    let admitted_comparison_basis_families = vec![
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        IdentityEvolutionComparisonBasisFamily::CurrentToHistorical,
        IdentityEvolutionComparisonBasisFamily::HistoricalToHistorical,
        IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative,
    ];
    let lineage_complexity_contracts = admitted_traversal_families
        .iter()
        .copied()
        .map(IdentityEvolutionComplexityContract::direct_lineage)
        .collect::<Vec<_>>();
    let comparison_complexity_contracts = admitted_comparison_basis_families
        .iter()
        .copied()
        .map(IdentityEvolutionComplexityContract::correspondence_identity_comparison)
        .collect::<Vec<_>>();
    let admitted_inspector_consumable_identity_classifications = vec![
        InspectorIdentityClassification::IdentitySummary,
        InspectorIdentityClassification::AuthoritativeContinuity,
        InspectorIdentityClassification::AdvisoryCandidates,
        InspectorIdentityClassification::Ambiguity,
        InspectorIdentityClassification::IdentityBreak,
        InspectorIdentityClassification::Denied,
    ];
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
            "comparison_basis:{}",
            admitted_comparison_basis_families
                .iter()
                .map(IdentityEvolutionComparisonBasisFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "deferred:{}",
            [
                IdentityEvolutionDeferredScopeMarker::RecursiveTraversal,
                IdentityEvolutionDeferredScopeMarker::BroadCollectionDiscovery,
                IdentityEvolutionDeferredScopeMarker::StoreBackedParity,
                IdentityEvolutionDeferredScopeMarker::IdentityAwareNonInspectorViews,
            ]
            .iter()
            .map(IdentityEvolutionDeferredScopeMarker::as_str)
            .collect::<Vec<_>>()
            .join(",")
        ),
        format!(
            "lineage_contracts:{}",
            lineage_complexity_contracts
                .iter()
                .map(|contract| contract.digest().as_str().to_string())
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "comparison_contracts:{}",
            comparison_complexity_contracts
                .iter()
                .map(|contract| contract.digest().as_str().to_string())
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "inspector_classifications:{}",
            admitted_inspector_consumable_identity_classifications
                .iter()
                .map(InspectorIdentityClassification::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]);
    IdentityEvolutionSupportProfile {
        admitted_traversal_families,
        admitted_comparison_basis_families,
        deferred_scope_markers: vec![
            IdentityEvolutionDeferredScopeMarker::RecursiveTraversal,
            IdentityEvolutionDeferredScopeMarker::BroadCollectionDiscovery,
            IdentityEvolutionDeferredScopeMarker::StoreBackedParity,
            IdentityEvolutionDeferredScopeMarker::IdentityAwareNonInspectorViews,
        ],
        lineage_complexity_contracts,
        comparison_complexity_contracts,
        admitted_inspector_consumable_identity_classifications,
        profile_digest,
    }
}
