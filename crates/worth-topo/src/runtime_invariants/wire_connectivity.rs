use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use forge_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError, CustomInvariantRegistration,
    CustomInvariantRule, CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
    CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantCostClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup, InvariantGroupSet,
};
use worth_schema::facade::{WorthEntityKind, WorthRelationKind, WorthTopologyEntityKind, WorthTopologyRelationKind};

use super::shared::{connected_components, RuntimeEntityRef, RuntimeTopologyGraph};

pub fn registration() -> Result<CustomInvariantRegistration, forge_relational::facade::runtime::CustomInvariantRegistrationError> {
    CustomInvariantRegistration::new(WireConnectivityRule)
}

struct WireConnectivityRule;

impl CustomInvariantRule for WireConnectivityRule {
    type Scope = RuntimeTopologyGraph;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: forge_relational::facade::runtime::CustomInvariantRuleId::new(
                    "worth.m1.topology.wire_connectivity",
                ),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from("Worth Milestone 1 Wire Connectivity"),
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
        let wire_kind = WorthEntityKind::Topology(WorthTopologyEntityKind::Wire).kind_id();
        let owns_halfedge = WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge);
        let start_kind = WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex);
        let end_kind = WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex);

        for (wire_id, kind_id) in &scope.topology_entities {
            if *kind_id != wire_kind {
                continue;
            }

            let halfedges = scope.outgoing_kind(wire_id, owns_halfedge);
            if halfedges.is_empty() {
                return Err(CustomInvariantExecutionError::new(format!(
                    "wire {:?} must own at least one halfedge",
                    wire_id
                )));
            }

            let mut vertices = BTreeSet::new();
            let mut adjacency: BTreeMap<RuntimeEntityRef, BTreeSet<RuntimeEntityRef>> =
                BTreeMap::new();
            for halfedge in &halfedges {
                let starts = scope.outgoing_kind(&halfedge.target, start_kind);
                let ends = scope.outgoing_kind(&halfedge.target, end_kind);
                if starts.len() != 1 || ends.len() != 1 {
                    return Err(CustomInvariantExecutionError::new(format!(
                        "wire-owned halfedge {:?} must have exactly one start and one end vertex",
                        halfedge.target
                    )));
                }
                let a = starts[0].target.clone();
                let b = ends[0].target.clone();
                vertices.insert(a.clone());
                vertices.insert(b.clone());
                adjacency.entry(a.clone()).or_default().insert(b.clone());
                adjacency.entry(b).or_default().insert(a);
            }

            if connected_components(&vertices, &adjacency) != 1 {
                return Err(CustomInvariantExecutionError::new(format!(
                    "wire {:?} must form one connected vertex graph",
                    wire_id
                )));
            }
        }

        Ok(CustomInvariantVerdict::Pass)
    }
}
