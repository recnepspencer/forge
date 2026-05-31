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
use crate::topology_operators::{ShellOrWireMembershipKind, TopologyEditContract};

use super::super::shared::canonical_relation_id;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyDetachShellOrWireMembershipDeclaration {
    relation_id: RelationId,
    kind: ShellOrWireMembershipKind,
}

impl TopologyDetachShellOrWireMembershipDeclaration {
    pub fn new(relation_id: RelationId, kind: ShellOrWireMembershipKind) -> Self {
        Self { relation_id, kind }
    }

    pub fn relation_id(&self) -> RelationId {
        self.relation_id
    }

    pub fn kind(&self) -> ShellOrWireMembershipKind {
        self.kind
    }

    pub(crate) fn into_contracts(self) -> Vec<TopologyEditContract> {
        vec![TopologyEditContract::detach_shell_or_wire_membership(
            self.relation_id,
            self.kind,
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyDetachShellOrWireMembershipFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologyDetachShellOrWireMembershipFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.detach_shell_or_wire_membership"
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
    for TopologyDetachShellOrWireMembershipDeclaration
{
    type Family = TopologyDetachShellOrWireMembershipFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.detach_shell_or_wire_membership".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.detach_shell_or_wire_membership.relation_id",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_relation_id(
                    self.relation_id,
                )),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.detach_shell_or_wire_membership.kind",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    self.kind.relation_kind().kind_name().to_string(),
                ),
            ),
        ]
    }
}

#[cfg(test)]
pub(crate) fn declaration_for_canonical_single_detach_shell_or_wire_batch(
    batch: &TopologyEditBatch,
) -> Option<TopologyDetachShellOrWireMembershipDeclaration> {
    let [contract] = batch.contracts() else {
        return None;
    };
    let TopologyEditAction::DetachShellOrWireMembership { relation_id, kind } = contract.action
    else {
        return None;
    };
    let declaration = TopologyDetachShellOrWireMembershipDeclaration::new(relation_id, kind);
    let canonical_batch = TopologyEditBatch::new(declaration.clone().into_contracts())
        .expect("single detach-shell-or-wire declaration should always form a non-empty batch");
    (batch == &canonical_batch).then_some(declaration)
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{PartitionId, RelationId};

    use super::{
        declaration_for_canonical_single_detach_shell_or_wire_batch,
        TopologyDetachShellOrWireMembershipDeclaration,
    };
    use crate::topology_operators::{
        ShellOrWireMembershipKind, TopologyEditBatch, TopologyEditContract,
        TopologyEditDerivedFallbackPolicy,
    };

    #[test]
    fn canonical_single_detach_shell_or_wire_batch_promotes_to_query_declaration() {
        let batch =
            TopologyEditBatch::new(vec![TopologyEditContract::detach_shell_or_wire_membership(
                RelationId::new(PartitionId::main(), 7, 1),
                ShellOrWireMembershipKind::WireOwnsHalfEdge,
            )])
            .expect("detach shell-or-wire batch should be non-empty");

        let declaration = declaration_for_canonical_single_detach_shell_or_wire_batch(&batch)
            .expect("canonical detach shell-or-wire batch should promote");

        assert_eq!(
            declaration,
            TopologyDetachShellOrWireMembershipDeclaration::new(
                RelationId::new(PartitionId::main(), 7, 1),
                ShellOrWireMembershipKind::WireOwnsHalfEdge,
            )
        );
    }

    #[test]
    fn non_canonical_detach_shell_or_wire_batch_stays_off_query_declaration_promotion() {
        let batch =
            TopologyEditBatch::new(vec![TopologyEditContract::detach_shell_or_wire_membership(
                RelationId::new(PartitionId::main(), 7, 1),
                ShellOrWireMembershipKind::WireOwnsHalfEdge,
            )
            .with_derived_fallback_policy(
                TopologyEditDerivedFallbackPolicy::RejectAnyFallback,
            )])
            .expect("detach shell-or-wire batch should be non-empty");

        assert!(declaration_for_canonical_single_detach_shell_or_wire_batch(&batch).is_none());
    }
}
