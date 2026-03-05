//! KillVertexEdge — inverse of SplitEdge (MakeVertexEdge).
//!
//! DOMAIN: Removes a 2-valent vertex (in terms of edges), merging the two
//! incident edges into a single edge.
//!
//! INVARIANTS:
//! - ΔV=-1, ΔHE=-N (where N = number of halfedges originating from M), ΔE=-1
//! - The vertex must connect exactly two distinct edges.
//! - The operation restores the topology as it was before a SplitEdge.
//!
//! DEPENDENCIES: `arena` (entity storage)

use forge_core::KernelError;

use crate::handles::{EdgeId, HalfEdgeId, VertexId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;
use crate::validators::invariant_id::InvariantContract;

/// Merges two edges by removing their shared vertex.
///
/// This is the exact topological inverse of `SplitEdge`.
/// Given vertex M that sits between A and B (A→M→B), removes M
/// and merges the two edges back into one edge (A→B).
#[derive(Debug)]
pub struct KillVertexEdge {
    /// The vertex to remove.
    pub vertex: VertexId,
}

/// Output of the KillVertexEdge operator.
pub struct KveOutput {
    /// The surviving (merged) edge.
    pub surviving_edge: EdgeId,
}

impl TopoOperator for KillVertexEdge {
    type Output = KveOutput;

    const NAME: &'static str = "kill_vertex_edge";

    const INVARIANT_CONTRACT: InvariantContract = crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        format!("Remove vertex {} by merging its incident edges", self.vertex.index())
    }

    fn execute(&self, draft: &mut MutableDraft, _recorder: &mut crate::provenance::LineageRecorder) -> Result<ExecutionResult<Self::Output>, KernelError> {
        let vertex_m = self.vertex;

        let outgoing_he = draft.arena().get_vertex(vertex_m)?.primary_disk();
        if outgoing_he == HalfEdgeId::DANGLING {
            return Err(KernelError::InvalidInput {
                message: "KillVertexEdge: vertex has no outgoing halfedge".to_string(),
                context: None,
            });
        }

        // Collect ALL halfedges originating from M via vertex index.
        let from_m: Vec<HalfEdgeId> = draft.arena().halfedges_from_vertex(vertex_m).to_vec();

        if from_m.is_empty() {
            return Err(KernelError::InvalidInput {
                message: "KillVertexEdge: vertex has no outgoing halfedges".to_string(),
                context: None,
            });
        }

        // Determine the two distinct edges incident on M.
        // Each halfedge from M belongs to one of the two edges.
        // The halfedge ARRIVING at M (i.e. whose next's origin is M) belongs to the other edge.
        let mut edge_set: Vec<EdgeId> = Vec::new();
        for &h in &from_m {
            let e = draft.arena().get_half_edge(h)?.edge();
            if !edge_set.contains(&e) {
                edge_set.push(e);
            }
        }

        // Derive halfedges arriving at M from prev pointers of from_m halfedges.
        let mut arriving_at_m: Vec<HalfEdgeId> = Vec::new();
        for &h in &from_m {
            let prev = draft.arena().get_half_edge(h)?.prev();
            let prev_data = draft.arena().get_half_edge(prev)?;
            if prev_data.origin() != vertex_m {
                arriving_at_m.push(prev);
                let e = prev_data.edge();
                if !edge_set.contains(&e) {
                    edge_set.push(e);
                }
            }
        }

        if edge_set.len() != 2 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "KillVertexEdge: vertex must be 2-valent in edges, but has {} distinct edges",
                    edge_set.len()
                ),
                context: None,
            });
        }

        let surviving_edge = edge_set[0];
        let killed_edge = edge_set[1];

        // Categorize halfedges from M into which edge they belong to.
        // SplitEdge creates pairs: for each original halfedge in the radial chain,
        // it creates one halfedge on the old edge and one on the new edge.
        // The "from M" halfedges are the ones SplitEdge inserted.

        // For each halfedge from M: rewire its predecessor to skip over it.
        // pred.next = from_m_he.next
        // from_m_he.next.prev = pred
        let num_removed = from_m.len();

        for &h in &from_m {
            let h_prev = draft.arena().get_half_edge(h)?.prev();
            let h_next = draft.arena().get_half_edge(h)?.next();
            let h_face = draft.arena().get_half_edge(h)?.face();

            // Rewire prev/next to skip h
            draft.arena_mut().get_half_edge_mut(h_prev)?.set_next(h_next);
            draft.arena_mut().get_half_edge_mut(h_next)?.set_prev(h_prev);

            // If h_prev was on the killed edge, reassign it to the surviving edge
            let prev_edge = draft.arena().get_half_edge(h_prev)?.edge();
            if prev_edge == killed_edge {
                draft.arena_mut().get_half_edge_mut(h_prev)?.set_edge(surviving_edge);
            }

            // If loop's representative halfedge was h, update it
            let loop_id = draft.arena().get_face(h_face)?.outer_loop();
            let loop_he = draft.arena().get_loop(loop_id)?.half_edge();
            if loop_he == h {
                draft.arena_mut().get_loop_mut(loop_id)?.set_half_edge(h_next);
            }
            // Check inner loops too
            let inner_loops: Vec<_> = draft.arena().get_face(h_face)?.inner_loops().to_vec();
            for il in inner_loops {
                let il_he = draft.arena().get_loop(il)?.half_edge();
                if il_he == h {
                    draft.arena_mut().get_loop_mut(il)?.set_half_edge(h_next);
                }
            }

            draft.arena_mut().bump_face_version(h_face)?;
        }

        // Merge radial rings: all surviving halfedges that were on killed_edge
        // need to be reassigned to surviving_edge and spliced into its radial ring.
        let mut surviving_radials: Vec<HalfEdgeId> = Vec::new();
        for (id, data) in draft.arena().iter_half_edges() {
            if data.edge() == surviving_edge && !from_m.contains(&id) {
                surviving_radials.push(id);
            }
        }

        // Wire radial ring for the surviving edge
        if !surviving_radials.is_empty() {
            for i in 0..surviving_radials.len() {
                let next_i = (i + 1) % surviving_radials.len();
                draft
                    .arena_mut()
                    .get_half_edge_mut(surviving_radials[i])?
                    .set_radial_next(surviving_radials[next_i]);
            }
        }

        // Update surviving edge's representative halfedge
        if let Some(&first) = surviving_radials.first() {
            draft
                .arena_mut()
                .get_edge_mut(surviving_edge)?
                .set_half_edge(first);
        }

        // Update vertex outgoing pointers: vertices that pointed through M
        // need updated outgoing if their outgoing was one of the removed halfedges.
        // The predecessors of removed halfedges (which are the A→M halfedges now wired to skip M)
        // originate from various vertices — those vertices might need their outgoing updated.
        for &h in &arriving_at_m {
            let origin_v = draft.arena().get_half_edge(h)?.origin();
            let current_out = draft.arena().get_vertex(origin_v)?.primary_disk();
            // If outgoing was pointing at a removed halfedge, fix it
            if from_m.contains(&current_out) {
                draft.arena_mut().get_vertex_mut(origin_v)?.set_primary_disk(h);
            }
        }

        // Remove all halfedges from M
        for &h in &from_m {
            draft.remove_half_edge(h)?;
        }

        // Remove vertex M
        draft.remove_vertex(vertex_m)?;

        // Remove the killed edge
        draft.remove_edge(killed_edge)?;

        Ok(ExecutionResult {
            value: KveOutput {
                surviving_edge,
            },
            declared_delta: EulerDelta {
                vertices: -1,
                half_edges: -(num_removed as i32),
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
}
