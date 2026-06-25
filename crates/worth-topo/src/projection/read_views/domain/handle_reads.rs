use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext, ForgeQueryWorkspace,
};

use super::{
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
    TopologyLocalRewireNeighborhoodView, TopologyLoopCycleView, TopologyReadAggregateReport,
    TopologyReadAnchorIdentity, TopologyReadCloseoutReport, TopologyReadError,
    TopologyReadFallbackPosture, TopologyReadLedger, TopologyReadProofReport,
    TopologyReadRequestFamily, TopologyShellBoundaryNeighborhoodView,
};
use crate::projection::runtime_boundary::read_execution::TopologyReadExecutionTarget;
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
    execution_target: TopologyReadExecutionTarget,
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
        execution_target: TopologyReadExecutionTarget,
    ) -> Self {
        Self {
            handle,
            workspace,
            execution_target,
            state: TopologyReadLedger::new(),
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        self.handle.handle_identity_digest()
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        self.handle.operating_context_identity_digest()
    }

    pub fn aggregate_report(&self) -> TopologyReadAggregateReport {
        self.state.aggregate_report()
    }

    pub fn proof_report(&self) -> TopologyReadProofReport {
        self.state.proof_report()
    }

    pub fn closeout_report(&self) -> TopologyReadCloseoutReport {
        self.state.closeout_report()
    }

    pub fn fallback_posture(&self) -> TopologyReadFallbackPosture {
        self.state.fallback_posture()
    }

    pub fn supported_request_families(&self) -> Vec<TopologyReadRequestFamily> {
        self.state.supported_request_families()
    }

    pub fn shared_vertex_half_edge_neighborhood(
        &mut self,
        source_identity: &TopologyReadAnchorIdentity,
    ) -> Result<TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyReadError> {
        self.state.shared_vertex_half_edge_neighborhood(
            self.workspace,
            &self.execution_target,
            source_identity,
        )
    }

    pub fn radial_half_edge_neighborhood(
        &mut self,
        source_identity: &TopologyReadAnchorIdentity,
    ) -> Result<TopologyHalfEdgeRadialNeighborhoodView, TopologyReadError> {
        self.state.radial_half_edge_neighborhood(
            self.workspace,
            &self.execution_target,
            source_identity,
        )
    }

    pub fn shell_boundary_neighborhood(
        &mut self,
        source_identity: &TopologyReadAnchorIdentity,
    ) -> Result<TopologyShellBoundaryNeighborhoodView, TopologyReadError> {
        self.state.shell_boundary_neighborhood(
            self.workspace,
            &self.execution_target,
            source_identity,
        )
    }

    pub fn loop_cycle(
        &mut self,
        start_identity: &TopologyReadAnchorIdentity,
        count: usize,
    ) -> Result<TopologyLoopCycleView, TopologyReadError> {
        self.state.loop_cycle(
            self.workspace,
            &self.execution_target,
            start_identity,
            count,
        )
    }

    pub fn local_rewire_neighborhood(
        &mut self,
        moved_identity: &TopologyReadAnchorIdentity,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyReadError> {
        self.state.local_rewire_neighborhood(
            self.workspace,
            &self.execution_target,
            moved_identity,
            cycle_count,
        )
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
        TopologyConfiguredDomainReadSession::new(
            self,
            workspace,
            TopologyReadExecutionTarget::current_head(),
        )
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
        let snapshot_identity = workspace.snapshot_identity();
        TopologyConfiguredDomainReadSession::new(
            self,
            workspace,
            TopologyReadExecutionTarget::historical_snapshot(snapshot_identity),
        )
    }
}
