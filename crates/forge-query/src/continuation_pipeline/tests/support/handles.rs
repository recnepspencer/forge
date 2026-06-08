use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryConfig, ForgeQueryDeclarationBridgeContinuationMode,
    ForgeQueryDeclarationBridgeContinuationRequest, ForgeQueryDeclarationBridgeTruthContext,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryRelationalConfig, ForgeQueryRuntimeBridgeConfig,
};
use crate::binding_pipeline::{
    ForgeQueryBindingSourceKind, ForgeQueryBindingSpecificity,
    ForgeQueryContinuationBindingRequest, ForgeQueryEnvelopeContextCandidate,
    ForgeQueryResolveContinuationFromTargetRequest,
};

use super::{
    ContinuationDomain, ContinuationWorld, DriftedContinuationWorld, Input,
    LenientContinuationWorld, ReadmissionDrift, RuntimeFamily,
};

pub(crate) fn admitted_handle(
    world: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContinuationDomain,
    ContinuationWorld,
> {
    configured_handle(world, ForgeQueryConfig::runtime_backed_default())
}

pub(crate) fn configured_handle(
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

pub(crate) fn preview_disabled_handle(
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

pub(crate) fn historical_disabled_handle(
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

pub(crate) fn drifted_readmission_handle(
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

pub(crate) fn runtime_route_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        ForgeQueryDeclarationBridgeTruthContext::Current,
    )
}

pub(crate) fn historical_truth_view_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::TruthView,
        ForgeQueryDeclarationBridgeTruthContext::Historical,
    )
}

pub(crate) fn preview_session_request() -> ForgeQueryDeclarationBridgeContinuationRequest {
    ForgeQueryDeclarationBridgeContinuationRequest::new(
        ForgeQueryDeclarationBridgeContinuationMode::PreviewSession,
        ForgeQueryDeclarationBridgeTruthContext::Preview,
    )
}

pub(crate) fn envelope<I>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
) -> ForgeQueryDeclarationEnvelope<ContinuationDomain, Input<I>>
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

pub(crate) fn target_request<I>(
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

pub(crate) fn context_request(
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
