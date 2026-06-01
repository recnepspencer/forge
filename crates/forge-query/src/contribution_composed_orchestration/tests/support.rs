use std::marker::PhantomData;

use forge_foundational::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
};

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
};
use crate::domain_capabilities::ForgeQueryDeclarationBoundContributionTarget;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContributionDomain;

impl ForgeQueryDomainEntryMarker for ContributionDomain {
    fn domain_key(&self) -> &'static str {
        "test.contribution.composed.domain"
    }

    fn display_name(&self) -> &'static str {
        "ContributionComposedDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContributionWorld(pub(super) &'static str);

impl ForgeQueryDomainOperatingContext<ContributionDomain> for ContributionWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("contribution-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContributionFamily;

impl ForgeQueryDeclarationFamilyMarker<ContributionDomain> for ContributionFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ContributionFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.face", "selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.face", "selection.active_face"],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeferredContributionFamily;

impl ForgeQueryDeclarationFamilyMarker<ContributionDomain> for DeferredContributionFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "DeferredContributionFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ContributionFamily::aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ContributionFamily::aspect_coverage()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        ForgeQueryDeclarationProgressionContract::deferred_support()
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

impl ForgeQueryDeclarationInput<ContributionDomain> for ContributionInput {
    type Family = ContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
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

impl ForgeQueryDeclarationInput<ContributionDomain> for DeferredContributionInput {
    type Family = DeferredContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeferredAdmissionContributionFamily;

impl ForgeQueryDeclarationFamilyMarker<ContributionDomain> for DeferredAdmissionContributionFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "DeferredAdmissionContributionFamily"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::DurableArtifacts]
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ContributionFamily::aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ContributionFamily::aspect_coverage()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
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

impl ForgeQueryDeclarationInput<ContributionDomain> for DeferredAdmissionContributionInput {
    type Family = DeferredAdmissionContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

pub(super) fn admitted_handle() -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContributionDomain,
    ContributionWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionDomain)
        .with_operating_context(ContributionWorld("main"))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

pub(super) fn target_for_envelope(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        ContributionDomain,
        ContributionWorld,
    >,
    id: &'static str,
) -> ForgeQueryDeclarationBoundContributionTarget {
    let progressed = handle
        .declare_review_and_progress(ContributionInput::new(id))
        .unwrap_or_else(|_| panic!("expected progressed declaration"));
    ForgeQueryDeclarationBoundContributionTarget::for_canonical_declaration(
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
