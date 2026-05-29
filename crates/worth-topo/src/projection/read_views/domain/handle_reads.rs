use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext, ForgeQueryWorkspace,
};

use super::{
    TopologyDomainQuery, TopologyDomainQueryAggregateReport, TopologyDomainQueryCloseoutReport,
    TopologyDomainQueryError, TopologyDomainQueryProofReport,
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
    TopologyLocalRewireNeighborhoodView, TopologyLoopCycleView,
};
use crate::projection::domain_entry::{
    TopologyCurrentHeadConfiguredDomainHandle, TopologyQueryDomain,
    TopologySnapshotReadOnlyConfiguredDomainHandle,
};

pub struct TopologyConfiguredDomainReadSession<
    'a,
    C: ForgeQueryDomainOperatingContext<TopologyQueryDomain>,
> {
    handle: &'a ForgeQueryAdmittedConfiguredDomainHandle<TopologyQueryDomain, C>,
    workspace: &'a mut ForgeQueryWorkspace,
    domain_query: TopologyDomainQuery,
}

pub type TopologyCurrentHeadReadSession<'a> = TopologyConfiguredDomainReadSession<
    'a,
    crate::projection::TopologyCurrentHeadAuthoritativeContext,
>;
pub type TopologySnapshotReadOnlyReadSession<'a> =
    TopologyConfiguredDomainReadSession<'a, crate::projection::TopologySnapshotReadOnlyContext>;

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
            domain_query: TopologyDomainQuery::load(),
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.handle.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.handle.operating_context_identity_digest()
    }

    pub fn aggregate_report(&self) -> TopologyDomainQueryAggregateReport {
        self.domain_query.aggregate_report()
    }

    pub fn proof_report(&self) -> TopologyDomainQueryProofReport {
        self.domain_query.proof_report()
    }

    pub fn closeout_report(&self) -> TopologyDomainQueryCloseoutReport {
        self.domain_query.closeout_report()
    }

    pub fn shared_vertex_half_edge_neighborhood(
        &mut self,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyDomainQueryError> {
        self.domain_query
            .shared_vertex_half_edge_neighborhood(self.workspace, source_identity)
    }

    pub fn radial_half_edge_neighborhood(
        &mut self,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeRadialNeighborhoodView, TopologyDomainQueryError> {
        self.domain_query
            .radial_half_edge_neighborhood(self.workspace, source_identity)
    }

    pub fn loop_cycle(
        &mut self,
        start_identity: &str,
        count: usize,
    ) -> Result<TopologyLoopCycleView, TopologyDomainQueryError> {
        self.domain_query
            .loop_cycle(self.workspace, start_identity, count)
    }

    pub fn local_rewire_neighborhood(
        &mut self,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyDomainQueryError> {
        self.domain_query
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
