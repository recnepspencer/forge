use crate::application::{
    WorthQueryApplicationFacade, WorthQueryConfig, WorthQueryDeclarationBridgeContinuationMode,
    WorthQueryDeclarationBridgeContinuationRequest, WorthQueryDeclarationBridgeTruthContext,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryRelationalConfig, WorthQueryRuntimeBridgeConfig,
};
use crate::binding_pipeline::{
    WorthQueryBindingSourceKind, WorthQueryBindingSpecificity,
    WorthQueryContinuationBindingRequest, WorthQueryEnvelopeContextCandidate,
    WorthQueryResolveContinuationFromTargetRequest,
};

use super::{
    ContinuationDomain, ContinuationWorld, DriftedContinuationWorld, Input,
    LenientContinuationWorld, ReadmissionDrift, RuntimeFamily,
};

pub(crate) fn admitted_handle(
    world: &'static str,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    ContinuationWorld,
> {
    configured_handle(world, WorthQueryConfig::runtime_backed_default())
}

pub(crate) fn configured_handle(
    world: &'static str,
    config: WorthQueryConfig,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    ContinuationWorld,
> {
    WorthQueryApplicationFacade::new(config)
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
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    LenientContinuationWorld,
> {
    WorthQueryApplicationFacade::new(
        WorthQueryConfig::runtime_backed_default()
            .with_runtime_bridge(WorthQueryRuntimeBridgeConfig::disabled()),
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
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    LenientContinuationWorld,
> {
    WorthQueryApplicationFacade::new(
        WorthQueryConfig::runtime_backed_default().with_relational(
            WorthQueryRelationalConfig::enabled().with_historical_evaluation(false),
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
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    DriftedContinuationWorld,
> {
    WorthQueryApplicationFacade::new(WorthQueryConfig::runtime_backed_default())
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

pub(crate) fn runtime_route_request() -> WorthQueryDeclarationBridgeContinuationRequest {
    WorthQueryDeclarationBridgeContinuationRequest::new(
        WorthQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        WorthQueryDeclarationBridgeTruthContext::Current,
    )
}

pub(crate) fn historical_truth_view_request() -> WorthQueryDeclarationBridgeContinuationRequest {
    WorthQueryDeclarationBridgeContinuationRequest::new(
        WorthQueryDeclarationBridgeContinuationMode::TruthView,
        WorthQueryDeclarationBridgeTruthContext::Historical,
    )
}

pub(crate) fn preview_session_request() -> WorthQueryDeclarationBridgeContinuationRequest {
    WorthQueryDeclarationBridgeContinuationRequest::new(
        WorthQueryDeclarationBridgeContinuationMode::PreviewSession,
        WorthQueryDeclarationBridgeTruthContext::Preview,
    )
}

pub(crate) fn envelope<I>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
) -> WorthQueryDeclarationEnvelope<ContinuationDomain, Input<I>>
where
    Input<I>: WorthQueryDeclarationInput<ContinuationDomain, Family = I>,
    I: WorthQueryDeclarationFamilyMarker<ContinuationDomain>,
{
    let progressed = handle
        .declare_review_and_progress(Input::<I>::new(id))
        .unwrap_or_else(|_| panic!("expected progressed continuation declaration"));
    handle
        .envelope_routes_from_progressed(progressed)
        .unwrap_or_else(|_| panic!("expected envelope"))
}

pub(crate) fn target_request<I>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
    bridge_request: WorthQueryDeclarationBridgeContinuationRequest,
) -> WorthQueryResolveContinuationFromTargetRequest<ContinuationDomain, Input<I>>
where
    Input<I>: WorthQueryDeclarationInput<ContinuationDomain, Family = I>,
    I: WorthQueryDeclarationFamilyMarker<ContinuationDomain>,
{
    WorthQueryResolveContinuationFromTargetRequest::new(
        envelope::<I>(handle, id),
        I::aspect_contract(),
    )
    .with_bridge_request(bridge_request)
}

pub(crate) fn context_request(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
) -> WorthQueryContinuationBindingRequest<ContinuationDomain, Input<RuntimeFamily>> {
    WorthQueryContinuationBindingRequest::new(
        vec![WorthQueryEnvelopeContextCandidate::new(
            "current envelope",
            WorthQueryBindingSourceKind::CurrentEnvelope,
            WorthQueryBindingSpecificity::TypedCurrentArtifact,
            envelope::<RuntimeFamily>(handle, id),
        )],
        RuntimeFamily::aspect_contract(),
        vec![WorthQueryBindingSourceKind::CurrentEnvelope],
    )
    .with_bridge_request(runtime_route_request())
}
