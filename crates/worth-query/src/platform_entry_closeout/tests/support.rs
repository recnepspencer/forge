use std::marker::PhantomData;

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CloseoutDomain;

impl WorthQueryDomainEntryMarker for CloseoutDomain {
    fn domain_key(&self) -> &'static str {
        "test.platform_entry_closeout"
    }

    fn display_name(&self) -> &'static str {
        "PlatformEntryCloseoutDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CloseoutWorld(pub(super) &'static str);

impl WorthQueryDomainOperatingContext<CloseoutDomain> for CloseoutWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("platform-entry-closeout.{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct IntentRequiredFamily;

impl WorthQueryDeclarationFamilyMarker<CloseoutDomain> for IntentRequiredFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "IntentRequiredFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::required_relational_intent()
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

impl WorthQueryDeclarationInput<CloseoutDomain> for CloseoutInput {
    type Family = IntentRequiredFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

pub(super) fn admitted_handle(
    regime: &'static str,
) -> crate::application::WorthQueryAdmittedConfiguredDomainHandle<CloseoutDomain, CloseoutWorld> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(CloseoutDomain)
        .with_operating_context(CloseoutWorld(regime))
        .validate()
        .expect("closeout world should validate")
        .admit()
        .expect("closeout world should admit")
}
