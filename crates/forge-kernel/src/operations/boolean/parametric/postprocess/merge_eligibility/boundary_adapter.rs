//! Boundary adapter — extract boundary candidate from topology + geometry.
//!
//! DOMAIN: Convert topology-derived face-group boundaries into raw geometry
//! inputs suitable for the `forge-geom::boundary_cert` certifier.
//!
//! DEPENDENCIES: `forge-topo` (arena, region_extraction), `GeometryState`.
//! INVARIANTS: No policy decisions. Pure data extraction.

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::VertexId;
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
    /// Stable segment transport provenance identifier (hash of directed
    /// generational endpoint handles).
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
#[allow(dead_code)]
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

fn pack_vertex_handle(v: VertexId) -> u64 {
    ((v.generation() as u64) << 32) | (v.index() as u64)
}

fn hash_directed_segment_provenance(start: VertexId, end: VertexId) -> u64 {
    // FNV-1a style mixing over two packed generational handles. Ordered endpoints
    // preserve boundary traversal direction in the transport ID.
    let mut h: u64 = 0xcbf29ce484222325;
    for word in [pack_vertex_handle(start), pack_vertex_handle(end)] {
        h ^= word;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Topology-derived boundary candidate for certification.
///
/// Contains boundary segments in 3D with stable provenance.
/// Kernel-owned (spec §4.3).
#[allow(dead_code)]
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
            provenance: hash_directed_segment_provenance(v_start, v_end),
        });

        provenance.push(BoundaryProvenance {
            start_vertex: v_start,
            end_vertex: v_end,
        });
    }

    Ok(BoundaryCycleCandidate { segments_3d, provenance })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_provenance_hash_changes_with_generation() {
        let a0 = VertexId::from_raw_parts(12, 0);
        let a1 = VertexId::from_raw_parts(12, 1);
        let b0 = VertexId::from_raw_parts(34, 0);

        let h0 = hash_directed_segment_provenance(a0, b0);
        let h1 = hash_directed_segment_provenance(a1, b0);

        assert_ne!(
            h0, h1,
            "ABA slot reuse must change segment provenance when vertex generation changes",
        );
    }

    #[test]
    fn segment_provenance_hash_uses_both_endpoints() {
        let a = VertexId::from_raw_parts(10, 2);
        let b = VertexId::from_raw_parts(20, 3);
        let c = VertexId::from_raw_parts(21, 3);

        let ab = hash_directed_segment_provenance(a, b);
        let ac = hash_directed_segment_provenance(a, c);
        let ba = hash_directed_segment_provenance(b, a);

        assert_ne!(ab, ac, "changing endpoint must change segment provenance");
        assert_ne!(ab, ba, "directed boundary segment provenance must be order-sensitive");
    }
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
        if group.contains(face_id.index())? {
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
