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

use crate::arena::{FaceData, HalfEdgeData, LoopData, EdgeData};
use crate::handles::{FaceId, HalfEdgeId, LoopId, VertexId, EdgeId};
use crate::lineage::{Lineage, OpSignature};
use crate::EulerOperator;
use crate::operator::{ExecutionResult, EulerDelta};
use crate::state::MutableDraft;

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

impl EulerOperator for MakeEdgeFace {
    type Output = MefOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<ExecutionResult<Self::Output>, KernelError> {
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

        let face_lineage = draft.arena().get_face(self.face)?.lineage().cloned();
        let source_shell = draft.arena().get_face(self.face)?.shell();
        let he_ab_lineage = Lineage::derive_from(&face_lineage, sig.clone());
        let he_ba_lineage = Lineage::derive_from(&face_lineage, sig.clone());
        let new_face_lineage = Lineage::derive_from(&face_lineage, sig.clone());
        let edge_lineage = Lineage::derive_from(&face_lineage, sig.clone());

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);

        let new_face = draft.insert_face(FaceData::with_lineage(
            placeholder_loop,
            source_shell,
            Some(new_face_lineage),
        ));

        let new_loop = draft.insert_loop(LoopData::new(placeholder_he, new_face));

        let edge = draft.insert_edge(EdgeData::with_lineage(
            placeholder_he,
            Some(edge_lineage),
        ));

        let (he_ab, he_ba) = draft.insert_half_edge_pair(
            HalfEdgeData::with_lineage(
                placeholder_he,
                he_from_b,
                prev_a,
                self.face,
                self.vertex_a,
                edge,
                Some(he_ab_lineage),
            ),
            HalfEdgeData::with_lineage(
                placeholder_he,
                he_from_a,
                prev_b,
                new_face,
                self.vertex_b,
                edge,
                Some(he_ba_lineage),
            ));

        {
            let arena = draft.arena_mut();
            arena.get_half_edge_mut(prev_a)?.set_next(he_ab);
            arena.get_half_edge_mut(he_from_b)?.set_prev(he_ab);
            arena.get_half_edge_mut(prev_b)?.set_next(he_ba);
            arena.get_half_edge_mut(he_from_a)?.set_prev(he_ba);
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
            declared_delta: EulerDelta { vertices: 0, half_edges: 2, faces: 1, loops: 1, edges: 1, shells: 0 },
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("make_edge_face")
    }
}

/// Collect all halfedges originating from `vertex` that lie on `face`.
///
/// Uses a vertex orbit walk (O(valence)) instead of a face-loop walk
/// (O(face_size)). Vertex valence is typically 3–6.
fn find_all_halfedges_from_vertex(
    draft: &MutableDraft,
    face: FaceId,
    vertex: VertexId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let outgoing = draft.arena().get_vertex(vertex)?.outgoing();
    let start = outgoing;
    let mut current = start;
    let mut result = Vec::new();
    let bound = draft.arena().half_edge_count();

    for step in 0..=bound {
        if draft.arena().get_half_edge(current)?.face() == face {
            result.push(current);
        }
        let twin = draft.arena().get_half_edge(current)?.twin();
        current = draft.arena().get_half_edge(twin)?.next();
        if current == start { break; }
        if step == bound {
            return Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "vertex_orbit".into(),
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
            message: format!("Vertex {} not found on face {}", vertex.index(), face.index()),
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
        if current == he_a { return Ok(false); }
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
        message: "No valid split pair found: vertices may be adjacent or on the same sub-path".to_string(),
        context: None,
    })
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
        
        draft.arena_mut().get_half_edge_mut(current)?.set_face(new_face);
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
