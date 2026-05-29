use crate::application::{
    ForgeQueryDeclarationCanonicalizationError, ForgeQueryDeclarationCapabilityVerb,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFamilySupportReport,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityDenial, ForgeQueryDomainEntryMarker,
};

pub(super) fn declare_row_reason<
    D: ForgeQueryDomainEntryMarker,
    F: ForgeQueryDeclarationFamilyMarker<D>,
>(
    report: &ForgeQueryDeclarationFamilySupportReport<D, F>,
) -> &'static str {
    report
        .row(ForgeQueryDeclarationCapabilityVerb::Declare)
        .expect("declare row should exist")
        .reason()
}

pub(super) fn legality_denial_reason<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    denial: &ForgeQueryDeclarationLegalityDenial<D, I>,
) -> &'static str {
    match denial {
        ForgeQueryDeclarationLegalityDenial::WrongAdmittedWorld { .. } => {
            "declaration legality requires the same admitted world that produced the canonical declaration"
        }
        ForgeQueryDeclarationLegalityDenial::IllegalRoleClaim { .. } => {
            "the declaration role claim is illegal for this legality boundary"
        }
        ForgeQueryDeclarationLegalityDenial::IllegalSurfaceDisposition { .. } => {
            "the declaration surface disposition is illegal for this legality boundary"
        }
        ForgeQueryDeclarationLegalityDenial::DeferredByLegalityBoundary { .. } => {
            "declaration legality remains explicitly deferred"
        }
        ForgeQueryDeclarationLegalityDenial::UnsupportedLegalityClass { .. } => {
            "this declaration legality class is not yet admitted for generic orchestration"
        }
    }
}

pub(super) fn canonicalization_reason(
    error: &ForgeQueryDeclarationCanonicalizationError,
) -> &'static str {
    match error {
        ForgeQueryDeclarationCanonicalizationError::EmptyDeclarationEntries { .. } => {
            "declaration canonicalization requires at least one canonical declaration entry"
        }
        ForgeQueryDeclarationCanonicalizationError::BasisConstructionDenied(_) => {
            "declaration canonicalization basis construction was denied"
        }
        ForgeQueryDeclarationCanonicalizationError::DigestDerivationDenied(_) => {
            "declaration canonicalization digest derivation was denied"
        }
        ForgeQueryDeclarationCanonicalizationError::ComparisonPreparationFailed => {
            "declaration canonicalization comparison preparation failed"
        }
    }
}
