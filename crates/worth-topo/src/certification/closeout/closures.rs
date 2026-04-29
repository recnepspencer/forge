use super::*;

pub(super) fn ensure_validator_expectation_closure(
    report: &WorthMilestoneOneValidatorCoverageReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for expectation in &requirements.validator_expectations {
        for validator in &expectation.validators {
            let satisfied = report.rows.iter().any(|row| {
                row.family == expectation.family
                    && row.validator == *validator
                    && row.passed_count >= 1
            });
            if !satisfied {
                return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                    "milestone one closeout missing validator coverage for family `{}` validator `{validator}`",
                    expectation.family
                )));
            }
        }
    }
    Ok(())
}

pub(super) fn ensure_family_coverage_closure(
    report: &crate::certification::report::WorthPrimitiveCorpusCoverageMatrix,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.entries.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing family coverage row for family `{family}`"
            )));
        };
        if !row.role_closure_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout family coverage is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_parity_closure(
    report: &crate::certification::report::WorthPrimitiveCorpusParityReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_parity_rows {
        let Some(row) = report.entries.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing parity row for family `{family}`"
            )));
        };
        if !row.parity_closure_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout parity is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_rejection_class_closure(
    report: &WorthMilestoneOneRejectionClassReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        let has_row = report
            .rows
            .iter()
            .any(|row| row.family == *family && row.case_count > 0);
        if !has_row {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing rejection-class coverage for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_sweep_closure(
    report: &WorthAdmittedRangeSweepReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing admitted-range sweep row for family `{family}`"
            )));
        };
        if !row.sweep_closure_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout admitted-range sweep is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_failure_locality_closure(
    report: &WorthFailureLocalityReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        let has_row = report.rows.iter().any(|row| row.family == *family);
        if !has_row {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing failure locality row for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_bridge_coverage_closure(
    report: &crate::certification::report::WorthBridgeFamilyCoverageReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for bridge_family in &requirements.required_bridge_rows {
        let Some(row) = report
            .rows
            .iter()
            .find(|row| row.family == bridge_family.family)
        else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing bridge coverage row for family `{}`",
                bridge_family.family
            )));
        };
        if !row.proof_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout bridge proof is incomplete for family `{}`",
                bridge_family.family
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_required_output_closure(
    closeout: &WorthMilestoneOneCloseoutReport,
    requirements: &WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for output in &requirements.required_outputs {
        let present = match output {
            WorthCertificationRequiredOutput::TopologyTruthDigest => {
                closeout.topology_truth_digest.row_count > 0
            }
            WorthCertificationRequiredOutput::NamingTruthDigest => {
                closeout.naming_truth_digest.row_count > 0
            }
            WorthCertificationRequiredOutput::TopologyValidationDigest => {
                closeout.topology_validation_digest.row_count > 0
            }
            WorthCertificationRequiredOutput::TopologyValidationReport => {
                !closeout.topology_validation_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::TopologyLocalizationReport => {
                !closeout
                    .topology_localization_report
                    .topology_entities
                    .is_empty()
                    || !closeout
                        .topology_localization_report
                        .topology_relations
                        .is_empty()
            }
            WorthCertificationRequiredOutput::NamingAttachmentReport => {
                !closeout.naming_attachment_report.attachments.is_empty()
            }
            WorthCertificationRequiredOutput::PrimitiveFamilyCoverageMatrix => {
                !closeout.primitive_family_coverage_matrix.entries.is_empty()
            }
            WorthCertificationRequiredOutput::PrimitiveCorpusParityReport => {
                !closeout.primitive_corpus_parity_report.entries.is_empty()
            }
            WorthCertificationRequiredOutput::AdmittedRangeSweepReport => {
                !closeout.admitted_range_sweep_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::ValidatorCoverageReport => {
                !closeout.validator_coverage_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::BranchLocalTopologyReport => {
                !closeout.branch_local_topology_report.branch_ids.is_empty()
            }
            WorthCertificationRequiredOutput::ReplayParityReport => {
                closeout
                    .milestone_1_replay_parity_report
                    .replay_checked_case_count
                    > 0
            }
            WorthCertificationRequiredOutput::RejectionClassReport => {
                !closeout.rejection_class_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::FailureLocalityReport => {
                !closeout.failure_locality_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::BridgeFamilyCoverageReport => {
                !closeout.bridge_family_coverage_report.rows.is_empty()
            }
            WorthCertificationRequiredOutput::BridgeProofReport => {
                closeout.bridge_proof_report.proof_case_count > 0
            }
            WorthCertificationRequiredOutput::CounterReport => {
                closeout
                    .milestone_1_counter_report
                    .commit_boundary_validator_count
                    > 0
            }
            _ => true,
        };
        if !present {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing required output `{output:?}`"
            )));
        }
    }
    Ok(())
}
