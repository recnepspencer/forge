use crate::application::ForgeQueryDomainEntryMarker;

use super::input::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryRawDeclarationInput<D: ForgeQueryDomainEntryMarker, I> {
    declaration_family: &'static str,
    canonical_entries: Vec<ForgeQueryDeclarationCanonicalEntry>,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D, I> ForgeQueryRawDeclarationInput<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    pub(crate) fn new(marker: D, input: I) -> Self {
        let declaration_family = input.declaration_family();
        let canonical_entries = input.canonical_entries();
        let _ = marker;
        let _ = input;
        Self {
            declaration_family,
            canonical_entries,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn declaration_family(&self) -> &'static str {
        self.declaration_family
    }

    pub(crate) fn canonical_entries(&self) -> &[ForgeQueryDeclarationCanonicalEntry] {
        &self.canonical_entries
    }
}
