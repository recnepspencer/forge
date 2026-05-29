use crate::derived_topology::traversal_views::types::{
    BoundaryInterpretationSummary, TopologyInterpretationSet,
};

pub fn interpret_boundaries(
    interpretations: &TopologyInterpretationSet,
) -> Vec<BoundaryInterpretationSummary> {
    interpretations
        .shells
        .iter()
        .map(|shell| BoundaryInterpretationSummary {
            shell_id: shell.shell_id,
            boundary_component_count: shell.boundary_component_count,
            boundary_half_edge_count: shell.boundary_half_edge_count,
            closed_boundary: shell.boundary_half_edge_count == 0,
        })
        .collect()
}




