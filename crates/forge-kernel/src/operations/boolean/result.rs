//! Boolean operation result types.
//!
//! DOMAIN: Output data shapes for completed boolean operations.
//! Domain-only — audit metadata (replay, lineage, decisions) lives in
//! the `OperationResult` envelope, not here.

use serde::{Deserialize, Serialize};
use forge_topo::state::TopologyState;
use crate::geometry_state::GeometryState;
use crate::brep::state::BrepState;
use crate::core::KernelState;
use super::classify_schema::ClassifiedFace;

/// Structured introspection data for Boolean operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BooleanIntrospection {
    /// Number of split events (edges/faces split).
    pub split_count: usize,
    /// Classification counts for the target solid.
    pub target_classification: std::collections::BTreeMap<super::classify_schema::FaceClassification, usize>,
    /// Classification counts for the tool solid.
    pub tool_classification: std::collections::BTreeMap<super::classify_schema::FaceClassification, usize>,
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
        let mut target_map = std::collections::BTreeMap::new();
        for f in target_classified {
            *target_map.entry(f.classification()).or_insert(0) += 1;
        }

        let mut tool_map = std::collections::BTreeMap::new();
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

/// Result of a Boolean operation — domain data only.
///
/// Audit metadata (replay log, lineage events, decision log, metrics)
/// lives in the `OperationResult<BooleanResult>` envelope, not here.
/// This separation matches the `FeatureOutput` slimming pattern from
/// the Abstraction Plan.
pub struct BooleanResult {
    /// The resulting topology.
    topology: TopologyState,
    /// The resulting geometry.
    geometry: GeometryState,
    /// The resulting B-Rep data.
    brep: BrepState,
    /// Number of faces from the target that were kept.
    target_faces_kept: usize,
    /// Number of faces from the tool that were kept.
    tool_faces_kept: usize,
    /// Introspection data (classification counts, timing).
    introspection: BooleanIntrospection,
}

impl BooleanResult {
    /// Create a new Boolean result.
    pub fn new(
        topology: TopologyState,
        geometry: GeometryState,
        brep: BrepState,
        target_faces_kept: usize,
        tool_faces_kept: usize,
        introspection: BooleanIntrospection,
    ) -> Self {
        Self {
            topology,
            geometry,
            brep,
            target_faces_kept,
            tool_faces_kept,
            introspection,
        }
    }

    /// Create a new Boolean result from an owned `KernelState`.
    pub fn from_kernel_state(
        state: KernelState,
        target_faces_kept: usize,
        tool_faces_kept: usize,
        introspection: BooleanIntrospection,
    ) -> Self {
        let (topology, geometry, brep) = state.into_parts();
        Self {
            topology,
            geometry,
            brep,
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
    pub fn geometry(&self) -> &GeometryState {
        &self.geometry
    }

    /// The resulting B-Rep data.
    pub fn brep(&self) -> &BrepState {
        &self.brep
    }

    /// Number of target faces kept.
    pub fn target_faces_kept(&self) -> usize {
        self.target_faces_kept
    }

    /// Number of tool faces kept.
    pub fn tool_faces_kept(&self) -> usize {
        self.tool_faces_kept
    }

    /// Override face counts (used when the assembly order differs).
    pub fn set_face_counts(&mut self, target: usize, tool: usize) {
        self.target_faces_kept = target;
        self.tool_faces_kept = tool;
    }

    /// Introspection data.
    pub fn introspection(&self) -> &BooleanIntrospection {
        &self.introspection
    }

    /// Update the duration metric.
    pub fn update_duration(&mut self, duration: std::time::Duration) {
        self.introspection.duration_micros = duration.as_micros() as u64;
    }

    /// Mutable access to the geometry store (for coordinate restoration).
    pub fn geometry_mut(&mut self) -> &mut GeometryState {
        &mut self.geometry
    }

    /// Mutable access to B-Rep store.
    pub fn brep_mut(&mut self) -> &mut BrepState {
        &mut self.brep
    }

    /// Consume and return state components.
    pub fn into_states(self) -> (TopologyState, GeometryState, BrepState) {
        (self.topology, self.geometry, self.brep)
    }

    /// Consume and return all domain parts.
    pub fn into_parts(self) -> (TopologyState, GeometryState, BrepState, BooleanIntrospection) {
        (self.topology, self.geometry, self.brep, self.introspection)
    }
}
