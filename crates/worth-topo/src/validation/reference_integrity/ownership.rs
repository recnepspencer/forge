use std::sync::Arc;

use forge_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRule, CustomInvariantScopePlanner,
    CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion, CustomInvariantVerdict,
    InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup,
    InvariantGroupSet,
};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::RelationKind;

use super::shared::RuntimeTopologyGraph;
use super::shared_queries::{kind_name, owner_relation_for_kind};

pub fn registration() -> Result<
    CustomInvariantRegistration,
    forge_relational::facade::runtime::CustomInvariantRegistrationError,
> {
    CustomInvariantRegistration::new(OwnershipSurfaceRule)
}

struct OwnershipSurfaceRule;

impl CustomInvariantRule for OwnershipSurfaceRule {
    type Scope = RuntimeTopologyGraph;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: forge_relational::facade::runtime::CustomInvariantRuleId::new(
                    ".m1.topology.ownership_surface",
                ),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from(" Milestone 1 Ownership Surface"),
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
        for (entity_id, kind_id) in &scope.topology_entities {
            if *kind_id == schema::facade::platform::entities::EntityKind::Topology(TopologyEntityKind::Model).kind_id()
                || *kind_id
                    == schema::facade::platform::entities::EntityKind::Topology(TopologyEntityKind::Wire).kind_id()
                || *kind_id
                    == schema::facade::platform::entities::EntityKind::Topology(TopologyEntityKind::Edge).kind_id()
                || *kind_id
                    == schema::facade::platform::entities::EntityKind::Topology(TopologyEntityKind::Vertex).kind_id()
            {
                continue;
            }

            if let Some(owner_kind) = owner_relation_for_kind(*kind_id) {
                let owners = scope.incoming_kind(entity_id, owner_kind);
                if owners.len() != 1 {
                    return Err(CustomInvariantExecutionError::new(format!(
                        "entity {:?} of kind {} must have exactly one incoming owner relation {:?}, found {}",
                        entity_id,
                        kind_name(*kind_id),
                        owner_kind,
                        owners.len()
                    )));
                }
                continue;
            }

            if *kind_id == schema::facade::platform::entities::EntityKind::Topology(TopologyEntityKind::Loop).kind_id()
            {
                let face_outer = scope.incoming_kind(
                    entity_id,
                    RelationKind::Topology(schema::facade::platform::relations::TopologyRelationKind::FaceOuterLoop),
                );
                let face_inner = scope.incoming_kind(
                    entity_id,
                    RelationKind::Topology(schema::facade::platform::relations::TopologyRelationKind::FaceInnerLoop),
                );
                let owners = face_outer.len() + face_inner.len();
                if owners != 1 {
                    return Err(CustomInvariantExecutionError::new(format!(
                        "loop {:?} must have exactly one owning face loop relation, found {}",
                        entity_id, owners
                    )));
                }
                continue;
            }

            if *kind_id
                == schema::facade::platform::entities::EntityKind::Topology(TopologyEntityKind::HalfEdge).kind_id()
            {
                let loop_owners = scope.incoming_kind(
                    entity_id,
                    RelationKind::Topology(schema::facade::platform::relations::TopologyRelationKind::LoopOwnsHalfEdge),
                );
                let wire_owners = scope.incoming_kind(
                    entity_id,
                    RelationKind::Topology(schema::facade::platform::relations::TopologyRelationKind::WireOwnsHalfEdge),
                );
                if loop_owners.len() != 1 || wire_owners.len() != 1 {
                    return Err(CustomInvariantExecutionError::new(format!(
                        "halfedge {:?} must have exactly one loop owner and one wire owner, found loop={} wire={}",
                        entity_id,
                        loop_owners.len(),
                        wire_owners.len()
                    )));
                }
            }
        }

        Ok(CustomInvariantVerdict::Pass)
    }
}




