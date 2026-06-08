use std::marker::PhantomData;

use crate::application::{
    forge_query_checked_declaration_signal_compatibility_on_handle, review_declaration_legality,
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily, ForgeQueryConfig,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationBridgeContinuationMode,
    ForgeQueryDeclarationBridgeContinuationRequest, ForgeQueryDeclarationBridgeTruthContext,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFoundationalEvidenceInput,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationLegalityInput,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationRoutePlanInput, ForgeQueryDeclarationSignalCompatibilityChecked,
    ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDeclarationSignalCompatibilityInput,
    ForgeQueryDeclarationSignalCompatibilitySupportRow,
    ForgeQueryDeclarationSignalCompatibilitySupportStatus, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQuerySignalCompatiblePosture, ForgeQueryTemporalDeclarationClause,
    ForgeQueryTemporalDeclarationSupport, ForgeQueryTemporalDuration,
};
use crate::runtime::ForgeQueryRuntimeFamilySupportStatus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FutureSignalDomain;

impl ForgeQueryDomainEntryMarker for FutureSignalDomain {
    fn domain_key(&self) -> &'static str {
        "test.future.signal.domain"
    }

    fn display_name(&self) -> &'static str {
        "FutureSignalDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FutureSignalWorld(pub(crate) &'static str);

impl ForgeQueryDomainOperatingContext<FutureSignalDomain> for FutureSignalWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
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

        impl ForgeQueryDeclarationFamilyMarker<FutureSignalDomain> for $name {
            type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
            type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                ForgeQueryDeclarationRouteContract::bridge_only()
            }

            fn bridge_continuation_contract(
            ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
                Some(
                    crate::application::ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current(),
                )
            }

            fn signal_compatibility_contract(
            ) -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
                Some(
                    ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution(),
                )
            }

            fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
                ForgeQueryDeclarationAspectContract::from_slices(
                    &["selection.face"],
                    &[],
                    &[],
                    &[],
                    &[],
                )
            }

            fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
                ForgeQueryDeclarationAspectCoverage::from_slices(&["selection.face"], &[], &[])
            }
        }
    };
}

define_family!(OrdinaryFutureSignalFamily);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TemporalFutureSignalFamily;

impl ForgeQueryDeclarationFamilyMarker<FutureSignalDomain> for TemporalFutureSignalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalFutureSignalFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract(
    ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
        Some(
            crate::application::ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current(),
        )
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(&["selection.face"], &[], &[])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AsyncFutureSignalFamily;

impl ForgeQueryDeclarationFamilyMarker<FutureSignalDomain> for AsyncFutureSignalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncFutureSignalFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract(
    ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
        Some(
            crate::application::ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current(),
        )
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(&["selection.face"], &[], &[])
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

impl ForgeQueryDeclarationInput<FutureSignalDomain>
    for FutureSignalInput<OrdinaryFutureSignalFamily>
{
    type Family = OrdinaryFutureSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

impl ForgeQueryDeclarationInput<FutureSignalDomain>
    for FutureSignalInput<TemporalFutureSignalFamily>
{
    type Family = TemporalFutureSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        vec![ForgeQueryTemporalDeclarationClause::stale_after(
            ForgeQueryTemporalDuration::seconds(30),
        )]
    }
}

impl ForgeQueryDeclarationInput<FutureSignalDomain> for FutureSignalInput<AsyncFutureSignalFamily> {
    type Family = AsyncFutureSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
        vec![ForgeQueryAsyncDeclarationClause::resource_request(
            ForgeQueryAsyncSourceFamily::BridgeResource,
            ForgeQueryAsyncLoadingPosture::Blocking,
            ForgeQueryAsyncFailurePosture::FailClosed,
            vec![ForgeQueryAsyncRequestIdentityPart::text("id", self.id)],
        )]
    }
}

pub(crate) fn future_signal_admitted_handle(
    world: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<FutureSignalDomain, FutureSignalWorld> {
    ForgeQueryApplicationFacade::new(ForgeQueryConfig::runtime_backed_default())
        .unwrap()
        .domain(FutureSignalDomain)
        .with_operating_context(FutureSignalWorld(world))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

pub(crate) fn future_signal_bridge_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        ForgeQueryDeclarationBridgeTruthContext::Current,
    )
}

pub(crate) fn checked_signal_public_posture<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<FutureSignalDomain, FutureSignalWorld>,
    input: FutureSignalInput<F>,
) -> ForgeQueryDeclarationSignalCompatibilityChecked<FutureSignalDomain, FutureSignalInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<FutureSignalDomain>,
    FutureSignalInput<F>: ForgeQueryDeclarationInput<FutureSignalDomain, Family = F> + Clone,
{
    let envelope_checked = future_supported_runtime_envelope_checked(handle, input);
    let support = handle.signal_compatibility_support::<FutureSignalInput<F>>();
    forge_query_checked_declaration_signal_compatibility_on_handle(
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        support.rows(),
        ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
    )
}

pub(crate) fn checked_signal_supported_runtime_test_posture<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<FutureSignalDomain, FutureSignalWorld>,
    input: FutureSignalInput<F>,
) -> ForgeQueryDeclarationSignalCompatibilityChecked<FutureSignalDomain, FutureSignalInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<FutureSignalDomain>,
    FutureSignalInput<F>: ForgeQueryDeclarationInput<FutureSignalDomain, Family = F> + Clone,
{
    let contract = F::signal_compatibility_contract()
        .expect("future test family must expose a signal contract");
    let support_rows = contract
        .required_basis_families()
        .iter()
        .copied()
        .map(|basis_family| {
            ForgeQueryDeclarationSignalCompatibilitySupportRow::new(
                contract.execution_family(),
                basis_family,
                contract.dependency_aspects(),
                contract.produced_aspects(),
                F::aspect_coverage(),
                crate::application::ForgeQueryDeclarationAspectFit::Exact,
                None,
                ForgeQueryDeclarationSignalCompatibilitySupportStatus::Admitted,
                "supported runtime test posture admits this future signal compatibility row",
            )
        })
        .collect::<Vec<_>>();
    let envelope_checked = future_supported_runtime_envelope_checked(handle, input);
    forge_query_checked_declaration_signal_compatibility_on_handle(
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        &support_rows,
        ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
    )
}

fn future_supported_runtime_envelope_checked<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<FutureSignalDomain, FutureSignalWorld>,
    input: FutureSignalInput<F>,
) -> crate::application::ForgeQueryDeclarationEnvelopeChecked<
    FutureSignalDomain,
    FutureSignalInput<F>,
>
where
    F: ForgeQueryDeclarationFamilyMarker<FutureSignalDomain>,
    FutureSignalInput<F>: ForgeQueryDeclarationInput<FutureSignalDomain, Family = F> + Clone,
{
    let canonical = handle
        .declare(input.clone())
        .unwrap_or_else(|_| panic!("future declaration should canonicalize"));
    let support_report = handle.family_support::<F>();
    let legal = match review_declaration_legality(
        handle.handle_identity_digest(),
        ForgeQueryDeclarationLegalityInput::new(
            canonical,
            support_report,
            F::legality_contract(),
            handle.retained_world_basis(),
            Some(ForgeQueryRuntimeFamilySupportStatus::Supported),
            Some(ForgeQueryRuntimeFamilySupportStatus::Supported),
        ),
    ) {
        ForgeQueryDeclarationLegalityChecked::Legal(legal) => legal,
        ForgeQueryDeclarationLegalityChecked::Illegal(_) => {
            panic!("future declaration should become legal under supported runtime test posture")
        }
    };
    let progressed = handle
        .progress_declaration(legal)
        .unwrap_or_else(|_| panic!("future progression should admit"));
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("future foundational evidence should materialize"));
    let route_checked = handle.plan_routes_checked(ForgeQueryDeclarationRoutePlanInput::admitted(
        progressed, evidence,
    ));
    let receipt_checked = handle.receipt_routes_checked(
        ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
    );
    handle.envelope_routes_checked(ForgeQueryDeclarationEnvelopeInput::receipt_checked(
        receipt_checked,
    ))
}
