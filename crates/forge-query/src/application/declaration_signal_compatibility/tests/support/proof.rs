use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRoutePlanInput,
    ForgeQueryDeclarationSignalCompatibility, ForgeQueryDeclarationSignalCompatibilityInput,
};

use super::domain::{GeometryDomain, GeometryWorld};

pub fn envelope_checked_for<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    input: super::domain::Input<F>,
) -> ForgeQueryDeclarationEnvelopeChecked<GeometryDomain, super::domain::Input<F>>
where
    super::domain::Input<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progressed = handle
        .declare_review_and_progress(input)
        .unwrap_or_else(|_| panic!("progression should admit"));
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    let route_checked = handle.plan_routes_checked(ForgeQueryDeclarationRoutePlanInput::admitted(
        progressed, evidence,
    ));
    let receipt_checked = handle.receipt_routes_checked(
        ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
    );
    handle.envelope_routes_checked(
        crate::application::ForgeQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
    )
}

pub fn compatibility_from_envelope_input<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    input: super::domain::Input<F>,
) -> ForgeQueryDeclarationSignalCompatibility<GeometryDomain, super::domain::Input<F>>
where
    super::domain::Input<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .signal_compatibility(
            ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked_for(
                handle, input,
            )),
        )
        .unwrap_or_else(|_| panic!("advanced signal compatibility lane should admit"))
}
