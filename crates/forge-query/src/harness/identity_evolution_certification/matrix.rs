use super::lane::{
    IdentityEvolutionCertificationLane, IdentityEvolutionCertificationMatrix,
    IdentityEvolutionCertificationRejection,
};
use super::row_catalog::{
    IdentityEvolutionCanonicalRowSpec, IdentityEvolutionRejectionRowSpec,
    IDENTITY_EVOLUTION_CANONICAL_ROW_SPECS, IDENTITY_EVOLUTION_REJECTION_ROW_SPECS,
};
use crate::harness::certification::{
    CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::identity::{BasisDigest, CanonicalQueryDigest};
use crate::identity_evolution::{
    admit_identity_evolution_query_for_scenario, execute_admitted_identity_evolution_query,
    CorrespondenceIdentityComparison, IdentityEvolutionComparisonBasisFamily,
    IdentityEvolutionExecutionArtifact, IdentityEvolutionQueryContext,
    IdentityEvolutionSyntheticScenario, LineageTraversalDescriptor,
};

pub struct MilestoneSevenIdentityEvolutionCertificationAdapter;

impl MilestoneSevenIdentityEvolutionCertificationAdapter {
    pub fn lineage_and_correspondence_query_parity_test(
    ) -> IdentityEvolutionCertificationMatrix {
        let replacement = replacement_lane();
        let split = split_lane();
        let branch_local = branch_local_lane();
        let ambiguous = ambiguous_comparison_lane();
        let identity_break = identity_break_lane();
        let advisory_disagreement = advisory_disagreement_lane();
        let branch_to_branch = branch_to_branch_authoritative_lane();
        let current_to_historical = current_to_historical_advisory_lane();
        let preview_to_authoritative = preview_to_authoritative_lane();

        IdentityEvolutionCertificationMatrix {
            suite_name: "Lineage And Correspondence Query Parity Test",
            rows: IDENTITY_EVOLUTION_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &replacement,
                        &split,
                        &branch_local,
                        &ambiguous,
                        &identity_break,
                        &advisory_disagreement,
                        &branch_to_branch,
                        &current_to_historical,
                        &preview_to_authoritative,
                    )
                })
                .collect(),
            rejection_rows: IDENTITY_EVOLUTION_REJECTION_ROW_SPECS
                .iter()
                .map(rejection_row)
                .collect(),
        }
    }
}

fn replacement_lane() -> IdentityEvolutionCertificationLane {
    execute_lineage(
        "replacement-lineage-traversal",
        "basis:current",
        LineageTraversalDescriptor::direct_replacement("entity:replacement"),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

fn split_lane() -> IdentityEvolutionCertificationLane {
    execute_lineage(
        "split-successor-lineage-traversal",
        "basis:current",
        LineageTraversalDescriptor::direct_split_successors("entity:split"),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

fn branch_local_lane() -> IdentityEvolutionCertificationLane {
    execute_lineage(
        "branch-local-divergence-stays-local",
        "basis:branch-local",
        LineageTraversalDescriptor::branch_local_direct_evolution("entity:branch-local-divergence"),
        IdentityEvolutionSyntheticScenario::BranchLocalDivergence,
    )
}

fn ambiguous_comparison_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "branch-to-branch-correspondence-ambiguous",
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        "basis:left-branch",
        "basis:right-branch",
        CorrespondenceIdentityComparison::advisory_between("entity:left", "entity:right"),
        IdentityEvolutionSyntheticScenario::AmbiguousCorrespondence,
    )
}

fn identity_break_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "identity-break-explicit",
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        "basis:identity-break-left",
        "basis:identity-break-right",
        CorrespondenceIdentityComparison::authoritative_between("entity:left", "entity:right"),
        IdentityEvolutionSyntheticScenario::IdentityBreak,
    )
}

fn advisory_disagreement_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "lineage-versus-structural-disagreement-explicit",
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        "basis:disagreement-left",
        "basis:disagreement-right",
        CorrespondenceIdentityComparison::advisory_between("entity:left", "entity:right"),
        IdentityEvolutionSyntheticScenario::IdentityBreak,
    )
}

fn branch_to_branch_authoritative_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "branch-to-branch-authoritative-comparison",
        IdentityEvolutionComparisonBasisFamily::BranchToBranch,
        "basis:branch-authoritative-left",
        "basis:branch-authoritative-right",
        CorrespondenceIdentityComparison::authoritative_between("entity:left", "entity:right"),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

fn current_to_historical_advisory_lane() -> IdentityEvolutionCertificationLane {
    execute_comparison(
        "current-to-historical-advisory-comparison",
        IdentityEvolutionComparisonBasisFamily::CurrentToHistorical,
        "basis:current",
        "basis:historical",
        CorrespondenceIdentityComparison::advisory_between(
            "entity:current",
            "entity:historical",
        ),
        IdentityEvolutionSyntheticScenario::Standard,
    )
}

fn preview_to_authoritative_lane() -> IdentityEvolutionCertificationLane {
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

fn execute_lineage(
    query_seed: &str,
    basis_seed: &str,
    descriptor: LineageTraversalDescriptor,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionCertificationLane {
    let artifact = execute_artifact_for_lineage(query_seed, basis_seed, descriptor, scenario);
    IdentityEvolutionCertificationLane::from_execution_artifact(&artifact)
}

fn execute_comparison(
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

fn canonical_row(
    spec: &IdentityEvolutionCanonicalRowSpec,
    replacement: &IdentityEvolutionCertificationLane,
    split: &IdentityEvolutionCertificationLane,
    branch_local: &IdentityEvolutionCertificationLane,
    ambiguous: &IdentityEvolutionCertificationLane,
    identity_break: &IdentityEvolutionCertificationLane,
    advisory_disagreement: &IdentityEvolutionCertificationLane,
    branch_to_branch: &IdentityEvolutionCertificationLane,
    current_to_historical: &IdentityEvolutionCertificationLane,
    preview_to_authoritative: &IdentityEvolutionCertificationLane,
) -> CanonicalCertificationRow<
    super::IdentityEvolutionCertificationPerturbationClass,
    IdentityEvolutionCertificationLane,
> {
    let (control_lane, hostile_lane, parity_lane) = match spec.row_name {
        "replacement-continuity-explicitness" => (
            replacement.clone(),
            execute_lineage(
                "replacement-continuity-hostile",
                "basis:current",
                LineageTraversalDescriptor::direct_successor("entity:replacement"),
                IdentityEvolutionSyntheticScenario::Standard,
            ),
            replacement_lane(),
        ),
        "split-successor-explicitness" => (replacement.clone(), split.clone(), split_lane()),
        "branch-local-divergence-explicitness" => {
            (replacement.clone(), branch_local.clone(), branch_local_lane())
        }
        "ambiguous-correspondence-explicitness" => (
            branch_to_branch.clone(),
            ambiguous.clone(),
            ambiguous_comparison_lane(),
        ),
        "identity-break-explicitness" => (
            branch_to_branch.clone(),
            identity_break.clone(),
            identity_break_lane(),
        ),
        "lineage-versus-structural-disagreement-explicitness" => (
            identity_break.clone(),
            advisory_disagreement.clone(),
            identity_break_lane(),
        ),
        "lineage-replay-parity" => (replacement.clone(), replacement_lane(), replacement_lane()),
        "lineage-replay-preserves-classification" => (
            replacement.clone(),
            replacement_lane(),
            replacement_lane(),
        ),
        "preview-to-authoritative-identity-comparison" => (
            current_to_historical.clone(),
            preview_to_authoritative.clone(),
            preview_to_authoritative_lane(),
        ),
        "identity-evolution-width-drift-explicitness" => {
            (replacement.clone(), split.clone(), split_lane())
        }
        "lineage-complexity-contract-parity" => (
            replacement.clone(),
            branch_local.clone(),
            replacement_lane(),
        ),
        "correspondence-complexity-contract-parity" => (
            branch_to_branch.clone(),
            current_to_historical.clone(),
            branch_to_branch_authoritative_lane(),
        ),
        "complexity-status-honesty" => (
            replacement.clone(),
            current_to_historical.clone(),
            replacement_lane(),
        ),
        other => panic!("unexpected identity-evolution canonical row {other}"),
    };

    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

fn rejection_row(
    spec: &IdentityEvolutionRejectionRowSpec,
) -> RejectionCertificationRow<
    super::IdentityEvolutionCertificationPerturbationClass,
    IdentityEvolutionCertificationLane,
    IdentityEvolutionCertificationRejection,
> {
    let control_lane = replacement_lane();
    let parity_lane = replacement_lane();
    let hostile_lane = match spec.row_name {
        "unsupported-lineage-traversal-family" => unsupported_lineage_rejection(),
        "unsupported-correspondence-family" => unsupported_comparison_rejection(),
        "advisory-as-authoritative-forbidden" => {
            IdentityEvolutionCertificationRejection::from_execution_artifact(
                &execute_artifact_for_comparison(
                    "advisory-as-authoritative-forbidden",
                    IdentityEvolutionComparisonBasisFamily::BranchToBranch,
                    "basis:authoritative-left",
                    "basis:authoritative-right",
                    CorrespondenceIdentityComparison::authoritative_between(
                        "entity:left",
                        "entity:right",
                    ),
                    IdentityEvolutionSyntheticScenario::AdvisoryAsAuthoritativeDenied,
                ),
            )
        }
        "lineage-to-correspondence-fallback-forbidden" => {
            IdentityEvolutionCertificationRejection::from_execution_artifact(
                &execute_artifact_for_lineage(
                    "lineage-to-correspondence-fallback-forbidden",
                    "basis:fallback",
                    LineageTraversalDescriptor::direct_predecessor("entity:fallback"),
                    IdentityEvolutionSyntheticScenario::LineageToCorrespondenceFallbackDenied,
                ),
            )
        }
        "branch-crossing-lineage-forbidden" => {
            IdentityEvolutionCertificationRejection::from_execution_artifact(
                &execute_artifact_for_lineage(
                    "branch-crossing-lineage-forbidden",
                    "basis:branch-local",
                    LineageTraversalDescriptor::branch_local_direct_evolution("entity:branch"),
                    IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied,
                ),
            )
        }
        "broad-lineage-scan-forbidden" => {
            IdentityEvolutionCertificationRejection::from_execution_artifact(
                &execute_artifact_for_lineage(
                    "broad-lineage-scan-forbidden",
                    "basis:broad-scan",
                    LineageTraversalDescriptor::direct_predecessor("entity:scan"),
                    IdentityEvolutionSyntheticScenario::BroadLineageScanDenied,
                ),
            )
        }
        "fabricated-branch-local-continuity-forbidden" => {
            let query_digest = query_digest("fabricated-branch-local-continuity-forbidden");
            let basis_digest = basis_digest("basis:compile-fail");
            IdentityEvolutionCertificationRejection::compile_fail(
                spec.row_name,
                "tests/ui/identity_evolution_branch_local_promotion_forbidden.rs",
                &query_digest,
                &basis_digest,
            )
        }
        "complexity-contract-violation-denied" => {
            IdentityEvolutionCertificationRejection::from_execution_artifact(
                &execute_artifact_for_lineage(
                    "complexity-contract-violation-denied",
                    "basis:contract-violation",
                    LineageTraversalDescriptor::direct_predecessor("entity:lineage"),
                    IdentityEvolutionSyntheticScenario::ComplexityContractViolationDenied,
                ),
            )
        }
        other => panic!("unexpected identity-evolution rejection row {other}"),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

fn unsupported_lineage_rejection() -> IdentityEvolutionCertificationRejection {
    let query_digest = query_digest("unsupported-lineage-traversal-family");
    let basis_digest = basis_digest("basis:unsupported-lineage");
    let error = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::lineage_traversal(
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

fn unsupported_comparison_rejection() -> IdentityEvolutionCertificationRejection {
    let query_digest = query_digest("unsupported-correspondence-family");
    let left_basis = basis_digest("basis:unsupported-left");
    let right_basis = basis_digest("basis:unsupported-right");
    let error = admit_identity_evolution_query_for_scenario(
        IdentityEvolutionQueryContext::correspondence_identity_comparison(
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

fn execute_artifact_for_lineage(
    query_seed: &str,
    basis_seed: &str,
    descriptor: LineageTraversalDescriptor,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionExecutionArtifact {
    let query_context = IdentityEvolutionQueryContext::lineage_traversal(
        query_digest(query_seed),
        basis_digest(basis_seed),
        descriptor,
    );
    let admitted = admit_identity_evolution_query_for_scenario(query_context, scenario)
        .expect("identity-evolution lineage should admit");
    execute_admitted_identity_evolution_query(&admitted)
        .expect("identity-evolution lineage should execute")
}

fn execute_artifact_for_comparison(
    query_seed: &str,
    basis_family: IdentityEvolutionComparisonBasisFamily,
    left_basis_seed: &str,
    right_basis_seed: &str,
    comparison: CorrespondenceIdentityComparison,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionExecutionArtifact {
    let query_context = IdentityEvolutionQueryContext::correspondence_identity_comparison(
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

fn query_digest(seed: &str) -> CanonicalQueryDigest {
    CanonicalQueryDigest::from_parts(&[format!("identity-evolution-query:{seed}")])
}

fn basis_digest(seed: &str) -> BasisDigest {
    BasisDigest::from_parts(&[format!("identity-evolution-basis:{seed}")])
}
