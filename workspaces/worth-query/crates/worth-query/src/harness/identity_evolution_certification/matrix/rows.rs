use crate::harness::certification::{
    CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::identity_evolution::{
    CorrespondenceIdentityComparison, IdentityEvolutionComparisonBasisFamily,
    IdentityEvolutionSyntheticScenario, LineageTraversalDescriptor,
};

use super::super::lane::{
    IdentityEvolutionCertificationLane, IdentityEvolutionCertificationRejection,
};
use super::super::row_catalog::{
    IdentityEvolutionCanonicalRowSpec, IdentityEvolutionRejectionRowSpec,
};
use super::super::IdentityEvolutionCertificationPerturbationClass;
use super::lanes::{
    ambiguous_comparison_lane, branch_local_lane,
    branch_to_branch_authoritative_bundle_inspector_lane, branch_to_branch_authoritative_lane,
    current_to_historical_advisory_lane, execute_artifact_for_comparison,
    execute_artifact_for_lineage, execute_lineage, identity_break_lane,
    preview_to_authoritative_lane, replacement_lane, split_lane,
};
use super::rejections::{unsupported_comparison_rejection, unsupported_lineage_rejection};

pub(super) fn canonical_row(
    spec: &IdentityEvolutionCanonicalRowSpec,
    replacement: &IdentityEvolutionCertificationLane,
    split: &IdentityEvolutionCertificationLane,
    branch_local: &IdentityEvolutionCertificationLane,
    ambiguous: &IdentityEvolutionCertificationLane,
    identity_break: &IdentityEvolutionCertificationLane,
    branch_to_branch: &IdentityEvolutionCertificationLane,
    current_to_historical: &IdentityEvolutionCertificationLane,
    preview_to_authoritative: &IdentityEvolutionCertificationLane,
) -> CanonicalCertificationRow<
    IdentityEvolutionCertificationPerturbationClass,
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
        "branch-local-divergence-explicitness" => (
            replacement.clone(),
            branch_local.clone(),
            branch_local_lane(),
        ),
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
        "identity-aware-inspector-consumption-parity" => (
            branch_to_branch.clone(),
            branch_to_branch_authoritative_bundle_inspector_lane(),
            branch_to_branch_authoritative_bundle_inspector_lane(),
        ),
        "lineage-versus-structural-disagreement-explicitness" => (
            identity_break.clone(),
            current_to_historical.clone(),
            current_to_historical_advisory_lane(),
        ),
        "lineage-replay-parity" => (replacement.clone(), replacement_lane(), replacement_lane()),
        "lineage-replay-preserves-classification" => {
            (replacement.clone(), replacement_lane(), replacement_lane())
        }
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

pub(super) fn rejection_row(
    spec: &IdentityEvolutionRejectionRowSpec,
) -> RejectionCertificationRow<
    IdentityEvolutionCertificationPerturbationClass,
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
