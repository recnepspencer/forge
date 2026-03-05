//! SplitEdge — split an edge by inserting a vertex at a parameter.
//!
//! DOMAIN: Takes an existing halfedge (A→B) and inserts a midpoint vertex M,
//! producing two halfedges (A→M) and (M→B) and their twins.
//!
//! INVARIANTS:
//! - The new vertex M is on the edge at parameter `t`
//! - All twin, next, prev pointers are correctly wired
//! - Euler formula: V+1, E+1 (net: same V-E+F)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::b_rep::{EdgeData, HalfEdgeData, VertexData};
use crate::handles::{EdgeId, HalfEdgeId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::invariant_id::InvariantContract;


/// Split an existing edge by inserting a midpoint vertex.
///
/// Given halfedge `edge` (A→B), creates vertex M and splits into:
/// - `edge` becomes A→M
/// - `new_edge` becomes M→B  
/// - Plus their twins: `edge.twin` becomes M→A, `new_edge_twin` becomes B→M
///
/// Handles the degenerate case where A→B is a self-loop (twin == self).
#[derive(Debug)]
pub struct SplitEdge {
    /// The halfedge to split.
    pub edge: HalfEdgeId,
}

/// Output of the SplitEdge operator.
///
/// # Degenerate Case (Self-Loop Split)
///
/// When splitting a self-loop halfedge (one where `twin == self`),
/// the fields `he_bm` and `he_ma` will alias `he_mb`. This is
/// correct — the degenerate case has fewer distinct entities.
/// Use `is_degenerate()` to detect this case.
pub struct SplitEdgeOutput {
    /// The original halfedge, now A→M.
    pub he_am: HalfEdgeId,
    /// The new halfedge M→B (or M→A for self-loop).
    pub he_mb: HalfEdgeId,
    /// The new twin halfedge B→M (degenerate: same as he_mb for self-loop).
    pub he_bm: HalfEdgeId,
    /// The original twin, now M→A (degenerate: same as he_mb for self-loop).
    pub he_ma: HalfEdgeId,
    /// The newly created midpoint vertex.
    pub new_vertex: crate::handles::VertexId,
}

impl SplitEdgeOutput {
    /// Whether this split was on a self-loop (degenerate case).
    ///
    /// When `true`, `he_bm == he_mb` and `he_ma == he_mb`.
    pub fn is_degenerate(&self) -> bool {
        self.he_bm == self.he_mb
    }
}

impl TopoOperator for SplitEdge {
    type Output = SplitEdgeOutput;

    const NAME: &'static str = "split_edge";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        format!("Split edge at halfedge {}", self.edge.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let he_ab = self.edge;
        let (vertex_a, vertex_b) = {
            let ab_data = draft.arena().get_half_edge(he_ab)?;
            let ab_next = draft.arena().get_half_edge(ab_data.next())?;
            (
                ab_data.origin(),
                ab_next.origin(),
            )
        };

        let chain: Vec<HalfEdgeId> =
            crate::queries::traverse::RadialEdgeIterator::new(draft.arena(), he_ab)?
                .collect::<Result<_, _>>()?;
        let old_edge = draft.arena().get_half_edge(he_ab)?.edge();
        let is_closed_edge = vertex_a == vertex_b;
        let new_vertex = draft.insert_vertex(VertexData::new(
            HalfEdgeId::DANGLING, // sentinel
        ));

        let new_edge = draft.insert_edge(EdgeData::new(
            HalfEdgeId::DANGLING,
        ));

        let mut e_old_list = Vec::new();
        let mut e_new_list = Vec::new();
        let mut new_ids = std::collections::HashMap::new();

        for (radial_index, &h) in chain.iter().enumerate() {
            let (h_face, h_orig, h_next) = {
                let h_data = draft.arena().get_half_edge(h)?;
                (
                    h_data.face(),
                    h_data.origin(),
                    h_data.next(),
                )
            };

            let is_forward = h_orig == vertex_a;
            if !is_closed_edge {
                let expected_origin = if is_forward { vertex_a } else { vertex_b };
                if h_orig != expected_origin {
                    return Err(KernelError::InternalError {
                        message: format!(
                            "Radial edge origin {} does not match expected endpoint {} at radial index {}",
                            h_orig.index(),
                            expected_origin.index(),
                            radial_index
                        ),
                        context: None,
                    });
                }
            }
            if is_closed_edge && h_orig != vertex_a {
                return Err(KernelError::InternalError {
                    message: format!(
                        "Closed-edge radial halfedge origin {} does not match closed endpoint {}",
                        h_orig.index(),
                        vertex_a.index()
                    ),
                    context: None,
                });
            }
            let h_new = draft.insert_half_edge(HalfEdgeData::new(
                HalfEdgeId::DANGLING, // radial_next
                HalfEdgeId::DANGLING, // next
                HalfEdgeId::DANGLING, // prev
                h_face,
                new_vertex,               // H_new ALWAYS originates at M
                EdgeId::DANGLING, // sentinel edge
            ));

            new_ids.insert(h, h_new);

            if is_forward {
                // h is A->M (on E_old), h_new is M->B (on E_new)
                draft.arena_mut().get_half_edge_mut(h)?.set_edge(old_edge);
                draft
                    .arena_mut()
                    .get_half_edge_mut(h_new)?
                    .set_edge(new_edge);
                e_old_list.push(h);
                e_new_list.push(h_new);
            } else {
                // h is B->M (on E_new), h_new is M->A (on E_old)
                draft.arena_mut().get_half_edge_mut(h)?.set_edge(new_edge);
                draft
                    .arena_mut()
                    .get_half_edge_mut(h_new)?
                    .set_edge(old_edge);
                e_new_list.push(h);
                e_old_list.push(h_new);
            }

            // Wire next/prev
            draft.arena_mut().get_half_edge_mut(h_new)?.set_next(h_next);
            draft.arena_mut().get_half_edge_mut(h_new)?.set_prev(h);
            draft.arena_mut().get_half_edge_mut(h_next)?.set_prev(h_new);
            draft.arena_mut().get_half_edge_mut(h)?.set_next(h_new);

            draft.arena_mut().bump_face_version(h_face)?;
        }

        // Wire radial next loops
        for i in 0..e_old_list.len() {
            let next_i = (i + 1) % e_old_list.len();
            let curr = e_old_list[i];
            let nxt = e_old_list[next_i];
            draft
                .arena_mut()
                .get_half_edge_mut(curr)?
                .set_radial_next(nxt);
        }
        for i in 0..e_new_list.len() {
            let next_i = (i + 1) % e_new_list.len();
            let curr = e_new_list[i];
            let nxt = e_new_list[next_i];
            draft
                .arena_mut()
                .get_half_edge_mut(curr)?
                .set_radial_next(nxt);
        }

        // Wire vertices and edges
        let first_h_new = *new_ids.get(&chain[0]).unwrap();
        draft
            .arena_mut()
            .get_vertex_mut(new_vertex)?
            .set_primary_disk(first_h_new);

        let e_old_he = e_old_list[0];
        let e_new_he = e_new_list[0];
        let old_edge = draft.arena().get_half_edge(he_ab)?.edge();
        draft
            .arena_mut()
            .get_edge_mut(old_edge)?
            .set_half_edge(e_old_he);
        draft
            .arena_mut()
            .get_edge_mut(new_edge)?
            .set_half_edge(e_new_he);

        let he_twin = chain[1 % chain.len()];
        let he_mb = *new_ids.get(&he_ab).unwrap();

        let (he_bm, he_ma) = if chain.len() == 1 {
            (he_mb, he_ab)
        } else {
            let twin_new = *new_ids.get(&he_twin).unwrap();
            (he_twin, twin_new)
        };

        // ── Provenance Stamping (O(1)) ─────────────────────────────────
        use forge_core::{EntityRef, EntityKind};
        let parent = EntityRef::new(EntityKind::HalfEdge, self.edge.index(), self.edge.generation());
        let mut children: Vec<EntityRef> = vec![
            EntityRef::new(EntityKind::Vertex, new_vertex.index(), new_vertex.generation()),
            EntityRef::new(EntityKind::Edge, new_edge.index(), new_edge.generation()),
        ];
        for &h_new in new_ids.values() {
            children.push(EntityRef::new(EntityKind::HalfEdge, h_new.index(), h_new.generation()));
        }
        draft.stamp_children_of(_recorder, parent, &children);

        Ok(ExecutionResult {
            value: SplitEdgeOutput {
                he_am: he_ab,
                he_mb,
                he_bm,
                he_ma,
                new_vertex,
            },
            declared_delta: EulerDelta {
                vertices: 1,
                half_edges: chain.len() as i32,
                faces: 0,
                loops: 0,
                edges: 1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}
