use crate::identity_evolution::{
    admit_identity_evolution_query_for_scenario, CorrespondenceIdentityComparison,
    IdentityEvolutionComparisonBasisFamily, IdentityEvolutionQueryContext,
    IdentityEvolutionSyntheticScenario, LineageTraversalDescriptor,
};

use super::super::lane::IdentityEvolutionCertificationRejection;
use super::identity_inputs::{basis_digest, query_digest};

pub(super) fn unsupported_lineage_rejection() -> IdentityEvolutionCertificationRejection {
    let query_digest = query_digest("unsupported-lineage-traversal-family");
    let basis_digest = basis_digest("basis:unsupported-lineage");
    let error = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::lineage_traversal_for_test(
            query_digest.clone(),
            basis_digest.clone(),
            LineageTraversalDescriptor::direct_predecessor("entity:lineage"),
        ),
        IdentityEvolutionSyntheticScenario::UnsupportedLineageTraversal,
    )
    .expect_err("unsupported lineage traversal marker should deny");
    IdentityEvolutionCertificationRejection::from_admission_error(
        &error,
        &query_digest,
        &basis_digest,
    )
}

pub(super) fn unsupported_comparison_rejection() -> IdentityEvolutionCertificationRejection {
    let query_digest = query_digest("unsupported-correspondence-family");
    let left_basis = basis_digest("basis:unsupported-left");
    let right_basis = basis_digest("basis:unsupported-right");
    let error = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
            query_digest.clone(),
            IdentityEvolutionComparisonBasisFamily::BranchToBranch,
            left_basis.clone(),
            right_basis.clone(),
            CorrespondenceIdentityComparison::advisory_between("entity:left", "entity:right"),
        ),
        IdentityEvolutionSyntheticScenario::UnsupportedComparisonFamily,
    )
    .expect_err("unsupported comparison marker should deny");
    IdentityEvolutionCertificationRejection::from_admission_error(
        &error,
        &query_digest,
        &left_basis,
    )
}
