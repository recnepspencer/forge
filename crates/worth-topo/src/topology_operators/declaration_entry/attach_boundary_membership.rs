use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};
use schema::facade::platform::authority::EntityReference;

use super::shared::canonical_entity_reference_entry;
use crate::query_domain::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyDeclaredMutationSequence,
    TopologyDeclaredMutationSequenceBuilder,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyAttachBoundaryMembershipDeclaration {
    create_key: String,
    kind: BoundaryMembershipKind,
    owner: EntityReference,
    member: EntityReference,
}

impl TopologyAttachBoundaryMembershipDeclaration {
    pub fn new(
        create_key: impl Into<String>,
        kind: BoundaryMembershipKind,
        owner: impl Into<EntityReference>,
        member: impl Into<EntityReference>,
    ) -> Self {
        Self {
            create_key: create_key.into(),
            kind,
            owner: owner.into(),
            member: member.into(),
        }
    }

    pub fn create_key(&self) -> &str {
        &self.create_key
    }

    pub fn kind(&self) -> BoundaryMembershipKind {
        self.kind
    }

    pub fn owner(&self) -> &EntityReference {
        &self.owner
    }

    pub fn member(&self) -> &EntityReference {
        &self.member
    }

    pub(crate) fn declared_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder.attach_boundary_membership(self.create_key, self.kind, self.owner, self.member);
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyAttachBoundaryMembershipFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologyAttachBoundaryMembershipFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.attach_boundary_membership"
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
    for TopologyAttachBoundaryMembershipDeclaration
{
    type Family = TopologyAttachBoundaryMembershipFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.attach_boundary_membership".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.attach_boundary_membership.create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.create_key.clone()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.attach_boundary_membership.kind",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    self.kind.relation_kind().kind_name().to_string(),
                ),
            ),
            canonical_entity_reference_entry(
                "topology.attach_boundary_membership.owner",
                &self.owner,
            ),
            canonical_entity_reference_entry(
                "topology.attach_boundary_membership.member",
                &self.member,
            ),
        ]
    }
}
