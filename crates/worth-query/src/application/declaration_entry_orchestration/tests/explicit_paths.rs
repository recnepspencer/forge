use crate::application::{
    WorthQueryDeclarationEntryOrchestrationOutcome, WorthQueryDeclarationEntryOrchestrationStage,
    WorthQueryDeclarationEnvelopeChecked, WorthQueryDeclarationEnvelopeInput,
    WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationReceiptChecked,
    WorthQueryDeclarationReceiptInput, WorthQueryDeclarationRoutePlanChecked,
    WorthQueryDeclarationRoutePlanInput,
};
use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use super::domain::{
    AdmittedFamily, CollaborativeWorld, DeferredRouteFamily, GeometryDomain, Input,
};

pub(super) fn explicit_success_path_parity(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<
        GeometryDomain,
        CollaborativeWorld,
    >,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<GeometryDomain, Input<AdmittedFamily>> {
    let declaration = match handle.declare_checked(Input::<AdmittedFamily>::new("edge:42")) {
        crate::application::WorthQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("declaration should admit"),
    };
    let legal = match handle.review_legality_checked(declaration) {
        crate::application::WorthQueryDeclarationLegalityChecked::Legal(legal) => legal,
        _ => panic!("legality should pass"),
    };
    let progressed = match handle.progress_declaration_checked(legal) {
        crate::application::WorthQueryDeclarationProgressionChecked::Admitted(progressed) => {
            progressed
        }
        _ => panic!("progression should admit"),
    };
    let evidence = handle
        .describe_foundational_with_profile(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    let route_plan = match handle.plan_routes_checked(
        WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    ) {
        WorthQueryDeclarationRoutePlanChecked::Planned(plan) => plan,
        _ => panic!("route plan should materialize"),
    };
    let receipt = match handle
        .receipt_routes_checked(WorthQueryDeclarationReceiptInput::planned(route_plan))
    {
        WorthQueryDeclarationReceiptChecked::Issued(receipt) => receipt,
        _ => panic!("receipt should issue"),
    };
    match handle.envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::issued(receipt)) {
        WorthQueryDeclarationEnvelopeChecked::Enveloped(envelope) => {
            WorthQueryDeclarationEntryOrchestrationOutcome::Enveloped(envelope)
        }
        _ => panic!("envelope should materialize"),
    }
}

pub(super) fn explicit_deferred_route_path_parity(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<
        GeometryDomain,
        CollaborativeWorld,
    >,
) -> WorthQueryDeclarationEntryOrchestrationOutcome<GeometryDomain, Input<DeferredRouteFamily>> {
    let declaration = match handle.declare_checked(Input::<DeferredRouteFamily>::new("edge:42")) {
        crate::application::WorthQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("declaration should admit"),
    };
    let legal = match handle.review_legality_checked(declaration) {
        crate::application::WorthQueryDeclarationLegalityChecked::Legal(legal) => legal,
        _ => panic!("legality should pass"),
    };
    let progressed = match handle.progress_declaration_checked(legal) {
        crate::application::WorthQueryDeclarationProgressionChecked::Admitted(progressed) => {
            progressed
        }
        _ => panic!("progression should admit"),
    };
    let evidence = handle
        .describe_foundational_with_profile(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
            FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics,
        )
        .unwrap_or_else(|_| panic!("foundational evidence should materialize"));
    let route_plan = match handle.plan_routes_checked(
        WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence),
    ) {
        WorthQueryDeclarationRoutePlanChecked::Deferred(route) => route,
        _ => panic!("route plan should defer"),
    };
    match handle.receipt_routes_checked(WorthQueryDeclarationReceiptInput::deferred(route_plan)) {
        WorthQueryDeclarationReceiptChecked::Deferred(receipt) => {
            match handle
                .envelope_routes_checked(WorthQueryDeclarationEnvelopeInput::deferred(receipt))
            {
                WorthQueryDeclarationEnvelopeChecked::Deferred(envelope) => {
                    WorthQueryDeclarationEntryOrchestrationOutcome::Deferred(
                        crate::application::WorthQueryDeclarationEntryOrchestrationDeferred::new(
                            envelope.envelope().declaration_family_key(),
                            WorthQueryDeclarationEntryOrchestrationStage::ReceiptIssued,
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

fn digest_text(digest: &worth_foundational::facade::CanonicalDerivedDigest) -> String {
    let hex = digest
        .value()
        .bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{}:{hex}", digest.metadata().algorithm().id().as_str())
}
