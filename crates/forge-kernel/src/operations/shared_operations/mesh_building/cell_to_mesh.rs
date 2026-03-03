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

use forge_core::KernelError;
use forge_geom::ConvexCell;
use forge_topo::b_rep::{EdgeData, FaceData, HalfEdgeData, LoopData};
use forge_topo::handles::{EdgeId, HalfEdgeId, LoopId, ShellId, VertexId};
use forge_topo::provenance::OpSignature;
use forge_topo::transactions::MutableDraft;

use crate::geometry::facade::GeometryStore;

// ── EdgeMap ──────────────────────────────────────────────────────────────

/// Dense bitmap mapping (vertex_a, vertex_b) → HalfEdgeId.
///
/// Flat Vec of size vertex_count², indexed by `a * n + b`.
/// O(1) insert/lookup, zero hash overhead, deterministic iteration order.
pub(crate) struct EdgeMap {
    data: Vec<Option<HalfEdgeId>>,
    vertex_count: usize,
}

impl EdgeMap {
    /// Create a new edge map for the given vertex count.
    fn new(vertex_count: usize) -> Self {
        Self {
            data: vec![None; vertex_count * vertex_count],
            vertex_count,
        }
    }

    /// Insert a directed edge.
    fn insert(&mut self, a: usize, b: usize, he: HalfEdgeId) {
        self.data[a * self.vertex_count + b] = Some(he);
    }

    /// Look up a directed edge.
    fn get(&self, a: usize, b: usize) -> Option<HalfEdgeId> {
        self.data[a * self.vertex_count + b]
    }

    /// Iterate all entries in deterministic ascending order by (a, b).
    fn iter_ascending(&self) -> impl Iterator<Item = (usize, usize, HalfEdgeId)> + '_ {
        self.data.iter().enumerate().filter_map(move |(idx, opt)| {
            opt.map(|he| {
                let a = idx / self.vertex_count;
                let b = idx % self.vertex_count;
                (a, b, he)
            })
        })
    }
}

// ── Face + loop construction ─────────────────────────────────────────────

/// Create faces, loops, and halfedge chains for each ConvexCell face.
///
/// Each face's vertex list forms a closed loop of halfedges.
/// Returns a directed-edge map: (cell_vert_a, cell_vert_b) → HalfEdgeId,
/// used by [`stitch_twins`] to pair up twin halfedges.
pub(crate) fn insert_faces_and_loops(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    cell: &ConvexCell,
    vertex_ids: &[VertexId],
    shell: ShellId,
    _sig: &OpSignature,
    ordinal: &mut u64,
) -> Result<EdgeMap, KernelError> {
    let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
    let placeholder_loop = LoopId::new(u32::MAX, 0);
    let placeholder_edge = EdgeId::new(u32::MAX, 0);
    let cell_planes = cell.planes();

    let vertex_count = vertex_ids.len();
    let mut edge_map = EdgeMap::new(vertex_count);

    for cell_face in cell.faces() {
        let face_verts = cell_face.vertices();
        if face_verts.len() < 3 {
            continue;
        }

        let face_id = draft.insert_face(FaceData::new(
            placeholder_loop, shell,
        ));
        *ordinal += 1;

        let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face_id));

        let plane_idx = cell_face.plane_idx();
        if plane_idx < cell_planes.len() {
            geometry.planes.set(face_id, cell_planes[plane_idx].clone());
        }

        let vert_count = face_verts.len();
        let mut he_ids = Vec::with_capacity(vert_count);

        for &cell_vert_idx in face_verts {
            let origin = vertex_ids[cell_vert_idx];
            let he_id = draft.insert_half_edge(HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face_id, origin, placeholder_edge));
            *ordinal += 1;
            he_ids.push(he_id);
        }

        for i in 0..vert_count {
            let next_i = (i + 1) % vert_count;
            let prev_i = if i == 0 { vert_count - 1 } else { i - 1 };

            let arena = draft.arena_mut();
            arena.get_half_edge_mut(he_ids[i])?.set_next(he_ids[next_i]);
            arena.get_half_edge_mut(he_ids[i])?.set_prev(he_ids[prev_i]);

            edge_map.insert(face_verts[i], face_verts[next_i], he_ids[i]);
        }

        draft
            .arena_mut()
            .get_face_mut(face_id)?
            .set_outer_loop(loop_id);
        draft
            .arena_mut()
            .get_loop_mut(loop_id)?
            .set_half_edge(he_ids[0]);

        for &he_id in &he_ids {
            let origin = draft.arena().get_half_edge(he_id)?.origin();
            draft
                .arena_mut()
                .get_vertex_mut(origin)?
                .set_outgoing(he_id);
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
    _sig: &OpSignature,
    ordinal: &mut u64,
) -> Result<(), KernelError> {
    for (a, b, he_id) in edge_map.iter_ascending() {
        if a < b {
            if let Some(twin_id) = edge_map.get(b, a) {
                let edge = draft.insert_edge(EdgeData::new(
                    he_id,
                ));
                *ordinal += 1;
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
    Ok(())
}
