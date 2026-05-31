use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};
use forge_relational::facade::identity::RelationId;

use crate::facade::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
#[cfg(test)]
use crate::topology_operators::TopologyEditAction;
#[cfg(test)]
use crate::topology_operators::TopologyEditBatch;
use crate::topology_operators::TopologyEditContract;

use super::super::shared::canonical_relation_id;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDetachRadialAdjacencyDeclaration {
    relation_id: RelationId,
}

impl TopologyDetachRadialAdjacencyDeclaration {
    pub fn new(relation_id: RelationId) -> Self {
        Self { relation_id }
    }

    pub fn relation_id(&self) -> RelationId {
        self.relation_id
    }

    pub(crate) fn into_contracts(self) -> Vec<TopologyEditContract> {
        vec![TopologyEditContract::detach_radial_adjacency(
            self.relation_id,
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyDetachRadialAdjacencyFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologyDetachRadialAdjacencyFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.detach_radial_adjacency"
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

impl ForgeQueryDeclarationInput<TopologyQueryDomain> for TopologyDetachRadialAdjacencyDeclaration {
    type Family = TopologyDetachRadialAdjacencyFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.detach_radial_adjacency".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.detach_radial_adjacency.relation_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_relation_id(
                    self.relation_id,
                )),
            ),
        ]
    }
}

#[cfg(test)]
pub(crate) fn declaration_for_canonical_single_detach_radial_batch(
    batch: &TopologyEditBatch,
) -> Option<TopologyDetachRadialAdjacencyDeclaration> {
    let [contract] = batch.contracts() else {
        return None;
    };
    let TopologyEditAction::DetachRadialAdjacency { relation_id } = contract.action else {
        return None;
    };
    let declaration = TopologyDetachRadialAdjacencyDeclaration::new(relation_id);
    let canonical_batch = TopologyEditBatch::new(declaration.clone().into_contracts())
        .expect("single detach-radial declaration should always form a non-empty batch");
    (batch == &canonical_batch).then_some(declaration)
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{PartitionId, RelationId};

    use super::{
        declaration_for_canonical_single_detach_radial_batch,
        TopologyDetachRadialAdjacencyDeclaration,
    };
    use crate::topology_operators::{
        TopologyEditBatch, TopologyEditContract, TopologyEditDerivedFallbackPolicy,
    };

    #[test]
    fn canonical_single_detach_radial_batch_promotes_to_query_declaration() {
        let batch = TopologyEditBatch::new(vec![TopologyEditContract::detach_radial_adjacency(
            RelationId::new(PartitionId::main(), 7, 1),
        )])
        .expect("detach radial batch should be non-empty");

        let declaration = declaration_for_canonical_single_detach_radial_batch(&batch)
            .expect("canonical detach-radial batch should promote");

        assert_eq!(
            declaration,
            TopologyDetachRadialAdjacencyDeclaration::new(RelationId::new(
                PartitionId::main(),
                7,
                1,
            ))
        );
    }

    #[test]
    fn non_canonical_detach_radial_batch_stays_off_query_declaration_promotion() {
        let batch = TopologyEditBatch::new(vec![TopologyEditContract::detach_radial_adjacency(
            RelationId::new(PartitionId::main(), 7, 1),
        )
        .with_derived_fallback_policy(TopologyEditDerivedFallbackPolicy::RejectAnyFallback)])
        .expect("detach radial batch should be non-empty");

        assert!(declaration_for_canonical_single_detach_radial_batch(&batch).is_none());
    }
}
