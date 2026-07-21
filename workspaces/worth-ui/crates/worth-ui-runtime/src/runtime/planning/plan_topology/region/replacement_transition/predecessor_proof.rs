use std::rc::Rc;

use super::WorthUiPlanRegionDelta;

#[derive(Clone, Debug)]
pub(crate) struct WorthUiPredecessorRegionProof {
    exact_predecessor: Rc<crate::runtime::active::WorthUiSealedExecutionPlanBundle>,
    delta: WorthUiPlanRegionDelta,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiPredecessorRegionProofDenial {
    MissingReplacementDelta,
    PredecessorArtifactMismatch,
    PredecessorPlanMismatch,
    CandidateArtifactMismatch,
    AllocationIdentityMismatch,
}

impl WorthUiPredecessorRegionProof {
    pub(crate) fn from_active_plan(
        exact_predecessor: Rc<crate::runtime::active::WorthUiSealedExecutionPlanBundle>,
        active_plan_digest: u64,
        active_artifact_digest: u64,
        authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
    ) -> Result<Self, WorthUiPredecessorRegionProofDenial> {
        let delta = authority
            .region_delta()
            .ok_or(WorthUiPredecessorRegionProofDenial::MissingReplacementDelta)?;
        if delta.predecessor_artifact_digest() != active_artifact_digest {
            return Err(WorthUiPredecessorRegionProofDenial::PredecessorArtifactMismatch);
        }
        if delta.predecessor_plan_digest() != active_plan_digest {
            return Err(WorthUiPredecessorRegionProofDenial::PredecessorPlanMismatch);
        }
        if delta.candidate_artifact_digest()
            != authority.plan_input().basis().candidate_artifact_digest()
        {
            return Err(WorthUiPredecessorRegionProofDenial::CandidateArtifactMismatch);
        }
        if delta.allocation_identity_digest() != authority.allocation_identity_digest() {
            return Err(WorthUiPredecessorRegionProofDenial::AllocationIdentityMismatch);
        }
        Ok(Self {
            exact_predecessor,
            delta: delta.clone(),
        })
    }

    pub(crate) fn exact_predecessor(&self) -> &crate::runtime::WorthUiExecutionPlan {
        self.exact_predecessor.execution_plan()
    }

    pub(crate) fn delta(&self) -> &WorthUiPlanRegionDelta {
        &self.delta
    }
}
