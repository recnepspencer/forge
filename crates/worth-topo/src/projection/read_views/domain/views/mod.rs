mod adjacency;
#[cfg(test)]
mod boundary_tests;
mod local_rewire;
mod loop_cycle;
mod shell_boundary;

pub use crate::projection::read_views::{
    TopologyAdjacentHalfEdgeEvidence, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyLoopNeighborEvidence, TopologyRadialCandidateEvidence,
    TopologyShellBoundaryNeighborhoodView,
};
