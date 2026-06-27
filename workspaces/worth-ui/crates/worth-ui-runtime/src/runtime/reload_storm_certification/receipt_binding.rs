use crate::runtime::{
    WorthUiCandidateAuthoringLane, WorthUiFileRustReplacementPipelineReport,
    WorthUiReplacementCandidateBasis, WorthUiRuntimeArtifactComparison,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormReceiptBinding {
    authoring_lane: WorthUiCandidateAuthoringLane,
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    candidate_lowering_basis_digest: u64,
    source_package_digest: u64,
    candidate_plan_digest: Option<u64>,
    swap_receipt_digest: Option<u64>,
}

impl WorthUiReloadStormReceiptBinding {
    pub(crate) fn from_no_op(
        authoring_lane: WorthUiCandidateAuthoringLane,
        candidate_basis: WorthUiReplacementCandidateBasis,
        comparison: &WorthUiRuntimeArtifactComparison,
        source_package_digest: u64,
    ) -> Self {
        Self {
            authoring_lane,
            active_artifact_digest: comparison.active_artifact_digest(),
            candidate_artifact_digest: comparison.candidate_artifact_digest(),
            candidate_lowering_basis_digest: candidate_basis.lowering_basis_digest(),
            source_package_digest,
            candidate_plan_digest: None,
            swap_receipt_digest: None,
        }
    }

    pub(crate) fn from_activation(
        report: &WorthUiFileRustReplacementPipelineReport,
        source_package_digest: u64,
    ) -> Self {
        Self {
            authoring_lane: report.authoring_lane(),
            active_artifact_digest: report.active_artifact_digest(),
            candidate_artifact_digest: report.candidate_artifact_digest(),
            candidate_lowering_basis_digest: report.candidate_basis().lowering_basis_digest(),
            source_package_digest,
            candidate_plan_digest: Some(report.candidate_plan_digest()),
            swap_receipt_digest: Some(swap_receipt_digest(report)),
        }
    }

    pub fn authoring_lane(self) -> WorthUiCandidateAuthoringLane {
        self.authoring_lane
    }

    pub fn candidate_artifact_digest(self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn source_package_digest(self) -> u64 {
        self.source_package_digest
    }

    pub fn candidate_plan_digest(self) -> Option<u64> {
        self.candidate_plan_digest
    }

    pub(crate) fn reusable_for_candidate(
        self,
        authoring_lane: WorthUiCandidateAuthoringLane,
        candidate_basis: WorthUiReplacementCandidateBasis,
        source_package_digest: u64,
    ) -> bool {
        self.authoring_lane == authoring_lane
            && self.candidate_artifact_digest == candidate_basis.artifact_digest().raw()
            && self.candidate_lowering_basis_digest == candidate_basis.lowering_basis_digest()
            && self.source_package_digest == source_package_digest
    }
}

fn swap_receipt_digest(report: &WorthUiFileRustReplacementPipelineReport) -> u64 {
    let swap = report.swap_receipt();
    super::digest::fold_texts([
        format!("prev-artifact:{}", swap.previous_active_artifact_digest()),
        format!("prev-plan:{}", swap.previous_active_plan_digest()),
        format!("next-artifact:{}", swap.next_active_artifact_digest()),
        format!("next-plan:{}", swap.next_active_plan_digest()),
    ])
}
