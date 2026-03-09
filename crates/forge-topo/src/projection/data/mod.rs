mod error;
mod handles;
mod topology;

pub use error::ProjectedTopologyError;
pub use handles::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId,
    ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedVertexId,
};
pub use topology::{
    ProjectedBodyData, ProjectedEdgeData, ProjectedEntityRef, ProjectedFaceData,
    ProjectedHalfEdgeData, ProjectedLoopData, ProjectedLumpData, ProjectedRegionData,
    ProjectedShellData, ProjectedTopology, ProjectedVertexData,
};
