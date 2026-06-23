pub(crate) mod branch_local;
pub(crate) mod current_head;
pub(crate) mod replay;

use crate::construction::digest::digest_owned_parts;
use crate::construction::tests::support::runtime_truth::PrimitiveConstructionCertificationRuntimeTruth;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCorpusLaneGap {
    code: &'static str,
    detail: String,
}

impl PrimitiveConstructionCorpusLaneGap {
    pub(crate) fn new(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }

    pub(crate) fn code(&self) -> &'static str {
        self.code
    }

    pub(crate) fn digest(&self) -> String {
        digest_owned_parts(&[self.code.to_string(), self.detail.clone()])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionCorpusCurrentHeadLane {
    runtime_truth: PrimitiveConstructionCertificationRuntimeTruth,
    lane_digest: String,
}

impl PrimitiveConstructionCorpusCurrentHeadLane {
    pub(crate) fn new(runtime_truth: PrimitiveConstructionCertificationRuntimeTruth) -> Self {
        let lane_digest = digest_owned_parts(&[
            "current_head".to_string(),
            runtime_truth.family().as_str().to_string(),
            runtime_truth.outcome_digest().to_string(),
        ]);
        Self {
            runtime_truth,
            lane_digest,
        }
    }

    pub(crate) fn runtime_truth(&self) -> &PrimitiveConstructionCertificationRuntimeTruth {
        &self.runtime_truth
    }

    pub(crate) fn lane_digest(&self) -> &str {
        &self.lane_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCorpusBranchLocalLane {
    branch_preview_contract_digest: String,
    preview_admission_digest: String,
    branch_admission_digest: String,
    execution_gap: PrimitiveConstructionCorpusLaneGap,
    lane_digest: String,
}

impl PrimitiveConstructionCorpusBranchLocalLane {
    pub(crate) fn new(
        branch_preview_contract_digest: String,
        preview_admission_digest: String,
        branch_admission_digest: String,
        execution_gap: PrimitiveConstructionCorpusLaneGap,
    ) -> Self {
        let lane_digest = digest_owned_parts(&[
            "branch_local".to_string(),
            branch_preview_contract_digest.clone(),
            preview_admission_digest.clone(),
            branch_admission_digest.clone(),
            execution_gap.digest(),
        ]);
        Self {
            branch_preview_contract_digest,
            preview_admission_digest,
            branch_admission_digest,
            execution_gap,
            lane_digest,
        }
    }

    pub(crate) fn preview_admission_digest(&self) -> &str {
        &self.preview_admission_digest
    }

    pub(crate) fn branch_admission_digest(&self) -> &str {
        &self.branch_admission_digest
    }

    pub(crate) fn execution_gap(&self) -> &PrimitiveConstructionCorpusLaneGap {
        &self.execution_gap
    }

    pub(crate) fn lane_digest(&self) -> &str {
        &self.lane_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCorpusReplayLane {
    replay_gap: PrimitiveConstructionCorpusLaneGap,
    lane_digest: String,
}

impl PrimitiveConstructionCorpusReplayLane {
    pub(crate) fn new(replay_gap: PrimitiveConstructionCorpusLaneGap) -> Self {
        let lane_digest = digest_owned_parts(&["replay".to_string(), replay_gap.digest()]);
        Self {
            replay_gap,
            lane_digest,
        }
    }

    pub(crate) fn replay_gap(&self) -> &PrimitiveConstructionCorpusLaneGap {
        &self.replay_gap
    }

    pub(crate) fn lane_digest(&self) -> &str {
        &self.lane_digest
    }
}
