use crate::interpretation::types::{
    WorthBoundaryInterpretationSummary, WorthTopologyInterpretationSet,
};

pub fn interpret_boundaries(
    interpretations: &WorthTopologyInterpretationSet,
) -> Vec<WorthBoundaryInterpretationSummary> {
    interpretations
        .shells
        .iter()
        .map(|shell| WorthBoundaryInterpretationSummary {
            shell_id: shell.shell_id,
            boundary_component_count: shell.boundary_component_count,
            boundary_half_edge_count: shell.boundary_half_edge_count,
            closed_boundary: shell.boundary_half_edge_count == 0,
        })
        .collect()
}
