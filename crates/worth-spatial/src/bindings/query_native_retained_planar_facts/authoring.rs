use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_retained_planar_facts::domain::{
    RetainedPlanarFactsDeclarationFamily, RetainedPlanarFactsQueryDomain,
};
use crate::planar_contracts::retained_planar_facts::{
    retained_planar_fact_authority_entries, RetainedPlanarFactsBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedPlanarFactsCase {
    basis: RetainedPlanarFactsBasis,
}

impl RetainedPlanarFactsCase {
    pub fn from_basis(basis: RetainedPlanarFactsBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &RetainedPlanarFactsBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedPlanarFactsEntry {
    case: RetainedPlanarFactsCase,
}

impl RetainedPlanarFactsEntry {
    pub fn case(&self) -> &RetainedPlanarFactsCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<RetainedPlanarFactsQueryDomain> for RetainedPlanarFactsEntry {
    type Family = RetainedPlanarFactsDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        retained_planar_fact_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn retained_planar_facts_entry(case: RetainedPlanarFactsCase) -> RetainedPlanarFactsEntry {
    RetainedPlanarFactsEntry { case }
}
