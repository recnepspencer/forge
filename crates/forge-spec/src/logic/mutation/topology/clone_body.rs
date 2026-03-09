use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::data::error::SpecError;
use crate::data::identity::SpecNodeId;
use crate::data::schema::{RelationKind, SpecNodeKind};
use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
use crate::logic::transaction::SpecDraft;

pub struct CloneBodyMutation {
    pub body: SpecNodeId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloneBodyOutput {
    pub cloned_body: SpecNodeId,
}

impl std::fmt::Debug for CloneBodyMutation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CloneBodyMutation")
            .field("body", &self.body)
            .finish()
    }
}

impl SpecMutation for CloneBodyMutation {
    type Output = CloneBodyOutput;

    const NAME: &'static str = "clone_body";

    fn execute(
        &self,
        draft: &mut SpecDraft,
        _recorder: &mut SpecLineageRecorder,
    ) -> Result<MutationResult<Self::Output>, SpecError> {
        if draft.node_kind(self.body)? != SpecNodeKind::Body {
            return Err(SpecError::invalid(format!(
                "CloneBodyMutation requires Body input, got {:?}",
                draft.node_kind(self.body)?
            )));
        }

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::from([self.body]);
        let mut relations = Vec::new();

        while let Some(source) = queue.pop_front() {
            if !visited.insert(source) {
                continue;
            }

            for relation in draft.outgoing_relations(source) {
                if is_cloneable_topology_relation(relation.kind) {
                    relations.push(relation.clone());
                    if !visited.contains(&relation.target) {
                        queue.push_back(relation.target);
                    }
                } else if is_uncloned_binding_relation(relation.kind) {
                    return Err(SpecError::invalid(format!(
                        "CloneBodyMutation does not yet support cloning {:?} relations",
                        relation.kind
                    )));
                }
            }
        }

        let mut node_map = BTreeMap::new();
        for old in &visited {
            let kind = draft.node_kind(*old)?;
            let cloned = match kind {
                SpecNodeKind::Shell => {
                    draft.create_shell(draft.shell_kind(*old)?, node_role(kind))?
                }
                _ => draft.create_node(kind, None, node_role(kind))?,
            };
            node_map.insert(*old, cloned);
        }

        relations.sort_by_key(|relation| {
            (
                relation.kind,
                relation.source,
                relation.target,
                relation.ordinal,
                relation.id,
            )
        });
        for relation in relations {
            draft.add_relation(
                relation.kind,
                node_map[&relation.source],
                node_map[&relation.target],
                relation.ordinal,
                relation_role(relation.kind),
            )?;
        }

        Ok(MutationResult {
            value: CloneBodyOutput {
                cloned_body: node_map[&self.body],
            },
            touched_domains: vec![TouchedDomain::Topology],
            mutation_trace: vec![
                format!(
                    "clone body {} into new body {}",
                    self.body, node_map[&self.body]
                ),
                format!(
                    "clone {} topology nodes in induced body subgraph",
                    visited.len()
                ),
            ],
        })
    }

    fn semantic_summary(&self) -> String {
        format!("Clone body {} and its owned topology subgraph", self.body)
    }
}

fn is_cloneable_topology_relation(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::BodyOwnsLump
            | RelationKind::LumpOwnsRegion
            | RelationKind::RegionOwnsShell
            | RelationKind::ShellOwnsFace
            | RelationKind::FaceOuterLoop
            | RelationKind::FaceInnerLoop
            | RelationKind::LoopEntryHalfEdge
            | RelationKind::HalfEdgeNext
            | RelationKind::HalfEdgeRadialNext
            | RelationKind::HalfEdgeUsesEdge
            | RelationKind::HalfEdgeOriginVertex
            | RelationKind::HalfEdgeBoundsFace
    )
}

fn is_uncloned_binding_relation(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::FaceUsesSurfaceBinding
            | RelationKind::EdgeUsesCurveBinding
            | RelationKind::HalfEdgeUsesCoedgeBinding
            | RelationKind::VertexUsesGeometryBinding
    )
}

fn node_role(kind: SpecNodeKind) -> &'static str {
    match kind {
        SpecNodeKind::Model => "clone-model",
        SpecNodeKind::Body => "clone-body",
        SpecNodeKind::Lump => "clone-lump",
        SpecNodeKind::Region => "clone-region",
        SpecNodeKind::Shell => "clone-shell",
        SpecNodeKind::Face => "clone-face",
        SpecNodeKind::Loop => "clone-loop",
        SpecNodeKind::HalfEdge => "clone-half-edge",
        SpecNodeKind::Edge => "clone-edge",
        SpecNodeKind::Vertex => "clone-vertex",
        SpecNodeKind::Feature => "clone-feature",
        SpecNodeKind::Constraint => "clone-constraint",
        SpecNodeKind::Parameter => "clone-parameter",
        SpecNodeKind::SurfaceBinding => "clone-surface-binding",
        SpecNodeKind::CurveBinding => "clone-curve-binding",
        SpecNodeKind::CoedgeBinding => "clone-coedge-binding",
        SpecNodeKind::VertexGeometryBinding => "clone-vertex-geometry-binding",
        SpecNodeKind::NamingAnchor => "clone-naming-anchor",
        SpecNodeKind::LineageAnchor => "clone-lineage-anchor",
        SpecNodeKind::ReplayRecord => "clone-replay-record",
        SpecNodeKind::DesignDecision => "clone-decision",
    }
}

fn relation_role(kind: RelationKind) -> &'static str {
    match kind {
        RelationKind::BodyOwnsLump => "clone-body-lump",
        RelationKind::LumpOwnsRegion => "clone-lump-region",
        RelationKind::RegionOwnsShell => "clone-region-shell",
        RelationKind::ShellOwnsFace => "clone-shell-face",
        RelationKind::FaceOuterLoop => "clone-face-outer-loop",
        RelationKind::FaceInnerLoop => "clone-face-inner-loop",
        RelationKind::LoopEntryHalfEdge => "clone-loop-entry",
        RelationKind::HalfEdgeNext => "clone-half-edge-next",
        RelationKind::HalfEdgeRadialNext => "clone-half-edge-radial",
        RelationKind::HalfEdgeUsesEdge => "clone-half-edge-edge",
        RelationKind::HalfEdgeOriginVertex => "clone-half-edge-origin",
        RelationKind::HalfEdgeBoundsFace => "clone-half-edge-face",
        _ => "clone-relation",
    }
}
