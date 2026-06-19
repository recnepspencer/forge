use topology::facade::TopologyWorkloadReceipt;
mod boolean_loop_reconstruction_closeout;
mod boolean_loop_reconstruction_handoff;
mod boolean_loop_reconstruction_products;
mod boolean_split_handoff;
mod boolean_stage_requirements;
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, CompleteWorkloadEvidenceLedger, DiagnosticWorkloadReceipt,
    GeometryBindingWorkloadReceipt, ProjectionWorkloadReceipt, ResponseWorkloadReceipt,
    RetainedReplayWorkloadReceipt, SurfaceSupportWorkloadReceipt, TransformWorkloadReceipt,
    WorkloadEvidenceStage, WorkloadStageSupport,
};

use super::{boolean_evidence_requirement::map_boolean_ledger_error, WorkloadStageRequirement};
pub use boolean_loop_reconstruction_closeout::PlanarBooleanLoopReconstructionCloseoutInput;
pub use boolean_loop_reconstruction_handoff::{
    CompletedBooleanLoopReconstructionHandoff, PlanarBooleanLoopRuntimeRegistrationProof,
};
pub use boolean_loop_reconstruction_products::CompletedBooleanLoopReconstructionProducts;
pub use boolean_split_handoff::CompletedBooleanSplitHandoff;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkload {
    topology: TopologyWorkloadReceipt,
    geometry_binding: GeometryBindingWorkloadReceipt,
    surface_support: SurfaceSupportWorkloadReceipt,
    projection: ProjectionWorkloadReceipt,
    transform: TransformWorkloadReceipt,
    retained_replay: RetainedReplayWorkloadReceipt,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkloadCompositionError {
    UnsupportedStage(WorkloadStageRequirement),
    MissingEvidenceStage(WorkloadEvidenceStage),
    ManualEvidenceStage(WorkloadEvidenceStage),
    CounterlessEvidenceStage(WorkloadEvidenceStage),
    MismatchedEvidenceStage(WorkloadEvidenceStage),
    LoopReconstructionCloseout(String),
    LoopRuntimeRegistration(String),
}

impl WorkloadCompositionError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::UnsupportedStage(stage) => format!(
                "{} is not admitted for operator composition",
                stage.human_name()
            ),
            Self::MissingEvidenceStage(stage) => {
                format!("workload evidence ledger is missing {}", stage.human_name())
            }
            Self::ManualEvidenceStage(stage) => format!(
                "workload evidence ledger has hand-filled {} instead of a source receipt",
                stage.human_name()
            ),
            Self::CounterlessEvidenceStage(stage) => format!(
                "workload evidence ledger cannot count {} without receipt-backed counters",
                stage.human_name()
            ),
            Self::MismatchedEvidenceStage(stage) => format!(
                "workload evidence ledger does not match the {} receipt",
                stage.human_name()
            ),
            Self::LoopReconstructionCloseout(reason) => reason.clone(),
            Self::LoopRuntimeRegistration(reason) => reason.clone(),
        }
    }
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

fn require_evidence_stage(
    ledger: &CompleteWorkloadEvidenceLedger,
    stage: WorkloadEvidenceStage,
    expected_identity: &str,
) -> Result<(), WorkloadCompositionError> {
    let actual_identity = ledger
        .evidence_for_stage(stage)
        .ok_or(WorkloadCompositionError::MissingEvidenceStage(stage))?;
    if actual_identity == expected_identity {
        Ok(())
    } else {
        Err(WorkloadCompositionError::MismatchedEvidenceStage(stage))
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
