pub use crate::projection::data::{
    ProjectedBodyData, ProjectedBodyId, ProjectedEdgeData, ProjectedEdgeId, ProjectedFaceData,
    ProjectedFaceId, ProjectedHalfEdgeData, ProjectedHalfEdgeId, ProjectedLoopData,
    ProjectedLoopId, ProjectedLumpData, ProjectedLumpId, ProjectedRegionData, ProjectedRegionId,
    ProjectedShellData, ProjectedShellId, ProjectedTopology, ProjectedTopologyError,
    ProjectedEntityRef,
    ProjectedVertexData, ProjectedVertexId,
};
pub use crate::projection::logic::{
    ProjectedTopologyQueries, ProjectionBuilder, compute_projected_topology_hash,
    validate_projected_broken_boundary, validate_projected_face_adjacency,
    validate_projected_loop_wiring, validate_projected_manifold_edges,
    validate_projected_radial_edge, validate_projected_shell_closure,
    validate_projected_topology_baseline,
};
pub use crate::projection::presentation::contracts::ProjectedTopologyContract;
