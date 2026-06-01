use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationEnvelopeInput, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanChecked, ForgeQueryDeclarationRoutePlanInput,
};

use super::domain::{EnvelopeInput, GeometryDomain, GeometryWorld};

pub(crate) use super::domain::DeferredEnvelopeFamily;

pub(crate) fn progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: EnvelopeInput<F>,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<GeometryDomain, EnvelopeInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    EnvelopeInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("envelope progression should admit"))
}

pub(crate) fn route_checked_with_intent<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: EnvelopeInput<F>,
    intent: ForgeQueryDeclarationRouteIntent,
) -> ForgeQueryDeclarationRoutePlanChecked<GeometryDomain, EnvelopeInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    EnvelopeInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progression = progressed(handle, declaration);
    let evidence = handle
        .describe_foundational(
            crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    handle.plan_routes_checked(ForgeQueryDeclarationRoutePlanInput::with_intent(
        progression,
        evidence,
        intent,
    ))
}

pub(crate) fn route_checked_from_input<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: EnvelopeInput<F>,
) -> ForgeQueryDeclarationRoutePlanChecked<GeometryDomain, EnvelopeInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    EnvelopeInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progression = progressed(handle, declaration);
    let evidence = handle
        .describe_foundational(
            crate::application::ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progression.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    handle.plan_routes_checked(ForgeQueryDeclarationRoutePlanInput::admitted(
        progression,
        evidence,
    ))
}

pub(crate) fn envelope_checked_from_receipt<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: EnvelopeInput<F>,
) -> ForgeQueryDeclarationEnvelopeChecked<GeometryDomain, EnvelopeInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    EnvelopeInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let receipt = handle
        .receipt_routes_from_progressed(progressed(handle, declaration))
        .unwrap_or_else(|_| panic!("receipt should issue"));
    handle.envelope_routes_checked(ForgeQueryDeclarationEnvelopeInput::issued(receipt))
}
