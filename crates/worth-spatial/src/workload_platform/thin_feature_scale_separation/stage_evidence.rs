use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::thin_feature_policy::ThinFeatureScaleSeparationWorkloadError;
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

pub(crate) fn stage_counters(
    evidence_ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
) -> Result<WorkloadEvidenceStageCounters, ThinFeatureScaleSeparationWorkloadError> {
    evidence_ledger
        .row_for_stage(stage)
        .filter(|row| row.is_receipt_backed() && row.is_admitted())
        .map(|row| row.counters())
        .ok_or(ThinFeatureScaleSeparationWorkloadError::MissingReceiptBackedStage(stage))
}

pub(crate) fn require_platform_stage_evidence(
    topology: WorkloadEvidenceStageCounters,
    binding: WorkloadEvidenceStageCounters,
    support: WorkloadEvidenceStageCounters,
    projection: WorkloadEvidenceStageCounters,
    transform: WorkloadEvidenceStageCounters,
    diagnostics: WorkloadEvidenceStageCounters,
    response: WorkloadEvidenceStageCounters,
) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
    if topology.topology_face_count() == 0 || binding.binding_target_count() == 0 {
        return Err(ThinFeatureScaleSeparationWorkloadError::MissingTopologyEvidence);
    }
    if support.surface_support_count() == 0 {
        return Err(ThinFeatureScaleSeparationWorkloadError::MissingSurfaceSupportEvidence);
    }
    if projection.local_basis_part_count() == 0 || projection.projected_entity_count() == 0 {
        return Err(ThinFeatureScaleSeparationWorkloadError::MissingProjectionEvidence);
    }
    if transform.transform_step_count() == 0 {
        return Err(ThinFeatureScaleSeparationWorkloadError::MissingTransformEvidence);
    }
    if diagnostics.diagnostic_count() == 0 || response.user_outcome_count() == 0 {
        return Err(ThinFeatureScaleSeparationWorkloadError::MissingResponseEvidence);
    }
    Ok(())
}

pub(crate) fn require_thin_feature_topology_breadth(
    required_thin_feature_count: usize,
    topology: WorkloadEvidenceStageCounters,
) -> Result<(), ThinFeatureScaleSeparationWorkloadError> {
    if required_thin_feature_count < 12
        || topology.topology_relation_count() < required_thin_feature_count
    {
        return Err(ThinFeatureScaleSeparationWorkloadError::MissingTopologyEvidence);
    }
    Ok(())
}

pub(crate) fn workload_identity(
    evidence_ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<String, ThinFeatureScaleSeparationWorkloadError> {
    let mut parts = Vec::new();
    for stage in WorkloadEvidenceStage::AUTHORITY_STAGES {
        let row = evidence_ledger
            .row_for_stage(stage)
            .filter(|row| row.is_receipt_backed() && row.is_admitted())
            .ok_or(ThinFeatureScaleSeparationWorkloadError::MissingReceiptBackedStage(stage))?;
        parts.push(format!("{stage:?}:{}", row.evidence_identity()));
    }
    Ok(truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "thin-feature-scale-separation-ledger".to_string(),
            parts.join("|"),
        ],
    ))
}
