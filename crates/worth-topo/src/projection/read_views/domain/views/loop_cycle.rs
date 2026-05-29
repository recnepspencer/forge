use forge_query::facade::ForgeQueryWorkspace;

use super::super::error::TopologyDomainQueryError;
use super::super::request::TopologyDomainQueryRequest;
use super::super::topology::TopologyDomainQuery;
use crate::projection::read_views::TopologyLoopCycleView;
use crate::projection::runtime_boundary::read_execution::{
    decode_loop_cycle, execute_loop_cycle_read, successor_relation_name, ExecutedTopologyReadFamily,
};

impl TopologyDomainQuery {
    #[allow(dead_code)]
    pub fn loop_cycle(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        start_identity: &str,
        count: usize,
    ) -> Result<TopologyLoopCycleView, TopologyDomainQueryError> {
        let request = TopologyDomainQueryRequest::LoopCycleNeighborhood {
            start_half_edge_identity: start_identity.to_string(),
            depth: u8::try_from(count).expect("supported traversal depth must fit in u8"),
        };
        Self::require_supported_traversal_depth(request.family(), count)?;
        let executed =
            self.build_loop_cycle_read_report(workspace, &request, start_identity, count)?;
        let request_report = self.record_report(executed.report);
        let cycle_identities = decode_loop_cycle(
            executed.result.rows(),
            start_identity,
            count,
            &successor_relation_name(),
            "loop cycle",
        )?;
        Ok(TopologyLoopCycleView {
            request_report,
            start_half_edge_identity: start_identity.to_string(),
            cycle_identities,
        })
    }

    fn build_loop_cycle_read_report(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        request: &TopologyDomainQueryRequest,
        start_identity: &str,
        count: usize,
    ) -> Result<ExecutedTopologyReadFamily, TopologyDomainQueryError> {
        execute_loop_cycle_read(workspace, request, start_identity, count)
    }
}




