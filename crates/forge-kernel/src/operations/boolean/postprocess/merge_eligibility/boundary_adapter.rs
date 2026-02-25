//! Boundary adapter — extract boundary candidate from topology + geometry.
//!
//! DOMAIN: Convert topology-derived face-group boundaries into raw geometry
//! inputs suitable for the `forge-geom::boundary_cert` certifier.
//!
//! DEPENDENCIES: `forge-topo` (arena, region_extraction), `GeometryState`.
//! INVARIANTS: No policy decisions. Pure data extraction.

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{VertexId, FaceId};
use forge_topo::bitset::EntityBitset;
use forge_topo::algorithms::region_extraction::walk_face_group_boundary_perimeter;

use crate::geometry_state::GeometryView;

/// A 3D boundary segment with provenance tracking.
#[derive(Debug, Clone)]
pub struct BoundarySegment3D {
    /// Start position in 3D.
    start: [f64; 3],
    /// End position in 3D.
    end: [f64; 3],
    /// Stable provenance identifier (vertex index-based).
    provenance: u64,
}

impl BoundarySegment3D {
    /// Start position.
    pub fn get_start(&self) -> [f64; 3] { self.start }

    /// End position.
    pub fn get_end(&self) -> [f64; 3] { self.end }

    /// Provenance identifier.
    pub fn get_provenance(&self) -> u64 { self.provenance }
}

/// Source metadata for a boundary segment (for diagnostics/tracing).
#[derive(Debug, Clone)]
pub struct BoundaryProvenance {
    /// The start vertex handle.
    start_vertex: VertexId,
    /// The end vertex handle.
    end_vertex: VertexId,
}

impl BoundaryProvenance {
    /// The start vertex.
    pub fn get_start_vertex(&self) -> VertexId { self.start_vertex }

    /// The end vertex.
    pub fn get_end_vertex(&self) -> VertexId { self.end_vertex }
}

/// Topology-derived boundary candidate for certification.
///
/// Contains boundary segments in 3D with stable provenance.
/// Kernel-owned (spec §4.3).
#[derive(Debug, Clone)]
pub struct BoundaryCycleCandidate {
    /// Boundary segments in 3D.
    segments_3d: Vec<BoundarySegment3D>,
    /// Source metadata for diagnostics.
    provenance: Vec<BoundaryProvenance>,
}

impl BoundaryCycleCandidate {
    /// The 3D boundary segments.
    pub fn get_segments_3d(&self) -> &[BoundarySegment3D] { &self.segments_3d }

    /// Source metadata.
    pub fn get_provenance(&self) -> &[BoundaryProvenance] { &self.provenance }

    /// Number of segments.
    pub fn segment_count(&self) -> usize { self.segments_3d.len() }
}

/// Extract a boundary candidate from a face group.
///
/// Uses `walk_face_group_boundary_perimeter` to find the boundary vertices,
/// then resolves their 3D positions from the GeometryState.
pub fn extract_boundary_candidate(
    arena: &TopologyArena,
    group: &EntityBitset,
    geom: &dyn GeometryView,
) -> Result<BoundaryCycleCandidate, KernelError> {
    let perimeter_vertices = walk_face_group_boundary_perimeter(arena, group)?;

    if perimeter_vertices.len() < 3 {
        return Err(KernelError::InvalidInput {
            message: format!(
                "Face group boundary has only {} vertices (need >= 3)",
                perimeter_vertices.len()
            ),
            context: None,
        });
    }

    let n = perimeter_vertices.len();
    let mut segments_3d = Vec::with_capacity(n);
    let mut provenance = Vec::with_capacity(n);

    for i in 0..n {
        let v_start = perimeter_vertices[i];
        let v_end = perimeter_vertices[(i + 1) % n];

        let pos_start = geom.get_vertex_position(v_start).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for boundary vertex {}", v_start.index()),
                context: None,
            }
        })?;

        let pos_end = geom.get_vertex_position(v_end).ok_or_else(|| {
            KernelError::InvalidInput {
                message: format!("No position for boundary vertex {}", v_end.index()),
                context: None,
            }
        })?;

        segments_3d.push(BoundarySegment3D {
            start: *pos_start,
            end: *pos_end,
            provenance: i as u64,
        });

        provenance.push(BoundaryProvenance {
            start_vertex: v_start,
            end_vertex: v_end,
        });
    }

    Ok(BoundaryCycleCandidate { segments_3d, provenance })
}

/// Get the plane normal for a face group (using the first face's plane).
///
/// Iterates the arena's live faces to get generation-correct FaceIds,
/// then matches against the bitset by index.
pub fn get_group_plane_normal(
    arena: &TopologyArena,
    group: &EntityBitset,
    geom: &dyn GeometryView,
) -> Result<[f64; 3], KernelError> {
    for (face_id, _face_data) in arena.iter_faces() {
        if group.contains(face_id.index()).unwrap_or(false) {
            if let Some(plane) = geom.get_face_plane(face_id) {
                return Ok(plane.normal());
            }
        }
    }
    Err(KernelError::InvalidInput {
        message: "No face in group has a plane binding".to_string(),
        context: None,
    })
}
