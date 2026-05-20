use super::super::error::TopologyDomainQueryError;
use super::super::request::TopologyDomainQueryRequest;
use super::super::topology::TopologyDomainQuery;
use crate::projection::diagnostic_surfaces::read_proof::report::TopologyDomainQueryRequestFamily;
use crate::projection::read_views::TopologyLocalRewireNeighborhoodView;
use crate::projection::runtime_boundary::read_execution::{
    decode_local_rewire_neighborhood, execute_local_rewire_read, prev_relation_name,
    successor_relation_name,
};
use forge_query::facade::ForgeQueryWorkspace;

impl TopologyDomainQuery {
    pub fn local_rewire_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        moved_identity: &str,
        cycle_count: usize,
    ) -> Result<TopologyLocalRewireNeighborhoodView, TopologyDomainQueryError> {
        let request = TopologyDomainQueryRequest::LocalRewireNeighborhood {
            moved_half_edge_identity: moved_identity.to_string(),
            cycle_depth: u8::try_from(cycle_count)
                .expect("supported traversal depth must fit in u8"),
        };
        Self::require_supported_traversal_depth(
            TopologyDomainQueryRequestFamily::LocalRewireNeighborhood,
            cycle_count,
        )?;
        let executed = execute_local_rewire_read(workspace, &request, moved_identity, cycle_count)?;
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
