use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};
use forge_relational::facade::identity::{EntityId, RelationId};

use crate::facade::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
#[cfg(test)]
use crate::topology_operators::TopologyEditAction;
use crate::topology_operators::TopologyEditContract;

use super::super::shared::{canonical_entity_id, canonical_relation_id};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologySpliceRadialAdjacencyDeclaration {
    relation_id: RelationId,
    half_edge_id: EntityId,
    radial_next_half_edge_id: EntityId,
}

impl TopologySpliceRadialAdjacencyDeclaration {
    pub fn new(
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    ) -> Self {
        Self {
            relation_id,
            half_edge_id,
            radial_next_half_edge_id,
        }
    }

    pub fn relation_id(&self) -> RelationId {
        self.relation_id
    }

    pub fn half_edge_id(&self) -> EntityId {
        self.half_edge_id
    }

    pub fn radial_next_half_edge_id(&self) -> EntityId {
        self.radial_next_half_edge_id
    }

    pub(crate) fn into_contracts(self) -> Vec<TopologyEditContract> {
        vec![TopologyEditContract::splice_radial_adjacency(
            self.relation_id,
            self.half_edge_id,
            self.radial_next_half_edge_id,
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologySpliceRadialAdjacencyFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologySpliceRadialAdjacencyFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.splice_radial_adjacency"
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

impl ForgeQueryDeclarationInput<TopologyQueryDomain> for TopologySpliceRadialAdjacencyDeclaration {
    type Family = TopologySpliceRadialAdjacencyFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.splice_radial_adjacency".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.splice_radial_adjacency.relation_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_relation_id(
                    self.relation_id,
                )),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.splice_radial_adjacency.half_edge_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(
                    self.half_edge_id,
                )),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.splice_radial_adjacency.radial_next_half_edge_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(
                    self.radial_next_half_edge_id,
                )),
            ),
        ]
    }
}

#[cfg(test)]
pub(crate) fn declaration_for_canonical_single_splice_radial_contracts(
    contracts: &[TopologyEditContract],
) -> Option<TopologySpliceRadialAdjacencyDeclaration> {
    let [contract] = contracts else {
        return None;
    };
    let TopologyEditAction::SpliceRadialAdjacency {
        relation_id,
        half_edge_id,
        radial_next_half_edge_id,
    } = contract.action
    else {
        return None;
    };
    let declaration = TopologySpliceRadialAdjacencyDeclaration::new(
        relation_id,
        half_edge_id,
        radial_next_half_edge_id,
    );
    let canonical_contracts = declaration.clone().into_contracts();
    (contracts == canonical_contracts.as_slice()).then_some(declaration)
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

    use super::{
        declaration_for_canonical_single_splice_radial_contracts,
        TopologySpliceRadialAdjacencyDeclaration,
    };
    use crate::topology_operators::{TopologyEditContract, TopologyEditDerivedFallbackPolicy};

    #[test]
    fn canonical_single_splice_radial_contracts_promote_to_query_declaration() {
        let contracts = vec![TopologyEditContract::splice_radial_adjacency(
            RelationId::new(PartitionId::main(), 7, 1),
            EntityId::new(PartitionId::main(), 8, 1),
            EntityId::new(PartitionId::main(), 9, 1),
        )];

        let declaration = declaration_for_canonical_single_splice_radial_contracts(&contracts)
            .expect("canonical radial splice contracts should promote");

        assert_eq!(
            declaration,
            TopologySpliceRadialAdjacencyDeclaration::new(
                RelationId::new(PartitionId::main(), 7, 1),
                EntityId::new(PartitionId::main(), 8, 1),
                EntityId::new(PartitionId::main(), 9, 1),
            )
        );
    }

    #[test]
    fn non_canonical_splice_radial_contracts_stay_off_query_declaration_promotion() {
        let contracts = vec![TopologyEditContract::splice_radial_adjacency(
            RelationId::new(PartitionId::main(), 7, 1),
            EntityId::new(PartitionId::main(), 8, 1),
            EntityId::new(PartitionId::main(), 9, 1),
        )
        .with_derived_fallback_policy(TopologyEditDerivedFallbackPolicy::RejectAnyFallback)];

        assert!(declaration_for_canonical_single_splice_radial_contracts(&contracts).is_none());
    }
}
