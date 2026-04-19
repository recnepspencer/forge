//! ConvexCell → halfedge topology construction.
//!
//! DOMAIN: Builds face loops, halfedge chains, and stitches twin pointers
//! from ConvexCell face/vertex data. This is the topology construction
//! machinery that bridges BSP output to halfedge b-rep.
//!
//! CONSUMED BY: primitives (make_cube, etc.), booleans (CSG meshing).
//!
//! INVARIANTS:
//!   - Every directed edge (a→b) has a twin (b→a) in a closed mesh.
//!   - Halfedge next/prev chains form closed loops per face.
//!   - Vertex outgoing pointers are set for all vertices in face loops.

use std::sync::Arc;

use forge_core::tolerance::ToleranceProvider;
use forge_core::KernelError;
use forge_topo::b_rep::{EdgeData, FaceData, HalfEdgeData, LoopData, TopologyArena};
use forge_topo::handles::{EdgeId, HalfEdgeId, LoopId, ShellId, VertexId};
use forge_topo::provenance::LineageRecorder;
use forge_topo::queries::edge_endpoint_ids;
use forge_topo::transactions::MutableDraft;
use worth_geom::facade::{CurveGeom, CurveKind, SurfaceData};
use worth_geom::ConvexCell;

use crate::geometry::facade::GeometryStore;

use std::collections::BTreeMap;

// ── EdgeMap ──────────────────────────────────────────────────────────────

/// Deterministic map mapping (vertex_a, vertex_b) → HalfEdgeId.
pub(crate) struct EdgeMap {
    data: BTreeMap<(VertexId, VertexId), HalfEdgeId>,
}

impl EdgeMap {
    fn new(_vertex_count: usize) -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn insert(&mut self, a: VertexId, b: VertexId, he: HalfEdgeId) {
        self.data.insert((a, b), he);
    }

    fn get(&self, a: VertexId, b: VertexId) -> Option<HalfEdgeId> {
        self.data.get(&(a, b)).copied()
    }

    fn iter_ascending(&self) -> impl Iterator<Item = (VertexId, VertexId, HalfEdgeId)> + '_ {
        self.data.iter().map(|(&(a, b), &he)| (a, b, he))
    }
}

// ── Face + loop construction ─────────────────────────────────────────────

pub(crate) fn insert_faces_and_loops(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    cell: &ConvexCell,
    vertex_ids: &[VertexId],
    shell: ShellId,
    recorder: &mut LineageRecorder,
) -> Result<EdgeMap, KernelError> {
    let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
    let placeholder_loop = LoopId::new(u32::MAX, 0);
    let placeholder_edge = EdgeId::new(u32::MAX, 0);
    let cell_planes = cell.planes();

    let vertex_count = vertex_ids.len();
    let mut edge_map = EdgeMap::new(vertex_count);

    for cell_face in cell.faces() {
        let face_verts = cell_face.vertices();

        // Map to VertexIds and deduplicate adjacent identical vertices.
        let mut clean_vids = Vec::with_capacity(face_verts.len());
        for &cv_idx in face_verts {
            let vid = vertex_ids[cv_idx];
            if clean_vids.is_empty() || *clean_vids.last().unwrap() != vid {
                clean_vids.push(vid);
            }
        }
        if clean_vids.len() > 1 && clean_vids.first() == clean_vids.last() {
            clean_vids.pop();
        }

        let vert_count = clean_vids.len();
        if vert_count < 3 {
            continue;
        }

        let face_id = draft.insert_face(FaceData::new(placeholder_loop, shell));
        recorder.stamp(draft.lineage_store_mut(), face_id);

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face_id));
        recorder.stamp(draft.lineage_store_mut(), loop_id);

        let plane_idx = cell_face.plane_idx();
        if plane_idx < cell_planes.len() {
            let plane = &cell_planes[plane_idx];
            geometry.planes.set(face_id, plane.clone());

            // Derive SurfaceData from the Plane's normalized f64 cache.
            // SurfaceData is a derived projection of Plane, not a peer.
            geometry.surfaces.set(
                face_id,
                Arc::new(SurfaceData::plane(plane.normal(), plane.offset())),
            );
        }

        let mut he_ids = Vec::with_capacity(vert_count);

        for &origin in &clean_vids {
            let he_id = draft.insert_half_edge(HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                face_id,
                origin,
                placeholder_edge,
            ));
            recorder.stamp(draft.lineage_store_mut(), he_id);
            he_ids.push(he_id);
        }

        for i in 0..vert_count {
            let next_i = (i + 1) % vert_count;
            let prev_i = if i == 0 { vert_count - 1 } else { i - 1 };

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(he_ids[i])?.set_next(he_ids[next_i]);
            arena.get_half_edge_mut(he_ids[i])?.set_prev(he_ids[prev_i]);

            edge_map.insert(clean_vids[i], clean_vids[next_i], he_ids[i]);
        }

        draft
            .arena_mut()
            .get_face_mut(face_id)?
            .loops
            .set_outer(loop_id);
        draft
            .arena_mut()
            .get_loop_mut(loop_id)?
            .set_half_edge(he_ids[0]);

        for &he_id in &he_ids {
            let origin = draft.arena().get_half_edge(he_id)?.origin();
            draft
                .arena_mut()
                .get_vertex_mut(origin)?
                .set_primary_disk(he_id);
        }
    }

    Ok(edge_map)
}

// ── Twin stitching ───────────────────────────────────────────────────────

/// Stitch twin pointers between halfedges on adjacent faces.
///
/// For each directed edge (a→b), find the matching (b→a) and set twins.
/// Iterates in deterministic ascending order by vertex-pair key.
pub(crate) fn stitch_twins(
    draft: &mut MutableDraft,
    edge_map: &EdgeMap,
    recorder: &mut LineageRecorder,
) -> Result<Vec<EdgeId>, KernelError> {
    let mut edges_created = Vec::new();
    for (a, b, he_id) in edge_map.iter_ascending() {
        if a < b {
            if let Some(twin_id) = edge_map.get(b, a) {
                let edge = draft.insert_edge(EdgeData::new(he_id));
                recorder.stamp(draft.lineage_store_mut(), edge);
                draft
                    .arena_mut()
                    .get_half_edge_mut(he_id)?
                    .set_radial_next(twin_id);
                draft
                    .arena_mut()
                    .get_half_edge_mut(twin_id)?
                    .set_radial_next(he_id);
                draft.arena_mut().get_half_edge_mut(he_id)?.set_edge(edge);
                draft.arena_mut().get_half_edge_mut(twin_id)?.set_edge(edge);
                edges_created.push(edge);
            } else {
                return Err(KernelError::InternalError {
                    message: format!(
                        "No twin found for directed edge ({} -> {}); mesh is not closed",
                        a, b
                    ),
                    context: None,
                });
            }
        }
    }
    Ok(edges_created)
}

// ── Geometry emission (decoupled from topology) ─────────────────────────

/// Emit `CurveGeom` for each edge based on vertex positions.
///
/// This is a **post-pass** that runs after topology construction is complete.
/// It reads positions from the geometry store and emits `CurveKind::Line`
/// curves for edges whose length exceeds the tolerance threshold.
///
/// Decoupled from `stitch_twins` to keep topology wiring geometry-blind.
/// All vector math is delegated to `CurveGeom::line_from_endpoints`
/// (in `worth-geom`), keeping this function a pure orchestrator.
pub(crate) fn emit_edge_curves(
    arena: &TopologyArena,
    geometry: &mut GeometryStore,
    edges: &[EdgeId],
    tol: &dyn ToleranceProvider,
) -> Result<(), KernelError> {
    let threshold = tol.global_default();

    for &edge_id in edges {
        let edge = arena.get_edge(edge_id)?;
        let he_id = edge.half_edge();

        let (v_origin, v_dest) = edge_endpoint_ids(arena, he_id)?;

        let p_origin =
            geometry
                .positions
                .get(v_origin)
                .ok_or_else(|| KernelError::InternalError {
                    message: format!("Vertex {} has no position for curve emission", v_origin),
                    context: None,
                })?;
        let p_dest = geometry
            .positions
            .get(v_dest)
            .ok_or_else(|| KernelError::InternalError {
                message: format!("Vertex {} has no position for curve emission", v_dest),
                context: None,
            })?;

        if let Some(curve) =
            CurveGeom::line_from_endpoints(*p_origin.approx(), *p_dest.approx(), threshold)
        {
            geometry.curves.set(edge_id, Arc::new(curve));
        }
    }

    Ok(())
}
