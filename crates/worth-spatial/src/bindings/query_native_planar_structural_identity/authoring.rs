use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_structural_identity::domain::{
    PlanarStructuralIdentityDeclarationFamily, PlanarStructuralIdentityQueryDomain,
};
use crate::planar_contracts::structural_identity::{
    planar_structural_identity_authority_entries, PlanarStructuralIdentityBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarStructuralIdentityCase {
    basis: PlanarStructuralIdentityBasis,
}

impl PlanarStructuralIdentityCase {
    pub fn from_basis(basis: PlanarStructuralIdentityBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PlanarStructuralIdentityBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarStructuralIdentityEntry {
    case: PlanarStructuralIdentityCase,
}

impl PlanarStructuralIdentityEntry {
    pub fn case(&self) -> &PlanarStructuralIdentityCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarStructuralIdentityQueryDomain>
    for PlanarStructuralIdentityEntry
{
    type Family = PlanarStructuralIdentityDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        planar_structural_identity_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn planar_structural_identity_entry(
    case: PlanarStructuralIdentityCase,
) -> PlanarStructuralIdentityEntry {
    PlanarStructuralIdentityEntry { case }
}
