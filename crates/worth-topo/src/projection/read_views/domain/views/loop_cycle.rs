use forge_query::facade::ForgeQueryWorkspace;

use super::super::error::TopologyDomainQueryError;
use super::super::request::TopologyDomainQueryRequest;
use super::super::topology::TopologyDomainQuery;
use crate::projection::read_views::TopologyLoopCycleView;
use crate::projection::runtime_boundary::read_execution::{
    execute_loop_cycle_read, successor_relation_name, ExecutedTopologyReadFamily,
};
use crate::projection::runtime_boundary::read_execution::{relation_identity, row_payload};

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
        let cycle_identities =
            decode_cycle_identities(executed.result.rows(), start_identity, count, "loop cycle")?;
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

fn decode_cycle_identities(
    rows: &[forge_query::facade::ForgeQueryEntity],
    start_identity: &str,
    count: usize,
    label: &str,
) -> Result<Vec<String>, TopologyDomainQueryError> {
    let mut cycle = Vec::with_capacity(count);
    let mut current = start_identity;
    for _ in 0..count {
        cycle.push(current.to_string());
        current = relation_identity(
            Some(row_payload(rows, current, label)?),
            &successor_relation_name(),
            label,
        )?;
    }
    Ok(cycle)
}
