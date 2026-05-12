use super::*;

pub(super) fn ensure_milestone_two_family_coverage_closure(
    report: &DerivedFamilyCoverageMatrix,
    requirements: &crate::certification::core::CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing derived family coverage row for family `{family}`"
            )));
        };
        if !row.coverage_complete {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout derived family coverage is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_milestone_two_parity_closure(
    report: &DerivedFamilyParityMatrix,
    requirements: &crate::certification::core::CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for family in &requirements.required_parity_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing derived parity row for family `{family}`"
            )));
        };
        if !row.parity_complete {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout derived parity is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_milestone_two_bridge_closure(
    report: &crate::certification::support::reporting::BridgeFamilyCoverageReport,
    requirements: &crate::certification::core::CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for bridge_family in &requirements.required_bridge_rows {
        let Some(row) = report
            .rows
            .iter()
            .find(|row| row.family == bridge_family.family)
        else {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing bridge family row for family `{}`",
                bridge_family.family
            )));
        };
        if !row.proof_complete {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout bridge proof is incomplete for family `{}`",
                bridge_family.family
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_milestone_two_validator_closure(
    report: &DerivedValidatorCoverageReport,
    requirements: &crate::certification::core::CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for expectation in &requirements.validator_expectations {
        for validator in &expectation.validators {
            let Some(row) = report
                .rows
                .iter()
                .find(|row| row.family == expectation.family && row.validator == *validator)
            else {
                return Err(MilestoneOneCertificationError::ReadView(format!(
                    "milestone two closeout missing derived validator coverage for family `{}` validator `{validator}`",
                    expectation.family
                )));
            };
            if row.passed_count == 0 {
                return Err(MilestoneOneCertificationError::ReadView(format!(
                    "milestone two closeout derived validator coverage is incomplete for family `{}` validator `{validator}`",
                    expectation.family
                )));
            }
        }
    }
    Ok(())
}

pub(super) fn ensure_milestone_two_failure_locality_closure(
    report: &FailureLocalityReport,
    requirements: &crate::certification::core::CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        if !report.rows.iter().any(|row| row.family == *family) {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing failure locality for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_milestone_two_required_output_closure(
    closeout: &MilestoneTwoCloseoutReport,
    requirements: &crate::certification::core::CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for output in &requirements.required_outputs {
        let present = match output {
            crate::certification::core::CertificationRequiredOutput::MaterializedTopologyDigest => {
                closeout.materialized_topology_digest.row_count > 0
            }
            crate::certification::core::CertificationRequiredOutput::InterpretedTopologyDigest => {
                closeout.interpreted_topology_digest.row_count > 0
            }
            crate::certification::core::CertificationRequiredOutput::DerivedValidationDigest => {
                closeout.derived_validation_digest.row_count > 0
            }
            crate::certification::core::CertificationRequiredOutput::DerivedTruthBasisDigest => {
                closeout.derived_truth_basis_digest.row_count > 0
            }
            crate::certification::core::CertificationRequiredOutput::BridgeRoutingDigest => {
                closeout.bridge_routing_digest.row_count > 0
            }
            crate::certification::core::CertificationRequiredOutput::BridgeHistoricalEvaluationDigest => {
                closeout.bridge_historical_evaluation_digest.row_count > 0
            }
            crate::certification::core::CertificationRequiredOutput::DerivedFamilyCoverageMatrix => {
                !closeout.derived_family_coverage_matrix.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedFamilyParityMatrix => {
                !closeout.derived_family_parity_matrix.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedValidatorCoverageReport => {
                !closeout.derived_validator_coverage_report.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedInvalidationReport => {
                !closeout.derived_invalidation_report.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedRebuildReport => {
                !closeout.derived_rebuild_report.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedEquivalenceContractReport => {
                !closeout.derived_equivalence_contract_report.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedFallbackReport => {
                !closeout.derived_fallback_report.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedFailureLocalityReport => {
                !closeout.derived_failure_locality_report.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedBranchLocalParityReport => {
                !closeout.derived_branch_local_parity_report.branch_ids.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::DerivedReplayParityReport => {
                closeout.derived_replay_parity_report.replay_checked_case_count > 0
            }
            crate::certification::core::CertificationRequiredOutput::DerivedBridgeFamilyCoverageReport => {
                !closeout.derived_bridge_family_coverage_report.rows.is_empty()
            }
            crate::certification::core::CertificationRequiredOutput::MilestoneTwoCounterReport => {
                closeout.milestone_2_counter_report.derived_read_count > 0
            }
            _ => true,
        };
        if !present {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing required output `{output:?}`"
            )));
        }
    }
    Ok(())
}
