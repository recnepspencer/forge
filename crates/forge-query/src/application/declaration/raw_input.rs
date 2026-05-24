use crate::application::{ForgeQueryDeclarationFamilyMarker, ForgeQueryDomainEntryMarker};

use super::input::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};
use crate::application::ForgeQueryDeclarationFamilyTaxonomy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryRawDeclarationInput<D: ForgeQueryDomainEntryMarker, I> {
    declaration_family_key: &'static str,
    declaration_taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
    canonical_entries: Vec<ForgeQueryDeclarationCanonicalEntry>,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D, I> ForgeQueryRawDeclarationInput<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    pub(crate) fn new(input: I) -> Self {
        let declaration_family_key = I::Family::semantic_family_key();
        let declaration_taxonomy = I::Family::taxonomy();
        let canonical_entries = input.canonical_declaration_entries();
        let _ = input;
        Self {
            declaration_family_key,
            declaration_taxonomy,
            canonical_entries,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub(crate) fn declaration_taxonomy(&self) -> ForgeQueryDeclarationFamilyTaxonomy {
        self.declaration_taxonomy
    }

    pub(crate) fn canonical_entries(&self) -> &[ForgeQueryDeclarationCanonicalEntry] {
        &self.canonical_entries
    }
}
