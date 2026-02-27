//! Boolean operation input types.
//!
//! DOMAIN: Data shapes for boolean operation inputs only.
//! Output types are in `result.rs`, classification types in `classify_schema.rs`.

use serde::{Deserialize, Serialize};
use forge_topo::state::TopologyState;
use crate::geometry_state::GeometryState;
use crate::brep::state::BrepState;

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

/// Input data for a Boolean operation between two solids.
#[derive(Clone, Debug)]
pub struct BooleanInput {
    /// The target solid topology.
    target_topology: TopologyState,
    /// The target solid geometry.
    target_geometry: GeometryState,
    /// The target solid B-Rep data.
    target_brep: BrepState,
    /// The tool solid topology.
    tool_topology: TopologyState,
    /// The tool solid geometry.
    tool_geometry: GeometryState,
    /// The tool solid B-Rep data.
    tool_brep: BrepState,
    /// The Boolean operation to perform.
    operation: BooleanOp,
}

impl BooleanInput {
    /// Create a new Boolean input.
    pub fn new(
        target_topology: TopologyState,
        target_geometry: GeometryState,
        target_brep: BrepState,
        tool_topology: TopologyState,
        tool_geometry: GeometryState,
        tool_brep: BrepState,
        operation: BooleanOp,
    ) -> Self {
        Self {
            target_topology,
            target_geometry,
            target_brep,
            tool_topology,
            tool_geometry,
            tool_brep,
            operation,
        }
    }

    /// The target solid topology.
    pub fn target_topology(&self) -> &TopologyState {
        &self.target_topology
    }

    /// The target solid geometry.
    pub fn target_geometry(&self) -> &GeometryState {
        &self.target_geometry
    }

    /// The target solid B-Rep data.
    pub fn target_brep(&self) -> &BrepState {
        &self.target_brep
    }

    /// The tool solid topology.
    pub fn tool_topology(&self) -> &TopologyState {
        &self.tool_topology
    }

    /// The tool solid geometry.
    pub fn tool_geometry(&self) -> &GeometryState {
        &self.tool_geometry
    }

    /// The tool solid B-Rep data.
    pub fn tool_brep(&self) -> &BrepState {
        &self.tool_brep
    }

    /// The Boolean operation type.
    pub fn operation(&self) -> BooleanOp {
        self.operation
    }

    /// Validate that both input solids are well-formed for a Boolean operation.
    ///
    /// Checks:
    /// - Both solids have at least 4 faces (minimum for a closed polyhedron)
    /// - Every halfedge has a valid twin (not an unresolved self-twin)
    /// - Twin reciprocity: `he.twin.twin == he`
    pub fn validate(&self) -> Result<(), forge_core::KernelError> {
        validate_solid(self.target_topology.arena(), "target")?;
        validate_solid(self.tool_topology.arena(), "tool")?;
        Ok(())
    }

    /// Whether either input solid contains curved geometry (NURBS, cylinders, etc.).
    ///
    /// When `true`, the EMBER exact integer grid pipeline cannot be used
    /// and the operation must route through the parametric pipeline.
    /// Note: With the GeometryState/BrepState separation, this always returns false
    /// since GeometryState only contains planar data. Curved geometry detection
    /// should eventually query the BrepState instead.
    pub fn has_curved_geometry(&self) -> bool {
        false
    }

    /// Consume and return owned parts.
    pub fn into_parts(self) -> (TopologyState, GeometryState, TopologyState, GeometryState, BooleanOp) {
        (
            self.target_topology,
            self.target_geometry,
            self.tool_topology,
            self.tool_geometry,
            self.operation,
        )
    }
}

/// Validate a single solid's topology for Boolean input readiness.
fn validate_solid(arena: &forge_topo::arena::TopologyArena, label: &str) -> Result<(), forge_core::KernelError> {
    let face_count = arena.face_count();
    if face_count < 4 {
        return Err(forge_core::KernelError::InvalidInput {
            message: format!(
                "Boolean {} solid has only {} faces (minimum 4 for a closed polyhedron)",
                label, face_count
            ),
            context: None,
        });
    }

    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();

        if he_id == twin_id {
            return Err(forge_core::KernelError::InvalidInput {
                message: format!(
                    "Boolean {} solid: halfedge {} has self-referencing twin (unresolved sentinel twin)",
                    label, he_id
                ),
                context: None,
            });
        }

        let twin_data = arena.get_half_edge(twin_id).map_err(|_| {
            forge_core::KernelError::InvalidInput {
                message: format!(
                    "Boolean {} solid: halfedge {} twin {} is stale/deleted",
                    label, he_id, twin_id
                ),
                context: None,
            }
        })?;

        if twin_data.radial_next() != he_id {
            return Err(forge_core::KernelError::InvalidInput {
                message: format!(
                    "Boolean {} solid: twin reciprocity violated — he[{}].twin={}, he[{}].twin={} (expected {})",
                    label, he_id.index(), twin_id.index(), twin_id.index(), twin_data.radial_next().index(), he_id.index()
                ),
                context: None,
            });
        }
    }

    Ok(())
}
