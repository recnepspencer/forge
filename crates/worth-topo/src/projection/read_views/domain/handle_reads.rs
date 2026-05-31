use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext, ForgeQueryWorkspace,
};

use super::{
    TopologyDomainQueryAggregateReport, TopologyDomainQueryCloseoutReport,
    TopologyDomainQueryError, TopologyDomainQueryFallbackPosture, TopologyDomainQueryProofReport,
    TopologyDomainQueryRequestFamily, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyReadLedger,
};
use crate::query_domain::{
    TopologyCurrentHeadConfiguredDomainHandle, TopologyQueryDomain,
    TopologySnapshotReadOnlyConfiguredDomainHandle,
};

pub struct TopologyConfiguredDomainReadSession<
    'a,
    C: ForgeQueryDomainOperatingContext<TopologyQueryDomain>,
> {
    handle: &'a ForgeQueryAdmittedConfiguredDomainHandle<TopologyQueryDomain, C>,
    workspace: &'a mut ForgeQueryWorkspace,
    state: TopologyReadLedger,
}

pub type TopologyCurrentHeadReadSession<'a> = TopologyConfiguredDomainReadSession<
    'a,
    crate::query_domain::TopologyCurrentHeadAuthoritativeContext,
>;
pub type TopologySnapshotReadOnlyReadSession<'a> =
    TopologyConfiguredDomainReadSession<'a, crate::query_domain::TopologySnapshotReadOnlyContext>;

impl<'a, C: ForgeQueryDomainOperatingContext<TopologyQueryDomain>>
    TopologyConfiguredDomainReadSession<'a, C>
{
    fn new(
        handle: &'a ForgeQueryAdmittedConfiguredDomainHandle<TopologyQueryDomain, C>,
        workspace: &'a mut ForgeQueryWorkspace,
    ) -> Self {
        Self {
            handle,
            workspace,
            state: TopologyReadLedger::new(),
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.handle.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.handle.operating_context_identity_digest()
    }

    pub fn aggregate_report(&self) -> TopologyDomainQueryAggregateReport {
        self.state.aggregate_report()
    }

    pub fn proof_report(&self) -> TopologyDomainQueryProofReport {
        self.state.proof_report()
    }

    pub fn closeout_report(&self) -> TopologyDomainQueryCloseoutReport {
        self.state.closeout_report()
    }

    pub fn fallback_posture(&self) -> TopologyDomainQueryFallbackPosture {
        self.state.fallback_posture()
    }

    pub fn supported_request_families(&self) -> Vec<TopologyDomainQueryRequestFamily> {
        self.state.supported_request_families()
    }

    pub fn shared_vertex_half_edge_neighborhood(
        &mut self,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyDomainQueryError> {
        self.state
            .shared_vertex_half_edge_neighborhood(self.workspace, source_identity)
    }

    pub fn radial_half_edge_neighborhood(
        &mut self,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeRadialNeighborhoodView, TopologyDomainQueryError> {
        self.state
            .radial_half_edge_neighborhood(self.workspace, source_identity)
    }

    pub fn loop_cycle(
        &mut self,
        start_identity: &str,
        count: usize,
    ) -> Result<TopologyLoopCycleView, TopologyDomainQueryError> {
        self.state.loop_cycle(self.workspace, start_identity, count)
    }

    pub fn local_rewire_neighborhood(
        &mut self,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyDomainQueryError> {
        self.state
            .local_rewire_neighborhood(self.workspace, moved_identity, cycle_count)
    }
}

pub trait TopologyCurrentHeadReadHandleExt {
    fn topology_reads<'a>(
        &'a self,
        workspace: &'a mut ForgeQueryWorkspace,
    ) -> TopologyCurrentHeadReadSession<'a>;
}

impl TopologyCurrentHeadReadHandleExt for TopologyCurrentHeadConfiguredDomainHandle {
    fn topology_reads<'a>(
        &'a self,
        workspace: &'a mut ForgeQueryWorkspace,
    ) -> TopologyCurrentHeadReadSession<'a> {
        TopologyConfiguredDomainReadSession::new(self, workspace)
    }
}

pub trait TopologySnapshotReadOnlyReadHandleExt {
    fn topology_reads<'a>(
        &'a self,
        workspace: &'a mut ForgeQueryWorkspace,
    ) -> TopologySnapshotReadOnlyReadSession<'a>;
}

impl TopologySnapshotReadOnlyReadHandleExt for TopologySnapshotReadOnlyConfiguredDomainHandle {
    fn topology_reads<'a>(
        &'a self,
        workspace: &'a mut ForgeQueryWorkspace,
    ) -> TopologySnapshotReadOnlyReadSession<'a> {
        TopologyConfiguredDomainReadSession::new(self, workspace)
    }
}
