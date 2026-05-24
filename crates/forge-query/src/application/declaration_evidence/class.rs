use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::{
    artifact::ForgeQueryDeclarationFoundationalEvidence,
    denial::ForgeQueryDeclarationFoundationalEvidenceDenial,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationFoundationalEvidenceClass {
    LegalityAdmitted,
    LegalityDenied,
    ProgressionAdmitted,
    ProgressionDeferred,
    ProgressionDenied,
    ProgressionStale,
    ProgressionRebindRequired,
    ProgressionFailed,
}

pub enum ForgeQueryDeclarationFoundationalEvidenceChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Described(ForgeQueryDeclarationFoundationalEvidence<D, I>),
    ConstructionDenied(ForgeQueryDeclarationFoundationalEvidenceDenial<D, I>),
}
