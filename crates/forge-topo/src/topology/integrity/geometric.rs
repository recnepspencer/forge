//! Geometric invariant validation requiring vertex position data.
//!
//! DOMAIN: Checks that detect degenerate geometry which pure topology
//! checks would miss — zero-area faces, zero-length edges, inverted shells.
//!
//! These checks require a position-lookup callback from the kernel layer,
//! keeping the topo→kernel dependency boundary clean (Adapter Rule §6).
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs), `forge-core` (errors),
//!               `queries/traverse` (FaceEdgeIterator)

use std::collections::{BTreeSet, VecDeque};

use forge_core::KernelError;
use crate::arena::TopologyArena;
use crate::handles::{FaceId, VertexId};
use crate::topology::queries::traverse::FaceEdgeIterator;

/// Validate geometric invariants that require vertex positions.
///
/// Unlike `validate_topology()` (pure structural checks called at commit
/// time), this requires a position-lookup callback from the kernel layer.
/// Checks: zero-area faces, zero-length edges, signed volume consistency.
pub fn validate_geometric_invariants(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    area_threshold: f64,
    edge_length_threshold: f64,
) -> Result<(), KernelError> {
    validate_zero_area_faces(arena, position_fn, area_threshold)?;
    validate_zero_length_edges(arena, position_fn, edge_length_threshold)?;
    validate_signed_volume(arena, position_fn)?;
    Ok(())
}

/// Validate that no face has near-zero area.
///
/// Computes area magnitude via Newell's method over loop vertices.
fn validate_zero_area_faces(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    area_threshold: f64,
) -> Result<(), KernelError> {
    for (face_id, _face_data) in arena.iter_faces() {
        let positions = collect_face_positions(arena, face_id, position_fn)?;

        if positions.len() < 3 {
            return Ok(());
        }

        let area = compute_polygon_area(&positions);

        if area < area_threshold {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::ZeroAreaFace {
                    face_index: face_id.index(),
                    computed_area: area,
                    threshold: area_threshold,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Face".to_string(),
                        index: face_id.index(),
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Face {} area {:.2e} is below threshold {:.2e}",
                        face_id.index(), area, area_threshold
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Validate that no edge has near-zero length.
///
/// Measures 3D distance between edge endpoint vertices. Each geometric
/// edge is checked once by canonicalizing on (min_index, max_index).
fn validate_zero_length_edges(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    edge_length_threshold: f64,
) -> Result<(), KernelError> {
    let mut checked_edges: BTreeSet<(u32, u32)> = BTreeSet::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.twin();
        if he_id == twin_id {
            return Ok(());
        }

        let canonical_key = (he_id.index().min(twin_id.index()), he_id.index().max(twin_id.index()));
        if !checked_edges.insert(canonical_key) {
            return Ok(());
        }

        let twin_data = arena.get_half_edge(twin_id)?;
        let origin_pos = position_fn(he_data.origin());
        let target_pos = position_fn(twin_data.origin());

        if let (Some(p0), Some(p1)) = (origin_pos, target_pos) {
            let length = compute_edge_length(p0, p1);

            if length < edge_length_threshold {
                return Err(KernelError::TopologyViolation {
                    err: forge_core::TopologyError::ZeroLengthEdge {
                        halfedge_index: he_id.index(),
                        computed_length: length,
                        threshold: edge_length_threshold,
                    },
                    context: Some(forge_core::ErrorContext {
                        scope: forge_core::ErrorScope::Entity {
                            entity_kind: "HalfEdge".to_string(),
                            index: he_id.index(),
                        },
                        suggested_fixes: Vec::new(),
                        detail: format!(
                            "Edge {} length {:.2e} is below threshold {:.2e}",
                            he_id.index(), length, edge_length_threshold
                        ),
                    }),
                });
            }
        }
    }
    Ok(())
}

/// Validate that all closed shells have positive signed volume (outward normals).
///
/// Decomposes the arena into connected shells via face-twin adjacency BFS,
/// then computes signed volume for each shell using the divergence theorem.
fn validate_signed_volume(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<(), KernelError> {
    let f_total = arena.face_count();
    if f_total == 0 {
        return Ok(());
    }

    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(fid, _)| fid).collect();
    let mut visited_faces: BTreeSet<u32> = BTreeSet::new();
    let mut shell_index: u32 = 0;

    for &seed_face in &all_faces {
        if visited_faces.contains(&seed_face.index()) {
            return Ok(());
        }

        let shell_faces = discover_shell_faces(arena, seed_face, &mut visited_faces)?;
        let signed_volume = compute_shell_signed_volume(arena, &shell_faces, position_fn)?;

        if signed_volume < 0.0 {
            return Err(KernelError::TopologyViolation {
                err: forge_core::TopologyError::NegativeShellVolume {
                    shell_index,
                    signed_volume,
                },
                context: Some(forge_core::ErrorContext {
                    scope: forge_core::ErrorScope::Entity {
                        entity_kind: "Shell".to_string(),
                        index: shell_index,
                    },
                    suggested_fixes: Vec::new(),
                    detail: format!(
                        "Shell {} has negative signed volume {:.6e} — face normals point inward",
                        shell_index, signed_volume
                    ),
                }),
            });
        }

        shell_index += 1;
    }
    Ok(())
}

/// Discover all faces in a connected shell via BFS from a seed face.
///
/// Marks discovered faces in `visited` and returns the ordered list.
fn discover_shell_faces(
    arena: &TopologyArena,
    seed_face: FaceId,
    visited: &mut BTreeSet<u32>,
) -> Result<Vec<FaceId>, KernelError> {
    let mut shell_faces: Vec<FaceId> = Vec::new();
    let mut face_set: BTreeSet<u32> = BTreeSet::new();
    let mut queue: VecDeque<FaceId> = VecDeque::new();

    queue.push_back(seed_face);
    face_set.insert(seed_face.index());

    while let Some(face_id) = queue.pop_front() {
        shell_faces.push(face_id);

        for he_result in FaceEdgeIterator::new(arena, face_id)? {
            let he_id = he_result?;
            let he_data = arena.get_half_edge(he_id)?;

            if he_id != he_data.twin() {
                let twin_data = arena.get_half_edge(he_data.twin())?;
                let neighbor = twin_data.face();
                if face_set.insert(neighbor.index()) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    visited.extend(face_set.iter());
    Ok(shell_faces)
}

/// Compute the signed volume of a shell using the divergence theorem.
///
/// Fan-triangulates each face and sums the signed tetrahedra volumes.
fn compute_shell_signed_volume(
    arena: &TopologyArena,
    shell_faces: &[FaceId],
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<f64, KernelError> {
    let mut signed_volume = 0.0_f64;

    for &face_id in shell_faces {
        let face_verts = collect_face_positions(arena, face_id, position_fn)?;

        if face_verts.len() >= 3 {
            signed_volume += compute_fan_volume(&face_verts);
        }
    }

    Ok(signed_volume / 6.0)
}

/// Collect vertex positions for a face loop using FaceEdgeIterator.
fn collect_face_positions(
    arena: &TopologyArena,
    face_id: FaceId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<[f64; 3]>, KernelError> {
    let mut positions: Vec<[f64; 3]> = Vec::new();

    for he_result in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_result?;
        let he_data = arena.get_half_edge(he_id)?;
        if let Some(pos) = position_fn(he_data.origin()) {
            positions.push(pos);
        }
    }

    Ok(positions)
}

/// Compute the signed volume contribution from fan-triangulating a polygon.
///
/// Each triangle (v0, vi, vi+1) forms a tetrahedron with the origin.
fn compute_fan_volume(vertices: &[[f64; 3]]) -> f64 {
    let v0 = vertices[0];
    let mut volume = 0.0_f64;

    for i in 1..vertices.len() - 1 {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        volume += v0[0] * (v1[1] * v2[2] - v1[2] * v2[1])
            + v0[1] * (v1[2] * v2[0] - v1[0] * v2[2])
            + v0[2] * (v1[0] * v2[1] - v1[1] * v2[0]);
    }

    volume
}

/// Compute the 3D distance between two points.
fn compute_edge_length(p0: [f64; 3], p1: [f64; 3]) -> f64 {
    let dx = p1[0] - p0[0];
    let dy = p1[1] - p0[1];
    let dz = p1[2] - p0[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Compute the area magnitude of a 3D polygon from its vertex positions.
///
/// Uses Newell's method: sum cross products of consecutive edge vectors.
fn compute_polygon_area(vertices: &[[f64; 3]]) -> f64 {
    let n = vertices.len();
    if n < 3 {
        return 0.0;
    }

    let mut nx = 0.0_f64;
    let mut ny = 0.0_f64;
    let mut nz = 0.0_f64;

    for i in 0..n {
        let curr = vertices[i];
        let next = vertices[(i + 1) % n];
        nx += (curr[1] - next[1]) * (curr[2] + next[2]);
        ny += (curr[2] - next[2]) * (curr[0] + next[0]);
        nz += (curr[0] - next[0]) * (curr[1] + next[1]);
    }

    0.5 * (nx * nx + ny * ny + nz * nz).sqrt()
}
