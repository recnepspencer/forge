use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::{
    artifact::WorthQueryDeclarationFoundationalEvidence,
    denial::WorthQueryDeclarationFoundationalEvidenceDenial,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationFoundationalEvidenceClass {
    LegalityAdmitted,
    LegalityDenied,
    ProgressionAdmitted,
    ProgressionDeferred,
    ProgressionDenied,
    ProgressionStale,
    ProgressionRebindRequired,
    ProgressionFailed,
}

pub enum WorthQueryDeclarationFoundationalEvidenceChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Described(WorthQueryDeclarationFoundationalEvidence<D, I>),
    ConstructionDenied(WorthQueryDeclarationFoundationalEvidenceDenial<D, I>),
}
