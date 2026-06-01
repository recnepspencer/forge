use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};
use forge_relational::facade::identity::{EntityId, RelationId};

use crate::query_domain::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
use crate::topology_operators::{
    TopologyDeclaredMutationSequence, TopologyDeclaredMutationSequenceBuilder,
};

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

    pub(crate) fn declared_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder.splice_radial_adjacency(
            self.relation_id,
            self.half_edge_id,
            self.radial_next_half_edge_id,
        );
        builder.finish()
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
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

    use super::TopologySpliceRadialAdjacencyDeclaration;
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::TopologyDeclaredMutationSequenceBuilder;

    #[test]
    fn declaration_reauthors_to_expected_splice_radial_mutation_sequence() {
        let declaration = TopologySpliceRadialAdjacencyDeclaration::new(
            RelationId::new(PartitionId::main(), 7, 1),
            EntityId::new(PartitionId::main(), 8, 1),
            EntityId::new(PartitionId::main(), 9, 1),
        );
        let actual_contracts = declaration
            .into_mutation_sequence()
            .members()
            .map(|member| member.record().clone())
            .collect::<Vec<_>>();
        let mut expected = TopologyDeclaredMutationSequenceBuilder::builder();
        expected.splice_radial_adjacency(
            RelationId::new(PartitionId::main(), 7, 1),
            EntityId::new(PartitionId::main(), 8, 1),
            EntityId::new(PartitionId::main(), 9, 1),
        );

        assert_eq!(
            actual_contracts,
            expected
                .finish()
                .members()
                .map(|member| member.record().clone())
                .collect::<Vec<_>>()
        );
    }
}
