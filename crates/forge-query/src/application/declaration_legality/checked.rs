use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};

use super::{ForgeQueryDeclarationLegalityDenial, ForgeQueryDeclarationLegalityEvidence};

pub enum ForgeQueryDeclarationLegalityChecked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Legal(ForgeQueryDeclarationLegalityEvidence<D, I>),
    Illegal(ForgeQueryDeclarationLegalityDenial<D, I>),
}
