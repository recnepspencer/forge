//! Data shapes for Boolean operations.

use forge_topo::state::TopologyState;
use forge_topo::handles::FaceId;
use crate::geometry_store::GeometryStore;

/// A Boolean operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOp {
    /// Material addition (A ∪ B).
    Union,
    /// Material removal (A − B).
    Subtraction,
    /// Common material (A ∩ B).
    Intersection,
}

/// Input to a Boolean operation: two solids with their geometry.
pub struct BooleanInput {
    /// The target (base) solid topology.
    target_topology: TopologyState,
    /// The target solid geometry.
    target_geometry: GeometryStore,
    /// The tool solid topology.
    tool_topology: TopologyState,
    /// The tool solid geometry.
    tool_geometry: GeometryStore,
    /// The Boolean operation to perform.
    operation: BooleanOp,
}

impl BooleanInput {
    /// Create a new Boolean input.
    pub fn new(
        target_topology: TopologyState,
        target_geometry: GeometryStore,
        tool_topology: TopologyState,
        tool_geometry: GeometryStore,
        operation: BooleanOp,
    ) -> Self {
        Self {
            target_topology,
            target_geometry,
            tool_topology,
            tool_geometry,
            operation,
        }
    }

    /// The target solid topology.
    pub fn target_topology(&self) -> &TopologyState {
        &self.target_topology
    }

    /// The target solid geometry.
    pub fn target_geometry(&self) -> &GeometryStore {
        &self.target_geometry
    }

    /// The tool solid topology.
    pub fn tool_topology(&self) -> &TopologyState {
        &self.tool_topology
    }

    /// The tool solid geometry.
    pub fn tool_geometry(&self) -> &GeometryStore {
        &self.tool_geometry
    }

    /// The Boolean operation type.
    pub fn operation(&self) -> BooleanOp {
        self.operation
    }

    /// Consume and return owned parts.
    pub fn into_parts(self) -> (TopologyState, GeometryStore, TopologyState, GeometryStore, BooleanOp) {
        (
            self.target_topology,
            self.target_geometry,
            self.tool_topology,
            self.tool_geometry,
            self.operation,
        )
    }
}

/// Classification of a face relative to the other solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceClassification {
    /// Face is inside the other solid.
    Inside,
    /// Face is outside the other solid.
    Outside,
    /// Face is on the boundary (coplanar with a face of the other solid).
    OnBoundary,
}

/// Which input solid a face originated from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceOrigin {
    /// Face came from the target solid.
    Target,
    /// Face came from the tool solid.
    Tool,
}

/// A classified face with its origin and classification.
pub struct ClassifiedFace {
    /// The face handle.
    face: FaceId,
    /// Which solid the face comes from.
    origin: FaceOrigin,
    /// Classification relative to the other solid.
    classification: FaceClassification,
}

impl ClassifiedFace {
    /// Create a new classified face.
    pub fn new(face: FaceId, origin: FaceOrigin, classification: FaceClassification) -> Self {
        Self { face, origin, classification }
    }

    /// The face handle.
    pub fn face(&self) -> FaceId {
        self.face
    }

    /// Which solid the face comes from.
    pub fn origin(&self) -> FaceOrigin {
        self.origin
    }

    /// Classification relative to the other solid.
    pub fn classification(&self) -> FaceClassification {
        self.classification
    }
}

/// Result of a Boolean operation.
pub struct BooleanResult {
    /// The resulting topology.
    topology: TopologyState,
    /// The resulting geometry.
    geometry: GeometryStore,
    /// Number of faces from the target that were kept.
    target_faces_kept: usize,
    /// Number of faces from the tool that were kept.
    tool_faces_kept: usize,
}

impl BooleanResult {
    /// Create a new Boolean result.
    pub fn new(
        topology: TopologyState,
        geometry: GeometryStore,
        target_faces_kept: usize,
        tool_faces_kept: usize,
    ) -> Self {
        Self {
            topology,
            geometry,
            target_faces_kept,
            tool_faces_kept,
        }
    }

    /// The resulting topology.
    pub fn topology(&self) -> &TopologyState {
        &self.topology
    }

    /// The resulting geometry.
    pub fn geometry(&self) -> &GeometryStore {
        &self.geometry
    }

    /// Number of target faces kept.
    pub fn target_faces_kept(&self) -> usize {
        self.target_faces_kept
    }

    /// Number of tool faces kept.
    pub fn tool_faces_kept(&self) -> usize {
        self.tool_faces_kept
    }

    /// Consume and return owned parts.
    pub fn into_parts(self) -> (TopologyState, GeometryStore) {
        (self.topology, self.geometry)
    }
}
