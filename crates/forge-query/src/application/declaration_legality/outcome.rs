use crate::application::{
    ForgeQueryDeclarationAdmissionError, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

use super::ForgeQueryDeclarationLegalityDenial;

pub enum ForgeQueryDeclarationAdmissionOrLegalityError<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Admission(ForgeQueryDeclarationAdmissionError<D, I>),
    Legality(ForgeQueryDeclarationLegalityDenial<D, I>),
}
