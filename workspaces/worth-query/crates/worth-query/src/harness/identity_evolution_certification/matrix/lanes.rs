use crate::identity_evolution::{
    admit_identity_evolution_query_for_scenario, execute_admitted_identity_evolution_query,
    CorrespondenceIdentityComparison, IdentityEvolutionComparisonBasisFamily,
    IdentityEvolutionExecutionArtifact, IdentityEvolutionQueryContext,
    IdentityEvolutionSyntheticScenario, LineageTraversalDescriptor,
};

use super::super::lane::IdentityEvolutionCertificationLane;
use super::identity_inputs::{basis_digest, query_digest};

pub(super) fn replacement_lane() -> IdentityEvolutionCertificationLane {
    execute_lineage(
        "replacement-lineage-traversal",
        "basis:current",
        LineageTraversalDescriptor::direct_replacement("entity:replacement"),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

pub(super) fn split_lane() -> IdentityEvolutionCertificationLane {
    execute_lineage(
        "split-successor-lineage-traversal",
        "basis:current",
        LineageTraversalDescriptor::direct_split_successors("entity:split"),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

pub(super) fn branch_local_lane() -> IdentityEvolutionCertificationLane {
    execute_lineage(
        "branch-local-divergence-stays-local",
        "basis:branch-local",
        LineageTraversalDescriptor::branch_local_direct_evolution("entity:branch-local-divergence"),
        IdentityEvolutionSyntheticScenario::BranchLocalDivergence,
    )
}

pub(super) fn ambiguous_comparison_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "branch-to-branch-correspondence-ambiguous",
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        "basis:left-branch",
        "basis:right-branch",
        CorrespondenceIdentityComparison::advisory_between("entity:left", "entity:right"),
        IdentityEvolutionSyntheticScenario::AmbiguousCorrespondence,
    )
}

pub(super) fn identity_break_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "identity-break-explicit",
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        "basis:identity-break-left",
        "basis:identity-break-right",
        CorrespondenceIdentityComparison::authoritative_between("entity:left", "entity:right"),
        IdentityEvolutionSyntheticScenario::IdentityBreak,
    )
}

pub(super) fn branch_to_branch_authoritative_lane() -> IdentityEvolutionCertificationLane {
    IdentityEvolutionCertificationLane::from_execution_artifact(
        &branch_to_branch_authoritative_artifact(),
    )
}

pub(super) fn branch_to_branch_authoritative_bundle_inspector_lane(
) -> IdentityEvolutionCertificationLane {
    IdentityEvolutionCertificationLane::from_execution_artifact_with_bundle_inspector(
        &branch_to_branch_authoritative_artifact(),
    )
}

pub(super) fn current_to_historical_advisory_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "current-to-historical-advisory-comparison",
        IdentityEvolutionComparisonBasisFamily::CurrentToHistorical,
        "basis:current",
        "basis:historical",
        CorrespondenceIdentityComparison::advisory_between("entity:current", "entity:historical"),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

pub(super) fn preview_to_authoritative_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "preview-to-authoritative-identity-comparison",
        IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative,
        "basis:preview",
        "basis:authoritative",
        CorrespondenceIdentityComparison::authoritative_between(
            "entity:preview",
            "entity:authoritative",
        ),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

pub(super) fn execute_lineage(
    query_seed: &str,
    basis_seed: &str,
    descriptor: LineageTraversalDescriptor,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionCertificationLane {
    let artifact = execute_artifact_for_lineage(query_seed, basis_seed, descriptor, scenario);
    IdentityEvolutionCertificationLane::from_execution_artifact(&artifact)
}

pub(super) fn execute_comparison(
    query_seed: &str,
    basis_family: IdentityEvolutionComparisonBasisFamily,
    left_basis_seed: &str,
    right_basis_seed: &str,
    comparison: CorrespondenceIdentityComparison,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionCertificationLane {
    let artifact = execute_artifact_for_comparison(
        query_seed,
        basis_family,
        left_basis_seed,
        right_basis_seed,
        comparison,
        scenario,
    );
    IdentityEvolutionCertificationLane::from_execution_artifact(&artifact)
}

pub(super) fn branch_to_branch_authoritative_artifact() -> IdentityEvolutionExecutionArtifact {
    execute_artifact_for_comparison(
        "branch-to-branch-authoritative-comparison",
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        "basis:branch-authoritative-left",
        "basis:branch-authoritative-right",
        CorrespondenceIdentityComparison::authoritative_between("entity:left", "entity:right"),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

pub(super) fn execute_artifact_for_lineage(
    query_seed: &str,
    basis_seed: &str,
    descriptor: LineageTraversalDescriptor,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionExecutionArtifact {
    let query_context = IdentityEvolutionQueryContext::lineage_traversal_for_test(
        query_digest(query_seed),
        basis_digest(basis_seed),
        descriptor,
    );
    let admitted = admit_identity_evolution_query_for_scenario(query_context, scenario)
        .expect("identity-evolution lineage should admit");
    execute_admitted_identity_evolution_query(&admitted)
        .expect("identity-evolution lineage should execute")
}

pub(super) fn execute_artifact_for_comparison(
    query_seed: &str,
    basis_family: IdentityEvolutionComparisonBasisFamily,
    left_basis_seed: &str,
    right_basis_seed: &str,
    comparison: CorrespondenceIdentityComparison,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionExecutionArtifact {
    let query_context = IdentityEvolutionQueryContext::correspondence_identity_comparison_for_test(
        query_digest(query_seed),
        basis_family,
        basis_digest(left_basis_seed),
        basis_digest(right_basis_seed),
        comparison,
    );
    let admitted = admit_identity_evolution_query_for_scenario(query_context, scenario)
        .expect("identity-evolution comparison should admit");
    execute_admitted_identity_evolution_query(&admitted)
        .expect("identity-evolution comparison should execute")
}
