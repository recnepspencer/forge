use forge_query::facade::ForgeQueryWorkspace;

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::projection::runtime_boundary::declared_query_surfaces::{
    materialize_declared_query_surface_row, TopologyDeclaredQuerySurfaces,
    TopologyQuerySurfaceError,
};

pub(crate) fn current_head_materialized_topology(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> Result<MaterializedTopologyView, TopologyQuerySurfaceError> {
    materialize_declared_query_surface_row(workspace, surfaces.materialized())
}

#[cfg(test)]
mod tests;
