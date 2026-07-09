use crate::runtime::{
    WorthUiCandidateAuthoringLane, WorthUiCandidateProvenanceHandle,
    WorthUiFileRustReplacementParityCounters, WorthUiPlanSwapReceipt,
    WorthUiReplacementCandidateBasis, WorthUiRuntimeArtifactComparisonOutcome,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiFileRustReplacementPipelineReport {
    authoring_lane: WorthUiCandidateAuthoringLane,
    candidate_basis: WorthUiReplacementCandidateBasis,
    provenance_handle: WorthUiCandidateProvenanceHandle,
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    artifact_comparison_outcome: WorthUiRuntimeArtifactComparisonOutcome,
    candidate_plan_digest: u64,
    lane_support_digest: u64,
    plan_node_count: usize,
    swap_receipt: WorthUiPlanSwapReceipt,
    counters: WorthUiFileRustReplacementParityCounters,
}

#[cfg(test)]
pub(crate) struct WorthUiFileRustReplacementPipelineReportParts {
    pub(crate) authoring_lane: WorthUiCandidateAuthoringLane,
    pub(crate) candidate_basis: WorthUiReplacementCandidateBasis,
    pub(crate) provenance_handle: WorthUiCandidateProvenanceHandle,
    pub(crate) active_artifact_digest: u64,
    pub(crate) candidate_artifact_digest: u64,
    pub(crate) artifact_comparison_outcome: WorthUiRuntimeArtifactComparisonOutcome,
    pub(crate) candidate_plan_digest: u64,
    pub(crate) lane_support_digest: u64,
    pub(crate) plan_node_count: usize,
    pub(crate) swap_receipt: WorthUiPlanSwapReceipt,
    pub(crate) counters: WorthUiFileRustReplacementParityCounters,
}

impl WorthUiFileRustReplacementPipelineReport {
    #[cfg(test)]
    pub(crate) fn new(parts: WorthUiFileRustReplacementPipelineReportParts) -> Self {
        Self {
            authoring_lane: parts.authoring_lane,
            candidate_basis: parts.candidate_basis,
            provenance_handle: parts.provenance_handle,
            active_artifact_digest: parts.active_artifact_digest,
            candidate_artifact_digest: parts.candidate_artifact_digest,
            artifact_comparison_outcome: parts.artifact_comparison_outcome,
            candidate_plan_digest: parts.candidate_plan_digest,
            lane_support_digest: parts.lane_support_digest,
            plan_node_count: parts.plan_node_count,
            swap_receipt: parts.swap_receipt,
            counters: parts.counters,
        }
    }

    pub fn authoring_lane(&self) -> WorthUiCandidateAuthoringLane {
        self.authoring_lane
    }

    pub fn candidate_basis(&self) -> WorthUiReplacementCandidateBasis {
        self.candidate_basis
    }

    pub fn provenance_handle(&self) -> WorthUiCandidateProvenanceHandle {
        self.provenance_handle
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn artifact_comparison_outcome(&self) -> WorthUiRuntimeArtifactComparisonOutcome {
        self.artifact_comparison_outcome
    }

    pub fn candidate_plan_digest(&self) -> u64 {
        self.candidate_plan_digest
    }

    pub fn lane_support_digest(&self) -> u64 {
        self.lane_support_digest
    }

    pub fn plan_node_count(&self) -> usize {
        self.plan_node_count
    }

    pub fn swap_receipt(&self) -> WorthUiPlanSwapReceipt {
        self.swap_receipt
    }

    pub fn counters(&self) -> WorthUiFileRustReplacementParityCounters {
        self.counters
    }
}
