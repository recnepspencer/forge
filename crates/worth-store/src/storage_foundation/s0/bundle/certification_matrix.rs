use super::super::artifacts::S0ValidatedBackendCapabilityMatrixArtifact;
use super::super::claims::S0ValidatedSemanticPhysicalClaimReportArtifact;
use super::super::counters::S0CounterSnapshot;
use super::super::deferred::S0ValidatedDeferredPhysicalGuaranteeMapArtifact;
use super::super::evidence::S0ArtifactValidationReport;
use super::super::handoff::S0ValidatedStorageFoundationS1HandoffArtifact;
use super::super::harness::S0ValidatedHarnessMaturityReportArtifact;
use super::super::migration::S0ValidatedTestMigrationNotesArtifact;
use super::super::milestones::S0ValidatedMilestonePhysicalStatusMatrixArtifact;
use super::super::terminology::S0ValidatedTerminologyRiskReportArtifact;
use super::certification::S0CertificationMatrixRow;
use super::validation::S0EvidenceBundleBuildRejection;

pub(super) struct CertificationInputs<'a> {
    pub(super) artifact_validation: &'a S0ArtifactValidationReport,
    pub(super) counters: &'a S0CounterSnapshot,
    pub(super) backend_matrix: &'a S0ValidatedBackendCapabilityMatrixArtifact,
    pub(super) milestone_matrix: &'a S0ValidatedMilestonePhysicalStatusMatrixArtifact,
    pub(super) claim_report: &'a S0ValidatedSemanticPhysicalClaimReportArtifact,
    pub(super) deferred_map: &'a S0ValidatedDeferredPhysicalGuaranteeMapArtifact,
    pub(super) terminology_report: &'a S0ValidatedTerminologyRiskReportArtifact,
    pub(super) migration_notes: &'a S0ValidatedTestMigrationNotesArtifact,
    pub(super) harness_report: &'a S0ValidatedHarnessMaturityReportArtifact,
    pub(super) s1_handoff: &'a S0ValidatedStorageFoundationS1HandoffArtifact,
}

pub(super) fn build_certification_matrix(
    inputs: &CertificationInputs<'_>,
) -> Result<Vec<S0CertificationMatrixRow>, S0EvidenceBundleBuildRejection> {
    let mut rows = Vec::with_capacity(10);
    rows.extend(super::certification_artifact_rows::build_artifact_rows(
        inputs,
    )?);
    rows.extend(super::certification_contract_rows::build_contract_rows(
        inputs,
    )?);
    rows.extend(super::certification_evidence_rows::build_evidence_rows(
        inputs,
    )?);
    rows.extend(super::certification_handoff_rows::build_handoff_rows(
        inputs,
    )?);
    Ok(rows)
}
