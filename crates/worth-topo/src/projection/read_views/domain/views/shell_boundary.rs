use forge_query::facade::ForgeQueryWorkspace;
use schema::facade::platform::relations::TopologyRelationKind;

use super::super::error::TopologyReadError;
use super::super::request::{TopologyReadAnchorIdentity, TopologyReadRequest};
use super::super::TopologyReadLedger;
use crate::projection::read_views::TopologyShellBoundaryNeighborhoodView;
use crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces;
use crate::projection::runtime_boundary::read_execution::{
    decode_radial_neighborhood, execute_shared_neighborhood_read, radial_next_relation_name,
    uses_edge_relation_name, ExecutedTopologyReadFamily, SharedNeighborhoodReadKind,
    TopologyReadExecutionTarget,
};
use crate::projection::TopologyQueryRowLookup;

impl TopologyReadLedger {
    pub(crate) fn shell_boundary_neighborhood(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        execution_target: &TopologyReadExecutionTarget,
        source_identity: &TopologyReadAnchorIdentity,
    ) -> Result<TopologyShellBoundaryNeighborhoodView, TopologyReadError> {
        let source_identity_value = source_identity.as_str();
        let request = TopologyReadRequest::ShellBoundaryNeighborhood {
            source_half_edge_identity: source_identity.clone(),
        };
        let executed = self.execute_shell_boundary_read(
            workspace,
            execution_target,
            &request,
            source_identity_value,
        )?;
        let request_report = self.record_report(executed.report);
        let decoded = decode_radial_neighborhood(
            executed.result.rows(),
            source_identity_value,
            &uses_edge_relation_name(),
            &radial_next_relation_name(),
            "shell boundary neighborhood",
        )?;
        let membership = resolve_shell_boundary_membership(workspace, source_identity_value)?;

        Ok(TopologyShellBoundaryNeighborhoodView {
            request_report,
            touched_shell_identity: membership.shell_identity,
            touched_face_identity: membership.face_identity,
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

    fn execute_shell_boundary_read(
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
            format!("topology.shell_boundary_neighborhood:{source_identity}"),
            [radial_next_relation_name(), uses_edge_relation_name()],
            SharedNeighborhoodReadKind::SharedAttachment,
            source_identity,
        )
    }
}

struct ShellBoundaryMembership {
    shell_identity: String,
    face_identity: String,
}

fn resolve_shell_boundary_membership(
    workspace: &mut ForgeQueryWorkspace,
    source_half_edge_identity: &str,
) -> Result<ShellBoundaryMembership, TopologyReadError> {
    let surfaces = declare_topology_query_surfaces(workspace)
        .map_err(TopologyReadError::from_query_runtime_error)?;
    let entity_rows = workspace.read(surfaces.entities());
    let relation_rows = workspace.read(surfaces.relations());
    let lookup = TopologyQueryRowLookup::new(&entity_rows, &relation_rows);
    let loop_identity = shell_boundary_loop_identity(&lookup, source_half_edge_identity)?;
    let face_identity = shell_boundary_face_identity(&lookup, &loop_identity)?;
    let shell_identity = shell_boundary_shell_identity(&lookup, &face_identity)?;
    Ok(ShellBoundaryMembership {
        shell_identity,
        face_identity,
    })
}

fn shell_boundary_loop_identity(
    lookup: &TopologyQueryRowLookup<'_>,
    source_half_edge_identity: &str,
) -> Result<String, TopologyReadError> {
    lookup
        .incoming_source_identity(
            source_half_edge_identity,
            TopologyRelationKind::LoopOwnsHalfEdge,
        )
        .map_err(|error| TopologyReadError::read_family_execution_denied(error.to_string()))
}

fn shell_boundary_face_identity(
    lookup: &TopologyQueryRowLookup<'_>,
    loop_identity: &str,
) -> Result<String, TopologyReadError> {
    lookup
        .incoming_source_identity(loop_identity, TopologyRelationKind::FaceOuterLoop)
        .or_else(|_| {
            lookup.incoming_source_identity(loop_identity, TopologyRelationKind::FaceInnerLoop)
        })
        .map_err(|error| TopologyReadError::read_family_execution_denied(error.to_string()))
}

fn shell_boundary_shell_identity(
    lookup: &TopologyQueryRowLookup<'_>,
    face_identity: &str,
) -> Result<String, TopologyReadError> {
    lookup
        .incoming_source_identity(face_identity, TopologyRelationKind::ShellOwnsFace)
        .map_err(|error| TopologyReadError::read_family_execution_denied(error.to_string()))
}
