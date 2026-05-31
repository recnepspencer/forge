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
use crate::topology_operators::{
    LoopEndpointKind, TopologyDeclaredMutationSequence, TopologyDeclaredMutationSequenceBuilder,
};

use super::super::shared::{canonical_entity_id, canonical_relation_id};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyRewireLoopEndpointDeclaration {
    relation_id: RelationId,
    endpoint: LoopEndpointKind,
    half_edge_id: EntityId,
    vertex_id: EntityId,
}

impl TopologyRewireLoopEndpointDeclaration {
    pub fn new(
        relation_id: RelationId,
        endpoint: LoopEndpointKind,
        half_edge_id: EntityId,
        vertex_id: EntityId,
    ) -> Self {
        Self {
            relation_id,
            endpoint,
            half_edge_id,
            vertex_id,
        }
    }

    pub fn relation_id(&self) -> RelationId {
        self.relation_id
    }

    pub fn endpoint(&self) -> LoopEndpointKind {
        self.endpoint
    }

    pub fn half_edge_id(&self) -> EntityId {
        self.half_edge_id
    }

    pub fn vertex_id(&self) -> EntityId {
        self.vertex_id
    }

    pub(crate) fn declared_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder.rewire_loop_endpoint(
            self.relation_id,
            self.endpoint,
            self.half_edge_id,
            self.vertex_id,
        );
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyRewireLoopEndpointFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain> for TopologyRewireLoopEndpointFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.rewire_loop_endpoint"
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

impl ForgeQueryDeclarationInput<TopologyQueryDomain> for TopologyRewireLoopEndpointDeclaration {
    type Family = TopologyRewireLoopEndpointFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.rewire_loop_endpoint".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.rewire_loop_endpoint.relation_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_relation_id(
                    self.relation_id,
                )),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.rewire_loop_endpoint.endpoint",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    self.endpoint.relation_kind().kind_name().to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.rewire_loop_endpoint.half_edge_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(
                    self.half_edge_id,
                )),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.rewire_loop_endpoint.vertex_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(self.vertex_id)),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

    use super::TopologyRewireLoopEndpointDeclaration;
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::{LoopEndpointKind, TopologyDeclaredMutationSequenceBuilder};

    #[test]
    fn declaration_reauthors_to_the_expected_rewire_loop_endpoint_mutation_sequence() {
        let declaration = TopologyRewireLoopEndpointDeclaration::new(
            RelationId::new(PartitionId::main(), 7, 1),
            LoopEndpointKind::End,
            EntityId::new(PartitionId::main(), 8, 1),
            EntityId::new(PartitionId::main(), 9, 1),
        );
        let actual_contracts = declaration
            .into_mutation_sequence()
            .members()
            .map(|member| member.record().clone())
            .collect::<Vec<_>>();
        let mut expected = TopologyDeclaredMutationSequenceBuilder::builder();
        expected.rewire_loop_endpoint(
            RelationId::new(PartitionId::main(), 7, 1),
            LoopEndpointKind::End,
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
