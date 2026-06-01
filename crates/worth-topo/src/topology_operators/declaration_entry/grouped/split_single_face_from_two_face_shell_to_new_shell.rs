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
use crate::query_domain::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
use crate::topology_operators::{
    ShellOrWireMembershipKind, TopologyDeclaredMutationSequence,
    TopologyDeclaredMutationSequenceBuilder,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration {
    shell_create_key: String,
    region_relation_create_key: String,
    face_relation_create_key: String,
    region_id: EntityId,
    face_id: EntityId,
}

impl TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration {
    pub fn new(
        shell_create_key: impl Into<String>,
        region_relation_create_key: impl Into<String>,
        face_relation_create_key: impl Into<String>,
        region_id: EntityId,
        face_id: EntityId,
    ) -> Self {
        Self {
            shell_create_key: shell_create_key.into(),
            region_relation_create_key: region_relation_create_key.into(),
            face_relation_create_key: face_relation_create_key.into(),
            region_id,
            face_id,
        }
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
        builder.attach_shell_or_wire_membership(
            self.face_relation_create_key,
            ShellOrWireMembershipKind::ShellOwnsFace,
            EntityReference::Created(CreateKey::new(self.shell_create_key)),
            self.face_id,
        );
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologySplitSingleFaceFromTwoFaceShellToNewShellFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologySplitSingleFaceFromTwoFaceShellToNewShellFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.split_single_face_from_two_face_shell_to_new_shell"
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
    for TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration
{
    type Family = TopologySplitSingleFaceFromTwoFaceShellToNewShellFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.split_single_face_from_two_face_shell_to_new_shell".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.split_single_face_from_two_face_shell_to_new_shell.shell_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.shell_create_key.clone()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.split_single_face_from_two_face_shell_to_new_shell.region_relation_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    self.region_relation_create_key.clone(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.split_single_face_from_two_face_shell_to_new_shell.face_relation_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.face_relation_create_key.clone()),
            ),
            canonical_entity_reference_entry(
                "topology.split_single_face_from_two_face_shell_to_new_shell.region",
                &EntityReference::Existing(self.region_id),
            ),
            canonical_entity_reference_entry(
                "topology.split_single_face_from_two_face_shell_to_new_shell.face",
                &EntityReference::Existing(self.face_id),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration;
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::TopologyDeclaredMutationSequenceBuilder;

    #[test]
    fn declaration_reauthors_to_the_expected_shell_split_mutation_sequence() {
        let declaration = TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
            "query-native.split-shell.new-shell",
            "query-native.split-shell.region-member",
            "query-native.split-shell.face-member",
            EntityId::new(PartitionId::main(), 2, 1),
            EntityId::new(PartitionId::main(), 8, 1),
        );
        let sequence = declaration.into_mutation_sequence();
        let actual_contracts = sequence
            .members()
            .map(|member| member.record().clone())
            .collect::<Vec<_>>();
        let mut expected = TopologyDeclaredMutationSequenceBuilder::builder();
        expected
            .create_topology_entity(
                "query-native.split-shell.new-shell",
                TopologyEntityKind::Shell,
            )
            .attach_shell_or_wire_membership(
                "query-native.split-shell.region-member",
                crate::topology_operators::ShellOrWireMembershipKind::RegionOwnsShell,
                EntityId::new(PartitionId::main(), 2, 1),
                schema::facade::topology_authoring::created_ref(
                    "query-native.split-shell.new-shell",
                ),
            )
            .attach_shell_or_wire_membership(
                "query-native.split-shell.face-member",
                crate::topology_operators::ShellOrWireMembershipKind::ShellOwnsFace,
                schema::facade::topology_authoring::created_ref(
                    "query-native.split-shell.new-shell",
                ),
                EntityId::new(PartitionId::main(), 8, 1),
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
