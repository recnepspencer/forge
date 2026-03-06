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

use crate::handles::HalfEdgeId;
use crate::operator::TopoOperator;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::validators::invariant_id::InvariantContract;

/// Collapse an edge by removing it and merging its target vertex into the origin.
///
/// `edge` is a halfedge A→B. Vertex B is removed; all references to B
/// become references to A. The edge (both halfedges) is removed.
///
/// # 1-Gon Face Deletion Parity
/// This operator explicitly refuses to collapse an edge where `vertex_a == vertex_b`
/// (a self-loop or 1-gon). Collapsing a 1-gon would annihilate the loop entirely,
/// leaving a Face with no boundary. To destroy a 1-gon face, higher-level
/// algorithms should use operations like `KillFaceVertex` or simply `RemoveFace`.
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

impl TopoOperator for KillEdgeVertex {
    type Output = KevOutput;

    const NAME: &'static str = "kill_edge_vertex";

    const INVARIANT_CONTRACT: InvariantContract =
        crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        format!(
            "Collapse edge at halfedge {}, merging target vertex into origin",
            self.edge.index()
        )
    }

    fn execute(
        &self,
        draft: &mut MutableDraft,
        _recorder: &mut crate::provenance::LineageRecorder,
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
            crate::queries::traverse::RadialEdgeIterator::new(draft.arena(), he)?
                .collect::<Result<_, _>>()?;

        // 1. Maintain loop representative pointers for faces.
        // Must search ALL loops (outer + inner) to find which loop
        // contains the halfedge being killed — not just outer_loop.
        for &h in &chain {
            let face = draft.arena().get_half_edge(h)?.face();
            // Wire edges have DANGLING face — skip face/loop maintenance.
            if face.is_dangling() {
                continue;
            }

            // Collect all loops on this face (outer + inner)
            let face_data = draft.arena().get_face(face)?;
            let mut all_loops = vec![face_data.loops.outer()];
            all_loops.extend_from_slice(face_data.loops.inners());

            for loop_id in all_loops {
                let start = draft.arena().get_loop(loop_id)?.half_edge();
                // Only update if this loop's representative is in the chain
                let mut needs_update = false;
                let mut curr = start;
                for _ in 0..draft.arena().half_edge_count() {
                    if chain.contains(&curr) {
                        if curr == start {
                            needs_update = true;
                        }
                        // Even if start is not in chain, we still might need
                        // to check — but we only need to update the loop
                        // pointer if the current representative IS in the chain.
                        break;
                    }
                    curr = draft.arena().get_half_edge(curr)?.next();
                    if curr == start {
                        break;
                    }
                }

                if !needs_update && !chain.contains(&start) {
                    continue;
                }

                // Find a surviving halfedge in this loop
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
                }
                // If no survivor in this loop, it means all halfedges in
                // this loop are being killed — this is OK, the loop will
                // be cleaned up by a higher-level operator.
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

        // 3. Migrate origins from V_b to V_a using O(1) index lookup.
        //    IMPORTANT: set_origin() only updates entity data — the adjacency
        //    index (vertex_halfedges) must be updated separately.
        let edges_from_b: Vec<(HalfEdgeId, crate::handles::FaceId)> = draft
            .arena()
            .halfedges_from_vertex(vertex_b)
            .iter()
            .filter(|id| !chain.contains(id))
            .map(|&id| {
                let face = draft
                    .arena()
                    .get_half_edge(id)
                    .map(|d| d.face())
                    .unwrap_or(crate::handles::FaceId::DANGLING);
                (id, face)
            })
            .collect();
        for (edge_id, face) in &edges_from_b {
            draft
                .arena_mut()
                .get_half_edge_mut(*edge_id)?
                .set_origin(vertex_a);
            // Update adjacency index: remove from vertex_b, add to vertex_a
            draft
                .arena_mut()
                .index_remove_halfedge(*edge_id, *face, vertex_b);
            draft
                .arena_mut()
                .index_add_halfedge(*edge_id, *face, vertex_a);
        }

        // 4. Ensure vertex_a has a surviving outgoing halfedge.
        // (Step 3 already migrated all vertex_b origins to vertex_a.)
        // Use O(1) index lookup instead of scanning the entire arena.
        let v_a_halfedges = draft.arena().halfedges_from_vertex(vertex_a);
        let v_a_survivor = v_a_halfedges.iter().find(|id| !chain.contains(id)).copied();

        if let Some(s) = v_a_survivor {
            draft
                .arena_mut()
                .get_vertex_mut(vertex_a)?
                .set_primary_disk(s);
        } else {
            return Err(KernelError::InvalidInput {
                message: "KillEdgeVertex would leave Vertex A isolated with no outgoing edges."
                    .into(),
                context: None,
            });
        }

        // 5. Bump face versions and clean up
        let num_half_edges_removed = chain.len() as i32;
        for &h in &chain {
            let f = draft.arena().get_half_edge(h)?.face();
            // Guard against DANGLING face (wireframe/wire edges).
            if !f.is_dangling() {
                draft.arena_mut().bump_face_version(f)?;
            }
            draft.remove_half_edge(h)?;
        }

        draft.remove_vertex(vertex_b)?;
        draft.remove_edge(killed_edge)?;

        // 6. Rebuild canonical disk entries on the survivor after collapse.
        let rebuilt = crate::queries::vertex_disks::rebuild_disk_entries(draft.arena(), vertex_a)?;
        if rebuilt.is_empty() {
            return Err(KernelError::InvalidInput {
                message: "KillEdgeVertex produced a surviving vertex with no disk entries".into(),
                context: None,
            });
        }
        {
            draft
                .arena_mut()
                .reset_disk_entries(vertex_a, rebuilt[0], &rebuilt[1..])?;
        }

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
}
