use super::super::{
    BranchLocalityClass, IdentityEvolutionComplexityContract, IdentityEvolutionComplexityReport,
    IdentityEvolutionMetadata, IdentityEvolutionOutcomeFamily, PromotionOrMergeAuthorityState,
};
use crate::identity::{BasisDigest, CanonicalQueryDigest, LineageDigest};

pub(super) fn query_digest(label: &str) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("query:{label}")])
}

pub(super) fn basis_digest(label: &str) -> BasisDigest {
    BasisDigest::from_parts(&[format!("basis:{label}")])
}

pub(super) fn lineage_digest(label: &str) -> LineageDigest {
    LineageDigest::from_parts(&[format!("lineage:{label}")])
}

pub(super) fn metadata(
    outcome_family: IdentityEvolutionOutcomeFamily,
    contract: IdentityEvolutionComplexityContract,
    branch_locality_class: BranchLocalityClass,
) -> IdentityEvolutionMetadata {
    IdentityEvolutionMetadata::from_parts(
        query_digest("phase-1"),
        basis_digest("read"),
        lineage_digest("direct"),
        outcome_family,
        basis_digest("anchor-branch"),
        basis_digest("origin-branch"),
        basis_digest("divergence-root"),
        branch_locality_class,
        PromotionOrMergeAuthorityState::NotRequired,
        IdentityEvolutionComplexityReport::from_contract(contract),
    )
}
