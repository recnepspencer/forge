use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CloseoutDomain;

impl ForgeQueryDomainEntryMarker for CloseoutDomain {
    fn domain_key(&self) -> &'static str {
        "test.platform_entry_closeout"
    }

    fn display_name(&self) -> &'static str {
        "PlatformEntryCloseoutDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CloseoutWorld(pub(super) &'static str);

impl ForgeQueryDomainOperatingContext<CloseoutDomain> for CloseoutWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("platform-entry-closeout.{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct IntentRequiredFamily;

impl ForgeQueryDeclarationFamilyMarker<CloseoutDomain> for IntentRequiredFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "IntentRequiredFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::required_relational_intent()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CloseoutInput {
    edge_ref: &'static str,
    _marker: PhantomData<IntentRequiredFamily>,
}

impl CloseoutInput {
    pub(super) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<CloseoutDomain> for CloseoutInput {
    type Family = IntentRequiredFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

pub(super) fn admitted_handle(
    regime: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<CloseoutDomain, CloseoutWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(CloseoutDomain)
        .with_operating_context(CloseoutWorld(regime))
        .validate()
        .expect("closeout world should validate")
        .admit()
        .expect("closeout world should admit")
}
