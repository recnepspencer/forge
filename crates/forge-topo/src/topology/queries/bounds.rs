//! Hierarchical topology bounds queries.
//!
//! DOMAIN: Read-only AABB aggregation over face, shell, region, lump, and body topology.
//!
//! INVARIANTS:
//! - No topology mutation
//! - Deterministic traversal order
//! - Returns `Ok(None)` when no vertices/points are available

use forge_core::KernelError;
use forge_geom::Aabb;

use crate::arena::TopologyArena;
use crate::handles::{BodyId, FaceId, LumpId, RegionId, ShellId, VertexId};

use super::hierarchy::shell_faces;
use super::traverse::FaceAllEdgesIterator;

/// Compute an AABB for a face by traversing all loops and collecting vertex positions.
pub fn face_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    face: FaceId,
) -> Result<Option<Aabb>, KernelError> {
    arena.get_face(face)?;

    let mut result: Option<Aabb> = None;

    for he_res in FaceAllEdgesIterator::new(arena, face)? {
        let he_id = he_res?;
        let vertex_id = arena.get_half_edge(he_id)?.origin();
        let Some(point) = position_fn(vertex_id) else {
            continue;
        };

        let point_box = Aabb::new(point, point);
        result = match result {
            Some(bounds) => Some(bounds.union(&point_box)),
            None => Some(point_box),
        };
    }

    Ok(result)
}

/// Compute AABBs for all faces in deterministic arena order.
pub fn all_face_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
) -> Result<Vec<(FaceId, Aabb)>, KernelError> {
    let mut list = Vec::new();
    for (face_id, _) in arena.iter_faces() {
        if let Some(bounds) = face_bounds(arena, position_fn, face_id)? {
            list.push((face_id, bounds));
        }
    }
    Ok(list)
}

/// Compute an AABB for a shell by unioning bounds of all member faces.
pub fn shell_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    shell: ShellId,
) -> Result<Option<Aabb>, KernelError> {
    let mut result: Option<Aabb> = None;

    for face_id in shell_faces(arena, shell)? {
        let Some(face_box) = face_bounds(arena, position_fn, face_id)? else {
            continue;
        };

        result = match result {
            Some(bounds) => Some(bounds.union(&face_box)),
            None => Some(face_box),
        };
    }

    Ok(result)
}

/// Compute an AABB for a region by unioning its outer and inner shell bounds.
pub fn region_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    region: RegionId,
) -> Result<Option<Aabb>, KernelError> {
    let region_data = arena.get_region(region)?;
    let mut result: Option<Aabb> = None;

    if let Some(outer_shell) = region_data.outer_shell() {
        if let Some(shell_box) = shell_bounds(arena, position_fn, outer_shell)? {
            result = Some(shell_box);
        }
    }

    for &inner_shell in region_data.inner_shells() {
        let Some(shell_box) = shell_bounds(arena, position_fn, inner_shell)? else {
            continue;
        };
        result = match result {
            Some(bounds) => Some(bounds.union(&shell_box)),
            None => Some(shell_box),
        };
    }

    Ok(result)
}

/// Compute an AABB for a lump by unioning bounds of all regions.
pub fn lump_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    lump: LumpId,
) -> Result<Option<Aabb>, KernelError> {
    let lump_data = arena.get_lump(lump)?;
    let mut result: Option<Aabb> = None;

    for &region_id in lump_data.regions() {
        let Some(region_box) = region_bounds(arena, position_fn, region_id)? else {
            continue;
        };
        result = match result {
            Some(bounds) => Some(bounds.union(&region_box)),
            None => Some(region_box),
        };
    }

    Ok(result)
}

/// Compute an AABB for a solid body by traversing body -> lump -> region -> shell.
pub fn solid_bounds(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    body: BodyId,
) -> Result<Option<Aabb>, KernelError> {
    let body_data = arena.get_body(body)?;
    let mut result: Option<Aabb> = None;

    for &lump_id in body_data.lumps() {
        let Some(lump_box) = lump_bounds(arena, position_fn, lump_id)? else {
            continue;
        };
        result = match result {
            Some(bounds) => Some(bounds.union(&lump_box)),
            None => Some(lump_box),
        };
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::traverse::FaceEdgeIterator;
    use std::collections::BTreeMap;

    #[test]
    fn bounds_queries_accumulate_face_shell_and_solid() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.25 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();
        let _se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.75 }).unwrap().into_value();

        let ordered_edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap())
            .collect();

        let mut positions = BTreeMap::new();
        let v0 = draft.arena().get_half_edge(ordered_edges[0]).unwrap().origin();
        let v1 = draft.arena().get_half_edge(ordered_edges[1]).unwrap().origin();
        let v2 = draft.arena().get_half_edge(ordered_edges[2]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(ordered_edges[3]).unwrap().origin();
        positions.insert(v0.index(), [0.0, 0.0, 0.0]);
        positions.insert(v1.index(), [1.0, 0.0, 0.0]);
        positions.insert(v2.index(), [1.0, 2.0, 0.0]);
        positions.insert(v3.index(), [-1.0, 1.0, 0.0]);

        let state = draft.commit().unwrap();
        let position_fn = |vertex: VertexId| positions.get(&vertex.index()).copied();

        let face_box = face_bounds(state.arena(), &position_fn, mvf.face).unwrap().unwrap();
        let all_face_boxes = all_face_bounds(state.arena(), &position_fn).unwrap();
        let shell_box = shell_bounds(state.arena(), &position_fn, mvf.shell).unwrap().unwrap();
        let region_box = region_bounds(state.arena(), &position_fn, mvf.region).unwrap().unwrap();
        let lump_box = lump_bounds(state.arena(), &position_fn, mvf.lump).unwrap().unwrap();
        let body_box = solid_bounds(state.arena(), &position_fn, mvf.solid).unwrap().unwrap();

        assert_eq!(face_box.min, [-1.0, 0.0, 0.0]);
        assert_eq!(face_box.max, [1.0, 2.0, 0.0]);
        assert_eq!(all_face_boxes.len(), 1);
        assert_eq!(all_face_boxes[0].0, mvf.face);
        assert_eq!(all_face_boxes[0].1, face_box);
        assert_eq!(shell_box, face_box);
        assert_eq!(region_box, face_box);
        assert_eq!(lump_box, face_box);
        assert_eq!(body_box, face_box);
    }
}
