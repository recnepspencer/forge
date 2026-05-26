use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRelationalRoutingChecked,
    ForgeQueryDeclarationRelationalRoutingInput,
};

use super::domain::{GeometryDomain, GeometryWorld, RoutingInput};

pub(crate) fn checked_from_progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RoutingInput<F>,
) -> ForgeQueryDeclarationRelationalRoutingChecked<GeometryDomain, RoutingInput<F>>
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
    handle.route_relational_truth_checked(
        ForgeQueryDeclarationRelationalRoutingInput::envelope_checked(envelope_checked),
    )
}
