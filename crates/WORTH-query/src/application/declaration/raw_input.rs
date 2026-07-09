use crate::application::{WorthQueryDeclarationFamilyMarker, WorthQueryDomainEntryMarker};

use super::async_resource::WorthQueryAsyncDeclarationClause;
use super::async_resource::{async_resource_entries, normalize_async_resource_clauses};
use super::input::{WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationInput};
use super::temporal::WorthQueryTemporalDeclarationClause;
use super::temporal::{normalize_temporal_clauses, temporal_entries};
use crate::application::WorthQueryDeclarationFamilyTaxonomy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryRawDeclarationInput<D: WorthQueryDomainEntryMarker, I> {
    declaration_family_key: &'static str,
    declaration_taxonomy: WorthQueryDeclarationFamilyTaxonomy,
    async_resource_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
    canonical_entries: Vec<WorthQueryDeclarationCanonicalEntry>,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D, I> WorthQueryRawDeclarationInput<D, I>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    pub(crate) fn new(input: I) -> Self {
        let declaration_family_key = I::Family::semantic_family_key();
        let declaration_taxonomy = I::Family::taxonomy();
        let mut canonical_entries = input.canonical_declaration_entries();
        let async_resource_clauses =
            normalize_async_resource_clauses(input.async_resource_declaration_clauses());
        canonical_entries.extend(async_resource_entries(&async_resource_clauses));
        let temporal_clauses = normalize_temporal_clauses(input.temporal_declaration_clauses());
        canonical_entries.extend(temporal_entries(&temporal_clauses));
        let _ = input;
        Self {
            declaration_family_key,
            declaration_taxonomy,
            async_resource_clauses,
            temporal_clauses,
            canonical_entries,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub(crate) fn declaration_taxonomy(&self) -> WorthQueryDeclarationFamilyTaxonomy {
        self.declaration_taxonomy
    }

    pub(crate) fn async_resource_clauses(&self) -> &[WorthQueryAsyncDeclarationClause] {
        &self.async_resource_clauses
    }

    pub(crate) fn temporal_clauses(&self) -> &[WorthQueryTemporalDeclarationClause] {
        &self.temporal_clauses
    }

    pub(crate) fn canonical_entries(&self) -> &[WorthQueryDeclarationCanonicalEntry] {
        &self.canonical_entries
    }
}
