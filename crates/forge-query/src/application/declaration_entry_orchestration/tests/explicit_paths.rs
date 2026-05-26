use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationOutcome, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEnvelopeChecked, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationReceiptChecked,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRoutePlanChecked,
    ForgeQueryDeclarationRoutePlanInput,
};

use super::domain::{
    AdmittedFamily, CollaborativeWorld, DeferredRouteFamily, GeometryDomain, Input,
};

pub(super) fn explicit_success_path_parity(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        GeometryDomain,
        CollaborativeWorld,
    >,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<GeometryDomain, Input<AdmittedFamily>> {
    let declaration = match handle.declare_checked(Input::<AdmittedFamily>::new("edge:42")) {
        crate::application::ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("declaration should admit"),
    };
    let legal = match handle.review_legality_checked(declaration) {
        crate::application::ForgeQueryDeclarationLegalityChecked::Legal(legal) => legal,
        _ => panic!("legality should pass"),
    };
    let progressed = match handle.progress_declaration_checked(legal) {
        crate::application::ForgeQueryDeclarationProgressionChecked::Admitted(progressed) => {
            progressed
        }
        _ => panic!("progression should admit"),
    };
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    let route_plan = match handle.plan_routes_checked(
        ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    ) {
        ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => plan,
        _ => panic!("route plan should materialize"),
    };
    let receipt = match handle
        .receipt_routes_checked(ForgeQueryDeclarationReceiptInput::planned(route_plan))
    {
        ForgeQueryDeclarationReceiptChecked::Issued(receipt) => receipt,
        _ => panic!("receipt should issue"),
    };
    match handle.envelope_routes_checked(ForgeQueryDeclarationEnvelopeInput::issued(receipt)) {
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            ForgeQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope)
        }
        _ => panic!("envelope should materialize"),
    }
}

pub(super) fn explicit_deferred_route_path_parity(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        GeometryDomain,
        CollaborativeWorld,
    >,
) -> ForgeQueryDeclarationEntryOrchestrationOutcome<GeometryDomain, Input<DeferredRouteFamily>> {
    let declaration = match handle.declare_checked(Input::<DeferredRouteFamily>::new("edge:42")) {
        crate::application::ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("declaration should admit"),
    };
    let legal = match handle.review_legality_checked(declaration) {
        crate::application::ForgeQueryDeclarationLegalityChecked::Legal(legal) => legal,
        _ => panic!("legality should pass"),
    };
    let progressed = match handle.progress_declaration_checked(legal) {
        crate::application::ForgeQueryDeclarationProgressionChecked::Admitted(progressed) => {
            progressed
        }
        _ => panic!("progression should admit"),
    };
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    let route_plan = match handle.plan_routes_checked(
        ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    ) {
        ForgeQueryDeclarationRoutePlanChecked::Deferred(route) => route,
        _ => panic!("route plan should defer"),
    };
    match handle.receipt_routes_checked(ForgeQueryDeclarationReceiptInput::deferred(route_plan)) {
        ForgeQueryDeclarationReceiptChecked::Deferred(receipt) => {
            match handle
                .envelope_routes_checked(ForgeQueryDeclarationEnvelopeInput::deferred(receipt))
            {
                ForgeQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                    ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(
                        crate::application::ForgeQueryDeclarationEntryOrchestrationDeferred::new(
                            envelope.envelope().declaration_family_key(),
                            ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
                            envelope.reason(),
                            Some(digest_text(envelope.envelope().envelope_digest())),
                        ),
                    )
                }
                _ => panic!("deferred envelope should materialize"),
            }
        }
        _ => panic!("receipt should defer"),
    }
}

fn digest_text(digest: &forge_foundational::facade::CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
