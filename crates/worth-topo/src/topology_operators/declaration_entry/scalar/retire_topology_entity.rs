use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};
use forge_relational::facade::identity::EntityId;
use schema::facade::platform::entities::TopologyEntityKind;

use crate::facade::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
#[cfg(test)]
use crate::topology_operators::TopologyEditAction;
#[cfg(test)]
use crate::topology_operators::TopologyEditBatch;
use crate::topology_operators::TopologyEditContract;

use super::super::shared::canonical_entity_id;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyRetireTopologyEntityDeclaration {
    entity_id: EntityId,
    kind: TopologyEntityKind,
}

impl TopologyRetireTopologyEntityDeclaration {
    pub fn new(entity_id: EntityId, kind: TopologyEntityKind) -> Self {
        Self { entity_id, kind }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn kind(&self) -> TopologyEntityKind {
        self.kind
    }

    pub(crate) fn into_contracts(self) -> Vec<TopologyEditContract> {
        vec![TopologyEditContract::retire_topology_entity(
            self.entity_id,
            self.kind,
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyRetireTopologyEntityFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain> for TopologyRetireTopologyEntityFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.retire_topology_entity"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::IdentityEvolution,
        ]
    }

    fn required_config_sections() -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Relational]
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        if operating_context_identity_digest == TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY {
            ForgeQueryDeclarationProgressionContract::rebind_required()
        } else {
            ForgeQueryDeclarationProgressionContract::admitted_current()
        }
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

impl ForgeQueryDeclarationInput<TopologyQueryDomain> for TopologyRetireTopologyEntityDeclaration {
    type Family = TopologyRetireTopologyEntityFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.retire_topology_entity".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.retire_topology_entity.entity_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(self.entity_id)),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.retire_topology_entity.kind",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.kind.kind_name().to_string()),
            ),
        ]
    }
}

#[cfg(test)]
pub(crate) fn declaration_for_canonical_single_retire_batch(
    batch: &TopologyEditBatch,
) -> Option<TopologyRetireTopologyEntityDeclaration> {
    let [contract] = batch.contracts() else {
        return None;
    };
    let TopologyEditAction::RetireTopologyEntity { entity_id, kind } = contract.action else {
        return None;
    };
    let declaration = TopologyRetireTopologyEntityDeclaration::new(entity_id, kind);
    let canonical_batch = TopologyEditBatch::new(declaration.clone().into_contracts())
        .expect("single retire declaration should always form a non-empty batch");
    (batch == &canonical_batch).then_some(declaration)
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::{
        declaration_for_canonical_single_retire_batch, TopologyRetireTopologyEntityDeclaration,
    };
    use crate::topology_operators::{
        TopologyEditBatch, TopologyEditContract, TopologyEditDerivedFallbackPolicy,
    };

    #[test]
    fn canonical_single_retire_batch_promotes_to_query_declaration() {
        let batch = TopologyEditBatch::new(vec![TopologyEditContract::retire_topology_entity(
            EntityId::new(PartitionId::main(), 7, 1),
            TopologyEntityKind::Vertex,
        )])
        .expect("retire batch should be non-empty");

        let declaration = declaration_for_canonical_single_retire_batch(&batch)
            .expect("canonical retire batch should promote");

        assert_eq!(
            declaration,
            TopologyRetireTopologyEntityDeclaration::new(
                EntityId::new(PartitionId::main(), 7, 1),
                TopologyEntityKind::Vertex,
            )
        );
    }

    #[test]
    fn non_canonical_retire_batch_stays_off_query_declaration_promotion() {
        let batch = TopologyEditBatch::new(vec![TopologyEditContract::retire_topology_entity(
            EntityId::new(PartitionId::main(), 7, 1),
            TopologyEntityKind::Vertex,
        )
        .with_derived_fallback_policy(TopologyEditDerivedFallbackPolicy::RejectAnyFallback)])
        .expect("retire batch should be non-empty");

        assert!(declaration_for_canonical_single_retire_batch(&batch).is_none());
    }
}
