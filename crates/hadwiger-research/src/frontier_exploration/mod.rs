mod evidence_bundle;
mod exploration_loop;
mod motif_mining;
mod research_projection;

pub use evidence_bundle::{FrontierExplorationEvidenceBundle, FrontierExplorationEvidencePosture};
pub use exploration_loop::{
    run_frontier_seed_exploration_iterations_checked, FrontierExplorationIterationReport,
    FrontierExplorationRunReport, FrontierExplorationRunRequest, FrontierMutationPolicy,
};
pub use motif_mining::{
    mine_virtual_edge_motifs_checked, BichromaticVirtualEdgeMotif, FrontierMotifMiningReport,
    TerminalColorForcingMotif, VirtualEdgeMotif,
};
pub use research_projection::{
    build_frontier_research_projection_graph_checked,
    propose_frontier_pressure_halo_hypotheses_checked, FrontierDegreeBucket,
    FrontierPressureHaloHypothesis, FrontierPressureHaloMotif, FrontierPressureSatellite,
    FrontierPressureVertex, FrontierProjectionEdge, FrontierProjectionNode,
    FrontierResearchProjectionGraph, FrontierResearchProjectionRequest,
};
