use super::*;

pub(super) fn future_supported_runtime_envelope_checked<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<FutureSignalDomain, FutureSignalWorld>,
    input: FutureSignalInput<F>,
) -> crate::application::WorthQueryDeclarationEnvelopeChecked<
    FutureSignalDomain,
    FutureSignalInput<F>,
>
where
    F: WorthQueryDeclarationFamilyMarker<FutureSignalDomain>,
    FutureSignalInput<F>: WorthQueryDeclarationInput<FutureSignalDomain, Family = F> + Clone,
{
    let canonical = handle
        .declare(input.clone())
        .unwrap_or_else(|_| panic!("future declaration should canonicalize"));
    let support_report = handle.family_support::<F>();
    let legal = match review_declaration_legality(
        handle.handle_identity_digest(),
        WorthQueryDeclarationLegalityInput::new(
            canonical,
            support_report,
            F::legality_contract(),
            handle.retained_world_basis(),
            Some(WorthQueryRuntimeFamilySupportStatus::Supported),
            Some(WorthQueryRuntimeFamilySupportStatus::Supported),
        ),
    ) {
        WorthQueryDeclarationLegalityChecked::Legal(legal) => legal,
        WorthQueryDeclarationLegalityChecked::Illegal(_) => {
            panic!("future declaration should become legal under supported runtime test posture")
        }
    };
    let progressed = handle
        .progress_declaration(legal)
        .unwrap_or_else(|_| panic!("future progression should admit"));
    let evidence = handle
        .describe_foundational(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("future foundational evidence should materialize"));
    let route_checked = handle.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::admitted(
        progressed, evidence,
    ));
    let receipt_checked = handle.receipt_routes_checked(
        WorthQueryDeclarationReceiptInput::route_checked(route_checked),
    );
    handle.envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::receipt_checked(
        receipt_checked,
    ))
}
