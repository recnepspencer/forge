use forge_query::facade::ForgeQueryWorkspace;

use crate::projection::read_views::domain::parity::{
    TopologyReadParityKind, TopologyReadViewParityArtifact,
};
use crate::projection::read_views::domain::read_proof::parity::TopologyReadViewParityReport;
use crate::projection::read_views::domain::{
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
    TopologyLocalRewireNeighborhoodView, TopologyLoopCycleView, TopologyReadAggregateReport,
    TopologyReadAnchorIdentity, TopologyReadCloseoutReport, TopologyReadError,
    TopologyReadFallbackPosture, TopologyReadLedger, TopologyReadProofReport,
    TopologyReadRequestFamily,
};
use crate::projection::runtime_boundary::read_execution::TopologyReadExecutionTarget;

pub(crate) struct TopologyReadProofHarness {
    execution_mode: TopologyReadProofHarnessExecutionMode,
    state: TopologyReadLedger,
}

#[derive(Clone, Copy)]
enum TopologyReadProofHarnessExecutionMode {
    CurrentHead,
    HistoricalFromWorkspaceToken,
}

impl TopologyReadProofHarness {
    pub(crate) fn current_head() -> Self {
        Self {
            execution_mode: TopologyReadProofHarnessExecutionMode::CurrentHead,
            state: TopologyReadLedger::new(),
        }
    }

    pub(crate) fn historical_from_workspace_token() -> Self {
        Self {
            execution_mode: TopologyReadProofHarnessExecutionMode::HistoricalFromWorkspaceToken,
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
        let execution_target = self.execution_target_for_workspace(workspace);
        let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(source_identity);
        self.state
            .shared_vertex_half_edge_neighborhood(workspace, &execution_target, &anchor)
    }

    pub(crate) fn radial_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> Result<TopologyHalfEdgeRadialNeighborhoodView, TopologyReadError> {
        let execution_target = self.execution_target_for_workspace(workspace);
        let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(source_identity);
        self.state
            .radial_half_edge_neighborhood(workspace, &execution_target, &anchor)
    }

    pub(crate) fn loop_cycle(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        start_identity: &str,
        count: usize,
    ) -> Result<TopologyLoopCycleView, TopologyReadError> {
        let execution_target = self.execution_target_for_workspace(workspace);
        let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(start_identity);
        self.state
            .loop_cycle(workspace, &execution_target, &anchor, count)
    }

    pub(crate) fn local_rewire_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyReadError> {
        let execution_target = self.execution_target_for_workspace(workspace);
        let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(moved_identity);
        self.state
            .local_rewire_neighborhood(workspace, &execution_target, &anchor, cycle_count)
    }

    fn execution_target_for_workspace(
        &self,
        workspace: &ForgeQueryWorkspace,
    ) -> TopologyReadExecutionTarget {
        match self.execution_mode {
            TopologyReadProofHarnessExecutionMode::CurrentHead => {
                TopologyReadExecutionTarget::current_head()
            }
            TopologyReadProofHarnessExecutionMode::HistoricalFromWorkspaceToken => {
                TopologyReadExecutionTarget::historical_snapshot(workspace.snapshot_identity())
            }
        }
    }
}

const _: fn(&TopologyReadProofHarness) -> TopologyReadAggregateReport =
    TopologyReadProofHarness::aggregate_report;
const _: fn(&TopologyReadProofHarness) -> TopologyReadProofReport =
    TopologyReadProofHarness::proof_report;
const _: fn(&TopologyReadProofHarness) -> TopologyReadFallbackPosture =
    TopologyReadProofHarness::fallback_posture;
const _: fn(&TopologyReadProofHarness) -> Vec<TopologyReadRequestFamily> =
    TopologyReadProofHarness::supported_request_families;
