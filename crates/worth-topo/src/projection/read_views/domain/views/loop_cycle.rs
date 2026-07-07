use forge_query::facade::ForgeQueryWorkspace;

use super::super::error::TopologyReadError;
use super::super::read_proof::report::TopologyReadRequestFamily;
use super::super::request::{TopologyReadAnchorIdentity, TopologyReadRequest};
use super::super::TopologyReadLedger;
use crate::projection::read_views::TopologyLoopCycleView;
use crate::projection::runtime_boundary::read_execution::{
    decode_loop_cycle, execute_loop_cycle_read, successor_relation_name,
    ExecutedTopologyReadFamily, TopologyReadExecutionTarget,
};

impl TopologyReadLedger {
    pub(crate) fn loop_cycle(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        execution_target: &TopologyReadExecutionTarget,
        start_identity: &TopologyReadAnchorIdentity,
        count: usize,
    ) -> Result<TopologyLoopCycleView, TopologyReadError> {
        let start_identity_value = start_identity.as_str();
        Self::require_supported_traversal_depth(
            TopologyReadRequestFamily::LoopCycleNeighborhood,
            count,
        )?;
        let request = TopologyReadRequest::LoopCycleNeighborhood {
            start_half_edge_identity: start_identity.clone(),
            depth: u8::try_from(count).map_err(|_| {
                TopologyReadError::unsupported_traversal_depth(
                    TopologyReadRequestFamily::LoopCycleNeighborhood,
                    count,
                    usize::from(u8::MAX),
                )
            })?,
        };
        let executed = self.build_loop_cycle_read_report(
            workspace,
            execution_target,
            &request,
            start_identity_value,
            count,
        )?;
        let request_report = self.record_report(executed.report);
        let cycle_identities = decode_loop_cycle(
            executed.result.rows(),
            start_identity_value,
            count,
            &successor_relation_name(),
            "loop cycle",
        )?;
        Ok(TopologyLoopCycleView {
            request_report,
            start_half_edge_identity: start_identity_value.to_string(),
            cycle_identities,
        })
    }

    fn build_loop_cycle_read_report(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        execution_target: &TopologyReadExecutionTarget,
        request: &TopologyReadRequest,
        start_identity: &str,
        count: usize,
    ) -> Result<ExecutedTopologyReadFamily, TopologyReadError> {
        execute_loop_cycle_read(workspace, execution_target, request, start_identity, count)
    }
}
