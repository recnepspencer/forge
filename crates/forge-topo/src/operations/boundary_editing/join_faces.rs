//! JoinFaces — merge two faces by removing a shared edge.
//!
//! DOMAIN: Given a halfedge whose two sides border different faces,
//! remove the edge and merge the two faces into one.
//!
//! INVARIANTS:
//! - The two faces must be distinct
//! - Removes 2 halfedges, 1 face, 1 loop
//! - Euler formula: E-1, F-1 (net: same V-E+F)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::{KernelError, TopologyError};

use crate::handles::{HalfEdgeId, LoopId};
use crate::operator::TopoOperator;
use crate::operator::{EulerDelta, ExecutionResult};
use crate::transactions::MutableDraft;
use crate::validators::invariant_id::InvariantContract;

/// Merge two faces by removing a shared edge.
///
/// `edge` is a halfedge on the shared edge. Its face and
/// `edge.twin`'s face must be distinct. The twin's face is removed;
/// the edge's face survives with merged lineage.
#[derive(Debug)]
pub struct JoinFaces {
    /// A halfedge on the edge to remove. This halfedge's face survives.
    pub edge: HalfEdgeId,
}

/// Output of the JoinFaces operator.
pub struct JfOutput {
    /// The surviving face.
    pub surviving_face: crate::handles::FaceId,
}

impl TopoOperator for JoinFaces {
    type Output = JfOutput;

    const NAME: &'static str = "join_faces";

    const INVARIANT_CONTRACT: InvariantContract =
        crate::validators::contract_registry::FULL_TOPO_WIRING;

    fn semantic_summary(&self) -> String {
        format!(
            "Join two faces by removing edge at halfedge {}",
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
        let he_twin = he_data.radial_next();
        let he_next = he_data.next();
        let he_prev = he_data.prev();
        let face_survive = he_data.face();
        let vertex_a = he_data.origin();
        let killed_edge = he_data.edge();

        let twin_data = draft.arena().get_half_edge(he_twin)?;
        let twin_next = twin_data.next();
        let twin_prev = twin_data.prev();
        let face_remove = twin_data.face();
        let vertex_b = twin_data.origin();

        if face_survive == face_remove {
            return Err(KernelError::InvalidInput {
                message: "JoinFaces: both sides of edge belong to the same face".to_string(),
                context: None,
            });
        }
        if draft.arena().get_face(face_survive)?.shell()
            != draft.arena().get_face(face_remove)?.shell()
        {
            return Err(KernelError::InvalidInput {
                message: "JoinFaces: faces belong to different shells".to_string(),
                context: None,
            });
        }

        let valence = crate::queries::traverse::radial_valence(draft.arena(), he)?;
        if valence != 2 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "JoinFaces: edge has radial valence {}, must be exactly 2 for joining",
                    valence
                ),
                context: None,
            });
        }

        draft
            .arena_mut()
            .get_half_edge_mut(he_prev)?
            .set_next(twin_next);
        draft
            .arena_mut()
            .get_half_edge_mut(twin_next)?
            .set_prev(he_prev);
        draft
            .arena_mut()
            .get_half_edge_mut(twin_prev)?
            .set_next(he_next);
        draft
            .arena_mut()
            .get_half_edge_mut(he_next)?
            .set_prev(twin_prev);

        reassign_face(draft, twin_next, face_survive)?;
        let loop_id = draft.arena().get_face(face_survive)?.loops.outer();
        let loop_he = draft.arena().get_loop(loop_id)?.half_edge();
        if loop_he == he || loop_he == he_twin {
            draft
                .arena_mut()
                .get_loop_mut(loop_id)?
                .set_half_edge(he_next);
        }

        // P10: Transfer inner loops from face_remove to face_survive
        let inner_loops: Vec<LoopId> = draft.arena().get_face(face_remove)?.loops.inners().to_vec();
        for il_id in inner_loops {
            let inner_start = draft.arena().get_loop(il_id)?.half_edge();
            draft
                .arena_mut()
                .get_face_mut(face_remove)?
                .loops
                .remove_inner(il_id);
            draft
                .arena_mut()
                .get_face_mut(face_survive)?
                .loops
                .add_inner(il_id);
            draft
                .arena_mut()
                .get_loop_mut(il_id)?
                .set_face(face_survive);
            reassign_face(draft, inner_start, face_survive)?;
        }

        if draft.arena().get_vertex(vertex_a)?.primary_disk() == he {
            let next_a = if twin_next == he_twin {
                he_next
            } else {
                twin_next
            };
            draft
                .arena_mut()
                .get_vertex_mut(vertex_a)?
                .set_primary_disk(next_a);
        }
        if draft.arena().get_vertex(vertex_b)?.primary_disk() == he_twin {
            let next_b = if he_next == he { twin_next } else { he_next };
            draft
                .arena_mut()
                .get_vertex_mut(vertex_b)?
                .set_primary_disk(next_b);
        }

        let remove_loop = draft.arena().get_face(face_remove)?.loops.outer();

        draft.remove_half_edge(he)?;
        draft.remove_half_edge(he_twin)?;
        draft.remove_loop(remove_loop)?;
        draft.remove_face(face_remove)?;
        draft.remove_edge(killed_edge)?;

        Ok(ExecutionResult {
            value: JfOutput {
                surviving_face: face_survive,
            },
            declared_delta: EulerDelta {
                vertices: 0,
                half_edges: -2,
                faces: -1,
                loops: -1,
                edges: -1,
                shells: 0,
                solids: 0,
                lumps: 0,
                regions: 0,
            },
        })
    }
}

/// Reassign all halfedges starting from `start` to `new_face`.
///
/// Walks the loop via `next()` until returning to `start`.
/// Uses `reassign_halfedge_face` to keep the reverse index in sync.
fn reassign_face(
    draft: &mut MutableDraft,
    start: HalfEdgeId,
    new_face: crate::handles::FaceId,
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
                    walk_kind: "reassign_face".into(),
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
