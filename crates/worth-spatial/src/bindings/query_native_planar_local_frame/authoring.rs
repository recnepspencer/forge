use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_local_frame::domain::{
    PlanarLocalFrameCertificateDeclarationFamily, PlanarLocalFrameCertificateQueryDomain,
};
use crate::planar_contracts::local_frame::{
    planar_local_frame_basis_identity_entries, PlanarLocalFrameBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalFrameCertificateCase {
    basis: PlanarLocalFrameBasis,
}

impl PlanarLocalFrameCertificateCase {
    pub fn from_precision_basis(basis: PlanarLocalFrameBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PlanarLocalFrameBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarLocalFrameCertificateEntry {
    case: PlanarLocalFrameCertificateCase,
}

impl PlanarLocalFrameCertificateEntry {
    pub fn case(&self) -> &PlanarLocalFrameCertificateCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarLocalFrameCertificateQueryDomain>
    for PlanarLocalFrameCertificateEntry
{
    type Family = PlanarLocalFrameCertificateDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let basis = self.case.basis();
        planar_local_frame_basis_identity_entries(basis)
            .into_iter()
            .map(|identity| entry(identity.query_locus(), identity.value()))
            .collect()
    }
}

pub fn planar_local_frame_certificate_entry(
    case: PlanarLocalFrameCertificateCase,
) -> PlanarLocalFrameCertificateEntry {
    PlanarLocalFrameCertificateEntry { case }
}

fn entry(key: impl Into<String>, value: impl Into<String>) -> ForgeQueryDeclarationCanonicalEntry {
    ForgeQueryDeclarationCanonicalEntry::text(key, value)
}
