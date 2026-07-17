use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiDurableStateReconciliationPlan,
    WorthUiNodeReplacementPlan, WorthUiPendingExecutionPlanLoweringInput,
    WorthUiQueryLiveRebindPlan, WorthUiReplacementImpactClassification, WorthUiRuntimeFrameEpoch,
    WorthUiRuntimeImpactNarrowing,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiStagedReplacement {
    frame_epoch: WorthUiRuntimeFrameEpoch,
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    admitted_candidate: WorthUiAdmittedReplacementCandidate,
    impact: WorthUiReplacementImpactClassification,
    narrowing: WorthUiRuntimeImpactNarrowing,
    node_plan: WorthUiNodeReplacementPlan,
    reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    query_rebind_plan: WorthUiQueryLiveRebindPlan,
    pending_execution_plan_lowering_input: WorthUiPendingExecutionPlanLoweringInput,
}

pub(crate) struct WorthUiStagedReplacementInput {
    pub frame_epoch: WorthUiRuntimeFrameEpoch,
    pub active_artifact_digest: u64,
    pub candidate_artifact_digest: u64,
    pub admitted_candidate: WorthUiAdmittedReplacementCandidate,
    pub impact: WorthUiReplacementImpactClassification,
    pub narrowing: WorthUiRuntimeImpactNarrowing,
    pub node_plan: WorthUiNodeReplacementPlan,
    pub reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    pub query_rebind_plan: WorthUiQueryLiveRebindPlan,
    pub pending_execution_plan_lowering_input: WorthUiPendingExecutionPlanLoweringInput,
}

impl WorthUiStagedReplacement {
    pub(crate) fn new(input: WorthUiStagedReplacementInput) -> Self {
        let WorthUiStagedReplacementInput {
            frame_epoch,
            active_artifact_digest,
            candidate_artifact_digest,
            admitted_candidate,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
        } = input;
        Self {
            frame_epoch,
            active_artifact_digest,
            candidate_artifact_digest,
            admitted_candidate,
            impact,
            narrowing,
            node_plan,
            reconciliation_plan,
            query_rebind_plan,
            pending_execution_plan_lowering_input,
        }
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn admitted_candidate(&self) -> &WorthUiAdmittedReplacementCandidate {
        &self.admitted_candidate
    }

    pub fn impact(&self) -> &WorthUiReplacementImpactClassification {
        &self.impact
    }

    pub fn narrowing(&self) -> &WorthUiRuntimeImpactNarrowing {
        &self.narrowing
    }

    pub fn node_plan(&self) -> &WorthUiNodeReplacementPlan {
        &self.node_plan
    }

    pub fn reconciliation_plan(&self) -> &WorthUiDurableStateReconciliationPlan {
        &self.reconciliation_plan
    }

    pub fn query_rebind_plan(&self) -> &WorthUiQueryLiveRebindPlan {
        &self.query_rebind_plan
    }

    pub fn pending_execution_plan_lowering_input(
        &self,
    ) -> &WorthUiPendingExecutionPlanLoweringInput {
        &self.pending_execution_plan_lowering_input
    }
}
