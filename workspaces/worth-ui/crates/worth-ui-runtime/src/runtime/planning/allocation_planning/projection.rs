use std::rc::Rc;

use crate::runtime::WorthUiRuntimeFrameEpoch;

#[derive(Debug)]
struct WorthUiAllocationPlanningProjectionAuthority;

/// Exact, non-executable binding between allocation planning and one pending
/// candidate. Allocation-relevant facts remain in the admitted measurement and
/// neighborhood bases; this projection carries no plan rows, Query posture,
/// host contacts, hooks, or reconstruction surface.
#[derive(Clone)]
pub(crate) struct WorthUiAllocationPlanningProjection {
    authority: Rc<WorthUiAllocationPlanningProjectionAuthority>,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    candidate_artifact_digest: u64,
    graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
}

impl WorthUiAllocationPlanningProjection {
    pub(crate) fn seal(
        frame_epoch: WorthUiRuntimeFrameEpoch,
        candidate_artifact_digest: u64,
        graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
    ) -> Self {
        Self {
            authority: Rc::new(WorthUiAllocationPlanningProjectionAuthority),
            frame_epoch,
            candidate_artifact_digest,
            graph_authority_identity,
        }
    }

    pub(crate) fn shares_authority_with(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.authority, &other.authority)
    }

    pub(crate) fn evidence_digest(&self) -> u64 {
        self.candidate_artifact_digest.rotate_left(17) ^ self.frame_epoch.as_u64().rotate_left(31)
    }

    pub(crate) fn graph_authority_identity(&self) -> crate::graph::UiGraphAuthorityIdentity {
        self.graph_authority_identity
    }

    pub(crate) fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub(crate) fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }
}

impl PartialEq for WorthUiAllocationPlanningProjection {
    fn eq(&self, other: &Self) -> bool {
        self.shares_authority_with(other)
    }
}

impl Eq for WorthUiAllocationPlanningProjection {}

impl std::fmt::Debug for WorthUiAllocationPlanningProjection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiAllocationPlanningProjection")
            .field("frame_epoch", &self.frame_epoch)
            .field("candidate_artifact_digest", &self.candidate_artifact_digest)
            .field("graph_authority", &"sealed")
            .finish_non_exhaustive()
    }
}
