use super::thin_feature_policy::ThinFeatureScaleSeparationWorkloadError;
use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceStage,
};
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;

pub(crate) fn require_platform_projection_matches_ledger(
    evidence_ledger: &CompleteWorkloadEvidenceLedger,
    platform_projection: &ProjectedPlanarWorkload,
) -> Result<String, ThinFeatureScaleSeparationWorkloadError> {
    let projection_identity = platform_projection
        .receipts()
        .stage_identity()
        .receipt_identity()
        .to_string();
    let projection_row = evidence_ledger
        .row_for_stage(WorkloadEvidenceStage::Projection)
        .filter(|row| row.is_receipt_backed() && row.is_admitted())
        .ok_or(
            ThinFeatureScaleSeparationWorkloadError::MissingReceiptBackedStage(
                WorkloadEvidenceStage::Projection,
            ),
        )?;

    if projection_row.evidence_identity() != projection_identity {
        return Err(ThinFeatureScaleSeparationWorkloadError::MissingPlatformProjectionEvidence);
    }

    let platform_counters = platform_projection.receipts().counters();
    let ledger_counters = projection_row.counters();
    if ledger_counters.projected_entity_count() != platform_counters.projected_topology_entities()
        || ledger_counters.local_basis_part_count() != platform_counters.local_basis_parts()
    {
        return Err(ThinFeatureScaleSeparationWorkloadError::MissingPlatformProjectionEvidence);
    }

    Ok(projection_identity)
}
