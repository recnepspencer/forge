//! MakeEdgeFace — split a face by inserting an edge between two vertices.
//!
//! DOMAIN: Given an existing face with two vertices on its boundary,
//! insert a new edge connecting them and split the face into two.
//!
//! INVARIANTS:
//! - Both vertices must lie on the same face loop
//! - Creates 2 new halfedges, 1 new face, 1 new loop
//! - Euler formula: E+1, F+1 (net: same V-E+F)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::{KernelError, TopologyError};

use crate::b_rep::{EdgeData, FaceData, HalfEdgeData, LoopData};
use crate::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, VertexId};
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::operator::TopoOperator;


/// Split a face by inserting a new edge between two of its vertices.
///
/// Given face `face` and two vertices `vertex_a` and `vertex_b` on its
/// boundary, inserts edge (A→B) and splits the face. The original face
/// keeps the loop containing the new edge A→B. A new face gets the
/// loop containing B→A.
#[derive(Debug)]
pub struct MakeEdgeFace {
    /// The face to split.
    pub face: FaceId,
    /// First vertex of the new edge (must be on the face boundary).
    pub vertex_a: VertexId,
    /// Second vertex of the new edge (must be on the face boundary).
    pub vertex_b: VertexId,
}

/// Output of the MakeEdgeFace operator.
pub struct MefOutput {
    /// The new halfedge A→B (on the original face).
    pub half_edge_ab: HalfEdgeId,
    /// The new halfedge B→A (on the new face).
    pub half_edge_ba: HalfEdgeId,
    /// The newly created face (gets the B→A side).
    pub new_face: FaceId,
    /// The newly created loop for the new face.
    pub new_loop: LoopId,
    /// The newly created edge (owns the halfedge pair).
    pub edge: EdgeId,
}

impl TopoOperator for MakeEdgeFace {
    type Output = MefOutput;

    const NAME: &'static str = "make_edge_face";

    fn semantic_summary(&self) -> String {
        format!(
            "Split face {} by inserting edge between vertices {} and {}",
            self.face.index(), self.vertex_a.index(), self.vertex_b.index()
        )
    }

    fn execute(
        &self,
        draft: &mut MutableDraft,
    ) -> Result<ExecutionResult<Self::Output>, KernelError> {
        if self.vertex_a == self.vertex_b {
            return Err(KernelError::InvalidInput {
                message: "MakeEdgeFace: vertex_a and vertex_b cannot be the same vertex".into(),
                context: None,
            });
        }

        let candidates_a = find_all_halfedges_from_vertex(draft, self.face, self.vertex_a)?;
        let candidates_b = find_all_halfedges_from_vertex(draft, self.face, self.vertex_b)?;

        let (he_from_a, he_from_b) = find_valid_split_pair(draft, &candidates_a, &candidates_b)?;

        let prev_a = draft.arena().get_half_edge(he_from_a)?.prev();
        let prev_b = draft.arena().get_half_edge(he_from_b)?.prev();

        let source_shell = draft.arena().get_face(self.face)?.shell();

        let placeholder_he = HalfEdgeId::DANGLING;
        let placeholder_loop = LoopId::DANGLING;

        let new_face = draft.insert_face(FaceData::new(
            placeholder_loop,
            source_shell,
        ));

        let new_loop = draft.insert_loop(LoopData::new(placeholder_he, new_face));

        let edge = draft.insert_edge(EdgeData::new(placeholder_he));

        let (he_ab, he_ba) = draft.insert_radial_pair(
            HalfEdgeData::new(
                placeholder_he,
                he_from_b,
                prev_a,
                self.face,
                self.vertex_a,
                edge,
            ),
            HalfEdgeData::new(
                placeholder_he,
                he_from_a,
                prev_b,
                new_face,
                self.vertex_b,
                edge,
            ),
        );

        {
            let arena = draft.arena_mut();
            arena.get_half_edge_mut(prev_a)?.set_next(he_ab);
            arena.get_half_edge_mut(he_from_b)?.set_prev(he_ab);
            arena.get_half_edge_mut(prev_b)?.set_next(he_ba);
            arena.get_half_edge_mut(he_from_a)?.set_prev(he_ba);
        }

        // ── Edge-consistency repair ──────────────────────────────────
        // Splitting the face loop changes which halfedge follows which.
        // Any halfedge whose effective destination (next.origin) now
        // differs from its radial twin's vertex pair needs its Edge
        // entity split. Scan both resulting sub-loops.
        let loop_a = collect_loop(draft, he_ab)?;
        let loop_b = collect_loop(draft, he_ba)?;
        let mut extra_edges = 0i32;
        for &he in loop_a.iter().chain(loop_b.iter()) {
            if repair_edge_after_next_change(draft, he)? {
                extra_edges += 1;
            }
        }

        reassign_face_loop(draft, he_ba, new_face)?;

        let original_loop = draft.arena().get_face(self.face)?.outer_loop();
        let arena = draft.arena_mut();
        arena.get_loop_mut(original_loop)?.set_half_edge(he_ab);
        arena.get_face_mut(new_face)?.set_outer_loop(new_loop);
        arena.get_loop_mut(new_loop)?.set_half_edge(he_ba);
        arena.get_edge_mut(edge)?.set_half_edge(he_ab);

        Ok(ExecutionResult {
            value: MefOutput {
                half_edge_ab: he_ab,
                half_edge_ba: he_ba,
                new_face,
                new_loop,
                edge,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: 2,
                faces: 1,
                loops: 1,
                edges: 1 + extra_edges,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }


}

/// Collect all halfedges originating from `vertex` that lie on `face`.
///
/// Walks the face boundary loop (O(face_size)) to find all halfedges
/// from `vertex`. This is robust against boundary edges (self-radial)
/// that can disconnect the vertex orbit from certain faces.
fn find_all_halfedges_from_vertex(
    draft: &MutableDraft,
    face: FaceId,
    vertex: VertexId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let outer_loop = draft.arena().get_face(face)?.outer_loop();
    let start = draft.arena().get_loop(outer_loop)?.half_edge();
    let mut current = start;
    let mut result = Vec::new();
    let bound = draft.arena().half_edge_count();

    for step in 0..=bound {
        if draft.arena().get_half_edge(current)?.origin() == vertex {
            result.push(current);
        }
        current = draft.arena().get_half_edge(current)?.next();
        if current == start {
            break;
        }
        if step == bound {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "face_loop_vertex_search".into(),
                    seed_index: start.index(),
                    last_visited_index: current.index(),
                    steps_taken: step,
                    entity_bound: bound,
                },
                context: None,
            });
        }
    }

    if result.is_empty() {
        return Err(KernelError::InvalidInput {
            message: format!(
                "Vertex {} not found on face {}",
                vertex.index(),
                face.index()
            ),
            context: None,
        });
    }

    result.sort_by_key(|he| he.index());
    Ok(result)
}

/// Validate that splitting a loop at `(he_a, he_b)` produces two
/// well-formed sub-loops. Walks `he_a → next → ... → he_b` and checks
/// that the path reaches `he_b` without revisiting `he_a`.
fn validate_split_pair(
    draft: &MutableDraft,
    he_a: HalfEdgeId,
    he_b: HalfEdgeId,
    ) -> Result<bool, KernelError> {
    if he_a == he_b {
        return Ok(false);
    }
    let bound = draft.arena().half_edge_count();
    let mut current = draft.arena().get_half_edge(he_a)?.next();
    let mut steps = 0usize;

    while current != he_b {
        if current == he_a {
            return Ok(false);
        }
        steps += 1;
        if steps > bound {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "validate_split_pair".into(),
                    seed_index: he_a.index(),
                    last_visited_index: current.index(),
                    steps_taken: steps,
                    entity_bound: bound,
                },
                context: None,
            });
        }
        current = draft.arena().get_half_edge(current)?.next();
    }

    Ok(true)
}

/// Find a valid `(he_from_a, he_from_b)` pair that splits the face loop
/// into two well-formed sub-loops. Tries all candidate combinations.
fn find_valid_split_pair(
    draft: &MutableDraft,
    candidates_a: &[HalfEdgeId],
    candidates_b: &[HalfEdgeId],
) -> Result<(HalfEdgeId, HalfEdgeId), KernelError> {
    for &he_a in candidates_a {
        for &he_b in candidates_b {
            if validate_split_pair(draft, he_a, he_b)? {
                return Ok((he_a, he_b));
            }
        }
    }
    Err(KernelError::InvalidInput {
        message: "No valid split pair found: vertices may be adjacent or on the same sub-path"
            .to_string(),
        context: None,
    })
}

/// Collect all halfedge IDs in a face loop starting from `start`.
fn collect_loop(draft: &MutableDraft, start: HalfEdgeId) -> Result<Vec<HalfEdgeId>, KernelError> {
    let bound = draft.arena().half_edge_count();
    let mut result = Vec::new();
    let mut current = start;
    loop {
        result.push(current);
        current = draft.arena().get_half_edge(current)?.next();
        if current == start {
            break;
        }
        if result.len() > bound {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "collect_loop".into(),
                    seed_index: start.index(),
                    last_visited_index: current.index(),
                    steps_taken: result.len(),
                    entity_bound: bound,
                },
                context: None,
            });
        }
    }
    Ok(result)
}

/// Reassign all halfedges in a loop (starting from `start`) to `new_face`.
fn reassign_face_loop(
    draft: &mut MutableDraft,
    start: HalfEdgeId,
    new_face: FaceId,
) -> Result<(), KernelError> {
    let bound = draft.arena().half_edge_count();
    let mut current = start;
    let mut steps = 0usize;
    loop {
        draft
            .arena_mut()
            .reassign_halfedge_face(current, new_face)?;
        let next = draft.arena().get_half_edge(current)?.next();
        current = next;
        if current == start {
            break;
        }
        steps += 1;
        if steps > bound {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "reassign_face_loop".into(),
                    seed_index: start.index(),
                    last_visited_index: current.index(),
                    steps_taken: steps,
                    entity_bound: bound,
                },
                context: None,
            });
        }
    }
    Ok(())
}

/// Repair edge-entity consistency after a halfedge's `.next` pointer changed.
///
/// When MEF redirects `h.next`, the effective destination of `h` changes
/// (destination = `h.next.origin`). If `h` has a non-self-radial twin, the
/// two halfedges may now span different vertex pairs, making the shared
/// Edge entity inconsistent.
///
/// This function detects the inconsistency by comparing the vertex pair
/// `{h.origin, h.next.origin}` against the twin's vertex pair
/// `{twin.origin, twin.next.origin}`. If they differ, the radial link is
/// broken and both become self-radial boundary edges on separate Edge
/// entities.
/// Returns `true` if a new Edge entity was created (for Euler delta tracking).
fn repair_edge_after_next_change(
    draft: &mut MutableDraft,
    he: HalfEdgeId,
) -> Result<bool, KernelError> {
    let he_data = draft.arena().get_half_edge(he)?;
    let twin = he_data.radial_next();

    if twin == he {
        return Ok(false);
    }

    let he_origin = he_data.origin();
    let he_dest = draft.arena().get_half_edge(he_data.next())?.origin();

    let twin_data = draft.arena().get_half_edge(twin)?;
    let twin_origin = twin_data.origin();
    let twin_dest = draft.arena().get_half_edge(twin_data.next())?.origin();

    let he_verts = vertex_pair(he_origin, he_dest);
    let twin_verts = vertex_pair(twin_origin, twin_dest);

    if he_verts == twin_verts {
        return Ok(false);
    }

    // Vertex pairs diverged — split the radial link.
    // The halfedge `he` keeps the old Edge entity.
    // The twin gets a new Edge entity and becomes self-radial.
    let old_edge = he_data.edge();

    let new_edge = draft.insert_edge(EdgeData::new(twin));

    let arena = draft.arena_mut();
    arena.get_half_edge_mut(he)?.set_radial_next(he);
    arena.get_half_edge_mut(twin)?.set_radial_next(twin);
    arena.get_half_edge_mut(twin)?.set_edge(new_edge);
    arena.get_edge_mut(old_edge)?.set_half_edge(he);

    Ok(true)
}

/// Canonical vertex pair for an edge (smaller index first).
fn vertex_pair(a: VertexId, b: VertexId) -> (u32, u32) {
    let ai = a.index();
    let bi = b.index();
    if ai <= bi {
        (ai, bi)
    } else {
        (bi, ai)
    }
}
