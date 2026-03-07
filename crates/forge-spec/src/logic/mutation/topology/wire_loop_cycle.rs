use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::transaction::SpecDraft;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WiredLoopCycle {
    pub loop_id: SpecNodeId,
    pub half_edges: Vec<SpecNodeId>,
    pub edges: Vec<SpecNodeId>,
}

pub fn create_loop_cycle(
    draft: &mut SpecDraft,
    face: SpecNodeId,
    vertices: &[SpecNodeId],
    loop_relation: RelationKind,
    ordinal: u32,
    role_prefix: &str,
) -> Result<WiredLoopCycle, SpecError> {
    if vertices.len() < 3 {
        return Err(SpecError::invalid(format!(
            "{role_prefix} requires at least 3 vertices, got {}",
            vertices.len()
        )));
    }

    for &vertex in vertices {
        if draft.node_kind(vertex)? != SpecNodeKind::Vertex {
            return Err(SpecError::invalid(format!(
                "{role_prefix} requires Vertex inputs, got {:?} for {}",
                draft.node_kind(vertex)?,
                vertex
            )));
        }
    }

    if draft.node_kind(face)? != SpecNodeKind::Face {
        return Err(SpecError::invalid(format!(
            "{role_prefix} requires Face target, got {:?}",
            draft.node_kind(face)?
        )));
    }

    let loop_id = draft.create_node(SpecNodeKind::Loop, None, "loop")?;
    draft.add_relation(
        loop_relation,
        face,
        loop_id,
        ordinal,
        &format!("{role_prefix}-face-loop"),
    )?;

    let mut half_edges = Vec::with_capacity(vertices.len());
    let mut edges = Vec::with_capacity(vertices.len());

    for _ in vertices {
        half_edges.push(draft.create_node(SpecNodeKind::HalfEdge, None, "half_edge")?);
        edges.push(draft.create_node(SpecNodeKind::Edge, None, "edge")?);
    }

    draft.add_relation(
        RelationKind::LoopEntryHalfEdge,
        loop_id,
        half_edges[0],
        0,
        &format!("{role_prefix}-loop-entry"),
    )?;

    for (index, &half_edge) in half_edges.iter().enumerate() {
        let next = half_edges[(index + 1) % half_edges.len()];
        let vertex = vertices[index];
        let edge = edges[index];

        draft.add_relation(
            RelationKind::HalfEdgeNext,
            half_edge,
            next,
            0,
            &format!("{role_prefix}-next-{index}"),
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeRadialNext,
            half_edge,
            half_edge,
            0,
            &format!("{role_prefix}-radial-{index}"),
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeUsesEdge,
            half_edge,
            edge,
            0,
            &format!("{role_prefix}-edge-{index}"),
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeOriginVertex,
            half_edge,
            vertex,
            0,
            &format!("{role_prefix}-origin-{index}"),
        )?;
        draft.add_relation(
            RelationKind::HalfEdgeBoundsFace,
            half_edge,
            face,
            0,
            &format!("{role_prefix}-face-{index}"),
        )?;
    }

    Ok(WiredLoopCycle {
        loop_id,
        half_edges,
        edges,
    })
}
