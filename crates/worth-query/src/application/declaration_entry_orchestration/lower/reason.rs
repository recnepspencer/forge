use crate::application::{
    WorthQueryDeclarationCanonicalizationError, WorthQueryDeclarationCapabilityVerb,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFamilySupportReport,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityDenial, WorthQueryDomainEntryMarker,
};

pub(super) fn declare_row_reason<
    D: WorthQueryDomainEntryMarker,
    F: WorthQueryDeclarationFamilyMarker<D>,
>(
    report: &WorthQueryDeclarationFamilySupportReport<D, F>,
) -> &'static str {
    report
        .row(WorthQueryDeclarationCapabilityVerb::Declare)
        .expect("declare row should exist")
        .reason()
}

pub(super) fn legality_denial_reason<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    denial: &WorthQueryDeclarationLegalityDenial<D, I>,
) -> &'static str {
    match denial {
        WorthQueryDeclarationLegalityDenial::WrongAdmittedWorld { .. } => {
            "declaration legality requires the same admitted world that produced the canonical declaration"
        }
        WorthQueryDeclarationLegalityDenial::IllegalRoleClaim { .. } => {
            "the declaration role claim is illegal for this legality boundary"
        }
        WorthQueryDeclarationLegalityDenial::IllegalSurfaceDisposition { .. } => {
            "the declaration surface disposition is illegal for this legality boundary"
        }
        WorthQueryDeclarationLegalityDenial::DeferredByLegalityBoundary { .. } => {
            "declaration legality remains explicitly deferred"
        }
        WorthQueryDeclarationLegalityDenial::UnsupportedLegalityClass { .. } => {
            "this declaration legality class is not yet admitted for generic orchestration"
        }
        WorthQueryDeclarationLegalityDenial::TemporalProjectionUnsupported { kind, .. } => {
            kind.reason()
        }
        WorthQueryDeclarationLegalityDenial::AsyncProjectionUnsupported { kind, .. } => {
            kind.reason()
        }
    }
}

pub(super) fn canonicalization_reason(
    error: &WorthQueryDeclarationCanonicalizationError,
) -> &'static str {
    match error {
        WorthQueryDeclarationCanonicalizationError::EmptyDeclarationEntries { .. } => {
            "declaration canonicalization requires at least one canonical declaration entry"
        }
        WorthQueryDeclarationCanonicalizationError::BasisConstructionDenied(_) => {
            "declaration canonicalization basis construction was denied"
        }
        WorthQueryDeclarationCanonicalizationError::DigestDerivationDenied(_) => {
            "declaration canonicalization digest derivation was denied"
        }
        WorthQueryDeclarationCanonicalizationError::ComparisonPreparationFailed => {
            "declaration canonicalization comparison preparation failed"
        }
    }
}
