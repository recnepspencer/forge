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
    CustomInvariantRegistration::new(LoopWiringRule)
}

struct LoopWiringRule;

impl CustomInvariantRule for LoopWiringRule {
    type Scope = RuntimeTopologyGraph;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: forge_relational::facade::runtime::CustomInvariantRuleId::new(
                    ".m1.topology.loop_wiring",
                ),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from(" Milestone 1 Loop Wiring"),
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
        let next_kind = RelationKind::Topology(TopologyRelationKind::HalfEdgeNext);
        let prev_kind = RelationKind::Topology(TopologyRelationKind::HalfEdgePrev);

        for (entity_id, kind_id) in &scope.topology_entities {
            if *kind_id != halfedge_kind {
                continue;
            }

            let nexts = scope.outgoing_kind(entity_id, next_kind);
            let prevs = scope.outgoing_kind(entity_id, prev_kind);
            if nexts.len() != 1 || prevs.len() != 1 {
                return Err(CustomInvariantExecutionError::new(format!(
                    "halfedge {:?} must have exactly one next and one prev relation, found next={} prev={}",
                    entity_id,
                    nexts.len(),
                    prevs.len()
                )));
            }

            let next_target = nexts[0].target.clone();
            let prev_target = prevs[0].target.clone();
            let next_back = scope.outgoing_kind(&next_target, prev_kind);
            let prev_forward = scope.outgoing_kind(&prev_target, next_kind);
            if next_back.len() != 1 || next_back[0].target != entity_id.clone() {
                let observed_prev_targets = next_back
                    .iter()
                    .map(|record| record.target.clone())
                    .collect::<Vec<_>>();
                let planned_prev_targets = scope
                    .planned_outgoing_kind(&next_target, prev_kind)
                    .iter()
                    .map(|record| record.target.clone())
                    .collect::<Vec<_>>();
                let planned_prev_updates = scope
                    .planned_kind(prev_kind)
                    .iter()
                    .map(|record| format!("{:?}->{:?}", record.source, record.target))
                    .collect::<Vec<_>>();
                return Err(CustomInvariantExecutionError::new(format!(
                    "halfedge {:?} next/prev symmetry is broken at {:?}; observed prev targets: {:?}; planned prev targets: {:?}; all planned prev updates: {:?}",
                    entity_id, next_target, observed_prev_targets, planned_prev_targets, planned_prev_updates
                )));
            }
            if prev_forward.len() != 1 || prev_forward[0].target != entity_id.clone() {
                let observed_next_targets = prev_forward
                    .iter()
                    .map(|record| record.target.clone())
                    .collect::<Vec<_>>();
                let planned_next_targets = scope
                    .planned_outgoing_kind(&prev_target, next_kind)
                    .iter()
                    .map(|record| record.target.clone())
                    .collect::<Vec<_>>();
                let planned_next_updates = scope
                    .planned_kind(next_kind)
                    .iter()
                    .map(|record| format!("{:?}->{:?}", record.source, record.target))
                    .collect::<Vec<_>>();
                return Err(CustomInvariantExecutionError::new(format!(
                    "halfedge {:?} prev/next symmetry is broken at {:?}; observed next targets: {:?}; planned next targets: {:?}; all planned next updates: {:?}",
                    entity_id, prev_target, observed_next_targets, planned_next_targets, planned_next_updates
                )));
            }
        }

        Ok(CustomInvariantVerdict::Pass)
    }
}




