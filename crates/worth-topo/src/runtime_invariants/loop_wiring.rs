use std::sync::Arc;

use forge_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRule, CustomInvariantScopePlanner,
    CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion, CustomInvariantVerdict,
    InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup,
    InvariantGroupSet,
};
use worth_schema::facade::{
    WorthEntityKind, WorthRelationKind, WorthTopologyEntityKind, WorthTopologyRelationKind,
};

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
                    "worth.m1.topology.loop_wiring",
                ),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from("Worth Milestone 1 Loop Wiring"),
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
        let halfedge_kind = WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge).kind_id();
        let next_kind = WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext);
        let prev_kind = WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgePrev);

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
                return Err(CustomInvariantExecutionError::new(format!(
                    "halfedge {:?} next/prev symmetry is broken at {:?}",
                    entity_id, next_target
                )));
            }
            if prev_forward.len() != 1 || prev_forward[0].target != entity_id.clone() {
                return Err(CustomInvariantExecutionError::new(format!(
                    "halfedge {:?} prev/next symmetry is broken at {:?}",
                    entity_id, prev_target
                )));
            }
        }

        Ok(CustomInvariantVerdict::Pass)
    }
}
