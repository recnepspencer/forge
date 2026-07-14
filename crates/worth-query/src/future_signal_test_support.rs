use std::marker::PhantomData;

use crate::application::{
    review_declaration_legality, worth_query_checked_declaration_signal_compatibility_on_handle,
    WorthQueryApplicationFacade, WorthQueryAsyncDeclarationClause,
    WorthQueryAsyncDeclarationSupport, WorthQueryAsyncFailurePosture,
    WorthQueryAsyncLoadingPosture, WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
    WorthQueryBridgeContinuationAuthority, WorthQueryCapabilityFamily, WorthQueryConfig,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationBridgeContinuationMode,
    WorthQueryDeclarationBridgeContinuationRequest, WorthQueryDeclarationBridgeTruthContext,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationEnvelopeInput,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFoundationalEvidenceInput,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityChecked,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationLegalityInput,
    WorthQueryDeclarationReceiptInput, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationRoutePlanInput, WorthQueryDeclarationSignalCompatibilityChecked,
    WorthQueryDeclarationSignalCompatibilityContract,
    WorthQueryDeclarationSignalCompatibilityInput,
    WorthQueryDeclarationSignalCompatibilitySupportRow,
    WorthQueryDeclarationSignalCompatibilitySupportStatus, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryInstalledDomainDeclarationContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQuerySignalCompatiblePosture,
    WorthQueryTemporalDeclarationClause, WorthQueryTemporalDeclarationSupport,
    WorthQueryTemporalDuration,
};
use crate::runtime::WorthQueryRuntimeFamilySupportStatus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FutureSignalDomain;

impl WorthQueryDomainEntryMarker for FutureSignalDomain {
    fn domain_key(&self) -> &'static str {
        "test.future.signal.domain"
    }

    fn display_name(&self) -> &'static str {
        "FutureSignalDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FutureSignalWorld(pub(crate) &'static str);

impl WorthQueryDomainOperatingContext<FutureSignalDomain> for FutureSignalWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("future-signal-world-{}", self.0)
    }
}

macro_rules! define_family {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

        impl WorthQueryDeclarationFamilyMarker<FutureSignalDomain> for $name {
            type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
            type SignalCompatibility = WorthQuerySignalCompatiblePosture;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                WorthQueryDeclarationRouteContract::bridge_only()
            }

            fn bridge_continuation_contract(
            ) -> Option<crate::application::WorthQueryDeclarationBridgeContinuationContract> {
                Some(
                    crate::application::WorthQueryDeclarationBridgeContinuationContract::runtime_route_current(),
                )
            }

            fn signal_compatibility_contract(
            ) -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
                Some(
                    WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution(),
                )
            }

            fn aspect_contract() -> WorthQueryDeclarationAspectContract {
                WorthQueryDeclarationAspectContract::from_slices(
                    &["selection.face"],
                    &[],
                    &[],
                    &[],
                    &[],
                )
            }

            fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
                WorthQueryDeclarationAspectCoverage::from_slices(&["selection.face"], &[], &[])
            }
        }
    };
}

define_family!(OrdinaryFutureSignalFamily);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TemporalFutureSignalFamily;

impl WorthQueryDeclarationFamilyMarker<FutureSignalDomain> for TemporalFutureSignalFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalFutureSignalFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract(
    ) -> Option<crate::application::WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            crate::application::WorthQueryDeclarationBridgeContinuationContract::runtime_route_current(),
        )
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(&["selection.face"], &[], &[])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AsyncFutureSignalFamily;

impl WorthQueryDeclarationFamilyMarker<FutureSignalDomain> for AsyncFutureSignalFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncFutureSignalFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract(
    ) -> Option<crate::application::WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            crate::application::WorthQueryDeclarationBridgeContinuationContract::runtime_route_current(),
        )
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(&["selection.face"], &[], &[])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FutureSignalInput<F> {
    id: &'static str,
    _marker: PhantomData<F>,
}

impl<F> FutureSignalInput<F> {
    pub(crate) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationInput<FutureSignalDomain>
    for FutureSignalInput<OrdinaryFutureSignalFamily>
{
    type Family = OrdinaryFutureSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

impl WorthQueryDeclarationInput<FutureSignalDomain>
    for FutureSignalInput<TemporalFutureSignalFamily>
{
    type Family = TemporalFutureSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        vec![WorthQueryTemporalDeclarationClause::stale_after(
            WorthQueryTemporalDuration::seconds(30),
        )]
    }
}

impl WorthQueryDeclarationInput<FutureSignalDomain> for FutureSignalInput<AsyncFutureSignalFamily> {
    type Family = AsyncFutureSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
        vec![WorthQueryAsyncDeclarationClause::resource_request(
            WorthQueryAsyncSourceFamily::BridgeResource,
            WorthQueryAsyncLoadingPosture::Blocking,
            WorthQueryAsyncFailurePosture::FailClosed,
            vec![WorthQueryAsyncRequestIdentityPart::text("id", self.id)],
        )]
    }
}

pub(crate) fn future_signal_admitted_handle(
    world: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<FutureSignalDomain, FutureSignalWorld> {
    WorthQueryApplicationFacade::new(WorthQueryConfig::runtime_backed_default())
        .unwrap()
        .domain(FutureSignalDomain)
        .with_operating_context(FutureSignalWorld(world))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

pub(crate) fn future_signal_bridge_request() -> WorthQueryDeclarationBridgeContinuationRequest {
    WorthQueryDeclarationBridgeContinuationRequest::new(
        WorthQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        WorthQueryDeclarationBridgeTruthContext::Current,
    )
}

pub(crate) fn checked_signal_public_posture<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<FutureSignalDomain, FutureSignalWorld>,
    input: FutureSignalInput<F>,
) -> WorthQueryDeclarationSignalCompatibilityChecked<FutureSignalDomain, FutureSignalInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<FutureSignalDomain>,
    FutureSignalInput<F>: WorthQueryDeclarationInput<FutureSignalDomain, Family = F> + Clone,
{
    let envelope_checked = future_supported_runtime_envelope_checked(handle, input);
    let support = handle.signal_compatibility_support::<FutureSignalInput<F>>();
    worth_query_checked_declaration_signal_compatibility_on_handle(
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        support.rows(),
        WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
    )
}

pub(crate) fn checked_signal_supported_runtime_test_posture<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<FutureSignalDomain, FutureSignalWorld>,
    input: FutureSignalInput<F>,
) -> WorthQueryDeclarationSignalCompatibilityChecked<FutureSignalDomain, FutureSignalInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<FutureSignalDomain>,
    FutureSignalInput<F>: WorthQueryDeclarationInput<FutureSignalDomain, Family = F> + Clone,
{
    let contract = F::signal_compatibility_contract()
        .expect("future test family must expose a signal contract");
    let support_rows = contract
        .required_basis_families()
        .iter()
        .copied()
        .map(|basis_family| {
            WorthQueryDeclarationSignalCompatibilitySupportRow::new(
                contract.execution_family(),
                basis_family,
                contract.dependency_aspects(),
                contract.produced_aspects(),
                F::aspect_coverage(),
                crate::application::WorthQueryDeclarationAspectFit::Exact,
                None,
                WorthQueryDeclarationSignalCompatibilitySupportStatus::Admitted,
                "supported runtime test posture admits this future signal compatibility row",
            )
        })
        .collect::<Vec<_>>();
    let envelope_checked = future_supported_runtime_envelope_checked(handle, input);
    worth_query_checked_declaration_signal_compatibility_on_handle(
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        &support_rows,
        WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
    )
}

fn future_supported_runtime_envelope_checked<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<FutureSignalDomain, FutureSignalWorld>,
    input: FutureSignalInput<F>,
) -> crate::application::WorthQueryDeclarationEnvelopeChecked<
    FutureSignalDomain,
    FutureSignalInput<F>,
>
where
    F: WorthQueryDeclarationFamilyMarker<FutureSignalDomain>,
    FutureSignalInput<F>: WorthQueryDeclarationInput<FutureSignalDomain, Family = F> + Clone,
{
    let canonical = handle
        .declare(input.clone())
        .unwrap_or_else(|_| panic!("future declaration should canonicalize"));
    let support_report = handle.family_support::<F>();
    let legal = match review_declaration_legality(
        handle.handle_identity_digest(),
        WorthQueryDeclarationLegalityInput::new(
            canonical,
            support_report,
            F::legality_contract(),
            handle.retained_world_basis(),
            Some(WorthQueryRuntimeFamilySupportStatus::Supported),
            Some(WorthQueryRuntimeFamilySupportStatus::Supported),
        ),
    ) {
        WorthQueryDeclarationLegalityChecked::Legal(legal) => legal,
        WorthQueryDeclarationLegalityChecked::Illegal(_) => {
            panic!("future declaration should become legal under supported runtime test posture")
        }
    };
    let progressed = handle
        .progress_declaration(legal)
        .unwrap_or_else(|_| panic!("future progression should admit"));
    let evidence = handle
        .describe_foundational(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("future foundational evidence should materialize"));
    let route_checked = handle.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::admitted(
        progressed, evidence,
    ));
    let receipt_checked = handle.receipt_routes_checked(
        WorthQueryDeclarationReceiptInput::route_checked(route_checked),
    );
    handle.envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::receipt_checked(
        receipt_checked,
    ))
}
