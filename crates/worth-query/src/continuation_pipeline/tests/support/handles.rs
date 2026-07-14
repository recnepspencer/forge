use crate::application::{
    WorthQueryConfig, WorthQueryDeclarationBridgeContinuationMode,
    WorthQueryDeclarationBridgeContinuationRequest, WorthQueryDeclarationBridgeTruthContext,
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
};
use crate::binding_pipeline::{
    WorthQueryBindingSourceKind, WorthQueryBindingSpecificity,
    WorthQueryContinuationBindingRequest, WorthQueryEnvelopeContextCandidate,
    WorthQueryResolveContinuationFromTargetRequest,
};

use super::{
    ContinuationDomain, ContinuationWorld, DriftedContinuationWorld, HistoricalFamily, Input,
    LenientContinuationWorld, PreviewFamily, ReadmissionDrift, RuntimeFamily,
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
    _config: WorthQueryConfig,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    ContinuationWorld,
> {
    installed_context(ContinuationWorld(world))
}

pub(crate) fn preview_disabled_handle(
    world: &'static str,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    LenientContinuationWorld,
> {
    installed_context(LenientContinuationWorld(world))
}

pub(crate) fn historical_disabled_handle(
    world: &'static str,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    LenientContinuationWorld,
> {
    installed_context(LenientContinuationWorld(world))
}

pub(crate) fn drifted_readmission_handle(
    world: &'static str,
    drift: ReadmissionDrift,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    ContinuationDomain,
    DriftedContinuationWorld,
> {
    installed_context(DriftedContinuationWorld {
        label: world,
        drift,
    })
}

fn installed_context<C>(
    context: C,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<ContinuationDomain, C>
where
    C: crate::application::WorthQueryDomainOperatingContext<ContinuationDomain>,
{
    crate::application::domain_test_support::installed_declaration_context(
        ContinuationDomain,
        context,
        [
            crate::application::domain_test_support::family::<ContinuationDomain, RuntimeFamily>(),
            crate::application::domain_test_support::family::<ContinuationDomain, HistoricalFamily>(
            ),
            crate::application::domain_test_support::family::<ContinuationDomain, PreviewFamily>(),
        ],
    )
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
