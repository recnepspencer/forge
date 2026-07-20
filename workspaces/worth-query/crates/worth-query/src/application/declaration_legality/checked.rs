use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

use super::{WorthQueryDeclarationLegalityDenial, WorthQueryDeclarationLegalityEvidence};

pub enum WorthQueryDeclarationLegalityChecked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Legal(WorthQueryDeclarationLegalityEvidence<D, I>),
    Illegal(WorthQueryDeclarationLegalityDenial<D, I>),
}
