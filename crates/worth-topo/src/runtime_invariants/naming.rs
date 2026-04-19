use std::sync::Arc;

use forge_relational::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRule, CustomInvariantScopePlanner,
    CustomInvariantSemanticIdentity, CustomInvariantSemanticVersion, CustomInvariantVerdict,
    InvariantCostClass, InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup,
    InvariantGroupSet,
};

use super::shared::{kind_name, naming_relation_kind, RuntimeTopologyGraph};

pub fn registration() -> Result<
    CustomInvariantRegistration,
    forge_relational::facade::runtime::CustomInvariantRegistrationError,
> {
    CustomInvariantRegistration::new(NamingCoverageRule)
}

struct NamingCoverageRule;

impl CustomInvariantRule for NamingCoverageRule {
    type Scope = RuntimeTopologyGraph;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: forge_relational::facade::runtime::CustomInvariantRuleId::new(
                    "worth.m1.naming.coverage",
                ),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from("Worth Milestone 1 Naming Coverage"),
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
        let name_kind = naming_relation_kind();
        for (entity_id, kind_id) in &scope.topology_entities {
            let incoming = scope.incoming_kind(entity_id, name_kind);
            if incoming.len() != 1 {
                return Err(CustomInvariantExecutionError::new(format!(
                    "topology entity {:?} of kind {} must have exactly one persistent-name attachment, found {}",
                    entity_id,
                    kind_name(*kind_id),
                    incoming.len()
                )));
            }
        }
        Ok(CustomInvariantVerdict::Pass)
    }
}
