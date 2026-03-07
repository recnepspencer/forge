use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::RelationKind;
use crate::logic::transaction::SpecDraft;

pub fn collect_radial_ring(
    draft: &SpecDraft,
    half_edge: SpecNodeId,
) -> Result<Vec<SpecNodeId>, SpecError> {
    let start = half_edge;
    let mut ring = Vec::new();
    let mut current = start;
    let max_steps = draft.current_node_count().max(1);

    for _ in 0..max_steps {
        ring.push(current);
        let next = draft.single_outgoing_target(current, RelationKind::HalfEdgeRadialNext)?;
        if next == start {
            return Ok(ring);
        }
        current = next;
    }

    Err(SpecError::invalid(format!(
        "radial ring at halfedge {} does not close within {} traversal steps",
        half_edge, max_steps
    )))
}

pub fn find_previous_radial(
    draft: &SpecDraft,
    half_edge: SpecNodeId,
) -> Result<SpecNodeId, SpecError> {
    let ring = collect_radial_ring(draft, half_edge)?;
    ring.iter()
        .copied()
        .find(|candidate| {
            draft
                .single_outgoing_target(*candidate, RelationKind::HalfEdgeRadialNext)
                .map(|next| next == half_edge)
                .unwrap_or(false)
        })
        .ok_or_else(|| {
            SpecError::not_found(format!(
                "could not find previous radial halfedge for {}",
                half_edge
            ))
        })
}
