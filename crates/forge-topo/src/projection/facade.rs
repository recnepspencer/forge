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
};
pub use crate::projection::presentation::contracts::ProjectedTopologyContract;
