use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage, WorkloadEvidenceStageIndexProduct,
};

use super::identity::stage_link_identity;
use super::link::WorkloadEvidenceStageLink;
use super::link_set::WorkloadEvidenceStageLinkSet;

pub(crate) fn link_required_stages(
    stage_index: &WorkloadEvidenceStageIndexProduct,
    required_stages: &[WorkloadEvidenceStage],
) -> Result<WorkloadEvidenceStageLinkSet, WorkloadEvidenceLedgerError> {
    let mut links = Vec::with_capacity(required_stages.len());
    let mut seen_stages = [false; WorkloadEvidenceStage::STAGE_COUNT];
    for stage in required_stages {
        let slot = stage.index_slot();
        if seen_stages[slot] {
            return Err(WorkloadEvidenceLedgerError::DuplicateEvidenceStage(*stage));
        }
        seen_stages[slot] = true;
        let row = stage_index
            .row_for_stage(*stage)
            .ok_or(WorkloadEvidenceLedgerError::MissingAuthorityStage(*stage))?;
        if !row.is_receipt_backed() {
            return Err(WorkloadEvidenceLedgerError::ManualAuthorityStage(*stage));
        }
        if !row.is_admitted() {
            return Err(WorkloadEvidenceLedgerError::UnadmittedAuthorityStage(
                *stage,
            ));
        }
        links.push(WorkloadEvidenceStageLink::new(
            *stage,
            row.evidence_identity().to_string(),
            stage_link_identity(*stage, row),
            row.counters(),
        ));
    }
    require_coherent_stage_bindings(stage_index, required_stages)?;
    Ok(WorkloadEvidenceStageLinkSet::new(
        stage_index.index_identity().to_string(),
        links,
    ))
}

fn require_coherent_stage_bindings(
    stage_index: &WorkloadEvidenceStageIndexProduct,
    required_stages: &[WorkloadEvidenceStage],
) -> Result<(), WorkloadEvidenceLedgerError> {
    require_stage_binding(
        stage_index,
        required_stages,
        WorkloadEvidenceStage::Transform,
        WorkloadEvidenceStage::Projection,
    )?;
    require_stage_binding(
        stage_index,
        required_stages,
        WorkloadEvidenceStage::RetainedReplay,
        WorkloadEvidenceStage::Transform,
    )
}

fn require_stage_binding(
    stage_index: &WorkloadEvidenceStageIndexProduct,
    required_stages: &[WorkloadEvidenceStage],
    dependent_stage: WorkloadEvidenceStage,
    upstream_stage: WorkloadEvidenceStage,
) -> Result<(), WorkloadEvidenceLedgerError> {
    if !required_stages.contains(&dependent_stage) || !required_stages.contains(&upstream_stage) {
        return Ok(());
    }
    let Some(dependent_row) = stage_index.row_for_stage(dependent_stage) else {
        return Ok(());
    };
    let Some(upstream_binding) = dependent_row.upstream_stage_binding() else {
        return Ok(());
    };
    let upstream_row = stage_index.row_for_stage(upstream_stage).ok_or(
        WorkloadEvidenceLedgerError::MissingAuthorityStage(upstream_stage),
    )?;
    if upstream_binding.upstream_stage() == upstream_stage
        && upstream_binding.upstream_evidence_identity() == upstream_row.evidence_identity()
    {
        Ok(())
    } else {
        Err(
            WorkloadEvidenceLedgerError::MismatchedAuthorityStageBinding(
                dependent_stage,
                upstream_stage,
            ),
        )
    }
}
