use topology::facade::TopologyWorkloadReceipt;
mod batch_execution_attachment;
mod boolean_chain_handoff;
mod boolean_chain_replay_undo_boundary;
mod boolean_loop_reconstruction_closeout;
mod boolean_loop_reconstruction_handoff;
mod boolean_loop_reconstruction_products;
mod boolean_split_handoff;
mod boolean_stage_requirements;
mod error;
mod lookup_consumed_workload;
mod ordinary_consumer_sweep;
mod query_obligation_selection;
mod replay_undo_boundary;
mod spatial_touch_authority;
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, CompleteWorkloadEvidenceLedger, DiagnosticWorkloadReceipt,
    GeometryBindingWorkloadReceipt, ProjectionWorkloadReceipt, ResponseWorkloadReceipt,
    RetainedReplayWorkloadReceipt, SurfaceSupportWorkloadReceipt, TransformWorkloadReceipt,
    WorkloadEvidenceStage, WorkloadStageSupport,
};

use super::BatchAdmissionExecutionReceipt;
use super::{boolean_evidence_requirement::map_boolean_ledger_error, WorkloadStageRequirement};
pub use boolean_chain_handoff::{
    BooleanChainCompletedReceiptGuard, BooleanChainIntegrationCounters,
    BooleanChainIntegrationHandoff, BooleanChainResidueBoundary, BooleanChainResidueRemovalTrigger,
    BooleanChainResidueRow,
};
pub use boolean_chain_replay_undo_boundary::BooleanChainReplayUndoBoundaryHandoff;
pub use boolean_loop_reconstruction_closeout::PlanarBooleanLoopReconstructionCloseoutInput;
pub use boolean_loop_reconstruction_handoff::{
    CompletedBooleanLoopReconstructionHandoff, PlanarBooleanLoopRuntimeRegistrationProof,
};
pub use boolean_loop_reconstruction_products::CompletedBooleanLoopReconstructionProducts;
pub use boolean_split_handoff::CompletedBooleanSplitHandoff;
pub use error::{LookupConsumedWorkloadDenial, ReplayUndoBoundaryDenial, WorkloadCompositionError};
pub use lookup_consumed_workload::{
    LookupConsumedWorkloadComposition, LookupConsumedWorkloadReuseProduct,
    LookupConsumedWorkloadReuseResolutionDenied,
};
#[cfg(test)]
pub(crate) use ordinary_consumer_sweep::{
    current_replay_undo_boundary_proof, ordinary_consumer_cutover_from_inventory_for_tests,
    ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override,
};
pub(crate) use ordinary_consumer_sweep::{
    current_replay_undo_boundary_route_authority, current_worth_workload_ordinary_consumer_cutover,
    WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverPosture,
    WorthWorkloadOrdinaryConsumerCutoverRow,
};
pub use ordinary_consumer_sweep::{
    current_worth_workload_ordinary_consumer_sweep_closeout,
    worth_workload_ordinary_consumer_residue_rows, CompletedBooleanSplitBatchExecutionCluster,
    LookupConsumedBatchExecutionCluster, WorthWorkloadCompositionExplainerDisposition,
    WorthWorkloadCompositionExplainerLedger, WorthWorkloadCompositionExplainerRow,
    WorthWorkloadOrdinaryConsumerClusterKind, WorthWorkloadOrdinaryConsumerClusterLedger,
    WorthWorkloadOrdinaryConsumerClusterRowDisposition,
    WorthWorkloadOrdinaryConsumerResidueBoundary, WorthWorkloadOrdinaryConsumerResidueRow,
    WorthWorkloadOrdinaryConsumerResidueSurface, WorthWorkloadOrdinaryConsumerSweepCloseoutError,
    WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind,
    WorthWorkloadOrdinaryConsumerSweepResidueRow,
};
pub use replay_undo_boundary::{
    AdmittedBooleanSplitReplayUndoBoundary, BooleanSplitReplayUndoBoundaryRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkload {
    topology: TopologyWorkloadReceipt,
    geometry_binding: GeometryBindingWorkloadReceipt,
    surface_support: SurfaceSupportWorkloadReceipt,
    projection: ProjectionWorkloadReceipt,
    transform: TransformWorkloadReceipt,
    retained_replay: RetainedReplayWorkloadReceipt,
    batch_admission_execution: Option<BatchAdmissionExecutionReceipt>,
    diagnostics: DiagnosticWorkloadReceipt,
    response: ResponseWorkloadReceipt,
    evidence_ledger: CompleteWorkloadEvidenceLedger,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadParts {
    pub topology: TopologyWorkloadReceipt,
    pub geometry_binding: GeometryBindingWorkloadReceipt,
    pub surface_support: SurfaceSupportWorkloadReceipt,
    pub projection: ProjectionWorkloadReceipt,
    pub transform: TransformWorkloadReceipt,
    pub retained_replay: RetainedReplayWorkloadReceipt,
    pub batch_admission_execution: Option<BatchAdmissionExecutionReceipt>,
    pub diagnostics: DiagnosticWorkloadReceipt,
    pub response: ResponseWorkloadReceipt,
    pub evidence_ledger: CompleteWorkloadEvidenceLedger,
}

impl WorthWorkload {
    pub fn compose(parts: WorthWorkloadParts) -> Result<Self, WorkloadCompositionError> {
        require_admitted_stage_postures(&parts)?;
        require_matching_evidence_ledger(&parts)?;

        Ok(Self {
            topology: parts.topology,
            geometry_binding: parts.geometry_binding,
            surface_support: parts.surface_support,
            projection: parts.projection,
            transform: parts.transform,
            retained_replay: parts.retained_replay,
            batch_admission_execution: parts.batch_admission_execution,
            diagnostics: parts.diagnostics,
            response: parts.response,
            evidence_ledger: parts.evidence_ledger,
        })
    }

    pub fn topology(&self) -> &TopologyWorkloadReceipt {
        &self.topology
    }

    pub fn geometry_binding(&self) -> &GeometryBindingWorkloadReceipt {
        &self.geometry_binding
    }

    pub fn projection(&self) -> &ProjectionWorkloadReceipt {
        &self.projection
    }

    pub fn surface_support(&self) -> &SurfaceSupportWorkloadReceipt {
        &self.surface_support
    }

    pub fn transform(&self) -> &TransformWorkloadReceipt {
        &self.transform
    }

    pub fn retained_replay(&self) -> &RetainedReplayWorkloadReceipt {
        &self.retained_replay
    }

    pub fn batch_admission_execution(&self) -> Option<&BatchAdmissionExecutionReceipt> {
        self.batch_admission_execution.as_ref()
    }

    pub fn diagnostics(&self) -> &DiagnosticWorkloadReceipt {
        &self.diagnostics
    }

    pub fn response(&self) -> &ResponseWorkloadReceipt {
        &self.response
    }

    pub fn evidence_ledger(&self) -> &CompleteWorkloadEvidenceLedger {
        &self.evidence_ledger
    }
}

fn require_admitted_stage_postures(
    parts: &WorthWorkloadParts,
) -> Result<(), WorkloadCompositionError> {
    require_admitted(
        WorkloadStageRequirement::GeometryBinding,
        parts.geometry_binding.envelope().posture().support(),
    )?;
    require_admitted(
        WorkloadStageRequirement::SurfaceSupport,
        parts.surface_support.envelope().posture().support(),
    )?;
    require_admitted(
        WorkloadStageRequirement::Projection,
        parts.projection.envelope().posture().support(),
    )?;
    require_admitted(
        WorkloadStageRequirement::Transform,
        parts.transform.envelope().posture().support(),
    )?;
    require_admitted(
        WorkloadStageRequirement::RetainedReplay,
        parts.retained_replay.envelope().posture().support(),
    )?;
    if parts.batch_admission_execution.is_some() {
        require_admitted(
            WorkloadStageRequirement::BatchAdmissionExecution,
            WorkloadStageSupport::Admitted,
        )?;
    }
    require_admitted(
        WorkloadStageRequirement::Diagnostics,
        parts.diagnostics.envelope().posture().support(),
    )?;
    require_admitted(
        WorkloadStageRequirement::Response,
        parts.response.envelope().posture().support(),
    )
}

fn require_matching_evidence_ledger(
    parts: &WorthWorkloadParts,
) -> Result<(), WorkloadCompositionError> {
    require_evidence_stage(
        &parts.evidence_ledger,
        WorkloadEvidenceStage::Topology,
        parts.topology.identity().name(),
    )?;
    require_evidence_stage(
        &parts.evidence_ledger,
        WorkloadEvidenceStage::GeometryBinding,
        &parts.geometry_binding.identity().receipt_identity(),
    )?;
    require_evidence_stage(
        &parts.evidence_ledger,
        WorkloadEvidenceStage::SurfaceSupport,
        &parts.surface_support.identity().receipt_identity(),
    )?;
    require_evidence_stage(
        &parts.evidence_ledger,
        WorkloadEvidenceStage::Projection,
        &parts.projection.identity().receipt_identity(),
    )?;
    require_evidence_stage(
        &parts.evidence_ledger,
        WorkloadEvidenceStage::Transform,
        &parts.transform.identity().receipt_identity(),
    )?;
    require_evidence_stage(
        &parts.evidence_ledger,
        WorkloadEvidenceStage::RetainedReplay,
        &parts.retained_replay.identity().receipt_identity(),
    )?;
    require_evidence_stage(
        &parts.evidence_ledger,
        WorkloadEvidenceStage::Diagnostics,
        &parts.diagnostics.identity().receipt_identity(),
    )?;
    require_evidence_stage(
        &parts.evidence_ledger,
        WorkloadEvidenceStage::Response,
        &parts.response.identity().receipt_identity(),
    )
}

fn require_admitted(
    stage: WorkloadStageRequirement,
    support: WorkloadStageSupport,
) -> Result<(), WorkloadCompositionError> {
    if support == WorkloadStageSupport::Admitted {
        Ok(())
    } else {
        Err(WorkloadCompositionError::UnsupportedStage(stage))
    }
}

pub(super) fn require_evidence_stage(
    ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
    expected_identity: &str,
) -> Result<(), WorkloadCompositionError> {
    let stage_link_set = ledger
        .link_required_stages(&[stage])
        .map_err(|error| match error {
            worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedgerError::MissingAuthorityStage(missing_stage) => {
                WorkloadCompositionError::MissingEvidenceStage(missing_stage)
            }
            worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedgerError::ManualAuthorityStage(manual_stage) => {
                WorkloadCompositionError::ManualEvidenceStage(manual_stage)
            }
            worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedgerError::CounterlessBooleanStage(counterless_stage) => {
                WorkloadCompositionError::CounterlessEvidenceStage(counterless_stage)
            }
            worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedgerError::UnsupportedBooleanStage(unsupported_stage) => {
                stage_requirement_for_boolean_evidence_stage(unsupported_stage)
                    .map(WorkloadCompositionError::UnsupportedStage)
                    .unwrap_or(WorkloadCompositionError::MismatchedEvidenceStage(unsupported_stage))
            }
            worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedgerError::UnadmittedAuthorityStage(unadmitted_stage) => {
                WorkloadCompositionError::MismatchedEvidenceStage(unadmitted_stage)
            }
            _ => WorkloadCompositionError::MismatchedEvidenceStage(stage),
        })?;
    if stage_link_set.links_to_identity(stage, expected_identity) {
        Ok(())
    } else {
        Err(WorkloadCompositionError::MismatchedEvidenceStage(stage))
    }
}

fn stage_requirement_for_boolean_evidence_stage(
    stage: WorkloadEvidenceStage,
) -> Option<WorkloadStageRequirement> {
    match stage {
        WorkloadEvidenceStage::BooleanDeclarationEntry => {
            Some(WorkloadStageRequirement::BooleanDeclarationEntry)
        }
        WorkloadEvidenceStage::BooleanRoutePlan => Some(WorkloadStageRequirement::BooleanRoutePlan),
        WorkloadEvidenceStage::BooleanOperandPairConstruction => {
            Some(WorkloadStageRequirement::BooleanOperandPairConstruction)
        }
        WorkloadEvidenceStage::BooleanBlockerProvenance => {
            Some(WorkloadStageRequirement::BooleanBlockerProvenance)
        }
        WorkloadEvidenceStage::BooleanPrecisionAgreement => {
            Some(WorkloadStageRequirement::BooleanPrecisionAgreement)
        }
        WorkloadEvidenceStage::BooleanSharedPlaneIdentity => {
            Some(WorkloadStageRequirement::BooleanSharedPlaneIdentity)
        }
        WorkloadEvidenceStage::BooleanLocalFrameSelection => {
            Some(WorkloadStageRequirement::BooleanLocalFrameSelection)
        }
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption => {
            Some(WorkloadStageRequirement::BooleanOperandAProjectionConsumption)
        }
        WorkloadEvidenceStage::BooleanOperandBProjectionConsumption => {
            Some(WorkloadStageRequirement::BooleanOperandBProjectionConsumption)
        }
        WorkloadEvidenceStage::BooleanReducedOperandPair => {
            Some(WorkloadStageRequirement::BooleanReducedOperandPair)
        }
        WorkloadEvidenceStage::BooleanEventExtractionRequest => {
            Some(WorkloadStageRequirement::BooleanEventExtractionRequest)
        }
        WorkloadEvidenceStage::BooleanSegmentPairEnumeration => {
            Some(WorkloadStageRequirement::BooleanSegmentPairEnumeration)
        }
        WorkloadEvidenceStage::BooleanEventLedger => {
            Some(WorkloadStageRequirement::BooleanEventLedger)
        }
        WorkloadEvidenceStage::BooleanSplit => Some(WorkloadStageRequirement::BooleanSplit),
        WorkloadEvidenceStage::BooleanLoopReconstruction => {
            Some(WorkloadStageRequirement::BooleanLoopReconstruction)
        }
        _ => None,
    }
}

fn require_boolean_evidence<T: BooleanEvidenceReceipt + 'static>(
    ledger: &CompleteWorkloadEvidenceLedger,
    receipt: &T,
    requirement: WorkloadStageRequirement,
) -> Result<(), WorkloadCompositionError> {
    ledger
        .require_boolean_receipt(receipt)
        .map_err(|error| map_boolean_ledger_error(error, requirement))
}
