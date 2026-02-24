//! Geometric invariant validation requiring vertex position data.
//!
//! DOMAIN: Checks that detect degenerate geometry which pure topology
//! checks would miss — zero-area faces, zero-length edges, inverted shells.
//!
//! These checks require a position-lookup callback and a `ToleranceProvider`
//! from the kernel layer, keeping the topo→kernel dependency boundary clean.
//!
//! DEPENDENCIES: `arena` (entity data), `handles` (typed IDs), `forge-core`
//!               (errors, ToleranceProvider), `queries/traverse`

use std::collections::BTreeSet;

use forge_core::{KernelError, ToleranceProvider};
use crate::arena::TopologyArena;
use crate::handles::{FaceId, VertexId};
use crate::topology::queries::traverse::FaceEdgeIterator;
use super::shell::{discover_shell_faces, compute_shell_signed_volume, collect_face_positions};

/// Validate geometric invariants that require vertex positions.
///
/// Unlike `validate_topology()` (pure structural checks called at commit time),
/// this requires position-lookup and per-entity tolerance from the kernel layer.
/// Checks: zero-area faces, zero-length edges, signed volume consistency.
///
/// The `is_planar` callback allows skipping area/volume checks for non-planar
/// faces (e.g., NURBS patches) where projected polygon area would be misleading.
pub fn validate_geometric_invariants(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(FaceId) -> bool,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    validate_zero_area_faces(arena, position_fn, is_planar, tolerance_provider)?;
    validate_zero_length_edges(arena, position_fn, tolerance_provider)?;
    validate_signed_volume(arena, position_fn)?;
    Ok(())
}

/// Validate that no planar face has area below its per-vertex tolerance.
///
/// Uses the maximum vertex tolerance of the face loop as the area threshold.
/// This means a face formed by large-tolerance-sphere vertices tolerates a
/// larger minimum area than one formed by exact planar vertices.
fn validate_zero_area_faces(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    is_planar: &dyn Fn(FaceId) -> bool,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    for (face_id, _face_data) in arena.iter_faces() {
        if !is_planar(face_id) {
            continue;
        }

        let positions = collect_face_positions(arena, face_id, position_fn)?;

        if positions.len() < 3 {
            continue;
        }

        // Threshold = max vertex tolerance of the face loop, squared then sqrt.
        // For planar vertices this is PLANAR_VERTEX_TOLERANCE² area ≈ 1e-20m².
        let area_threshold = FaceEdgeIterator::new(arena, face_id)
            .map_err(|e| e)?
            .filter_map(|r| r.ok())
            .filter_map(|he_id| arena.get_half_edge(he_id).ok())
            .map(|he| tolerance_provider.vertex_tolerance(he.origin().index(), he.origin().generation()))
            .fold(0.0_f64, f64::max)
            .powi(2);

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
                        "Face {} area {:.2e} is below per-entity threshold {:.2e}",
                        face_id.index(), area, area_threshold
                    ),
                }),
            });
        }
    }
    Ok(())
}

/// Validate that no edge has length below its endpoint vertex tolerances.
///
/// Uses `max(origin_tolerance, target_tolerance)` as the degenerate threshold
/// for each edge. For exact planar vertices this is PLANAR_VERTEX_TOLERANCE ≈1e-10.
fn validate_zero_length_edges(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    let mut checked_edges: BTreeSet<u32> = BTreeSet::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        let edge_id = he_data.edge();
        if !checked_edges.insert(edge_id.index()) {
            continue;
        }

        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let target = next_data.origin();

        if origin == target {
            continue;
        }

        let origin_pos = position_fn(origin);
        let target_pos = position_fn(target);

        if let (Some(p0), Some(p1)) = (origin_pos, target_pos) {
            let length = compute_edge_length(p0, p1);
            let edge_length_threshold = tolerance_provider
                .vertex_tolerance(origin.index(), origin.generation())
                .max(tolerance_provider.vertex_tolerance(target.index(), target.generation()));

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
                            "Edge {} length {:.2e} is below per-entity threshold {:.2e}",
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
            continue;
        }

        let shell_faces = discover_shell_faces(arena, seed_face, &mut visited_faces)?;

        let shell_id = arena.get_face(seed_face)?.shell();
        if !matches!(arena.get_shell(shell_id)?.kind(), crate::arena::ShellKind::Solid(_)) {
            shell_index += 1;
            continue;
        }

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

