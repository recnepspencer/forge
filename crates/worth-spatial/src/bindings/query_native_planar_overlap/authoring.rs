use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_overlap::domain::{
    CoplanarOverlapContractDeclarationFamily, CoplanarOverlapContractQueryDomain,
};
use crate::planar_contracts::coplanar_overlap_contract::{
    coplanar_overlap_contract_identity_entries, CoplanarOverlapContractBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CoplanarOverlapContractCase {
    basis: CoplanarOverlapContractBasis,
}

impl CoplanarOverlapContractCase {
    pub fn from_certified_face_pair(basis: CoplanarOverlapContractBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &CoplanarOverlapContractBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoplanarOverlapContractEntry {
    case: CoplanarOverlapContractCase,
}

impl CoplanarOverlapContractEntry {
    pub fn case(&self) -> &CoplanarOverlapContractCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<CoplanarOverlapContractQueryDomain>
    for CoplanarOverlapContractEntry
{
    type Family = CoplanarOverlapContractDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        coplanar_overlap_contract_identity_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn coplanar_overlap_contract_entry(
    case: CoplanarOverlapContractCase,
) -> CoplanarOverlapContractEntry {
    CoplanarOverlapContractEntry { case }
}
