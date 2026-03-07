use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::RelationKind;
use crate::logic::transaction::SpecDraft;

pub fn collect_loop_half_edges(
    draft: &SpecDraft,
    loop_id: SpecNodeId,
) -> Result<Vec<SpecNodeId>, SpecError> {
    let start = draft.single_outgoing_target(loop_id, RelationKind::LoopEntryHalfEdge)?;
    let mut result = Vec::new();
    let mut current = start;
    let max_steps = draft.current_node_count().max(1);

    for _ in 0..max_steps {
        result.push(current);
        let next = draft.single_outgoing_target(current, RelationKind::HalfEdgeNext)?;
        if next == start {
            return Ok(result);
        }
        current = next;
    }

    Err(SpecError::invalid(format!(
        "loop {} does not close within {} traversal steps",
        loop_id, max_steps
    )))
}

pub fn loop_contains_half_edge(
    draft: &SpecDraft,
    loop_id: SpecNodeId,
    half_edge: SpecNodeId,
) -> Result<bool, SpecError> {
    Ok(collect_loop_half_edges(draft, loop_id)?
        .into_iter()
        .any(|candidate| candidate == half_edge))
}

pub fn find_face_loop_containing_half_edge(
    draft: &SpecDraft,
    face: SpecNodeId,
    half_edge: SpecNodeId,
) -> Result<SpecNodeId, SpecError> {
    let outer_loop = draft.single_outgoing_target(face, RelationKind::FaceOuterLoop)?;
    if loop_contains_half_edge(draft, outer_loop, half_edge)? {
        return Ok(outer_loop);
    }

    for loop_id in draft.outgoing_targets_of_kind(face, RelationKind::FaceInnerLoop) {
        if loop_contains_half_edge(draft, loop_id, half_edge)? {
            return Ok(loop_id);
        }
    }

    Err(SpecError::not_found(format!(
        "halfedge {} not found in any loop of face {}",
        half_edge, face
    )))
}
