//! Serializable summary for `TopologyError`.

use serde::{Deserialize, Serialize};

use crate::errors::data::TopologyError;

/// Serializable typed summary of `TopologyError`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TopologyErrorSummary {
    MissingTwin {
        halfedge_index: u32,
    },
    BrokenLoop {
        face_index: u32,
        starting_halfedge: u32,
    },
    EulerFormulaViolation {
        vertices: usize,
        edges: usize,
        faces: usize,
        expected_chi: i64,
        actual_chi: i64,
    },
    NonManifoldEdge {
        edge_index: u32,
        valence: usize,
    },
    GeneralizedEulerViolation {
        shell_index: u32,
        vertices: usize,
        edges: usize,
        faces: usize,
        genus: usize,
        rings: usize,
        expected_chi: i64,
        actual_chi: i64,
    },
    OrientationInconsistency {
        face_index: u32,
    },
    StaleHandle {
        entity_kind: String,
        index: u32,
        expected_generation: u32,
        actual_generation: u32,
    },
    ZeroAreaFace {
        face_index: u32,
        computed_area: f64,
        threshold: f64,
    },
    ZeroLengthEdge {
        halfedge_index: u32,
        computed_length: f64,
        threshold: f64,
    },
    NegativeShellVolume {
        shell_index: u32,
        signed_volume: f64,
    },
    DegenerateLoop {
        face_index: u32,
        distinct_vertices: usize,
    },
    LoopCorruption {
        walk_kind: String,
        seed_index: u32,
        last_visited_index: u32,
        steps_taken: usize,
        entity_bound: usize,
    },
    MissingVertexPosition {
        vertex_index: u32,
        face_index: u32,
    },
    NonOrientableSurface {
        shell_index: u32,
    },
    BoundaryEdgeInSolid {
        halfedge_index: u32,
        shell_index: u32,
    },
    InvalidOperation {
        detail: String,
    },
    HierarchyViolation {
        parent_kind: String,
        parent_index: u32,
        child_kind: String,
        child_index: u32,
        detail: String,
    },
    RadialEdgeInconsistency {
        halfedge_index: u32,
        actual_edge: u32,
        seed_halfedge_index: u32,
        expected_edge: u32,
    },
    ValidatorFailure {
        validator: String,
        detail: String,
    },
}

impl From<&TopologyError> for TopologyErrorSummary {
    fn from(value: &TopologyError) -> Self {
        match value {
            TopologyError::MissingTwin { halfedge_index } => Self::MissingTwin {
                halfedge_index: *halfedge_index,
            },
            TopologyError::BrokenLoop {
                face_index,
                starting_halfedge,
            } => Self::BrokenLoop {
                face_index: *face_index,
                starting_halfedge: *starting_halfedge,
            },
            TopologyError::EulerFormulaViolation {
                vertices,
                edges,
                faces,
                expected_chi,
                actual_chi,
            } => Self::EulerFormulaViolation {
                vertices: *vertices,
                edges: *edges,
                faces: *faces,
                expected_chi: *expected_chi,
                actual_chi: *actual_chi,
            },
            TopologyError::NonManifoldEdge {
                edge_index,
                valence,
            } => Self::NonManifoldEdge {
                edge_index: *edge_index,
                valence: *valence,
            },
            TopologyError::GeneralizedEulerViolation {
                shell_index,
                vertices,
                edges,
                faces,
                genus,
                rings,
                expected_chi,
                actual_chi,
            } => Self::GeneralizedEulerViolation {
                shell_index: *shell_index,
                vertices: *vertices,
                edges: *edges,
                faces: *faces,
                genus: *genus,
                rings: *rings,
                expected_chi: *expected_chi,
                actual_chi: *actual_chi,
            },
            TopologyError::OrientationInconsistency { face_index } => {
                Self::OrientationInconsistency {
                    face_index: *face_index,
                }
            }
            TopologyError::StaleHandle {
                entity_kind,
                index,
                expected_generation,
                actual_generation,
            } => Self::StaleHandle {
                entity_kind: entity_kind.clone(),
                index: *index,
                expected_generation: *expected_generation,
                actual_generation: *actual_generation,
            },
            TopologyError::ZeroAreaFace {
                face_index,
                computed_area,
                threshold,
            } => Self::ZeroAreaFace {
                face_index: *face_index,
                computed_area: *computed_area,
                threshold: *threshold,
            },
            TopologyError::ZeroLengthEdge {
                halfedge_index,
                computed_length,
                threshold,
            } => Self::ZeroLengthEdge {
                halfedge_index: *halfedge_index,
                computed_length: *computed_length,
                threshold: *threshold,
            },
            TopologyError::NegativeShellVolume {
                shell_index,
                signed_volume,
            } => Self::NegativeShellVolume {
                shell_index: *shell_index,
                signed_volume: *signed_volume,
            },
            TopologyError::DegenerateLoop {
                face_index,
                distinct_vertices,
            } => Self::DegenerateLoop {
                face_index: *face_index,
                distinct_vertices: *distinct_vertices,
            },
            TopologyError::LoopCorruption {
                walk_kind,
                seed_index,
                last_visited_index,
                steps_taken,
                entity_bound,
            } => Self::LoopCorruption {
                walk_kind: walk_kind.clone(),
                seed_index: *seed_index,
                last_visited_index: *last_visited_index,
                steps_taken: *steps_taken,
                entity_bound: *entity_bound,
            },
            TopologyError::MissingVertexPosition {
                vertex_index,
                face_index,
            } => Self::MissingVertexPosition {
                vertex_index: *vertex_index,
                face_index: *face_index,
            },
            TopologyError::NonOrientableSurface { shell_index } => Self::NonOrientableSurface {
                shell_index: *shell_index,
            },
            TopologyError::BoundaryEdgeInSolid {
                halfedge_index,
                shell_index,
            } => Self::BoundaryEdgeInSolid {
                halfedge_index: *halfedge_index,
                shell_index: *shell_index,
            },
            TopologyError::InvalidOperation { detail } => Self::InvalidOperation {
                detail: detail.clone(),
            },
            TopologyError::HierarchyViolation {
                parent_kind,
                parent_index,
                child_kind,
                child_index,
                detail,
            } => Self::HierarchyViolation {
                parent_kind: parent_kind.clone(),
                parent_index: *parent_index,
                child_kind: child_kind.clone(),
                child_index: *child_index,
                detail: detail.clone(),
            },
            TopologyError::RadialEdgeInconsistency {
                halfedge_index,
                actual_edge,
                seed_halfedge_index,
                expected_edge,
            } => Self::RadialEdgeInconsistency {
                halfedge_index: *halfedge_index,
                actual_edge: *actual_edge,
                seed_halfedge_index: *seed_halfedge_index,
                expected_edge: *expected_edge,
            },
            TopologyError::ValidatorFailure { validator, detail } => Self::ValidatorFailure {
                validator: validator.clone(),
                detail: detail.clone(),
            },
        }
    }
}
