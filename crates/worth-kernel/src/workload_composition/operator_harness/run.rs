use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceGuardError, WorkloadEvidenceStage,
};

use super::declaration::{OperatorDeclarationReceipt, WorkloadOperatorFamily};
use super::evidence_binding::OperatorEvidenceBinding;
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
    evidence_binding: OperatorEvidenceBinding,
}

impl OperatorRun {
    pub(super) fn from_admitted(
        workload: &WorthWorkload,
        declaration: OperatorDeclarationReceipt,
        support: OperatorSupportReceipt,
    ) -> Result<Self, OperatorWorkloadError> {
        require_honest_evidence(workload)?;
        let stage = declaration.requirement().operator_evidence_stage()?;
        if let Some(stage) = stage {
            workload
                .evidence_ledger()
                .link_required_stages(&[stage])
                .map_err(|error| match error {
                    worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedgerError::MissingAuthorityStage(missing_stage) => {
                        OperatorWorkloadError::MissingRequiredStage(missing_stage)
                    }
                    other => OperatorWorkloadError::EvidenceStageBindingFailed(other),
                })?;
        }
        let evidence_binding = OperatorEvidenceBinding::from_ledger(
            workload.evidence_ledger(),
            &required_operator_stage_links(declaration.requirement())?,
        )?;
        Ok(Self {
            family: declaration.family(),
            requirement: declaration.requirement(),
            declaration,
            support,
            evidence_binding,
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

    pub fn evidence_binding(&self) -> &OperatorEvidenceBinding {
        &self.evidence_binding
    }

    pub fn evidence_rows(&self) -> usize {
        self.evidence_binding.evidence_row_count()
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
        .stage_index()
        .link_required_stages(&[WorkloadEvidenceStage::Projection])
        .map_err(|error| match error {
            worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedgerError::MissingAuthorityStage(stage) => {
                OperatorWorkloadError::MissingRequiredStage(stage)
            }
            other => OperatorWorkloadError::EvidenceStageBindingFailed(other),
        })?
        .link_for_stage(WorkloadEvidenceStage::Projection)
        .cloned()
        .ok_or(OperatorWorkloadError::MissingRequiredStage(
            WorkloadEvidenceStage::Projection,
        ))?;
    let counters = projection.counters();
    if counters.projected_entity_count() == 0 || counters.local_basis_part_count() == 0 {
        return Err(OperatorWorkloadError::SyntheticProjection);
    }
    Ok(())
}

fn required_operator_stage_links(
    requirement: WorkloadStageRequirement,
) -> Result<Vec<WorkloadEvidenceStage>, OperatorWorkloadError> {
    let mut stages = WorkloadEvidenceStage::AUTHORITY_STAGES.to_vec();
    if let Some(stage) = requirement.operator_evidence_stage()? {
        if !stages.contains(&stage) {
            stages.push(stage);
        }
    }
    Ok(stages)
}

impl WorkloadStageRequirement {
    pub(crate) fn operator_evidence_stage(
        self,
    ) -> Result<Option<WorkloadEvidenceStage>, OperatorWorkloadError> {
        match self {
            Self::Topology => Ok(Some(WorkloadEvidenceStage::Topology)),
            Self::GeometryBinding => Ok(Some(WorkloadEvidenceStage::GeometryBinding)),
            Self::SurfaceSupport => Ok(Some(WorkloadEvidenceStage::SurfaceSupport)),
            Self::Projection => Ok(Some(WorkloadEvidenceStage::Projection)),
            Self::Transform => Ok(Some(WorkloadEvidenceStage::Transform)),
            Self::RetainedReplay => Ok(Some(WorkloadEvidenceStage::RetainedReplay)),
            Self::Diagnostics => Ok(Some(WorkloadEvidenceStage::Diagnostics)),
            Self::Response => Ok(Some(WorkloadEvidenceStage::Response)),
            Self::BooleanDeclarationEntry
            | Self::BooleanRoutePlan
            | Self::BooleanOperandPairConstruction
            | Self::BooleanBlockerProvenance
            | Self::BooleanPrecisionAgreement
            | Self::BooleanSharedPlaneIdentity
            | Self::BooleanLocalFrameSelection
            | Self::BooleanOperandAProjectionConsumption
            | Self::BooleanOperandBProjectionConsumption
            | Self::BooleanReducedOperandPair
            | Self::BooleanEventExtractionRequest
            | Self::BooleanSegmentPairEnumeration
            | Self::BooleanEventLedger
            | Self::BooleanSplit
            | Self::BooleanLoopReconstruction => {
                Err(OperatorWorkloadError::UnsupportedRequirement(self))
            }
            Self::EvidenceLedger => Ok(None),
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
