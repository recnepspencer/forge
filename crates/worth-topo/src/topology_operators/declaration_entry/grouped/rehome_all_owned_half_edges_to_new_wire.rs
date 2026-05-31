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
pub struct TopologyWireRehomeHalfEdgeMember {
    relation_create_key: String,
    half_edge_id: EntityId,
}

impl TopologyWireRehomeHalfEdgeMember {
    pub fn new(relation_create_key: impl Into<String>, half_edge_id: EntityId) -> Self {
        Self {
            relation_create_key: relation_create_key.into(),
            half_edge_id,
        }
    }

    pub fn relation_create_key(&self) -> &str {
        &self.relation_create_key
    }

    pub fn half_edge_id(&self) -> EntityId {
        self.half_edge_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration {
    wire_create_key: String,
    retired_wire_id: EntityId,
    members: Vec<TopologyWireRehomeHalfEdgeMember>,
}

impl TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration {
    pub fn new(
        wire_create_key: impl Into<String>,
        retired_wire_id: EntityId,
        members: Vec<TopologyWireRehomeHalfEdgeMember>,
    ) -> Self {
        Self {
            wire_create_key: wire_create_key.into(),
            retired_wire_id,
            members,
        }
    }

    pub fn wire_create_key(&self) -> &str {
        &self.wire_create_key
    }

    pub fn retired_wire_id(&self) -> EntityId {
        self.retired_wire_id
    }

    pub fn members(&self) -> &[TopologyWireRehomeHalfEdgeMember] {
        &self.members
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
        builder.retire_topology_entity(self.retired_wire_id, TopologyEntityKind::Wire);
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyRehomeAllOwnedHalfEdgesToNewWireFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologyRehomeAllOwnedHalfEdgesToNewWireFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.rehome_all_owned_half_edges_to_new_wire"
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
    for TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration
{
    type Family = TopologyRehomeAllOwnedHalfEdgesToNewWireFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.rehome_all_owned_half_edges_to_new_wire".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.rehome_all_owned_half_edges_to_new_wire.wire_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.wire_create_key.clone()),
            ),
            canonical_entity_reference_entry(
                "topology.rehome_all_owned_half_edges_to_new_wire.retired_wire",
                &EntityReference::Existing(self.retired_wire_id),
            ),
        ];
        for (index, member) in self.members.iter().enumerate() {
            let prefix = format!(
                "topology.rehome_all_owned_half_edges_to_new_wire.members.{}",
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
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyWireRehomeHalfEdgeMember,
    };
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::TopologyDeclaredMutationSequenceBuilder;

    #[test]
    fn declaration_reauthors_to_the_expected_wire_rehome_mutation_sequence() {
        let declaration = TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
            "query-native.rehome-wire.new-wire",
            EntityId::new(PartitionId::main(), 5, 1),
            vec![
                TopologyWireRehomeHalfEdgeMember::new(
                    "query-native.rehome-wire.member-1",
                    EntityId::new(PartitionId::main(), 10, 1),
                ),
                TopologyWireRehomeHalfEdgeMember::new(
                    "query-native.rehome-wire.member-2",
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
            .create_topology_entity(
                "query-native.rehome-wire.new-wire",
                TopologyEntityKind::Wire,
            )
            .attach_shell_or_wire_membership(
                "query-native.rehome-wire.member-1",
                crate::topology_operators::ShellOrWireMembershipKind::WireOwnsHalfEdge,
                schema::facade::topology_authoring::created_ref(
                    "query-native.rehome-wire.new-wire",
                ),
                EntityId::new(PartitionId::main(), 10, 1),
            )
            .attach_shell_or_wire_membership(
                "query-native.rehome-wire.member-2",
                crate::topology_operators::ShellOrWireMembershipKind::WireOwnsHalfEdge,
                schema::facade::topology_authoring::created_ref(
                    "query-native.rehome-wire.new-wire",
                ),
                EntityId::new(PartitionId::main(), 11, 1),
            )
            .retire_topology_entity(
                EntityId::new(PartitionId::main(), 5, 1),
                TopologyEntityKind::Wire,
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
