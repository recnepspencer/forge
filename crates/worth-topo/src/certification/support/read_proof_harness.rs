use forge_query::facade::ForgeQueryWorkspace;

use crate::projection::diagnostic_surfaces::read_proof::parity::TopologyDomainQueryViewParityReport;
use crate::projection::read_views::domain::parity::{
    TopologyDomainQueryParityKind, TopologyDomainQueryViewParityArtifact,
};
use crate::projection::read_views::domain::{
    TopologyDomainQueryAggregateReport, TopologyDomainQueryCloseoutReport,
    TopologyDomainQueryError, TopologyDomainQueryFallbackPosture, TopologyDomainQueryProofReport,
    TopologyDomainQueryRequestFamily, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyReadLedger,
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

    pub(crate) fn aggregate_report(&self) -> TopologyDomainQueryAggregateReport {
        self.state.aggregate_report()
    }

    pub(crate) fn proof_report(&self) -> TopologyDomainQueryProofReport {
        self.state.proof_report()
    }

    pub(crate) fn closeout_report(&self) -> TopologyDomainQueryCloseoutReport {
        self.state.closeout_report()
    }

    pub(crate) fn fallback_posture(&self) -> TopologyDomainQueryFallbackPosture {
        self.state.fallback_posture()
    }

    pub(crate) fn supported_request_families(&self) -> Vec<TopologyDomainQueryRequestFamily> {
        self.state.supported_request_families()
    }

    pub(crate) fn record_view_parity(
        &self,
        parity_kind: TopologyDomainQueryParityKind,
        left: &TopologyDomainQueryViewParityArtifact,
        right: &TopologyDomainQueryViewParityArtifact,
    ) -> TopologyDomainQueryViewParityReport {
        self.state.record_view_parity(parity_kind, left, right)
    }

    pub(crate) fn shared_vertex_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyDomainQueryError> {
        self.state
            .shared_vertex_half_edge_neighborhood(workspace, source_identity)
    }

    pub(crate) fn radial_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeRadialNeighborhoodView, TopologyDomainQueryError> {
        self.state
            .radial_half_edge_neighborhood(workspace, source_identity)
    }

    pub(crate) fn loop_cycle(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        start_identity: &str,
        count: usize,
    ) -> Result<TopologyLoopCycleView, TopologyDomainQueryError> {
        self.state.loop_cycle(workspace, start_identity, count)
    }

    pub(crate) fn local_rewire_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyDomainQueryError> {
        self.state
            .local_rewire_neighborhood(workspace, moved_identity, cycle_count)
    }
}
