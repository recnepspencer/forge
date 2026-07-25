use super::{
    WorthUiPlanRegionIdentity, WorthUiPlanRegionSuccessor, WorthUiPlanRegionTransitionEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanRegionalEvidence {
    predecessor_artifact_digest: Option<u64>,
    predecessor_plan_digest: Option<u64>,
    candidate_artifact_digest: u64,
    allocation_identity_digest: u64,
    affected_region_count: usize,
    transitions: Vec<WorthUiPlanRegionTransitionEvidence>,
}

impl WorthUiPlanRegionalEvidence {
    pub(crate) fn from_lowering(
        authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
        successor: &WorthUiPlanRegionSuccessor,
    ) -> Self {
        let transitions = successor.evidence().to_vec();
        let regional_delta = authority.region_delta();
        Self {
            predecessor_artifact_digest: regional_delta
                .map(super::WorthUiPlanRegionDelta::predecessor_artifact_digest)
                .or_else(|| authority.plan_input().basis().prior_artifact_digest()),
            predecessor_plan_digest: regional_delta
                .map(super::WorthUiPlanRegionDelta::predecessor_plan_digest),
            candidate_artifact_digest: authority.plan_input().basis().candidate_artifact_digest(),
            allocation_identity_digest: authority.allocation_identity_digest(),
            affected_region_count: transitions.len(),
            transitions,
        }
    }

    pub fn predecessor_artifact_digest(&self) -> Option<u64> {
        self.predecessor_artifact_digest
    }

    pub fn predecessor_plan_digest(&self) -> Option<u64> {
        self.predecessor_plan_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn allocation_identity_digest(&self) -> u64 {
        self.allocation_identity_digest
    }

    pub fn affected_region_count(&self) -> usize {
        self.affected_region_count
    }

    pub fn transitions(&self) -> &[WorthUiPlanRegionTransitionEvidence] {
        &self.transitions
    }

    pub fn transition_for_region(
        &self,
        identity: &WorthUiPlanRegionIdentity,
    ) -> Option<&WorthUiPlanRegionTransitionEvidence> {
        self.transitions
            .binary_search_by(|evidence| evidence.region_identity().cmp(identity))
            .ok()
            .map(|index| &self.transitions[index])
    }
}
