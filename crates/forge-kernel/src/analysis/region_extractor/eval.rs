//! N-ring BFS extraction algorithm.
//!
//! DOMAIN: Given a seed face, BFS-expand through edge adjacency to collect
//! an N-ring neighborhood, then gather all halfedges, vertices, and geometry.
//!
//! DEPENDENCIES: `forge-topo` (arena, traverse), `geometry_store` (GeometryStore)

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::traverse::{FaceEdgeIterator, edge_faces};

use crate::geometry_store::GeometryStore;
use super::schema::{ExtractedRegion, SerializedHalfEdge, SerializedPlane};

/// Extract the N-ring neighborhood of a seed face from the arena.
///
/// BFS-expands from `seed_face` through shared edges (via halfedge twins)
/// for `depth` rings. Collects all faces, halfedges, vertices, and their
/// associated geometry from the `GeometryStore`.
///
/// Ring 0 = seed face only. Ring 1 = seed + all edge-adjacent faces. Etc.
pub fn extract_n_ring(
    arena: &TopologyArena,
    geometry_store: &GeometryStore,
    seed_face: FaceId,
    depth: usize,
) -> Result<ExtractedRegion, KernelError> {
    let mut visited_faces: BTreeSet<FaceId> = BTreeSet::new();
    let mut frontier: VecDeque<FaceId> = VecDeque::new();

    visited_faces.insert(seed_face);
    frontier.push_back(seed_face);

    for _ring in 0..depth {
        let frontier_size = frontier.len();
        for _ in 0..frontier_size {
            let face = match frontier.pop_front() {
                Some(f) => f,
                None => return Err(KernelError::InternalError {
                    message: "BFS frontier unexpectedly empty".into(),
                    context: None,
                }),
            };

            let neighbor_faces = collect_adjacent_faces(arena, face)?;
            for neighbor in neighbor_faces {
                if visited_faces.insert(neighbor) {
                    frontier.push_back(neighbor);
                }
            }
        }
    }

    let mut half_edges: BTreeSet<HalfEdgeId> = BTreeSet::new();
    let mut vertices: BTreeSet<VertexId> = BTreeSet::new();
    let mut half_edge_connectivity: BTreeMap<u32, SerializedHalfEdge> = BTreeMap::new();

    for &face in &visited_faces {
        let iter = FaceEdgeIterator::new(arena, face)?;
        for he_result in iter {
            let he_id = he_result?;
            half_edges.insert(he_id);

            let he_data = arena.get_half_edge(he_id)?;
            vertices.insert(he_data.origin());
            half_edge_connectivity.insert(
                he_id.index(),
                SerializedHalfEdge::from_half_edge_data(he_data),
            );

            let twin = he_data.twin();
            half_edges.insert(twin);

            let twin_data = arena.get_half_edge(twin)?;
            half_edge_connectivity.insert(
                twin.index(),
                SerializedHalfEdge::from_half_edge_data(twin_data),
            );
        }
    }

    let mut face_planes: BTreeMap<u32, SerializedPlane> = BTreeMap::new();
    for &face in &visited_faces {
        if let Some(plane) = geometry_store.get_face_plane(face) {
            face_planes.insert(face.index(), SerializedPlane::from_plane(plane));
        }
    }

    let mut vertex_positions: BTreeMap<u32, [f64; 3]> = BTreeMap::new();
    for &vtx in &vertices {
        if let Some(&pos) = geometry_store.get_vertex_position(vtx) {
            vertex_positions.insert(vtx.index(), pos);
        }
    }

    Ok(ExtractedRegion::new(
        seed_face,
        depth,
        visited_faces,
        half_edges,
        vertices,
        half_edge_connectivity,
        face_planes,
        vertex_positions,
    ))
}

/// Collect all faces adjacent to `face` via shared edges.
fn collect_adjacent_faces(
    arena: &TopologyArena,
    face: FaceId,
) -> Result<Vec<FaceId>, KernelError> {
    let mut neighbors = Vec::new();
    let iter = FaceEdgeIterator::new(arena, face)?;

    for he_result in iter {
        let he_id = he_result?;
        let (face_a, face_b) = edge_faces(arena, he_id)?;
        if face_a != face {
            neighbors.push(face_a);
        }
        if face_b != face {
            neighbors.push(face_b);
        }
    }

    Ok(neighbors)
}
