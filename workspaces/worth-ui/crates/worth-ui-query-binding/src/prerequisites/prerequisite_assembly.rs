use worth_query::facade::{ResolvedSnapshotBasis, SnapshotResolutionReport};

use super::{
    WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError,
    WorthUiQueryProjectionConsumptionLane,
};

pub(crate) fn construct_prerequisite_evidence(
    basis: ResolvedSnapshotBasis,
    resolution_report: SnapshotResolutionReport,
    basis_posture: WorthUiQueryBasisPosture,
    projection_consumption_lane: WorthUiQueryProjectionConsumptionLane,
    inspection_lane: WorthUiQueryInspectionLane,
    causal_explanation_lane: WorthUiQueryCausalExplanationLane,
) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
    WorthUiQueryPrerequisiteEvidence::new(
        basis,
        resolution_report,
        basis_posture,
        projection_consumption_lane,
        inspection_lane,
        causal_explanation_lane,
        None,
    )
}
