//! PlaneTable construction and per-vertex provenance assignment.
//!
//! DOMAIN: Build the global PlaneTable from both solids and assign implicit
//!   provenance keys to all original vertices before splitting begins.
//! DEPENDENCIES: schema (PlaneTable, LocalVertexDedup), GeometryState, forge_topo.

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::state::TopologyState;

use crate::geometry_state::GeometryState;
use crate::shared_ops::vertex::identity::VertexMatchKey;

use super::schema::{LocalVertexDedup, PlaneTable};

/// Build the global `PlaneTable` and per-face plane-index maps for both solids.
pub(super) fn build_plane_tables(
    target_topo: &TopologyState,
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
) -> (PlaneTable, BTreeMap<FaceId, usize>, BTreeMap<FaceId, usize>) {
    let mut plane_table = PlaneTable::new();
    let mut target_face_planes = BTreeMap::new();
    let mut tool_face_planes = BTreeMap::new();

    for (fid, _) in target_topo.arena().iter_faces() {
        if let Some(p) = target_geom.get_face_plane(fid) {
            target_face_planes.insert(fid, plane_table.intern(p));
        }
    }
    for (fid, _) in tool_topo.arena().iter_faces() {
        if let Some(p) = tool_geom.get_face_plane(fid) {
            tool_face_planes.insert(fid, plane_table.intern(p));
        }
    }

    (plane_table, target_face_planes, tool_face_planes)
}

/// Assign implicit provenance keys to every original vertex in a solid.
///
/// Derives `VertexMatchKey` from the exact 3-plane intersection of incident face planes,
/// not from stored coordinates, so the same physical corner always produces an identical
/// key regardless of which solid computed it.
pub(super) fn assign_original_vertex_provenance(
    arena: &TopologyArena,
    dedup: &mut LocalVertexDedup,
    geom: &GeometryState,
    face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &PlaneTable,
) -> Result<(), KernelError> {
    let mut implicit_count = 0;
    let mut fallback_count = 0;
    let mut symbolic_count = 0;

    for (vid, _vdata) in arena.iter_vertices() {
        let key = if geom.get_vertex_symbolic_planes(vid).is_some() {
            if let Some(exact) = geom.get_vertex_position_exact(vid) {
                symbolic_count += 1;
                implicit_count += 1;
                Some(VertexMatchKey::from_exact_position(
                    exact[0].clone(),
                    exact[1].clone(),
                    exact[2].clone(),
                ))
            } else {
                fallback_count += 1;
                compute_explicit_key(geom, vid)
            }
        } else {
            let incident = collect_incident_plane_indices(arena, vid, face_planes);
            if incident.len() >= 3 {
                implicit_count += 1;
                compute_implicit_key(&incident, plane_table, geom, vid)
            } else {
                fallback_count += 1;
                compute_explicit_key(geom, vid)
            }
        };

        if let Some(k) = key {
            dedup.insert(vid, k);
        }
    }

    eprintln!(
        "[provenance] {} vertices implicit ({} from symbolic planes), {} fallback",
        implicit_count, symbolic_count, fallback_count
    );
    Ok(())
}

// ── Private helpers ──────────────────────────────────────────────────────────

/// Collect distinct plane indices for all faces incident to a vertex.
///
/// Delegates the topology traversal to `forge_topo::classification::vertex_faces`.
fn collect_incident_plane_indices(
    arena: &TopologyArena,
    vid: VertexId,
    face_planes: &std::collections::BTreeMap<FaceId, usize>,
) -> Vec<usize> {
    let faces = match forge_topo::classification::vertex_faces(arena, vid) {
        Ok(fs) => fs,
        Err(_) => return Vec::new(),
    };
    let mut plane_indices = Vec::new();
    for face in faces {
        if let Some(&pi) = face_planes.get(&face) {
            if !plane_indices.contains(&pi) {
                plane_indices.push(pi);
            }
        }
    }
    plane_indices
}

/// Derive a `VertexMatchKey` from the exact 3-plane intersection of incident faces.
fn compute_implicit_key(
    incident: &[usize],
    plane_table: &PlaneTable,
    geom: &GeometryState,
    vid: VertexId,
) -> Option<VertexMatchKey> {
    let p0 = plane_table.get(incident[0]);
    let p1 = plane_table.get(incident[1]);
    let p2 = plane_table.get(incident[2]);

    match crate::geom_facade::intersect_three_planes_exact(p0, p1, p2) {
        Ok(exact_pos) => Some(VertexMatchKey::from_exact_position(
            exact_pos[0].clone(),
            exact_pos[1].clone(),
            exact_pos[2].clone(),
        )),
        Err(_) => compute_explicit_key(geom, vid),
    }
}

/// Fallback: derive a `VertexMatchKey` from the stored exact coordinates.
fn compute_explicit_key(geom: &GeometryState, vid: VertexId) -> Option<VertexMatchKey> {
    geom.get_vertex_position_exact(vid).map(|exact| {
        VertexMatchKey::from_exact_position(
            exact[0].clone(),
            exact[1].clone(),
            exact[2].clone(),
        )
    })
}
