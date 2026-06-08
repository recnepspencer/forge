use crate::application::{
    forge_query_checked_declaration_bridge_routing_on_handle,
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationBridgeRouting,
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingInput,
    ForgeQueryDeclarationBridgeRoutingSupportStatus, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityChecked, ForgeQueryDeclarationLegalityInput,
    ForgeQueryDeclarationReceiptInput,
};
use crate::runtime::ForgeQueryRuntimeFamilySupportStatus;

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

pub(crate) fn checked_from_future_supported_runtime_test_posture<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RoutingInput<F>,
) -> ForgeQueryDeclarationBridgeRoutingChecked<GeometryDomain, RoutingInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RoutingInput<F>: ForgeQueryDeclarationInput<GeometryDomain, Family = F>,
{
    let canonical = handle
        .declare(declaration.clone())
        .unwrap_or_else(|_| panic!("future declaration should canonicalize"));
    let support_report = handle.family_support::<F>();
    let legal = match crate::application::review_declaration_legality(
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
            crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("future foundational evidence should materialize"));
    let route_checked = handle.plan_routes_checked(
        crate::application::ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    );
    let receipt_checked = handle.receipt_routes_checked(
        ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
    );
    let envelope_checked = handle.envelope_routes_checked(
        ForgeQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
    );
    forge_query_checked_declaration_bridge_routing_on_handle(
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        ForgeQueryDeclarationBridgeRoutingSupportStatus::Admitted,
        ForgeQueryDeclarationBridgeRoutingInput::envelope_checked(envelope_checked),
    )
}

pub(crate) fn routed_from_future_supported_runtime_test_posture<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RoutingInput<F>,
) -> ForgeQueryDeclarationBridgeRouting<GeometryDomain, RoutingInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RoutingInput<F>: ForgeQueryDeclarationInput<GeometryDomain, Family = F>,
{
    match checked_from_future_supported_runtime_test_posture(handle, declaration) {
        ForgeQueryDeclarationBridgeRoutingChecked::Routed(routing) => routing,
        _ => panic!("future bridge routing should admit under supported runtime test posture"),
    }
}
