use std::marker::PhantomData;

use worth_foundational::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
};

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryBridgeContinuationAuthority, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationProgressionContract,
    WorthQueryDeclarationRouteContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping,
};
use crate::domain_capabilities::WorthQueryDeclarationBoundContributionTarget;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContributionDomain;

impl WorthQueryDomainEntryMarker for ContributionDomain {
    fn domain_key(&self) -> &'static str {
        "test.contribution.composed.domain"
    }

    fn display_name(&self) -> &'static str {
        "ContributionComposedDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContributionWorld(pub(super) &'static str);

impl WorthQueryDomainOperatingContext<ContributionDomain> for ContributionWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::HistoricalEvaluation,
            WorthQueryCapabilityFamily::WorkflowOrchestration,
            WorthQueryCapabilityFamily::PreviewSession,
            WorthQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("contribution-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContributionFamily;

impl WorthQueryDeclarationFamilyMarker<ContributionDomain> for ContributionFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ContributionFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.face", "selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &["selection.face", "selection.active_face"],
            &[],
            &[],
        )
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeferredContributionFamily;

impl WorthQueryDeclarationFamilyMarker<ContributionDomain> for DeferredContributionFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "DeferredContributionFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        ContributionFamily::aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        ContributionFamily::aspect_coverage()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> WorthQueryDeclarationProgressionContract {
        WorthQueryDeclarationProgressionContract::deferred_support()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ContributionInput {
    id: &'static str,
    _marker: PhantomData<ContributionFamily>,
}

impl ContributionInput {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationInput<ContributionDomain> for ContributionInput {
    type Family = ContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredContributionInput {
    id: &'static str,
    _marker: PhantomData<DeferredContributionFamily>,
}

impl DeferredContributionInput {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationInput<ContributionDomain> for DeferredContributionInput {
    type Family = DeferredContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeferredAdmissionContributionFamily;

impl WorthQueryDeclarationFamilyMarker<ContributionDomain> for DeferredAdmissionContributionFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "DeferredAdmissionContributionFamily"
    }

    fn required_capability_families() -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::DurableArtifacts]
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        ContributionFamily::aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        ContributionFamily::aspect_coverage()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeferredAdmissionContributionInput {
    id: &'static str,
    _marker: PhantomData<DeferredAdmissionContributionFamily>,
}

impl DeferredAdmissionContributionInput {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationInput<ContributionDomain> for DeferredAdmissionContributionInput {
    type Family = DeferredAdmissionContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

pub(super) fn admitted_handle() -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContributionDomain,
    ContributionWorld,
> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionDomain)
        .with_operating_context(ContributionWorld("main"))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

pub(super) fn target_for_envelope(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<
        ContributionDomain,
        ContributionWorld,
    >,
    id: &'static str,
) -> WorthQueryDeclarationBoundContributionTarget {
    let progressed = handle
        .declare_review_and_progress(ContributionInput::new(id))
        .unwrap_or_else(|_| panic!("expected progressed declaration"));
    WorthQueryDeclarationBoundContributionTarget::for_canonical_declaration(
        progressed.canonical_declaration(),
    )
}

pub(super) fn standard_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::Uncertified,
    })
    .unwrap()
}
