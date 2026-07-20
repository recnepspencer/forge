use super::WorthUiAllocationPlanningProjection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiInitialAllocationCommitDenial {
    CandidateGraphAuthorityMismatch,
    ActiveAllocationObligations { node_count: usize },
}

/// Real committed allocation truth for synchronous launch.
///
/// A prepared graph has no active allocation rows until mounted layout and
/// measurement authority exist. Launch may therefore commit exactly zero rows,
/// but only after proving that the exact prepared graph currently exposes no
/// allocation-planning obligations. The consumed projection is retained so a
/// copied zero-row claim cannot open plan lowering.
#[derive(Debug)]
pub(crate) struct WorthUiInitialAllocationCommit {
    projection: WorthUiAllocationPlanningProjection,
    allocation_identity_digest: u64,
}

impl WorthUiInitialAllocationCommit {
    pub(crate) fn commit(
        graph: &crate::graph::UiGraphSnapshot,
        projection: WorthUiAllocationPlanningProjection,
    ) -> Result<Self, WorthUiInitialAllocationCommitDenial> {
        if graph.authority_identity() != projection.graph_authority_identity() {
            return Err(WorthUiInitialAllocationCommitDenial::CandidateGraphAuthorityMismatch);
        }
        let node_count = graph.allocation_planning_node_identities().count();
        if node_count != 0 {
            return Err(
                WorthUiInitialAllocationCommitDenial::ActiveAllocationObligations { node_count },
            );
        }
        let allocation_identity_digest = crate::declaration::stable_text_digest(
            "worth-ui.runtime.initial-allocation.zero-row-commit",
        ) ^ projection.evidence_digest().rotate_left(23);
        Ok(Self {
            projection,
            allocation_identity_digest,
        })
    }

    pub(crate) fn projection(&self) -> &WorthUiAllocationPlanningProjection {
        &self.projection
    }

    pub(crate) fn allocation_identity_digest(&self) -> u64 {
        self.allocation_identity_digest
    }
}
