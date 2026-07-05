use forge_query::facade::{ResolvedSnapshotBasis, SnapshotResolutionReport};

use super::{
    WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryProjectionConsumptionLane,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryPrerequisiteEvidenceError {
    ResolutionReportMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryPrerequisiteEvidence {
    basis: ResolvedSnapshotBasis,
    resolution_report: SnapshotResolutionReport,
    basis_posture: WorthUiQueryBasisPosture,
    projection_consumption_lane: WorthUiQueryProjectionConsumptionLane,
    inspection_lane: WorthUiQueryInspectionLane,
    causal_explanation_lane: WorthUiQueryCausalExplanationLane,
    projection_contract_digest: Option<Box<str>>,
}

impl WorthUiQueryPrerequisiteEvidence {
    pub(crate) fn new(
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
        basis_posture: WorthUiQueryBasisPosture,
        projection_consumption_lane: WorthUiQueryProjectionConsumptionLane,
        inspection_lane: WorthUiQueryInspectionLane,
        causal_explanation_lane: WorthUiQueryCausalExplanationLane,
        projection_contract_digest: Option<Box<str>>,
    ) -> Result<Self, WorthUiQueryPrerequisiteEvidenceError> {
        if resolution_report.basis_digest() != basis.proof().digest()
            || resolution_report.resolution_mode() != basis.resolution_mode()
        {
            return Err(WorthUiQueryPrerequisiteEvidenceError::ResolutionReportMismatch);
        }

        Ok(Self {
            basis,
            resolution_report,
            basis_posture,
            projection_consumption_lane,
            inspection_lane,
            causal_explanation_lane,
            projection_contract_digest,
        })
    }

    pub fn basis(&self) -> &ResolvedSnapshotBasis {
        &self.basis
    }

    pub fn resolution_report(&self) -> &SnapshotResolutionReport {
        &self.resolution_report
    }

    pub fn basis_posture(&self) -> WorthUiQueryBasisPosture {
        self.basis_posture
    }

    pub fn projection_consumption_lane(&self) -> WorthUiQueryProjectionConsumptionLane {
        self.projection_consumption_lane
    }

    pub fn inspection_lane(&self) -> WorthUiQueryInspectionLane {
        self.inspection_lane
    }

    pub fn causal_explanation_lane(&self) -> WorthUiQueryCausalExplanationLane {
        self.causal_explanation_lane
    }

    pub fn projection_contract_digest(&self) -> Option<&str> {
        self.projection_contract_digest.as_deref()
    }

    pub(crate) fn bound_to_projection_contract(&self, projection_contract_digest: &str) -> Self {
        Self {
            basis: self.basis.clone(),
            resolution_report: self.resolution_report.clone(),
            basis_posture: self.basis_posture,
            projection_consumption_lane: self.projection_consumption_lane,
            inspection_lane: self.inspection_lane,
            causal_explanation_lane: self.causal_explanation_lane,
            projection_contract_digest: Some(projection_contract_digest.into()),
        }
    }
}
