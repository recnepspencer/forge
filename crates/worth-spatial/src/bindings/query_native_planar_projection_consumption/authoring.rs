use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_projection_consumption::domain::{
    ProjectionConsumedPlanarFactsDeclarationFamily, ProjectionConsumedPlanarFactsQueryDomain,
};
use crate::planar_contracts::projection_consumed_facts::{
    projection_consumed_planar_fact_authority_entries, ProjectionConsumedPlanarFactsBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectionConsumedPlanarFactsCase {
    basis: ProjectionConsumedPlanarFactsBasis,
}

impl ProjectionConsumedPlanarFactsCase {
    pub fn from_basis(basis: ProjectionConsumedPlanarFactsBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &ProjectionConsumedPlanarFactsBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectionConsumedPlanarFactsEntry {
    case: ProjectionConsumedPlanarFactsCase,
}

impl ProjectionConsumedPlanarFactsEntry {
    pub fn case(&self) -> &ProjectionConsumedPlanarFactsCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<ProjectionConsumedPlanarFactsQueryDomain>
    for ProjectionConsumedPlanarFactsEntry
{
    type Family = ProjectionConsumedPlanarFactsDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        projection_consumed_planar_fact_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn projection_consumed_planar_facts_entry(
    case: ProjectionConsumedPlanarFactsCase,
) -> ProjectionConsumedPlanarFactsEntry {
    ProjectionConsumedPlanarFactsEntry { case }
}
