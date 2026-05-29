use super::*;

pub(super) fn ensure_validator_expectation_closure(
    report: &MilestoneOneValidatorCoverageReport,
    requirements: &CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for expectation in &requirements.validator_expectations {
        for validator in &expectation.validators {
            let satisfied = report.rows.iter().any(|row| {
                row.family == expectation.family
                    && row.validator == *validator
                    && row.passed_count >= 1
            });
            if !satisfied {
                return Err(MilestoneOneCertificationError::ReadView(format!(
                    "milestone one closeout missing validator coverage for family `{}` validator `{validator}`",
                    expectation.family
                )));
            }
        }
    }
    Ok(())
}

pub(super) fn ensure_family_coverage_closure(
    report: &crate::certification::support::reporting::PrimitiveCorpusCoverageMatrix,
    requirements: &CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.entries.iter().find(|row| row.family == *family) else {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing family coverage row for family `{family}`"
            )));
        };
        if !row.role_closure_complete {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout family coverage is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_parity_closure(
    report: &crate::certification::support::reporting::PrimitiveCorpusParityReport,
    requirements: &CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for family in &requirements.required_parity_rows {
        let Some(row) = report.entries.iter().find(|row| row.family == *family) else {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing parity row for family `{family}`"
            )));
        };
        if !row.parity_closure_complete {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout parity is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_rejection_class_closure(
    report: &MilestoneOneRejectionClassReport,
    requirements: &CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        let has_row = report
            .rows
            .iter()
            .any(|row| row.family == *family && row.case_count > 0);
        if !has_row {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing rejection-class coverage for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_sweep_closure(
    report: &AdmittedRangeSweepReport,
    requirements: &CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing admitted-range sweep row for family `{family}`"
            )));
        };
        if !row.sweep_closure_complete {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout admitted-range sweep is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_failure_locality_closure(
    report: &FailureLocalityReport,
    requirements: &CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        let has_row = report.rows.iter().any(|row| row.family == *family);
        if !has_row {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing failure locality row for family `{family}`"
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_bridge_coverage_closure(
    report: &crate::certification::support::reporting::BridgeFamilyCoverageReport,
    requirements: &CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for bridge_family in &requirements.required_bridge_rows {
        let Some(row) = report
            .rows
            .iter()
            .find(|row| row.family == bridge_family.family)
        else {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing bridge coverage row for family `{}`",
                bridge_family.family
            )));
        };
        if !row.proof_complete {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout bridge proof is incomplete for family `{}`",
                bridge_family.family
            )));
        }
    }
    Ok(())
}

pub(super) fn ensure_required_output_closure(
    closeout: &MilestoneOneCloseoutReport,
    requirements: &CertificationSuiteRequirements,
) -> Result<(), MilestoneOneCertificationError> {
    for output in &requirements.required_outputs {
        let present = match output {
            CertificationRequiredOutput::TopologyTruthDigest => {
                closeout.topology_truth_digest.row_count > 0
            }
            CertificationRequiredOutput::NamingTruthDigest => {
                closeout.naming_truth_digest.row_count > 0
            }
            CertificationRequiredOutput::TopologyValidationDigest => {
                closeout.topology_validation_digest.row_count > 0
            }
            CertificationRequiredOutput::TopologyValidationReport => {
                !closeout.topology_validation_report.rows.is_empty()
            }
            CertificationRequiredOutput::TopologyLocalizationReport => {
                !closeout
                    .topology_localization_report
                    .topology_entities
                    .is_empty()
                    || !closeout
                        .topology_localization_report
                        .topology_relations
                        .is_empty()
            }
            CertificationRequiredOutput::NamingAttachmentReport => {
                !closeout.naming_attachment_report.attachments.is_empty()
            }
            CertificationRequiredOutput::PrimitiveFamilyCoverageMatrix => {
                !closeout.primitive_family_coverage_matrix.entries.is_empty()
            }
            CertificationRequiredOutput::PrimitiveCorpusParityReport => {
                !closeout.primitive_corpus_parity_report.entries.is_empty()
            }
            CertificationRequiredOutput::AdmittedRangeSweepReport => {
                !closeout.admitted_range_sweep_report.rows.is_empty()
            }
            CertificationRequiredOutput::ValidatorCoverageReport => {
                !closeout.validator_coverage_report.rows.is_empty()
            }
            CertificationRequiredOutput::BranchLocalTopologyReport => {
                !closeout.branch_local_topology_report.branch_ids.is_empty()
            }
            CertificationRequiredOutput::ReplayParityReport => {
                closeout
                    .milestone_1_replay_parity_report
                    .replay_checked_case_count
                    > 0
            }
            CertificationRequiredOutput::RejectionClassReport => {
                !closeout.rejection_class_report.rows.is_empty()
            }
            CertificationRequiredOutput::FailureLocalityReport => {
                !closeout.failure_locality_report.rows.is_empty()
            }
            CertificationRequiredOutput::BridgeFamilyCoverageReport => {
                !closeout.bridge_family_coverage_report.rows.is_empty()
            }
            CertificationRequiredOutput::BridgeProofReport => {
                closeout.bridge_proof_report.proof_case_count > 0
            }
            CertificationRequiredOutput::CounterReport => {
                closeout
                    .milestone_1_counter_report
                    .commit_boundary_validator_count
                    > 0
            }
            _ => true,
        };
        if !present {
            return Err(MilestoneOneCertificationError::ReadView(format!(
                "milestone one closeout missing required output `{output:?}`"
            )));
        }
    }
    Ok(())
}




