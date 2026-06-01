use crate::certification::topology_operator_closeout::MilestoneThreeHostileSuiteReport;
use crate::certification::{CertificationRequiredOutput, CertificationValidatorExpectation};
use crate::facade::{
    MilestoneThreeDeterminismRuleKind, MilestoneThreeHostileOutcomeClass, ReplayParityStatus,
};

pub(super) fn rejected_scenarios_from_report(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<String> {
    report
        .coverage_rows
        .iter()
        .filter(|row| {
            row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
                && row.rejection_class.is_some()
        })
        .map(|row| row.scenario.as_str().to_string())
        .collect()
}

pub(super) fn replay_scenarios_from_report(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<String> {
    report
        .coverage_rows
        .iter()
        .filter(|row| row.replay_checked && row.replay_parity_status == ReplayParityStatus::Match)
        .map(|row| row.scenario.as_str().to_string())
        .collect()
}

pub(super) fn accepted_branch_local_row_count(report: &MilestoneThreeHostileSuiteReport) -> usize {
    report
        .mutation_branch_local_parity_rows
        .iter()
        .filter(|row| {
            row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
                && row.branch_head_diverged_from_main
                && row.branch_truth_digest.is_some()
        })
        .count()
}

pub(super) fn accepted_branch_local_scenarios_from_report(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<String> {
    report
        .mutation_branch_local_parity_rows
        .iter()
        .filter(|row| {
            row.outcome_class == MilestoneThreeHostileOutcomeClass::Accepted
                && row.branch_head_diverged_from_main
                && row.branch_truth_digest.is_some()
        })
        .filter_map(|row| row.scenario.map(|scenario| scenario.as_str().to_string()))
        .collect()
}

pub(super) fn rejected_branch_local_scenarios_from_report(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<String> {
    report
        .mutation_branch_local_parity_rows
        .iter()
        .filter(|row| {
            row.outcome_class == MilestoneThreeHostileOutcomeClass::Rejected
                && row.branch_head_unchanged_after_rejection
                && row.rejection_class.is_some()
                && row.branch_truth_digest.is_none()
        })
        .filter_map(|row| row.scenario.map(|scenario| scenario.as_str().to_string()))
        .collect()
}

pub(super) fn stable_mutation_digest_scenarios_from_report(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<String> {
    report
        .determinism_rule_rows
        .iter()
        .filter(|row| {
            row.rule_kind == MilestoneThreeDeterminismRuleKind::StableMutationDigest
                && row.replay_verified
        })
        .map(|row| row.scenario.as_str().to_string())
        .collect()
}

pub(super) fn stable_mutation_order_scenarios_from_report(
    report: &MilestoneThreeHostileSuiteReport,
) -> Vec<String> {
    report
        .determinism_rule_rows
        .iter()
        .filter(|row| {
            row.rule_kind == MilestoneThreeDeterminismRuleKind::StableMutationOrder
                && row.replay_verified
                && row.row_digest.contains("order_policy=sequence_preserving")
        })
        .map(|row| row.scenario.as_str().to_string())
        .collect()
}

pub(super) fn validator_expectation_pairs(
    expectations: &[CertificationValidatorExpectation],
) -> Vec<(String, Vec<String>)> {
    expectations
        .iter()
        .map(|expectation| (expectation.family.clone(), expectation.validators.clone()))
        .collect()
}

pub(super) fn expected_milestone_three_validator_expectations() -> Vec<(String, Vec<String>)> {
    vec![
        (
            "BowtieAdjacentRewire".to_string(),
            vec![
                "mutation_local_continuity".to_string(),
                "naming_continuity".to_string(),
                "rejection_locality".to_string(),
            ],
        ),
        (
            "CancellationChainParity".to_string(),
            accepted_validator_expectations(),
        ),
        (
            "SplitCollapseChurn".to_string(),
            accepted_validator_expectations(),
        ),
        (
            "AmbiguousLocalRewireContinuity".to_string(),
            accepted_validator_expectations(),
        ),
        (
            "BrokenRadialLocalization".to_string(),
            vec![
                "mutation_local_continuity".to_string(),
                "naming_continuity".to_string(),
                "rejection_locality".to_string(),
            ],
        ),
    ]
}

fn accepted_validator_expectations() -> Vec<String> {
    vec![
        "mutation_local_continuity".to_string(),
        "naming_continuity".to_string(),
        "derived_validation_inspection".to_string(),
    ]
}

pub(super) fn assert_milestone_three_required_outputs(
    required_outputs: &[CertificationRequiredOutput],
) {
    for output in [
        CertificationRequiredOutput::MilestoneThreeHostileSuiteReport,
        CertificationRequiredOutput::MilestoneThreeHostileCertificationCategoryRows,
        CertificationRequiredOutput::MilestoneThreeOperatorFamilyClosureRows,
        CertificationRequiredOutput::MilestoneThreePrimitiveFamilyClosureRows,
        CertificationRequiredOutput::MilestoneThreeScalePressureRows,
        CertificationRequiredOutput::MilestoneThreeTopologyMutationDigestRows,
        CertificationRequiredOutput::MilestoneThreeNamingContinuityMatrixRows,
        CertificationRequiredOutput::MilestoneThreeNamingContinuityBreadthRows,
        CertificationRequiredOutput::MilestoneThreeRejectedMutationScopeReportRows,
        CertificationRequiredOutput::MilestoneThreeMutationReplayParityRows,
        CertificationRequiredOutput::MilestoneThreeMutationBranchLocalParityRows,
        CertificationRequiredOutput::MilestoneThreeReplayBranchBreadthRows,
        CertificationRequiredOutput::MilestoneThreeMutationTopologyQueryTraversalRows,
        CertificationRequiredOutput::MilestoneThreeChangedScopeCoverageRows,
        CertificationRequiredOutput::MilestoneThreeValidationBreadthRows,
        CertificationRequiredOutput::MilestoneThreeDerivedRegionCoverageRows,
        CertificationRequiredOutput::MilestoneThreeDeterminismRuleRows,
        CertificationRequiredOutput::MilestoneThreeMutationBreadthCounterRows,
        CertificationRequiredOutput::MilestoneThreeMutationFalloutBreadthRows,
        CertificationRequiredOutput::MilestoneThreeDerivedReuseLegalityRows,
        CertificationRequiredOutput::MilestoneThreeDerivedWorkBreadthRows,
        CertificationRequiredOutput::MilestoneThreeFailureLocalityRows,
        CertificationRequiredOutput::MilestoneThreeValidatorFamilyCoverageRows,
        CertificationRequiredOutput::MilestoneThreeSideQuestCloseoutReport,
        CertificationRequiredOutput::MilestoneThreeReturnGateReport,
    ] {
        assert!(required_outputs.contains(&output));
    }
}
