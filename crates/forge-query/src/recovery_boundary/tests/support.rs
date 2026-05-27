use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RecoveryDomain;

impl ForgeQueryDomainEntryMarker for RecoveryDomain {
    fn domain_key(&self) -> &'static str {
        "test.recovery.boundary"
    }

    fn display_name(&self) -> &'static str {
        "RecoveryBoundaryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RecoveryWorld {
    regime: &'static str,
}

impl RecoveryWorld {
    pub(super) fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl ForgeQueryDomainOperatingContext<RecoveryDomain> for RecoveryWorld {
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
        format!("recovery.{}", self.regime)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RequiredIntentRouteFamily;

impl ForgeQueryDeclarationFamilyMarker<RecoveryDomain> for RequiredIntentRouteFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "RequiredIntentRouteFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::required_relational_intent()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SignalReceiptFamily;

impl ForgeQueryDeclarationFamilyMarker<RecoveryDomain> for SignalReceiptFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "SignalReceiptFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::signal_only()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RecoveryInput<F> {
    edge_ref: &'static str,
    _marker: std::marker::PhantomData<F>,
}

impl<F> RecoveryInput<F> {
    pub(super) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _marker: std::marker::PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<RecoveryDomain> for RecoveryInput<RequiredIntentRouteFamily> {
    type Family = RequiredIntentRouteFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

impl ForgeQueryDeclarationInput<RecoveryDomain> for RecoveryInput<SignalReceiptFamily> {
    type Family = SignalReceiptFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}

pub(super) fn recovery_admitted_handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<RecoveryDomain, RecoveryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(RecoveryDomain)
        .with_operating_context(RecoveryWorld::named(regime))
        .validate()
        .expect("recovery world should validate")
        .admit()
        .expect("recovery world should admit")
}

pub(super) fn recovery_progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<RecoveryDomain, RecoveryWorld>,
    declaration: RecoveryInput<F>,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<RecoveryDomain, RecoveryInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<RecoveryDomain>,
    RecoveryInput<F>: ForgeQueryDeclarationInput<RecoveryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("recovery progression should admit"))
}

pub(super) fn recovery_foundational<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<RecoveryDomain, RecoveryWorld>,
    progression: crate::application::ForgeQueryAdmittedDeclarationProgression<
        RecoveryDomain,
        RecoveryInput<F>,
    >,
) -> crate::application::ForgeQueryDeclarationFoundationalEvidence<RecoveryDomain, RecoveryInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<RecoveryDomain>,
    RecoveryInput<F>: ForgeQueryDeclarationInput<RecoveryDomain>,
{
    handle
        .describe_foundational(
            crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression,
            ),
        )
        .unwrap_or_else(|_| panic!("recovery foundational evidence should materialize"))
}
