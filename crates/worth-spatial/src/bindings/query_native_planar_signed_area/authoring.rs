use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_signed_area::domain::{
    CertifiedSignedArea2DDeclarationFamily, CertifiedSignedArea2DQueryDomain,
};
use crate::planar_contracts::signed_area_2d::{
    certified_signed_area_2d_identity_entries, CertifiedSignedArea2DBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSignedArea2DCase {
    basis: CertifiedSignedArea2DBasis,
}

impl CertifiedSignedArea2DCase {
    pub fn from_certified_planar_basis(basis: CertifiedSignedArea2DBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &CertifiedSignedArea2DBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSignedArea2DEntry {
    case: CertifiedSignedArea2DCase,
}

impl CertifiedSignedArea2DEntry {
    pub fn case(&self) -> &CertifiedSignedArea2DCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<CertifiedSignedArea2DQueryDomain> for CertifiedSignedArea2DEntry {
    type Family = CertifiedSignedArea2DDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        certified_signed_area_2d_identity_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn certified_signed_area_2d_entry(
    case: CertifiedSignedArea2DCase,
) -> CertifiedSignedArea2DEntry {
    CertifiedSignedArea2DEntry { case }
}
