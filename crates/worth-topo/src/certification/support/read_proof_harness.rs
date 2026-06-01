use forge_query::facade::ForgeQueryWorkspace;

use crate::projection::diagnostic_surfaces::read_proof::parity::TopologyReadViewParityReport;
use crate::projection::read_views::domain::parity::{
    TopologyReadParityKind, TopologyReadViewParityArtifact,
};
use crate::projection::read_views::domain::{
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
    TopologyLocalRewireNeighborhoodView, TopologyLoopCycleView, TopologyReadAggregateReport,
    TopologyReadCloseoutReport, TopologyReadError, TopologyReadFallbackPosture, TopologyReadLedger,
    TopologyReadProofReport, TopologyReadRequestFamily,
};

pub(crate) struct TopologyReadProofHarness {
    state: TopologyReadLedger,
}

#[allow(dead_code)]
impl TopologyReadProofHarness {
    pub(crate) fn new() -> Self {
        Self {
            state: TopologyReadLedger::new(),
        }
    }

    pub(crate) fn aggregate_report(&self) -> TopologyReadAggregateReport {
        self.state.aggregate_report()
    }

    pub(crate) fn proof_report(&self) -> TopologyReadProofReport {
        self.state.proof_report()
    }

    pub(crate) fn closeout_report(&self) -> TopologyReadCloseoutReport {
        self.state.closeout_report()
    }

    pub(crate) fn fallback_posture(&self) -> TopologyReadFallbackPosture {
        self.state.fallback_posture()
    }

    pub(crate) fn supported_request_families(&self) -> Vec<TopologyReadRequestFamily> {
        self.state.supported_request_families()
    }

    pub(crate) fn record_view_parity(
        &self,
        parity_kind: TopologyReadParityKind,
        left: &TopologyReadViewParityArtifact,
        right: &TopologyReadViewParityArtifact,
    ) -> TopologyReadViewParityReport {
        self.state.record_view_parity(parity_kind, left, right)
    }

    pub(crate) fn shared_vertex_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyReadError> {
        self.state
            .shared_vertex_half_edge_neighborhood(workspace, source_identity)
    }

    pub(crate) fn radial_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeRadialNeighborhoodView, TopologyReadError> {
        self.state
            .radial_half_edge_neighborhood(workspace, source_identity)
    }

    pub(crate) fn loop_cycle(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        start_identity: &str,
        count: usize,
    ) -> Result<TopologyLoopCycleView, TopologyReadError> {
        self.state.loop_cycle(workspace, start_identity, count)
    }

    pub(crate) fn local_rewire_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyReadError> {
        self.state
            .local_rewire_neighborhood(workspace, moved_identity, cycle_count)
    }
}
