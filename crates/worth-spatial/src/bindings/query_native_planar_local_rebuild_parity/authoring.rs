use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_local_rebuild_parity::domain::{
    PlanarLocalRebuildParityDeclarationFamily, PlanarLocalRebuildParityQueryDomain,
};
use crate::planar_contracts::local_rebuild_parity::{
    planar_local_rebuild_parity_authority_entries, PlanarLocalRebuildParityBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalRebuildParityCase {
    basis: PlanarLocalRebuildParityBasis,
}

impl PlanarLocalRebuildParityCase {
    pub fn from_basis(basis: PlanarLocalRebuildParityBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PlanarLocalRebuildParityBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalRebuildParityEntry {
    case: PlanarLocalRebuildParityCase,
}

impl PlanarLocalRebuildParityEntry {
    pub fn case(&self) -> &PlanarLocalRebuildParityCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarLocalRebuildParityQueryDomain>
    for PlanarLocalRebuildParityEntry
{
    type Family = PlanarLocalRebuildParityDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        planar_local_rebuild_parity_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn planar_local_rebuild_parity_entry(
    case: PlanarLocalRebuildParityCase,
) -> PlanarLocalRebuildParityEntry {
    PlanarLocalRebuildParityEntry { case }
}
