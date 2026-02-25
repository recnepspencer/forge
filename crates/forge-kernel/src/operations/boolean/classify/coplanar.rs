//! Coplanar face detection and overlap testing.
//!
//! DOMAIN: Detect coplanar face pairs between two solids for
//! regularized Boolean operations (internal boundary elimination).

use std::collections::{BTreeMap, BTreeSet};

use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;
use forge_topo::state::TopologyState;

use crate::geometry_state::GeometryState;
use crate::operations::boolean::schema::FaceClassification;

/// Detect coplanar faces between source and other solids using exact rational arithmetic.
///
/// Two planes are coplanar iff their normals are parallel (cross product is zero)
/// AND they have the same offset (scale-invariant check: a1*d2 == a2*d1 for any
/// non-zero normal component). All checks use exact `Rational` arithmetic —
/// no floating-point noise, no tolerance thresholds.
///
/// Returns a map from source face index → pre-determined classification.
fn detect_coplanar_faces(
    source_arena: &TopologyArena,
    source_geom: &GeometryState,
    other_arena: &TopologyArena,
    other_geom: &GeometryState,
) -> BTreeMap<u32, FaceClassification> {
    let mut result = BTreeMap::new();

    let other_planes: Vec<(FaceId, _)> = other_arena.iter_faces()
        .filter_map(|(fid, _)| {
            other_geom.get_face_plane(fid).map(|p| (fid, p.clone()))
        })
        .collect();

    for (src_fid, _) in source_arena.iter_faces() {
        if let Some(src_plane) = source_geom.get_face_plane(src_fid) {
            let coplanar_match = other_planes.iter().find(|(_, other_plane)| {
                planes_are_coplanar_exact(src_plane, other_plane)
            });

            if let Some((_, other_plane)) = coplanar_match {
                let aligned = normals_aligned_exact(src_plane, other_plane);
                let class = if aligned {
                    FaceClassification::OnBoundary
                } else {
                    FaceClassification::OppositeBoundary
                };
                result.insert(src_fid.index(), class);
            }
        }
    }

    result
}

/// Check if two planes are coplanar using exact rational arithmetic.
///
/// Delegates to `forge_geom::coplanar_eq` which checks all three normal
/// components against offset for correctness.
fn planes_are_coplanar_exact(p1: &forge_geom::Plane, p2: &forge_geom::Plane) -> bool {
    forge_geom::primitives::plane::coplanar_eq(p1, p2)
}

/// Check if two coplanar planes have aligned normals (same direction).
///
/// For parallel normals, dot(n1, n2) > 0 means same direction.
/// Uses exact rational arithmetic: sign of (a1*a2 + b1*b2 + c1*c2).
fn normals_aligned_exact(p1: &forge_geom::Plane, p2: &forge_geom::Plane) -> bool {
    let (a1, b1, c1, _) = p1.exact_coefficients();
    let (a2, b2, c2, _) = p2.exact_coefficients();

    let dot = &(&(a1 * a2) + &(b1 * b2)) + &(c1 * c2);
    dot.sign() == forge_math::sign::TriSign::Pos
}

/// Find coplanar face pairs between two solids for regularized Boolean union.
///
/// Returns `(excluded_target_indices, excluded_tool_indices)` — the face
/// indices that should be dropped because they form an internal boundary.
///
/// Two faces are paired when:
///   1. They lie on the exact same geometric plane (`coplanar_eq`)
///   2. Their 2D polygon projections overlap in area (`polygons_overlap_2d`)
pub(crate) fn find_coplanar_face_pairs(
    target_topo: &TopologyState,
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
) -> (BTreeSet<u32>, BTreeSet<u32>) {
    let mut excluded_target: BTreeSet<u32> = BTreeSet::new();
    let mut excluded_tool: BTreeSet<u32> = BTreeSet::new();

    let tool_faces_data: Vec<(FaceId, forge_geom::Plane, Vec<[f64; 3]>)> =
        tool_topo.arena().iter_faces()
        .filter_map(|(fid, _)| {
            let plane = tool_geom.get_face_plane(fid)?.clone();
            let verts = extract_face_vertices_3d(tool_topo.arena(), tool_geom, fid)?;
            Some((fid, plane, verts))
        })
        .collect();

    for (target_fid, _) in target_topo.arena().iter_faces() {
        if let Some(target_plane) = target_geom.get_face_plane(target_fid) {
            if let Some(target_verts) = extract_face_vertices_3d(
                target_topo.arena(), target_geom, target_fid
            ) {
                let matched = tool_faces_data.iter().find(|(tool_fid, tool_plane, tool_verts)| {
                    let not_excluded = !excluded_tool.contains(&tool_fid.index());
                    let is_coplanar = forge_geom::primitives::plane::coplanar_eq(target_plane, tool_plane);
                    let overlaps = if not_excluded && is_coplanar {
                        forge_geom::algorithms::polygons_overlap_3d(
                            target_plane.raw_normal(),
                            &target_verts,
                            tool_verts,
                        )
                    } else {
                        false
                    };
                    not_excluded && is_coplanar && overlaps
                });

                if let Some((tool_fid, _, _)) = matched {
                    excluded_target.insert(target_fid.index());
                    excluded_tool.insert(tool_fid.index());
                }
            }
        }
    }

    (excluded_target, excluded_tool)
}

/// Extract ordered vertex positions of a single-loop face.
///
/// Faces with inner loops are skipped until overlap logic is upgraded to
/// multi-ring polygon handling.
fn extract_face_vertices_3d(
    arena: &TopologyArena,
    geom: &GeometryState,
    face: FaceId,
) -> Option<Vec<[f64; 3]>> {
    let loops = forge_topo::polygon::face_loop_vertices(arena, face).ok()?;
    if loops.len() != 1 {
        return None;
    }
    let mut verts = Vec::new();
    for v in &loops[0] {
        let pos = geom.get_vertex_position(*v)?;
        verts.push(*pos);
    }
    if verts.len() < 3 {
        return None;
    }
    Some(verts)
}
