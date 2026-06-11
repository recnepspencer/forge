use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceGuardError, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use super::declaration::{OperatorDeclarationReceipt, WorkloadOperatorFamily};
use super::support::{OperatorSupportReceipt, OperatorWorkloadError};
use crate::workload_composition::{WorkloadStageRequirement, WorthWorkload};

pub trait OperatorReadyWorkload: private::Sealed {
    fn evidence_rows(&self) -> usize;
}

impl OperatorReadyWorkload for WorthWorkload {
    fn evidence_rows(&self) -> usize {
        self.evidence_ledger().counters().rows()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorRun {
    family: WorkloadOperatorFamily,
    requirement: WorkloadStageRequirement,
    declaration: OperatorDeclarationReceipt,
    support: OperatorSupportReceipt,
    consumed_evidence: Vec<WorkloadEvidenceRow>,
}

impl OperatorRun {
    pub(super) fn from_admitted(
        workload: &WorthWorkload,
        declaration: OperatorDeclarationReceipt,
        support: OperatorSupportReceipt,
    ) -> Result<Self, OperatorWorkloadError> {
        require_honest_evidence(workload)?;
        let stage = declaration.requirement().evidence_stage();
        if let Some(stage) = stage {
            workload
                .evidence_ledger()
                .evidence_for_stage(stage)
                .ok_or(OperatorWorkloadError::MissingRequiredStage(stage))?;
        }
        Ok(Self {
            family: declaration.family(),
            requirement: declaration.requirement(),
            declaration,
            support,
            consumed_evidence: workload.evidence_ledger().rows().to_vec(),
        })
    }

    pub fn family(&self) -> WorkloadOperatorFamily {
        self.family
    }

    pub fn requirement(&self) -> WorkloadStageRequirement {
        self.requirement
    }

    pub fn declaration(&self) -> &OperatorDeclarationReceipt {
        &self.declaration
    }

    pub fn support(&self) -> &OperatorSupportReceipt {
        &self.support
    }

    pub fn consumed_evidence(&self) -> &[WorkloadEvidenceRow] {
        &self.consumed_evidence
    }

    pub fn evidence_rows(&self) -> usize {
        self.consumed_evidence.len()
    }
}

fn require_honest_evidence(workload: &WorthWorkload) -> Result<(), OperatorWorkloadError> {
    workload
        .evidence_ledger()
        .guards()
        .assert_no_synthetic_end_to_end_claim()
        .and_then(|guard| guard.assert_uses_real_topology())
        .and_then(|guard| guard.assert_projection_is_receipt_backed())
        .and_then(|guard| guard.assert_transform_changed_geometry())
        .and_then(|guard| guard.assert_replay_consumed_retained_artifact())
        .map(|_| ())
        .map_err(OperatorWorkloadError::EvidenceGuard)?;
    require_projection_counters(workload)?;
    Ok(())
}

fn require_projection_counters(workload: &WorthWorkload) -> Result<(), OperatorWorkloadError> {
    let projection = workload
        .evidence_ledger()
        .rows()
        .iter()
        .find(|row| row.stage() == WorkloadEvidenceStage::Projection)
        .ok_or(OperatorWorkloadError::MissingRequiredStage(
            WorkloadEvidenceStage::Projection,
        ))?;
    let counters = projection.counters();
    if counters.projected_entity_count() == 0 || counters.local_basis_part_count() == 0 {
        return Err(OperatorWorkloadError::SyntheticProjection);
    }
    Ok(())
}

impl WorkloadStageRequirement {
    pub(crate) fn evidence_stage(self) -> Option<WorkloadEvidenceStage> {
        match self {
            Self::Topology => Some(WorkloadEvidenceStage::Topology),
            Self::GeometryBinding => Some(WorkloadEvidenceStage::GeometryBinding),
            Self::SurfaceSupport => Some(WorkloadEvidenceStage::SurfaceSupport),
            Self::Projection => Some(WorkloadEvidenceStage::Projection),
            Self::Transform => Some(WorkloadEvidenceStage::Transform),
            Self::RetainedReplay => Some(WorkloadEvidenceStage::RetainedReplay),
            Self::Diagnostics => Some(WorkloadEvidenceStage::Diagnostics),
            Self::Response => Some(WorkloadEvidenceStage::Response),
            Self::EvidenceLedger => None,
        }
    }
}

impl From<WorkloadEvidenceGuardError> for OperatorWorkloadError {
    fn from(error: WorkloadEvidenceGuardError) -> Self {
        Self::EvidenceGuard(error)
    }
}

mod private {
    use crate::workload_composition::WorthWorkload;

    pub trait Sealed {}

    impl Sealed for WorthWorkload {}
}
