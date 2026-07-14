use crate::application::{
    WorthQueryDeclarationAdmissionError, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

use super::WorthQueryDeclarationLegalityDenial;

pub enum WorthQueryDeclarationAdmissionOrLegalityError<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Admission(WorthQueryDeclarationAdmissionError<D, I>),
    Legality(WorthQueryDeclarationLegalityDenial<D, I>),
}
