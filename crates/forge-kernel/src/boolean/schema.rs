//! Data shapes for Boolean operations.

use serde::{Deserialize, Serialize};
use forge_topo::state::TopologyState;
use forge_topo::handles::FaceId;
use crate::geometry_store::GeometryStore;

/// A Boolean operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Classification of a face relative to another solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FaceClassification {
    /// Face is strictly inside the other solid.
    Inside,
    /// Face is strictly outside the other solid.
    Outside,
    /// Face is on the boundary (coplanar) with same normal alignment.
    OnBoundary,
    /// Face is on the boundary (coplanar) with opposite normal alignment.
    OppositeBoundary,
}

/// Which input solid a face originated from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaceOrigin {
    /// Face came from the target solid.
    Target,
    /// Face came from the tool solid.
    Tool,
}

/// A classified face with its origin and classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedFace {
    /// The face handle.
    face: FaceId,
    /// Classification relative to the other solid.
    classification: FaceClassification,
}

impl ClassifiedFace {
    /// Create a new classified face.
    pub fn new(face: FaceId, classification: FaceClassification) -> Self {
        Self { face, classification }
    }

    /// The face handle.
    pub fn face(&self) -> FaceId {
        self.face
    }

    /// Classification relative to the other solid.
    pub fn classification(&self) -> FaceClassification {
        self.classification
    }
}

/// Structured introspection data for Boolean operations (Milestone 2.6).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BooleanIntrospection {
    /// Number of split events (edges/faces split).
    pub split_count: usize,
    /// Classification counts for the target solid.
    pub target_classification: std::collections::HashMap<FaceClassification, usize>,
    /// Classification counts for the tool solid.
    pub tool_classification: std::collections::HashMap<FaceClassification, usize>,
    /// Time taken for the operation in microseconds.
    pub duration_micros: u64,
}

impl BooleanIntrospection {
    /// Create a new introspection record.
    pub fn new(
        split_count: usize,
        target_classified: &[ClassifiedFace],
        tool_classified: &[ClassifiedFace],
        duration: std::time::Duration,
    ) -> Self {
        let mut target_map = std::collections::HashMap::new();
        for f in target_classified {
            *target_map.entry(f.classification()).or_insert(0) += 1;
        }

        let mut tool_map = std::collections::HashMap::new();
        for f in tool_classified {
            *tool_map.entry(f.classification()).or_insert(0) += 1;
        }

        Self {
            split_count,
            target_classification: target_map,
            tool_classification: tool_map,
            duration_micros: duration.as_micros() as u64,
        }
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
    /// Introspection data.
    introspection: BooleanIntrospection,
}

impl BooleanResult {
    /// Create a new Boolean result.
    pub fn new(
        topology: TopologyState,
        geometry: GeometryStore,
        target_faces_kept: usize,
        tool_faces_kept: usize,
        introspection: BooleanIntrospection,
    ) -> Self {
        Self {
            topology,
            geometry,
            target_faces_kept,
            tool_faces_kept,
            introspection,
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

    /// Introspection data.
    pub fn introspection(&self) -> &BooleanIntrospection {
        &self.introspection
    }

    /// Update the duration metric.
    pub fn update_duration(&mut self, duration: std::time::Duration) {
        self.introspection.duration_micros = duration.as_micros() as u64;
    }

    /// Consume and return owned parts.
    pub fn into_parts(self) -> (TopologyState, GeometryStore) {
        (self.topology, self.geometry)
    }
}
