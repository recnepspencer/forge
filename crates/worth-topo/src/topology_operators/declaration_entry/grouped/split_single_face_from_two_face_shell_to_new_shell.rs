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
#[cfg(test)]
use crate::topology_operators::application::TopologyQueryBindingIndex;
#[cfg(test)]
use crate::topology_operators::local_rewrites::sheet_wire_laminar::resolve_single_face_two_face_shell_split_program;
#[cfg(test)]
use crate::topology_operators::TopologyEditAction;
use crate::topology_operators::{ShellOrWireMembershipKind, TopologyEditContract};

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

    pub(crate) fn into_contracts(self) -> Vec<TopologyEditContract> {
        vec![
            TopologyEditContract::create_topology_entity(
                self.shell_create_key.clone(),
                TopologyEntityKind::Shell,
            ),
            TopologyEditContract::attach_shell_or_wire_membership(
                self.region_relation_create_key,
                ShellOrWireMembershipKind::RegionOwnsShell,
                self.region_id,
                EntityReference::Created(CreateKey::new(self.shell_create_key.clone())),
            ),
            TopologyEditContract::attach_shell_or_wire_membership(
                self.face_relation_create_key,
                ShellOrWireMembershipKind::ShellOwnsFace,
                EntityReference::Created(CreateKey::new(self.shell_create_key)),
                self.face_id,
            ),
        ]
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
pub(crate) fn declaration_for_canonical_split_single_face_from_two_face_shell_to_new_shell_contracts(
    bindings: &TopologyQueryBindingIndex,
    contracts: &[TopologyEditContract],
) -> Option<TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration> {
    let [create, attach_region, attach_face] = contracts else {
        return None;
    };
    let (
        TopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: TopologyEntityKind::Shell,
            ..
        },
        TopologyEditAction::AttachShellOrWireMembership {
            create_key: region_relation_create_key,
            kind: ShellOrWireMembershipKind::RegionOwnsShell,
            owner: EntityReference::Existing(region_id),
            member: EntityReference::Created(member_key),
        },
        TopologyEditAction::AttachShellOrWireMembership {
            create_key: face_relation_create_key,
            kind: ShellOrWireMembershipKind::ShellOwnsFace,
            owner: EntityReference::Created(owner_key),
            member: EntityReference::Existing(face_id),
        },
    ) = (&create.action, &attach_region.action, &attach_face.action)
    else {
        return None;
    };
    if create_key.as_str() != member_key.as_str() || create_key.as_str() != owner_key.as_str() {
        return None;
    }
    let declaration = TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
        create_key.as_str().to_string(),
        region_relation_create_key.as_str().to_string(),
        face_relation_create_key.as_str().to_string(),
        *region_id,
        *face_id,
    );
    let canonical_contracts = declaration.clone().into_contracts();
    (contracts == canonical_contracts.as_slice()
        && resolve_single_face_two_face_shell_split_program(bindings, contracts).is_some())
    .then_some(declaration)
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::{
        declaration_for_canonical_split_single_face_from_two_face_shell_to_new_shell_contracts,
        TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    };
    use crate::topology_operators::application::TopologyQueryBindingIndex;
    use crate::topology_operators::{TopologyEditContract, TopologyEditDerivedFallbackPolicy};

    #[test]
    fn declaration_reauthors_to_the_expected_shell_split_batch() {
        let declaration = TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration::new(
            "query-native.split-shell.new-shell",
            "query-native.split-shell.region-member",
            "query-native.split-shell.face-member",
            EntityId::new(PartitionId::main(), 2, 1),
            EntityId::new(PartitionId::main(), 8, 1),
        );

        assert_eq!(
            declaration.clone().into_contracts(),
            vec![
                TopologyEditContract::create_topology_entity(
                    "query-native.split-shell.new-shell",
                    TopologyEntityKind::Shell,
                ),
                TopologyEditContract::attach_shell_or_wire_membership(
                    "query-native.split-shell.region-member",
                    crate::topology_operators::ShellOrWireMembershipKind::RegionOwnsShell,
                    EntityId::new(PartitionId::main(), 2, 1),
                    schema::facade::topology_authoring::created_ref(
                        "query-native.split-shell.new-shell",
                    ),
                ),
                TopologyEditContract::attach_shell_or_wire_membership(
                    "query-native.split-shell.face-member",
                    crate::topology_operators::ShellOrWireMembershipKind::ShellOwnsFace,
                    schema::facade::topology_authoring::created_ref(
                        "query-native.split-shell.new-shell",
                    ),
                    EntityId::new(PartitionId::main(), 8, 1),
                ),
            ]
        );
    }

    #[test]
    fn non_canonical_shell_split_contracts_stay_off_query_declaration_promotion() {
        let contracts = vec![
            TopologyEditContract::create_topology_entity(
                "query-native.split-shell.new-shell",
                TopologyEntityKind::Shell,
            ),
            TopologyEditContract::attach_shell_or_wire_membership(
                "query-native.split-shell.region-member",
                crate::topology_operators::ShellOrWireMembershipKind::RegionOwnsShell,
                EntityId::new(PartitionId::main(), 2, 1),
                schema::facade::topology_authoring::created_ref(
                    "query-native.split-shell.new-shell",
                ),
            ),
            TopologyEditContract::attach_shell_or_wire_membership(
                "query-native.split-shell.face-member",
                crate::topology_operators::ShellOrWireMembershipKind::ShellOwnsFace,
                schema::facade::topology_authoring::created_ref(
                    "query-native.split-shell.new-shell",
                ),
                EntityId::new(PartitionId::main(), 8, 1),
            )
            .with_derived_fallback_policy(TopologyEditDerivedFallbackPolicy::RejectAnyFallback),
        ];

        assert!(
            declaration_for_canonical_split_single_face_from_two_face_shell_to_new_shell_contracts(
                &TopologyQueryBindingIndex::default(),
                &contracts,
            )
            .is_none(),
            "non-canonical shell split contracts should not be silently re-authored as a query declaration"
        );
    }
}
