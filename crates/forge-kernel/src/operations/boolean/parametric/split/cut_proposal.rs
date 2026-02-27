//! BVH-based cut proposal generation.
//!
//! DOMAIN: Build BVH trees, detect overlapping face pairs, propose which faces
//!   need to be cut by which planes, and supplement any missed cuts.
//! DEPENDENCIES: schema (PlaneTable), GeometryState, forge_geom BVH, gate.
//! INVARIANTS: All functions are read-only over topology — no mutation.

use std::collections::BTreeMap;

use forge_core::KernelError;
use crate::geom_facade::Aabb;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::FaceId;

use crate::geometry_state::GeometryState;
use crate::geom_facade::are_parallel_exact as planes_are_parallel;
use crate::shared_ops::{build_face_bvh_pairs, compute_face_aabbs};

use super::gate::compute_face_chord;
use super::schema::PlaneTable;

/// Build BVH trees for both solids and return overlapping `(target, tool)` face pairs.
pub(super) fn build_bvh_overlap_pairs(
    target_arena: &TopologyArena,
    target_geom: &GeometryState,
    tool_arena: &TopologyArena,
    tool_geom: &GeometryState,
    config: &crate::core::ToleranceConfig,
) -> Result<Vec<(FaceId, FaceId)>, KernelError> {
    let target_aabbs = compute_face_aabbs(target_arena, target_geom, config.get_aabb_inflation())?;
    let tool_aabbs = compute_face_aabbs(tool_arena, tool_geom, config.get_aabb_inflation())?;
    build_face_bvh_pairs(&target_aabbs, &tool_aabbs)
}


/// Transform BVH overlap pairs into per-face cut proposals.
///
/// Non-parallel pairs: each face is cut by the opposing face's plane.
/// Coplanar pairs: boundary planes of the opposing face are propagated.
pub(super) fn propose_cuts(
    bvh_pairs: &[(FaceId, FaceId)],
    target_face_planes: &BTreeMap<FaceId, usize>,
    tool_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
    target_arena: &TopologyArena,
    tool_arena: &TopologyArena,
) -> (BTreeMap<FaceId, Vec<usize>>, BTreeMap<FaceId, Vec<usize>>) {
    let mut target_cuts: BTreeMap<FaceId, Vec<usize>> = BTreeMap::new();
    let mut tool_cuts: BTreeMap<FaceId, Vec<usize>> = BTreeMap::new();

    for &(face_a, face_b) in bvh_pairs {
        let plane_idx_a = target_face_planes.get(&face_a).copied();
        let plane_idx_b = tool_face_planes.get(&face_b).copied();

        if let (Some(pa), Some(pb)) = (plane_idx_a, plane_idx_b) {
            let plane_a = plane_table.get(pa);
            let plane_b = plane_table.get(pb);

            if !planes_are_parallel(plane_a, plane_b) {
                target_cuts.entry(face_a).or_default().push(pb);
                tool_cuts.entry(face_b).or_default().push(pa);
            } else if crate::geom_facade::plane_exact_eq(plane_a, plane_b) {
                propagate_boundary_planes(
                    tool_arena, face_b, pb, tool_face_planes, plane_table, plane_a,
                    &mut target_cuts, face_a,
                );
                propagate_boundary_planes(
                    target_arena, face_a, pa, target_face_planes, plane_table, plane_b,
                    &mut tool_cuts, face_b,
                );
            }
        }
    }

    dedup_cut_lists(&mut target_cuts);
    dedup_cut_lists(&mut tool_cuts);
    (target_cuts, tool_cuts)
}

/// Add any cuts that BVH missed by chord-gating every face against every opposing plane.
///
/// Only supplements faces that already have at least one BVH-proposed cut —
/// this prevents false positives from geometrically irrelevant planes.
pub(super) fn supplement_cuts_exhaustive(
    target_arena: &TopologyArena,
    target_geom: &GeometryState,
    target_face_planes: &BTreeMap<FaceId, usize>,
    tool_arena: &TopologyArena,
    tool_geom: &GeometryState,
    tool_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
    config: &crate::core::ToleranceConfig,
    target_cuts: &mut BTreeMap<FaceId, Vec<usize>>,
    tool_cuts: &mut BTreeMap<FaceId, Vec<usize>>,
) -> Result<usize, KernelError> {
    let tool_plane_indices: Vec<usize> = tool_face_planes.values().copied().collect();
    let target_plane_indices: Vec<usize> = target_face_planes.values().copied().collect();

    let mut added = supplement_one_direction(
        target_arena, target_geom, target_face_planes,
        &tool_plane_indices, plane_table, config, target_cuts,
    )?;
    added += supplement_one_direction(
        tool_arena, tool_geom, tool_face_planes,
        &target_plane_indices, plane_table, config, tool_cuts,
    )?;

    dedup_cut_lists(target_cuts);
    dedup_cut_lists(tool_cuts);
    Ok(added)
}

// ── Private helpers ──────────────────────────────────────────────────────────

/// When two faces are coplanar, propagate their neighbor planes to the opposing face's cut list.
fn propagate_boundary_planes(
    source_arena: &TopologyArena,
    source_face: FaceId,
    source_plane_idx: usize,
    source_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
    dest_plane: &crate::geom_facade::Plane,
    dest_cuts: &mut BTreeMap<FaceId, Vec<usize>>,
    dest_face: FaceId,
) {
    let adjacent_faces =
        match forge_topo::classification::face_adjacent_faces(source_arena, source_face) {
            Ok(faces) => faces,
            Err(_) => return,
        };

    for adjacent_face in adjacent_faces {
        if let Some(&adj_plane_idx) = source_face_planes.get(&adjacent_face) {
            if adj_plane_idx == source_plane_idx {
                continue;
            }
            let adj_plane = plane_table.get(adj_plane_idx);
            if !planes_are_parallel(dest_plane, adj_plane) {
                dest_cuts.entry(dest_face).or_default().push(adj_plane_idx);
            }
        }
    }
}

fn supplement_one_direction(
    face_arena: &TopologyArena,
    face_geom: &GeometryState,
    face_planes: &BTreeMap<FaceId, usize>,
    opposing_planes: &[usize],
    plane_table: &PlaneTable,
    config: &crate::core::ToleranceConfig,
    cuts: &mut BTreeMap<FaceId, Vec<usize>>,
) -> Result<usize, KernelError> {
    let mut new_cuts: Vec<(FaceId, usize)> = Vec::new();
    let faces_with_cuts: Vec<FaceId> = cuts.keys().copied().collect();

    for face_id in &faces_with_cuts {
        let face_plane_idx = match face_planes.get(face_id) {
            Some(&idx) => idx,
            None => continue,
        };
        let face_plane = plane_table.get(face_plane_idx);
        let existing = cuts.get(face_id);

        for &cut_plane_idx in opposing_planes {
            if cut_plane_idx == face_plane_idx {
                continue;
            }
            let already_proposed = existing
                .map(|list| list.contains(&cut_plane_idx))
                .unwrap_or(false);
            if already_proposed {
                continue;
            }
            let cut_plane = plane_table.get(cut_plane_idx);
            if planes_are_parallel(face_plane, cut_plane) {
                continue;
            }
            if compute_face_chord(face_arena, face_geom, *face_id, face_plane, cut_plane, config)?.is_some() {
                new_cuts.push((*face_id, cut_plane_idx));
            }
        }
    }

    let added = new_cuts.len();
    for (face_id, cut_plane_idx) in new_cuts {
        cuts.entry(face_id).or_default().push(cut_plane_idx);
    }
    Ok(added)
}

fn dedup_cut_lists(cuts: &mut BTreeMap<FaceId, Vec<usize>>) {
    for list in cuts.values_mut() {
        list.sort_unstable();
        list.dedup();
    }
}
