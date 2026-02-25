//! N-ring BFS extraction algorithm.
//!
//! DOMAIN: Given a seed face, BFS-expand through edge adjacency to collect
//! an N-ring neighborhood, then gather all halfedges, vertices, and geometry.
//!
//! DEPENDENCIES: `forge-topo` (arena, traverse), `geometry_state` (GeometryState)

use std::collections::{BTreeMap, VecDeque};

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::bitset::EntityBitset;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::traverse::{FaceEdgeIterator, edge_faces};

use crate::geometry_state::GeometryState;
use super::schema::{ExtractedRegion, SerializedHalfEdge, SerializedPlane};

/// Extract the N-ring neighborhood of a seed face from the arena.
///
/// BFS-expands from `seed_face` through shared edges (via halfedge twins)
/// for `depth` rings. Collects all faces, halfedges, vertices, and their
/// associated geometry from the `GeometryState`.
///
/// Ring 0 = seed face only. Ring 1 = seed + all edge-adjacent faces. Etc.
pub fn extract_n_ring(
    arena: &TopologyArena,
    geometry_state: &GeometryState,
    seed_face: FaceId,
    depth: usize,
) -> Result<ExtractedRegion, KernelError> {
    let mut visited_faces = EntityBitset::for_faces(arena);
    let mut frontier: VecDeque<FaceId> = VecDeque::new();

    visited_faces.insert(seed_face.index())?;
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
                if visited_faces.insert(neighbor.index())? {
                    frontier.push_back(neighbor);
                }
            }
        }
    }

    let mut half_edges = EntityBitset::for_half_edges(arena);
    let mut vertices = EntityBitset::for_vertices(arena);
    let mut half_edge_connectivity: BTreeMap<u32, SerializedHalfEdge> = BTreeMap::new();

    for face_idx in visited_faces.iter_ones() {
        let face = FaceId::from_raw_parts(face_idx, 0);
        let iter = match FaceEdgeIterator::new(arena, face) {
            Ok(it) => it,
            Err(_) => continue, // Skip entirely broken faces
        };
        for he_result in iter {
            let he_id = match he_result {
                Ok(id) => id,
                Err(_) => break, // Stop on cycle/corruption
            };
            half_edges.insert(he_id.index())?;

            let he_data = match arena.get_half_edge(he_id) {
                Ok(d) => d,
                Err(_) => continue,
            };
            vertices.insert(he_data.origin().index())?;
            half_edge_connectivity.insert(
                he_id.index(),
                SerializedHalfEdge::from_half_edge_data(he_data),
            );

            let twin = he_data.radial_next();
            half_edges.insert(twin.index())?;

            if let Ok(twin_data) = arena.get_half_edge(twin) {
                half_edge_connectivity.insert(
                    twin.index(),
                    SerializedHalfEdge::from_half_edge_data(twin_data),
                );
            }
        }
    }

    let mut face_planes: BTreeMap<u32, SerializedPlane> = BTreeMap::new();
    for face_idx in visited_faces.iter_ones() {
        let face = FaceId::from_raw_parts(face_idx, 0);
        if let Some(plane) = geometry_state.get_face_plane(face) {
            face_planes.insert(face.index(), SerializedPlane::from_plane(plane));
        }
    }

    let mut vertex_positions: BTreeMap<u32, [f64; 3]> = BTreeMap::new();
    for vtx_idx in vertices.iter_ones() {
        let vtx = VertexId::from_raw_parts(vtx_idx, 0);
        if let Some(&pos) = geometry_state.get_vertex_position(vtx) {
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
    let iter = match FaceEdgeIterator::new(arena, face) {
        Ok(it) => it,
        Err(_) => return Ok(neighbors),
    };

    for he_result in iter {
        let he_id = match he_result {
            Ok(id) => id,
            Err(_) => break,
        };
        let adjacent_faces = match edge_faces(arena, he_id) {
            Ok(faces) => faces,
            Err(_) => vec![],
        };
        for adj_face in adjacent_faces {
            if adj_face != face {
                neighbors.push(adj_face);
            }
        }
    }

    Ok(neighbors)
}
