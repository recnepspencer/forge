//! Display implementation for `TopologyError`.

use std::fmt;

use crate::errors::data::TopologyError;

impl fmt::Display for TopologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TopologyError::MissingTwin { halfedge_index } => {
                write!(f, "Halfedge index {} is missing its twin", halfedge_index)
            }
            TopologyError::BrokenLoop {
                face_index,
                starting_halfedge,
            } => {
                write!(
                    f,
                    "Face {} has a broken loop starting at halfedge {}",
                    face_index, starting_halfedge
                )
            }
            TopologyError::EulerFormulaViolation {
                vertices,
                edges,
                faces,
                expected_chi,
                actual_chi,
            } => {
                write!(
                    f,
                    "Euler formula violation: V={} E={} F={}, χ={} (expected {})",
                    vertices, edges, faces, actual_chi, expected_chi
                )
            }
            TopologyError::NonManifoldEdge {
                edge_index,
                valence,
            } => {
                write!(
                    f,
                    "Edge index {} is non-manifold (valence {})",
                    edge_index, valence
                )
            }
            TopologyError::GeneralizedEulerViolation {
                shell_index,
                vertices,
                edges,
                faces,
                genus,
                rings,
                expected_chi,
                actual_chi,
            } => {
                write!(
                    f,
                    "Generalized Euler violation in shell {}: V={} E={} F={} G={} R={}, χ={} (expected {})",
                    shell_index, vertices, edges, faces, genus, rings, actual_chi, expected_chi
                )
            }
            TopologyError::OrientationInconsistency { face_index } => {
                write!(f, "Face {} has inconsistent orientation", face_index)
            }
            TopologyError::StaleHandle {
                entity_kind,
                index,
                expected_generation,
                actual_generation,
            } => {
                write!(
                    f,
                    "Stale {} handle at index {} (expected gen {}, got gen {})",
                    entity_kind, index, expected_generation, actual_generation
                )
            }
            TopologyError::ZeroAreaFace {
                face_index,
                computed_area,
                threshold,
            } => {
                write!(
                    f,
                    "Face {} has near-zero area {:.2e} (threshold: {:.2e})",
                    face_index, computed_area, threshold
                )
            }
            TopologyError::ZeroLengthEdge {
                halfedge_index,
                computed_length,
                threshold,
            } => {
                write!(
                    f,
                    "Edge {} has near-zero length {:.2e} (threshold: {:.2e})",
                    halfedge_index, computed_length, threshold
                )
            }
            TopologyError::NegativeShellVolume {
                shell_index,
                signed_volume,
            } => {
                write!(
                    f,
                    "Shell {} has negative signed volume {:.6e} (normals point inward)",
                    shell_index, signed_volume
                )
            }
            TopologyError::DegenerateLoop {
                face_index,
                distinct_vertices,
            } => {
                write!(
                    f,
                    "Face {} has degenerate loop with only {} distinct vertices (need >= 3)",
                    face_index, distinct_vertices
                )
            }
            TopologyError::LoopCorruption {
                walk_kind,
                seed_index,
                last_visited_index,
                steps_taken,
                entity_bound,
            } => {
                write!(
                    f,
                    "Loop corruption in {}: seed={}, last={}, steps={}/{}",
                    walk_kind, seed_index, last_visited_index, steps_taken, entity_bound
                )
            }
            TopologyError::MissingVertexPosition {
                vertex_index,
                face_index,
            } => {
                write!(
                    f,
                    "Vertex {} referenced by face {} has no position",
                    vertex_index, face_index
                )
            }
            TopologyError::NonOrientableSurface { shell_index } => {
                write!(f, "Shell {} has non-orientable surface topology (kernel targets orientable 2-manifolds only)",
                    shell_index)
            }
            TopologyError::BoundaryEdgeInSolid {
                halfedge_index,
                shell_index,
            } => {
                write!(f, "Halfedge {} is a boundary edge in solid shell {} (solid shells must be watertight)",
                    halfedge_index, shell_index)
            }
            TopologyError::InvalidOperation { detail } => {
                write!(f, "Invalid operation: {}", detail)
            }
            TopologyError::HierarchyViolation {
                parent_kind,
                parent_index,
                child_kind,
                child_index,
                detail,
            } => {
                write!(
                    f,
                    "Hierarchy violation: {} {} → {} {}: {}",
                    parent_kind, parent_index, child_kind, child_index, detail
                )
            }
            TopologyError::RadialEdgeInconsistency {
                halfedge_index,
                actual_edge,
                seed_halfedge_index,
                expected_edge,
            } => {
                write!(f, "Radial ring edge-entity inconsistency: he[{}].edge = {} but ring seed he[{}].edge = {}",
                    halfedge_index, actual_edge, seed_halfedge_index, expected_edge)
            }
            TopologyError::ValidatorFailure { validator, detail } => {
                write!(f, "Validator '{}' failed: {}", validator, detail)
            }
        }
    }
}
