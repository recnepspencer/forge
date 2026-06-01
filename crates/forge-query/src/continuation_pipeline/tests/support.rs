use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfig, ForgeQueryConfigSectionFamily,
    ForgeQueryContinuationExecutionReadmissionObservation, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationBridgeContinuationMode, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationBridgeTruthContext, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalConfig, ForgeQueryRuntimeBridgeConfig, ForgeQuerySignalCompatiblePosture,
};
use crate::binding_pipeline::{
    ForgeQueryBindingSourceKind, ForgeQueryBindingSpecificity,
    ForgeQueryContinuationBindingRequest, ForgeQueryEnvelopeContextCandidate,
    ForgeQueryResolveContinuationFromTargetRequest,
};
use crate::continuation_pipeline::execution::drifted_observation_from_retained;
use crate::continuation_pipeline::ForgeQueryPreparedContinuationFreshnessPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContinuationDomain;

impl ForgeQueryDomainEntryMarker for ContinuationDomain {
    fn domain_key(&self) -> &'static str {
        "test.continuation.domain"
    }

    fn display_name(&self) -> &'static str {
        "ContinuationDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContinuationWorld(pub(super) &'static str);

impl ForgeQueryDomainOperatingContext<ContinuationDomain> for ContinuationWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::PreviewSession]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct LenientContinuationWorld(pub(super) &'static str);

impl ForgeQueryDomainOperatingContext<ContinuationDomain> for LenientContinuationWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Query]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReadmissionDrift {
    Stale,
    BasisMismatch,
    AuthorityMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DriftedContinuationWorld {
    pub(super) label: &'static str,
    pub(super) drift: ReadmissionDrift,
}

impl ForgeQueryDomainOperatingContext<ContinuationDomain> for DriftedContinuationWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::PreviewSession]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.label)
    }

    fn continuation_execution_readmission_observation(
        &self,
        retained: &crate::continuation_pipeline::ForgeQueryPreparedContinuationExecutionReadmission,
        _support_snapshot: &crate::application::ForgeQueryDomainEntrySupportSnapshot,
    ) -> ForgeQueryContinuationExecutionReadmissionObservation {
        match self.drift {
            ReadmissionDrift::Stale => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stale,
                None,
                None,
            ),
            ReadmissionDrift::BasisMismatch => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                Some(format!(
                    "{}:drifted",
                    retained.basis_witness().basis_identity_digest()
                )),
                None,
            ),
            ReadmissionDrift::AuthorityMismatch => drifted_observation_from_retained(
                retained,
                ForgeQueryPreparedContinuationFreshnessPosture::Stable,
                None,
                Some(crate::basis_lifecycle::LowerRuntimeEvidenceAuthority::RuntimeBridgeFacade),
            ),
        }
    }
}

macro_rules! define_family {
    ($name:ident, $bridge:expr, $signal:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<ContinuationDomain> for $name {
            type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
            type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
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

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                ForgeQueryDeclarationRouteContract::relational_and_bridge()
            }

            fn bridge_continuation_contract(
            ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
                Some($bridge)
            }

            fn signal_compatibility_contract(
            ) -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
                Some($signal)
            }
        }
    };
}

define_family!(
    RuntimeFamily,
    crate::application::ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current(),
    ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
);
define_family!(
    HistoricalFamily,
    crate::application::ForgeQueryDeclarationBridgeContinuationContract::truth_view_historical(),
    ForgeQueryDeclarationSignalCompatibilityContract::historical_derived_execution()
);
define_family!(
    PreviewFamily,
    crate::application::ForgeQueryDeclarationBridgeContinuationContract::preview_session(),
    ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Input<F> {
    id: &'static str,
    _marker: PhantomData<F>,
}

impl<F> Input<F> {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_input {
    ($($family:ty),+ $(,)?) => {$(
        impl ForgeQueryDeclarationInput<ContinuationDomain> for Input<$family> {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
            }
        }
    )+};
}

impl_input!(RuntimeFamily, HistoricalFamily, PreviewFamily);

pub(super) fn admitted_handle(
    world: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContinuationDomain,
    ContinuationWorld,
> {
    configured_handle(world, ForgeQueryConfig::runtime_backed_default())
}

pub(super) fn configured_handle(
    world: &'static str,
    config: ForgeQueryConfig,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContinuationDomain,
    ContinuationWorld,
> {
    ForgeQueryApplicationFacade::new(config)
        .expect("test continuation config should validate")
        .domain(ContinuationDomain)
        .with_operating_context(ContinuationWorld(world))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

pub(super) fn preview_disabled_handle(
    world: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContinuationDomain,
    LenientContinuationWorld,
> {
    ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default()
            .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled()),
    )
    .expect("test continuation config should validate")
    .domain(ContinuationDomain)
    .with_operating_context(LenientContinuationWorld(world))
    .validate()
    .unwrap()
    .admit()
    .unwrap()
}

pub(super) fn historical_disabled_handle(
    world: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContinuationDomain,
    LenientContinuationWorld,
> {
    ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_relational(
            ForgeQueryRelationalConfig::enabled().with_historical_evaluation(false),
        ),
    )
    .expect("test continuation config should validate")
    .domain(ContinuationDomain)
    .with_operating_context(LenientContinuationWorld(world))
    .validate()
    .unwrap()
    .admit()
    .unwrap()
}

pub(super) fn drifted_readmission_handle(
    world: &'static str,
    drift: ReadmissionDrift,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContinuationDomain,
    DriftedContinuationWorld,
> {
    ForgeQueryApplicationFacade::new(ForgeQueryConfig::runtime_backed_default())
        .expect("test continuation config should validate")
        .domain(ContinuationDomain)
        .with_operating_context(DriftedContinuationWorld {
            label: world,
            drift,
        })
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

pub(super) fn runtime_route_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        ForgeQueryDeclarationBridgeTruthContext::Current,
    )
}

pub(super) fn historical_truth_view_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::TruthView,
        ForgeQueryDeclarationBridgeTruthContext::Historical,
    )
}

pub(super) fn preview_session_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::PreviewSession,
        ForgeQueryDeclarationBridgeTruthContext::Preview,
    )
}

pub(super) fn envelope<I>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
) -> crate::application::ForgeQueryDeclarationEnvelope<ContinuationDomain, Input<I>>
where
    Input<I>: ForgeQueryDeclarationInput<ContinuationDomain, Family = I>,
    I: ForgeQueryDeclarationFamilyMarker<ContinuationDomain>,
{
    let progressed = handle
        .declare_review_and_progress(Input::<I>::new(id))
        .unwrap_or_else(|_| panic!("expected progressed continuation declaration"));
    handle
        .envelope_routes_from_progressed(progressed)
        .unwrap_or_else(|_| panic!("expected envelope"))
}

pub(super) fn target_request<I>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
    bridge_request: ForgeQueryDeclarationBridgeContinuationRequest,
) -> ForgeQueryResolveContinuationFromTargetRequest<ContinuationDomain, Input<I>>
where
    Input<I>: ForgeQueryDeclarationInput<ContinuationDomain, Family = I>,
    I: ForgeQueryDeclarationFamilyMarker<ContinuationDomain>,
{
    ForgeQueryResolveContinuationFromTargetRequest::new(
        envelope::<I>(handle, id),
        I::aspect_contract(),
    )
    .with_bridge_request(bridge_request)
}

pub(super) fn context_request(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
) -> ForgeQueryContinuationBindingRequest<ContinuationDomain, Input<RuntimeFamily>> {
    ForgeQueryContinuationBindingRequest::new(
        vec![ForgeQueryEnvelopeContextCandidate::new(
            "current envelope",
            ForgeQueryBindingSourceKind::CurrentEnvelope,
            ForgeQueryBindingSpecificity::TypedCurrentArtifact,
            envelope::<RuntimeFamily>(handle, id),
        )],
        RuntimeFamily::aspect_contract(),
        vec![ForgeQueryBindingSourceKind::CurrentEnvelope],
    )
    .with_bridge_request(runtime_route_request())
}
