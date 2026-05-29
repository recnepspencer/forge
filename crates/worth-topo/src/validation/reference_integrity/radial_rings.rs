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

use super::shared::RuntimeTopologyGraph;

pub fn registration() -> Result<
    CustomInvariantRegistration,
    forge_relational::facade::runtime::CustomInvariantRegistrationError,
> {
    CustomInvariantRegistration::new(RadialSurfaceRule)
}

struct RadialSurfaceRule;

impl CustomInvariantRule for RadialSurfaceRule {
    type Scope = RuntimeTopologyGraph;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: forge_relational::facade::runtime::CustomInvariantRuleId::new(
                    ".m1.topology.radial_surface",
                ),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from(" Milestone 1 Radial Surface"),
            operational: CustomInvariantOperationalMetadata {
                execution_point: InvariantExecutionPoint::CommitBoundary,
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
        let halfedge_kind = EntityKind::Topology(TopologyEntityKind::HalfEdge).kind_id();
        let radial_kind = RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext);
        let edge_kind = RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge);

        for (entity_id, kind_id) in &scope.topology_entities {
            if *kind_id != halfedge_kind {
                continue;
            }

            let radials = scope.outgoing_kind(entity_id, radial_kind);
            let edges = scope.outgoing_kind(entity_id, edge_kind);
            if radials.len() != 1 || edges.len() != 1 {
                return Err(CustomInvariantExecutionError::new(format!(
                    "halfedge {:?} must have exactly one radial-next and one edge relation, found radial={} edge={}",
                    entity_id,
                    radials.len(),
                    edges.len()
                )));
            }

            let radial_target = radials[0].target.clone();
            let radial_edges = scope.outgoing_kind(&radial_target, edge_kind);
            if radial_edges.len() != 1 || radial_edges[0].target != edges[0].target {
                return Err(CustomInvariantExecutionError::new(format!(
                    "halfedge {:?} radial target {:?} must remain on the same edge",
                    entity_id, radial_target
                )));
            }
        }

        Ok(CustomInvariantVerdict::Pass)
    }
}
