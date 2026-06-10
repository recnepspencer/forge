mod projection_index;
mod projection_operations;
mod projection_types;

pub use projection_operations::build_frontier_research_projection_graph_checked;
pub use projection_types::{
    FrontierDegreeBucket, FrontierPressureHaloMotif, FrontierPressureSatellite,
    FrontierPressureVertex, FrontierProjectionEdge, FrontierProjectionNode,
    FrontierResearchProjectionGraph, FrontierResearchProjectionRequest,
};
