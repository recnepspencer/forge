use forge_query::facade::{ResolvedSnapshotBasis, SnapshotResolutionReport};

use super::{
    WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError,
    WorthUiQueryProjectionConsumptionLane,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryPrerequisiteBoundary {
    _sealed: (),
}

impl WorthUiQueryPrerequisiteBoundary {
    pub(crate) const fn new() -> Self {
        Self { _sealed: () }
    }

    pub fn assemble(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
        basis_posture: WorthUiQueryBasisPosture,
        projection_consumption_lane: WorthUiQueryProjectionConsumptionLane,
        inspection_lane: WorthUiQueryInspectionLane,
        causal_explanation_lane: WorthUiQueryCausalExplanationLane,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        let _ = self;
        WorthUiQueryPrerequisiteEvidence::new(
            basis,
            resolution_report,
            basis_posture,
            projection_consumption_lane,
            inspection_lane,
            causal_explanation_lane,
        )
    }

    pub fn graph_aligned(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::GraphAligned,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::WorkspaceInspect,
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection,
        )
    }

    pub fn wrong_world_projection(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::WrongWorldProjection,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::WorkspaceInspect,
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection,
        )
    }

    pub fn rebind_required(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::RebindRequired,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::WorkspaceInspect,
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection,
        )
    }

    pub fn stale_receipt(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::StaleReceipt,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::WorkspaceInspect,
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection,
        )
    }

    pub fn ambiguous_sources(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::AmbiguousSources,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::WorkspaceInspect,
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection,
        )
    }
}
