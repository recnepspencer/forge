use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
};
use forge_relational::facade::identity::EntityId;
use schema::facade::platform::authority::{CreateKey, EntityReference};
use schema::facade::platform::entities::TopologyEntityKind;

use super::super::shared::canonical_entity_reference_entry;
use crate::facade::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyDeclaredMutationSequence,
    TopologyDeclaredMutationSequenceBuilder,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyWireSplitHalfEdgeMember {
    relation_create_key: String,
    half_edge_id: EntityId,
}

impl TopologyWireSplitHalfEdgeMember {
    pub fn new(relation_create_key: impl Into<String>, half_edge_id: EntityId) -> Self {
        Self {
            relation_create_key: relation_create_key.into(),
            half_edge_id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologySplitConnectedHalfEdgeSetToNewWireDeclaration {
    wire_create_key: String,
    members: Vec<TopologyWireSplitHalfEdgeMember>,
}

impl TopologySplitConnectedHalfEdgeSetToNewWireDeclaration {
    pub fn new(
        wire_create_key: impl Into<String>,
        members: Vec<TopologyWireSplitHalfEdgeMember>,
    ) -> Self {
        Self {
            wire_create_key: wire_create_key.into(),
            members,
        }
    }

    pub(crate) fn declared_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder.create_topology_entity(self.wire_create_key.clone(), TopologyEntityKind::Wire);
        for member in self.members {
            builder.attach_shell_or_wire_membership(
                member.relation_create_key,
                ShellOrWireMembershipKind::WireOwnsHalfEdge,
                EntityReference::Created(CreateKey::new(self.wire_create_key.clone())),
                member.half_edge_id,
            );
        }
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologySplitConnectedHalfEdgeSetToNewWireFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologySplitConnectedHalfEdgeSetToNewWireFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.split_connected_half_edge_set_to_new_wire"
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

impl ForgeQueryDeclarationInput<TopologyQueryDomain>
    for TopologySplitConnectedHalfEdgeSetToNewWireDeclaration
{
    type Family = TopologySplitConnectedHalfEdgeSetToNewWireFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.split_connected_half_edge_set_to_new_wire".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.split_connected_half_edge_set_to_new_wire.wire_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.wire_create_key.clone()),
            ),
        ];
        for (index, member) in self.members.iter().enumerate() {
            let prefix = format!(
                "topology.split_connected_half_edge_set_to_new_wire.members.{}",
                index + 1
            );
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.relation_create_key"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(member.relation_create_key.clone()),
            ));
            entries.push(canonical_entity_reference_entry(
                format!("{prefix}.half_edge"),
                &EntityReference::Existing(member.half_edge_id),
            ));
        }
        entries
    }
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::{
        TopologySplitConnectedHalfEdgeSetToNewWireDeclaration, TopologyWireSplitHalfEdgeMember,
    };
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::TopologyDeclaredMutationSequenceBuilder;

    #[test]
    fn declaration_reauthors_to_the_expected_wire_split_mutation_sequence() {
        let declaration = TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
            "query-native.split-wire.new-wire",
            vec![
                TopologyWireSplitHalfEdgeMember::new(
                    "query-native.split-wire.member-1",
                    EntityId::new(PartitionId::main(), 10, 1),
                ),
                TopologyWireSplitHalfEdgeMember::new(
                    "query-native.split-wire.member-2",
                    EntityId::new(PartitionId::main(), 11, 1),
                ),
            ],
        );
        let sequence = declaration.into_mutation_sequence();
        let actual_contracts = sequence
            .members()
            .map(|member| member.record().clone())
            .collect::<Vec<_>>();
        let mut expected = TopologyDeclaredMutationSequenceBuilder::builder();
        expected
            .create_topology_entity("query-native.split-wire.new-wire", TopologyEntityKind::Wire)
            .attach_shell_or_wire_membership(
                "query-native.split-wire.member-1",
                crate::topology_operators::ShellOrWireMembershipKind::WireOwnsHalfEdge,
                schema::facade::topology_authoring::created_ref("query-native.split-wire.new-wire"),
                EntityId::new(PartitionId::main(), 10, 1),
            )
            .attach_shell_or_wire_membership(
                "query-native.split-wire.member-2",
                crate::topology_operators::ShellOrWireMembershipKind::WireOwnsHalfEdge,
                schema::facade::topology_authoring::created_ref("query-native.split-wire.new-wire"),
                EntityId::new(PartitionId::main(), 11, 1),
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
