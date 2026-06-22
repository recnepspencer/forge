use forge_query::facade::ForgeQueryWorkspace;

use super::super::error::TopologyReadError;
use super::super::request::{TopologyReadAnchorIdentity, TopologyReadRequest};
use super::super::TopologyReadLedger;
use crate::projection::read_views::{
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
};
use crate::projection::runtime_boundary::read_execution::{
    decode_radial_neighborhood, decode_shared_vertex_neighborhood, ends_at_vertex_relation_name,
    execute_shared_neighborhood_read, radial_next_relation_name, starts_at_vertex_relation_name,
    uses_edge_relation_name, ExecutedTopologyReadFamily, SharedNeighborhoodReadKind,
    TopologyReadExecutionTarget,
};

impl TopologyReadLedger {
    pub(crate) fn shared_vertex_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        execution_target: &TopologyReadExecutionTarget,
        source_identity: &TopologyReadAnchorIdentity,
    ) -> Result<TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyReadError> {
        let source_identity_value = source_identity.as_str();
        let request = TopologyReadRequest::HalfEdgeSharedVertexNeighborhood {
            source_half_edge_identity: source_identity.clone(),
        };
        let executed = self.execute_shared_vertex_read(
            workspace,
            execution_target,
            &request,
            source_identity_value,
        )?;
        let request_report = self.record_report(executed.report);
        let decoded = decode_shared_vertex_neighborhood(
            executed.result.rows(),
            source_identity_value,
            &uses_edge_relation_name(),
            &[
                starts_at_vertex_relation_name(),
                ends_at_vertex_relation_name(),
            ],
            "shared-vertex neighborhood",
        )?;
        Ok(TopologyHalfEdgeSharedVertexNeighborhoodView {
            request_report,
            source_half_edge_identity: source_identity_value.to_string(),
            source_edge_identity: decoded.source_edge_identity,
            source_vertex_identities: decoded.source_vertex_identities,
            vertex_adjacent_half_edge_identities: decoded.vertex_adjacent_half_edge_identities,
            vertex_adjacent_different_edge_half_edge_identities: decoded
                .vertex_adjacent_different_edge_half_edge_identities,
            vertex_adjacent_different_edge_half_edges: decoded
                .vertex_adjacent_different_edge_half_edges,
        })
    }

    pub(crate) fn radial_half_edge_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        execution_target: &TopologyReadExecutionTarget,
        source_identity: &TopologyReadAnchorIdentity,
    ) -> Result<TopologyHalfEdgeRadialNeighborhoodView, TopologyReadError> {
        let source_identity_value = source_identity.as_str();
        let request = TopologyReadRequest::HalfEdgeRadialNeighborhood {
            source_half_edge_identity: source_identity.clone(),
        };
        let executed =
            self.execute_radial_read(workspace, execution_target, &request, source_identity_value)?;
        let request_report = self.record_report(executed.report);
        let decoded = decode_radial_neighborhood(
            executed.result.rows(),
            source_identity_value,
            &uses_edge_relation_name(),
            &radial_next_relation_name(),
            "radial neighborhood",
        )?;
        Ok(TopologyHalfEdgeRadialNeighborhoodView {
            request_report,
            source_half_edge_identity: source_identity_value.to_string(),
            source_edge_identity: decoded.source_edge_identity,
            current_target_half_edge_identity: decoded.current_target_half_edge_identity,
            current_target_edge_identity: decoded.current_target_edge_identity,
            source_radial_next_relation_identity: decoded.source_radial_next_relation_identity,
            same_edge_half_edge_identities: decoded.same_edge_half_edge_identities,
            different_edge_half_edge_identities: decoded.different_edge_half_edge_identities,
            different_edge_half_edges: decoded.different_edge_half_edges,
        })
    }

    fn execute_shared_vertex_read(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        execution_target: &TopologyReadExecutionTarget,
        request: &TopologyReadRequest,
        source_identity: &str,
    ) -> Result<ExecutedTopologyReadFamily, TopologyReadError> {
        execute_shared_neighborhood_read(
            workspace,
            execution_target,
            request,
            format!("topology.shared_vertex_neighborhood:{source_identity}"),
            [
                starts_at_vertex_relation_name(),
                ends_at_vertex_relation_name(),
            ],
            SharedNeighborhoodReadKind::SharedEndpoint,
            source_identity,
        )
    }

    fn execute_radial_read(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        execution_target: &TopologyReadExecutionTarget,
        request: &TopologyReadRequest,
        source_identity: &str,
    ) -> Result<ExecutedTopologyReadFamily, TopologyReadError> {
        execute_shared_neighborhood_read(
            workspace,
            execution_target,
            request,
            format!("topology.radial_neighborhood:{source_identity}"),
            [radial_next_relation_name(), uses_edge_relation_name()],
            SharedNeighborhoodReadKind::SharedAttachment,
            source_identity,
        )
    }
}
