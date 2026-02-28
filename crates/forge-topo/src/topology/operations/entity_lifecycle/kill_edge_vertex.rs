//! KillEdgeVertex — collapse an edge by merging its target vertex into its origin.
//!
//! DOMAIN: Given a halfedge (A→B), removes the edge and vertex B.
//! All halfedges that used B as origin are rewired to A.
//!
//! INVARIANTS:
//! - Removes 1 vertex, 2 halfedges (the edge pair)
//! - Euler formula: V-1, E-1 (net: same V-E+F)
//! - Surviving vertex gets merged lineage
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::{KernelError, TopologyError};

use crate::handles::{EdgeId, HalfEdgeId};
use crate::lineage::{Lineage, OpSignature};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::state::MutableDraft;
use crate::EulerOperator;

/// Collapse an edge by removing it and merging its target vertex into the origin.
///
/// `edge` is a halfedge A→B. Vertex B is removed; all references to B
/// become references to A. The edge (both halfedges) is removed.
#[derive(Debug)]
pub struct KillEdgeVertex {
    /// The halfedge to kill. Its target vertex (twin's origin) is collapsed.
    pub edge: HalfEdgeId,
}

/// Output of the KillEdgeVertex operator.
pub struct KevOutput {
    /// The surviving vertex (the origin of `edge`).
    pub surviving_vertex: crate::handles::VertexId,
    /// Whether this collapse produced a degenerate self-loop halfedge.
    ///
    /// When `true`, the surviving vertex's outgoing halfedge has
    /// `twin == next == prev == self`. This is the same degenerate state
    /// as `MakeVertexFace`'s initial seed. Traverse code must handle
    /// `he.radial_next() == he` to avoid infinite loops.
    pub is_degenerate: bool,
}

impl EulerOperator for KillEdgeVertex {
    type Output = KevOutput;

    fn execute(
        &self,
        draft: &mut MutableDraft,
        sig: &OpSignature,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let he = self.edge;
        let he_data = draft.arena().get_half_edge(he)?;
        let vertex_a = he_data.origin();

        let he_next = draft.arena().get_half_edge(he_data.next())?;
        let vertex_b = he_next.origin();

        let killed_edge = he_data.edge();

        if vertex_a == vertex_b {
            return Err(KernelError::InvalidInput {
                message: "Cannot KillEdgeVertex on a geometric self-loop edge".into(),
                context: None,
            });
        }

        let chain: Vec<HalfEdgeId> =
            crate::topology::queries::traverse::RadialEdgeIterator::new(draft.arena(), he)?
                .collect::<Result<_, _>>()?;

        let v_a_lineage = draft.arena().get_vertex(vertex_a)?.lineage().cloned();
        let v_b_lineage = draft.arena().get_vertex(vertex_b)?.lineage().cloned();
        let merged_lineage = Lineage::merge(&v_a_lineage, &v_b_lineage, sig);

        // 1. Maintain outer_loop for faces
        for &h in &chain {
            let face = draft.arena().get_half_edge(h)?.face();
            let loop_id = draft.arena().get_face(face)?.outer_loop();
            let start = draft.arena().get_loop(loop_id)?.half_edge();

            let mut survivor = None;
            let mut curr = start;
            for _ in 0..draft.arena().half_edge_count() {
                if !chain.contains(&curr) {
                    survivor = Some(curr);
                    break;
                }
                curr = draft.arena().get_half_edge(curr)?.next();
                if curr == start {
                    break;
                }
            }

            if let Some(s) = survivor {
                draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(s);
            } else {
                return Err(KernelError::InvalidInput {
                    message: "Cannot KillEdgeVertex if it destroys an entire face loop. Use KillVertexFace instead.".into(),
                    context: None,
                });
            }
        }

        // 2. Determine rewires for prev/next
        let mut rewires = Vec::new();
        for &h in &chain {
            let h_prev = draft.arena().get_half_edge(h)?.prev();
            if !chain.contains(&h_prev) {
                let mut nxt = draft.arena().get_half_edge(h)?.next();
                let mut count = 0;
                while chain.contains(&nxt) {
                    if count > draft.arena().half_edge_count() {
                        return Err(KernelError::TopologyViolation {
                            err: TopologyError::LoopCorruption {
                                walk_kind: "kill_edge_vertex_bypass".into(),
                                seed_index: h.index(),
                                last_visited_index: nxt.index(),
                                steps_taken: count,
                                entity_bound: draft.arena().half_edge_count(),
                            },
                            context: None,
                        });
                    }
                    nxt = draft.arena().get_half_edge(nxt)?.next();
                    count += 1;
                }
                rewires.push((h_prev, nxt));
            }
        }

        // Apply rewires
        for (prv, nxt) in rewires {
            draft.arena_mut().get_half_edge_mut(prv)?.set_next(nxt);
            draft.arena_mut().get_half_edge_mut(nxt)?.set_prev(prv);
        }

        // 3. Migrate origins from V_b to V_a
        let mut edges_from_b = Vec::new();
        for (id, data) in draft.arena().iter_half_edges() {
            if data.origin() == vertex_b && !chain.contains(&id) {
                edges_from_b.push(id);
            }
        }
        for edge_id in edges_from_b {
            draft
                .arena_mut()
                .get_half_edge_mut(edge_id)?
                .set_origin(vertex_a);
        }

        // 4. Update vertex_a outgoing pointer
        // (Step 3 already migrated all vertex_b origins to vertex_a.)
        let mut v_a_survivor = None;
        for (id, data) in draft.arena().iter_half_edges() {
            if data.origin() == vertex_a && !chain.contains(&id) {
                v_a_survivor = Some(id);
                break;
            }
        }

        if let Some(s) = v_a_survivor {
            draft.arena_mut().get_vertex_mut(vertex_a)?.set_outgoing(s);
        } else {
            return Err(KernelError::InvalidInput {
                message: "KillEdgeVertex would leave Vertex A isolated with no outgoing edges."
                    .into(),
                context: None,
            });
        }

        draft
            .arena_mut()
            .get_vertex_mut(vertex_a)?
            .set_lineage(Some(merged_lineage));

        // 5. Bump face versions and clean up
        let num_half_edges_removed = chain.len() as i32;
        for &h in &chain {
            let f = draft.arena().get_half_edge(h)?.face();
            draft.arena_mut().bump_face_version(f)?;
            draft.remove_half_edge(h)?;
        }

        draft.remove_vertex(vertex_b)?;
        draft.remove_edge(killed_edge)?;

        Ok(ExecutionResult {
            value: KevOutput {
                surviving_vertex: vertex_a,
                is_degenerate: false,
            },
            declared_delta: EulerDelta {
                vertices: -1,
                half_edges: -num_half_edges_removed,
                faces: 0,
                loops: 0,
                edges: -1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("kill_edge_vertex")
    }
}
