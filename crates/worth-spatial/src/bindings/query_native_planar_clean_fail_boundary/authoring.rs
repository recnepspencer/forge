use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_clean_fail_boundary::domain::{
    PlanarCleanFailBoundaryDeclarationFamily, PlanarCleanFailBoundaryQueryDomain,
};
use crate::planar_contracts::clean_fail_boundary::{
    planar_clean_fail_boundary_authority_entries, PlanarCleanFailBoundaryBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarCleanFailBoundaryCase {
    basis: PlanarCleanFailBoundaryBasis,
}

impl PlanarCleanFailBoundaryCase {
    pub fn from_basis(basis: PlanarCleanFailBoundaryBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PlanarCleanFailBoundaryBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarCleanFailBoundaryEntry {
    case: PlanarCleanFailBoundaryCase,
}

impl PlanarCleanFailBoundaryEntry {
    pub fn case(&self) -> &PlanarCleanFailBoundaryCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarCleanFailBoundaryQueryDomain>
    for PlanarCleanFailBoundaryEntry
{
    type Family = PlanarCleanFailBoundaryDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        planar_clean_fail_boundary_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn planar_clean_fail_boundary_entry(
    case: PlanarCleanFailBoundaryCase,
) -> PlanarCleanFailBoundaryEntry {
    PlanarCleanFailBoundaryEntry { case }
}
