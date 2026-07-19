use crate::application::{
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationEnvelopeInput,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationRouteIntent, WorthQueryDeclarationRoutePlanChecked,
    WorthQueryDeclarationRoutePlanInput, WorthQueryInstalledDomainDeclarationContext,
};

use super::domain::{EnvelopeInput, GeometryDomain, GeometryWorld};

pub(crate) use super::domain::DeferredEnvelopeFamily;

pub(crate) fn progressed<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: EnvelopeInput<F>,
) -> crate::application::WorthQueryAdmittedDeclarationProgression<GeometryDomain, EnvelopeInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    EnvelopeInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("envelope progression should admit"))
}

pub(crate) fn route_checked_with_intent<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: EnvelopeInput<F>,
    intent: WorthQueryDeclarationRouteIntent,
) -> WorthQueryDeclarationRoutePlanChecked<GeometryDomain, EnvelopeInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    EnvelopeInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    let progression = progressed(handle, declaration);
    let evidence = handle
        .describe_foundational(
            crate::application::WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    handle.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::with_intent(
        progression,
        evidence,
        intent,
    ))
}

pub(crate) fn route_checked_from_input<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: EnvelopeInput<F>,
) -> WorthQueryDeclarationRoutePlanChecked<GeometryDomain, EnvelopeInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    EnvelopeInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    let progression = progressed(handle, declaration);
    let evidence = handle
        .describe_foundational(
            crate::application::WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    handle.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::admitted(
        progression,
        evidence,
    ))
}

pub(crate) fn envelope_checked_from_receipt<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: EnvelopeInput<F>,
) -> WorthQueryDeclarationEnvelopeChecked<GeometryDomain, EnvelopeInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    EnvelopeInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    let receipt = handle
        .receipt_routes_from_progressed(progressed(handle, declaration))
        .unwrap_or_else(|_| panic!("receipt should issue"));
    handle.envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::issued(receipt))
}
