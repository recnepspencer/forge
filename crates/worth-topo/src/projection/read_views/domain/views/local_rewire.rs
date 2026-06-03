use super::super::error::TopologyReadError;
use super::super::request::TopologyReadRequest;
use super::super::TopologyReadLedger;
use crate::projection::read_views::domain::read_proof::report::TopologyReadRequestFamily;
use crate::projection::read_views::TopologyLocalRewireNeighborhoodView;
use crate::projection::runtime_boundary::read_execution::{
    decode_local_rewire_neighborhood, execute_local_rewire_read, prev_relation_name,
    successor_relation_name, TopologyReadExecutionTarget,
};
use forge_query::facade::ForgeQueryWorkspace;

impl TopologyReadLedger {
    pub(crate) fn local_rewire_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        execution_target: &TopologyReadExecutionTarget,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyReadError> {
        let request = TopologyReadRequest::LocalRewireNeighborhood {
            moved_half_edge_identity: moved_identity.to_string(),
            cycle_depth: u8::try_from(cycle_count)
                .expect("supported traversal depth must fit in u8"),
        };
        Self::require_supported_traversal_depth(
            TopologyReadRequestFamily::LocalRewireNeighborhood,
            cycle_count,
        )?;
        let executed = execute_local_rewire_read(
            workspace,
            execution_target,
            &request,
            moved_identity,
            cycle_count,
        )?;
        let request_report = self.record_report(executed.report);
        let decoded = decode_local_rewire_neighborhood(
            executed.result.rows(),
            moved_identity,
            cycle_count,
            &successor_relation_name(),
            &prev_relation_name(),
            "local rewire neighborhood",
        )?;
        Ok(TopologyLocalRewireNeighborhoodView {
            request_report,
            moved_half_edge_identity: moved_identity.to_string(),
            old_successor_identity: decoded.old_successor_identity,
            old_predecessor_identity: decoded.old_predecessor_identity,
            cycle_identities: decoded.cycle_identities,
            cycle_half_edges: decoded.cycle_half_edges,
        })
    }
}
