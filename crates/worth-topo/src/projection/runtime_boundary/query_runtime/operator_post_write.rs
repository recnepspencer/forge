use forge_query::facade::ForgeQueryWorkspace;

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::projection::runtime_boundary::query_assembly::TopologyQueryAssembly;
use crate::topology_operators::application::TopologyOperatorExecutionError;

pub(crate) fn load_post_write_materialized_topology(
    workspace: &mut ForgeQueryWorkspace,
    assembly: &TopologyQueryAssembly,
) -> Result<MaterializedTopologyView, TopologyOperatorExecutionError> {
    let materialized_rows = workspace.materialize(assembly.materialized());
    serde_json::from_value(materialized_rows[0].clone()).map_err(|error| {
        TopologyOperatorExecutionError::MaterializedDecode(format!(
            "query-derived `materialized topology` row failed to decode: {error}"
        ))
    })
}




