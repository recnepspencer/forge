use forge_query::facade::ForgeQueryWorkspace;

use crate::derived_topology::materialized_graph::{
    MaterializedTopologyView, TopologyMaterializer, TopologyQueryMaterializationInput,
};
use crate::projection::runtime_boundary::declared_query_surfaces::{
    materialize_declared_query_surface_row, TopologyDeclaredQuerySurfaces,
    TopologyQuerySurfaceError,
};

pub(crate) fn current_head_materialized_topology(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> Result<MaterializedTopologyView, TopologyQuerySurfaceError> {
    match materialize_declared_query_surface_row(workspace, surfaces.materialized()) {
        Ok(materialized) => Ok(materialized),
        Err(_) => fallback_current_head_materialized_topology(workspace, surfaces),
    }
}

fn fallback_current_head_materialized_topology(
    workspace: &mut ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> Result<MaterializedTopologyView, TopologyQuerySurfaceError> {
    let entity_rows = workspace.read(surfaces.entities());
    let relation_rows = workspace.read(surfaces.relations());
    let materialized_input =
        TopologyQueryMaterializationInput::decode(&entity_rows, &relation_rows)
            .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    let materialized = TopologyMaterializer::materialize_query_input(&materialized_input)
        .map_err(|error| TopologyQuerySurfaceError::new(error.to_string()))?;
    Ok(materialized)
}

#[cfg(test)]
mod tests;
