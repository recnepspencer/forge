use std::collections::BTreeMap;
use std::sync::Arc;

use forge_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRule, CustomInvariantScopePlanner,
    CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion, CustomInvariantVerdict,
    InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup,
    InvariantGroupSet,
};
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};

use super::shared::{RuntimeEntityRef, RuntimeTopologyGraph};

pub(super) fn graph_composition_registration() -> Result<
    CustomInvariantRegistration,
    forge_relational::facade::runtime::CustomInvariantRegistrationError,
> {
    CustomInvariantRegistration::new(ShellClosureRule {
        execution_point: InvariantExecutionPoint::GraphComposition,
    })
}

pub(super) fn commit_backstop_registration() -> Result<
    CustomInvariantRegistration,
    forge_relational::facade::runtime::CustomInvariantRegistrationError,
> {
    CustomInvariantRegistration::new(ShellClosureRule {
        execution_point: InvariantExecutionPoint::CommitBoundary,
    })
}

struct ShellClosureRule {
    execution_point: InvariantExecutionPoint,
}

impl CustomInvariantRule for ShellClosureRule {
    type Scope = RuntimeTopologyGraph;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: forge_relational::facade::runtime::CustomInvariantRuleId::new(
                    ".m1.topology.shell_closure",
                ),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from(" Milestone 1 Shell Closure"),
            operational: CustomInvariantOperationalMetadata {
                execution_point: self.execution_point,
                groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                cost_class: InvariantCostClass::Touched,
                failure_effect: InvariantFailureEffect::BlockCommit,
            },
        }
    }

    fn prepare_scope(
        &self,
        planner: &mut CustomInvariantScopePlanner<'_>,
    ) -> Result<Self::Scope, CustomInvariantPreparationError> {
        Ok(RuntimeTopologyGraph::from_planner(planner))
    }

    fn evaluate(
        &self,
        _context: &CustomInvariantExecutionContext<'_>,
        scope: &Self::Scope,
    ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
        let shell_kind = EntityKind::Topology(TopologyEntityKind::Shell).kind_id();
        let shell_owns_face = RelationKind::Topology(TopologyRelationKind::ShellOwnsFace);
        let face_outer = RelationKind::Topology(TopologyRelationKind::FaceOuterLoop);
        let face_inner = RelationKind::Topology(TopologyRelationKind::FaceInnerLoop);
        let loop_owns_halfedge = RelationKind::Topology(TopologyRelationKind::LoopOwnsHalfEdge);
        let halfedge_uses_edge = RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge);

        for (shell_id, kind_id) in &scope.topology_entities {
            if *kind_id != shell_kind {
                continue;
            }

            let face_records = scope.outgoing_kind(shell_id, shell_owns_face);
            if face_records.is_empty() {
                return Err(CustomInvariantExecutionError::new(format!(
                    "shell {:?} must own at least one face",
                    shell_id
                )));
            }

            let mut edge_counts: BTreeMap<RuntimeEntityRef, usize> = BTreeMap::new();
            for face in &face_records {
                let mut loop_ids = scope
                    .outgoing_kind(&face.target, face_outer)
                    .into_iter()
                    .map(|record| record.target)
                    .collect::<Vec<_>>();
                loop_ids.extend(
                    scope
                        .outgoing_kind(&face.target, face_inner)
                        .into_iter()
                        .map(|record| record.target),
                );
                for loop_id in loop_ids {
                    for halfedge in scope.outgoing_kind(&loop_id, loop_owns_halfedge) {
                        let edges = scope.outgoing_kind(&halfedge.target, halfedge_uses_edge);
                        if edges.len() != 1 {
                            return Err(CustomInvariantExecutionError::new(format!(
                                "shell {:?} halfedge {:?} must resolve to exactly one edge",
                                shell_id, halfedge.target
                            )));
                        }
                        *edge_counts.entry(edges[0].target.clone()).or_insert(0) += 1;
                    }
                }
            }

            if edge_counts.is_empty() {
                return Err(CustomInvariantExecutionError::new(format!(
                    "shell {:?} must expose at least one boundary edge incidence",
                    shell_id
                )));
            }

            let boundary_edge_count = edge_counts.values().filter(|count| **count == 1).count();
            if boundary_edge_count == 0 {
                if face_records.len() < 4 || edge_counts.values().any(|count| *count != 2) {
                    return Err(CustomInvariantExecutionError::new(format!(
                        "shell {:?} is boundary-free and must therefore satisfy milestone-1 closed solid-shell requirements",
                        shell_id
                    )));
                }
            }
        }

        Ok(CustomInvariantVerdict::Pass)
    }
}
