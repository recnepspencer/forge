use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeRouting,
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingInput,
    ForgeQueryDeclarationEnvelopeInput, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceiptInput,
};

use super::domain::{GeometryDomain, GeometryWorld, RoutingInput};

pub(crate) fn checked_from_progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RoutingInput<F>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<GeometryDomain, RoutingInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RoutingInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progressed = handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("progression should admit"));
    let evidence = handle
        .describe_foundational(
            crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    let route_checked = handle.plan_routes_checked(
        crate::application::ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    );
    let receipt_checked = handle.receipt_routes_checked(
        ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
    );
    let envelope_checked = handle.envelope_routes_checked(
        ForgeQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
    );
    handle.route_bridge_continuation_checked(
        ForgeQueryDeclarationBridgeRoutingInput::envelope_checked(envelope_checked),
    )
}

pub(crate) fn routed_from_progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RoutingInput<F>,
) -> ForgeQueryDeclarationBridgeRouting<GeometryDomain, RoutingInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RoutingInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progressed = handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("progression should admit"));
    let envelope = handle
        .envelope_routes_from_progressed(progressed)
        .unwrap_or_else(|_| panic!("envelope should admit"));
    handle
        .route_bridge_continuation(ForgeQueryDeclarationBridgeRoutingInput::enveloped(envelope))
        .unwrap_or_else(|_| panic!("bridge routing should admit"))
}
