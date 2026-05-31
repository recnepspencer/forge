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
pub struct TopologyShellRehomeFaceMember {
    relation_create_key: String,
    face_id: EntityId,
}

impl TopologyShellRehomeFaceMember {
    pub fn new(relation_create_key: impl Into<String>, face_id: EntityId) -> Self {
        Self {
            relation_create_key: relation_create_key.into(),
            face_id,
        }
    }

    pub fn relation_create_key(&self) -> &str {
        &self.relation_create_key
    }

    pub fn face_id(&self) -> EntityId {
        self.face_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyRehomeAllOwnedFacesToNewShellDeclaration {
    shell_create_key: String,
    region_relation_create_key: String,
    region_id: EntityId,
    retired_shell_id: EntityId,
    members: Vec<TopologyShellRehomeFaceMember>,
}

impl TopologyRehomeAllOwnedFacesToNewShellDeclaration {
    pub fn new(
        shell_create_key: impl Into<String>,
        region_relation_create_key: impl Into<String>,
        region_id: EntityId,
        retired_shell_id: EntityId,
        members: Vec<TopologyShellRehomeFaceMember>,
    ) -> Self {
        Self {
            shell_create_key: shell_create_key.into(),
            region_relation_create_key: region_relation_create_key.into(),
            region_id,
            retired_shell_id,
            members,
        }
    }

    pub fn shell_create_key(&self) -> &str {
        &self.shell_create_key
    }

    pub fn region_relation_create_key(&self) -> &str {
        &self.region_relation_create_key
    }

    pub fn region_id(&self) -> EntityId {
        self.region_id
    }

    pub fn retired_shell_id(&self) -> EntityId {
        self.retired_shell_id
    }

    pub fn members(&self) -> &[TopologyShellRehomeFaceMember] {
        &self.members
    }

    pub(crate) fn declared_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder.create_topology_entity(self.shell_create_key.clone(), TopologyEntityKind::Shell);
        builder.attach_shell_or_wire_membership(
            self.region_relation_create_key,
            ShellOrWireMembershipKind::RegionOwnsShell,
            self.region_id,
            EntityReference::Created(CreateKey::new(self.shell_create_key.clone())),
        );
        for member in self.members {
            builder.attach_shell_or_wire_membership(
                member.relation_create_key,
                ShellOrWireMembershipKind::ShellOwnsFace,
                EntityReference::Created(CreateKey::new(self.shell_create_key.clone())),
                member.face_id,
            );
        }
        builder.retire_topology_entity(self.retired_shell_id, TopologyEntityKind::Shell);
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyRehomeAllOwnedFacesToNewShellFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologyRehomeAllOwnedFacesToNewShellFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.rehome_all_owned_faces_to_new_shell"
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
    for TopologyRehomeAllOwnedFacesToNewShellDeclaration
{
    type Family = TopologyRehomeAllOwnedFacesToNewShellFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.rehome_all_owned_faces_to_new_shell".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.rehome_all_owned_faces_to_new_shell.shell_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.shell_create_key.clone()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.rehome_all_owned_faces_to_new_shell.region_relation_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    self.region_relation_create_key.clone(),
                ),
            ),
            canonical_entity_reference_entry(
                "topology.rehome_all_owned_faces_to_new_shell.region",
                &EntityReference::Existing(self.region_id),
            ),
            canonical_entity_reference_entry(
                "topology.rehome_all_owned_faces_to_new_shell.retired_shell",
                &EntityReference::Existing(self.retired_shell_id),
            ),
        ];
        for (index, member) in self.members.iter().enumerate() {
            let prefix = format!(
                "topology.rehome_all_owned_faces_to_new_shell.members.{}",
                index + 1
            );
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.relation_create_key"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(member.relation_create_key.clone()),
            ));
            entries.push(canonical_entity_reference_entry(
                format!("{prefix}.face"),
                &EntityReference::Existing(member.face_id),
            ));
        }
        entries
    }
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::{TopologyRehomeAllOwnedFacesToNewShellDeclaration, TopologyShellRehomeFaceMember};
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::TopologyDeclaredMutationSequenceBuilder;

    #[test]
    fn declaration_reauthors_to_the_expected_shell_rehome_mutation_sequence() {
        let declaration = TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
            "query-native.rehome-shell.new-shell",
            "query-native.rehome-shell.region-member",
            EntityId::new(PartitionId::main(), 3, 1),
            EntityId::new(PartitionId::main(), 4, 1),
            vec![
                TopologyShellRehomeFaceMember::new(
                    "query-native.rehome-shell.face-1",
                    EntityId::new(PartitionId::main(), 10, 1),
                ),
                TopologyShellRehomeFaceMember::new(
                    "query-native.rehome-shell.face-2",
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
                "query-native.rehome-shell.new-shell",
                TopologyEntityKind::Shell,
            )
            .attach_shell_or_wire_membership(
                "query-native.rehome-shell.region-member",
                crate::topology_operators::ShellOrWireMembershipKind::RegionOwnsShell,
                EntityId::new(PartitionId::main(), 3, 1),
                schema::facade::topology_authoring::created_ref(
                    "query-native.rehome-shell.new-shell",
                ),
            )
            .attach_shell_or_wire_membership(
                "query-native.rehome-shell.face-1",
                crate::topology_operators::ShellOrWireMembershipKind::ShellOwnsFace,
                schema::facade::topology_authoring::created_ref(
                    "query-native.rehome-shell.new-shell",
                ),
                EntityId::new(PartitionId::main(), 10, 1),
            )
            .attach_shell_or_wire_membership(
                "query-native.rehome-shell.face-2",
                crate::topology_operators::ShellOrWireMembershipKind::ShellOwnsFace,
                schema::facade::topology_authoring::created_ref(
                    "query-native.rehome-shell.new-shell",
                ),
                EntityId::new(PartitionId::main(), 11, 1),
            )
            .retire_topology_entity(
                EntityId::new(PartitionId::main(), 4, 1),
                TopologyEntityKind::Shell,
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
