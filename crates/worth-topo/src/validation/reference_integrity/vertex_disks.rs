use std::collections::{BTreeMap, BTreeSet};
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
    CustomInvariantRegistration::new(VertexBranchingRule {
        execution_point: InvariantExecutionPoint::GraphComposition,
    })
}

pub(super) fn commit_backstop_registration() -> Result<
    CustomInvariantRegistration,
    forge_relational::facade::runtime::CustomInvariantRegistrationError,
> {
    CustomInvariantRegistration::new(VertexBranchingRule {
        execution_point: InvariantExecutionPoint::CommitBoundary,
    })
}

struct VertexBranchingRule {
    execution_point: InvariantExecutionPoint,
}

impl CustomInvariantRule for VertexBranchingRule {
    type Scope = RuntimeTopologyGraph;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: forge_relational::facade::runtime::CustomInvariantRuleId::new(
                    ".m1.topology.vertex_disks",
                ),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from(" Milestone 1 Vertex Branching"),
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
        let wire_kind = EntityKind::Topology(TopologyEntityKind::Wire).kind_id();
        let owns_halfedge = RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge);
        let start_kind = RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex);
        let end_kind = RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex);
        let edge_kind = RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge);

        for (wire_id, kind_id) in &scope.topology_entities {
            if *kind_id != wire_kind {
                continue;
            }

            let mut vertex_incident_halfedges: BTreeMap<
                RuntimeEntityRef,
                BTreeSet<RuntimeEntityRef>,
            > = BTreeMap::new();
            let mut vertex_incident_edges: BTreeMap<RuntimeEntityRef, BTreeSet<RuntimeEntityRef>> =
                BTreeMap::new();

            for halfedge in scope.outgoing_kind(wire_id, owns_halfedge) {
                let starts = scope.outgoing_kind(&halfedge.target, start_kind);
                let ends = scope.outgoing_kind(&halfedge.target, end_kind);
                let edges = scope.outgoing_kind(&halfedge.target, edge_kind);

                if starts.len() != 1 || ends.len() != 1 || edges.len() != 1 {
                    return Err(CustomInvariantExecutionError::new(format!(
                        "wire-owned halfedge {:?} must have exactly one start vertex, one end vertex, and one edge",
                        halfedge.target
                    )));
                }

                let edge_ref = edges[0].target.clone();
                for vertex_ref in [starts[0].target.clone(), ends[0].target.clone()] {
                    vertex_incident_halfedges
                        .entry(vertex_ref.clone())
                        .or_default()
                        .insert(halfedge.target.clone());
                    vertex_incident_edges
                        .entry(vertex_ref)
                        .or_default()
                        .insert(edge_ref.clone());
                }
            }

            for (vertex_ref, incident_halfedges) in vertex_incident_halfedges {
                if incident_halfedges.len() < 3 {
                    continue;
                }
                let distinct_edge_count = vertex_incident_edges
                    .get(&vertex_ref)
                    .map(BTreeSet::len)
                    .unwrap_or_default();
                if distinct_edge_count < 3 {
                    return Err(CustomInvariantExecutionError::new(format!(
                        "wire {:?} branch vertex {:?} has {} incident halfedges but only {} distinct edges",
                        wire_id,
                        vertex_ref,
                        incident_halfedges.len(),
                        distinct_edge_count
                    )));
                }
            }
        }

        Ok(CustomInvariantVerdict::Pass)
    }
}
