use crate::application::{
    worth_query_checked_declaration_signal_compatibility_on_handle, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityChecked, WorthQueryDeclarationLegalityInput,
    WorthQueryDeclarationReceiptInput, WorthQueryDeclarationRoutePlanInput,
    WorthQueryDeclarationSignalCompatibility, WorthQueryDeclarationSignalCompatibilityChecked,
    WorthQueryDeclarationSignalCompatibilityInput,
    WorthQueryDeclarationSignalCompatibilitySupportRow,
    WorthQueryDeclarationSignalCompatibilitySupportStatus,
    WorthQueryInstalledDomainDeclarationContext,
};
use crate::runtime::WorthQueryRuntimeFamilySupportStatus;

use super::domain::{GeometryDomain, GeometryWorld};

pub fn envelope_checked_for<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    input: super::domain::Input<F>,
) -> WorthQueryDeclarationEnvelopeChecked<GeometryDomain, super::domain::Input<F>>
where
    super::domain::Input<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    let progressed = handle
        .declare_review_and_progress(input)
        .unwrap_or_else(|_| panic!("progression should admit"));
    let evidence = handle
        .describe_foundational(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    let route_checked = handle.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::admitted(
        progressed, evidence,
    ));
    let receipt_checked = handle.receipt_routes_checked(
        WorthQueryDeclarationReceiptInput::route_checked(route_checked),
    );
    handle.envelope_routes_checked(
        crate::application::WorthQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
    )
}

pub fn compatibility_from_envelope_input<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    input: super::domain::Input<F>,
) -> WorthQueryDeclarationSignalCompatibility<GeometryDomain, super::domain::Input<F>>
where
    super::domain::Input<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    handle
        .signal_compatibility(
            WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked_for(
                handle, input,
            )),
        )
        .unwrap_or_else(|_| panic!("advanced signal compatibility lane should admit"))
}

pub fn checked_from_future_public_runtime_signal_posture<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    input: super::domain::Input<F>,
) -> WorthQueryDeclarationSignalCompatibilityChecked<GeometryDomain, super::domain::Input<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    super::domain::Input<F>: WorthQueryDeclarationInput<GeometryDomain, Family = F> + Clone,
{
    let envelope_checked = future_supported_runtime_envelope_checked(handle, input);
    let support = handle.signal_compatibility_support::<super::domain::Input<F>>();
    worth_query_checked_declaration_signal_compatibility_on_handle(
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        support.rows(),
        WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
    )
}

pub fn compatibility_from_future_supported_runtime_test_posture<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    input: super::domain::Input<F>,
) -> WorthQueryDeclarationSignalCompatibility<GeometryDomain, super::domain::Input<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    super::domain::Input<F>: WorthQueryDeclarationInput<GeometryDomain, Family = F> + Clone,
{
    let envelope_checked = future_supported_runtime_envelope_checked(handle, input);
    let support_rows = future_supported_runtime_signal_rows::<F>();
    match worth_query_checked_declaration_signal_compatibility_on_handle(
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        &support_rows,
        WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
    ) {
        WorthQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => compatibility,
        _ => {
            panic!("future signal compatibility should admit under supported runtime test posture")
        }
    }
}

fn future_supported_runtime_envelope_checked<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    input: super::domain::Input<F>,
) -> WorthQueryDeclarationEnvelopeChecked<GeometryDomain, super::domain::Input<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    super::domain::Input<F>: WorthQueryDeclarationInput<GeometryDomain, Family = F> + Clone,
{
    let canonical = handle
        .declare(input.clone())
        .unwrap_or_else(|_| panic!("future declaration should canonicalize"));
    let support_report = handle.family_support::<F>();
    let legal = match crate::application::review_declaration_legality(
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
    handle.envelope_routes_checked(
        crate::application::WorthQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
    )
}

fn future_supported_runtime_signal_rows<F>(
) -> Vec<WorthQueryDeclarationSignalCompatibilitySupportRow>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
{
    let contract = F::signal_compatibility_contract()
        .unwrap_or_else(|| panic!("future signal families must expose a signal contract"));
    contract
        .required_basis_families()
        .iter()
        .copied()
        .map(|basis_family| {
            WorthQueryDeclarationSignalCompatibilitySupportRow::new(
                contract.execution_family(),
                basis_family,
                contract.dependency_aspects(),
                contract.produced_aspects(),
                F::aspect_coverage(),
                WorthQueryDeclarationAspectFit::Exact,
                None,
                WorthQueryDeclarationSignalCompatibilitySupportStatus::Admitted,
                "supported runtime test posture admits this future signal compatibility row",
            )
        })
        .collect()
}
