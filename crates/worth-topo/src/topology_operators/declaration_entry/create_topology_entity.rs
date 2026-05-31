use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};
use schema::facade::platform::entities::TopologyEntityKind;

use crate::facade::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
#[cfg(test)]
use crate::topology_operators::TopologyEditAction;
use crate::topology_operators::TopologyEditContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyCreateTopologyEntityDeclaration {
    create_key: String,
    kind: TopologyEntityKind,
}

impl TopologyCreateTopologyEntityDeclaration {
    pub fn new(create_key: impl Into<String>, kind: TopologyEntityKind) -> Self {
        Self {
            create_key: create_key.into(),
            kind,
        }
    }

    pub fn create_key(&self) -> &str {
        &self.create_key
    }

    pub fn kind(&self) -> TopologyEntityKind {
        self.kind
    }

    pub(crate) fn into_contracts(self) -> Vec<TopologyEditContract> {
        vec![TopologyEditContract::create_topology_entity(
            self.create_key,
            self.kind,
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyCreateTopologyEntityFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain> for TopologyCreateTopologyEntityFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.create_topology_entity"
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

impl ForgeQueryDeclarationInput<TopologyQueryDomain> for TopologyCreateTopologyEntityDeclaration {
    type Family = TopologyCreateTopologyEntityFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.create_topology_entity".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.create_topology_entity.create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.create_key.clone()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.create_topology_entity.kind",
                ForgeQueryDeclarationCanonicalEntryKind::Field,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.kind.kind_name().to_string()),
            ),
        ]
    }
}

#[cfg(test)]
pub(crate) fn declaration_for_canonical_single_create_contracts(
    contracts: &[TopologyEditContract],
) -> Option<TopologyCreateTopologyEntityDeclaration> {
    let [contract] = contracts else {
        return None;
    };
    declaration_for_canonical_create_contract(contract)
}

#[cfg(test)]
fn declaration_for_canonical_create_contract(
    contract: &TopologyEditContract,
) -> Option<TopologyCreateTopologyEntityDeclaration> {
    let TopologyEditAction::CreateTopologyEntity {
        create_key, kind, ..
    } = &contract.action
    else {
        return None;
    };
    let declaration =
        TopologyCreateTopologyEntityDeclaration::new(create_key.as_str().to_string(), *kind);
    let canonical_contract = declaration.clone().into_contracts().pop()?;

    (contract == &canonical_contract).then_some(declaration)
}

#[cfg(test)]
mod tests {
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::{
        declaration_for_canonical_single_create_contracts, TopologyCreateTopologyEntityDeclaration,
    };
    use crate::topology_operators::TopologyEditContract;

    #[test]
    fn canonical_single_create_contracts_promote_to_query_declaration() {
        let contracts = vec![TopologyEditContract::create_topology_entity(
            "query-native.create.vertex",
            TopologyEntityKind::Vertex,
        )];

        let declaration = declaration_for_canonical_single_create_contracts(&contracts)
            .expect("canonical single-create contracts should promote");

        assert_eq!(
            declaration,
            TopologyCreateTopologyEntityDeclaration::new(
                "query-native.create.vertex",
                TopologyEntityKind::Vertex,
            )
        );
    }

    #[test]
    fn non_canonical_create_contracts_stay_off_query_declaration_promotion() {
        let contracts = vec![TopologyEditContract::create_topology_entity(
            "query-native.create.vertex",
            TopologyEntityKind::Vertex,
        )
        .with_derived_fallback_policy(
            crate::topology_operators::TopologyEditDerivedFallbackPolicy::RejectAnyFallback,
        )];

        assert!(
            declaration_for_canonical_single_create_contracts(&contracts).is_none(),
            "non-canonical create contract should not be silently re-authored as a query declaration"
        );
    }
}
