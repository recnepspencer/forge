use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_winding::domain::{
    CertifiedPolygonWinding2DDeclarationFamily, CertifiedPolygonWinding2DQueryDomain,
};
use crate::planar_contracts::polygon_winding_2d::{
    certified_polygon_winding_2d_identity_entries, CertifiedPolygonWinding2DBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedPolygonWinding2DCase {
    basis: CertifiedPolygonWinding2DBasis,
}

impl CertifiedPolygonWinding2DCase {
    pub fn from_projected_loops(basis: CertifiedPolygonWinding2DBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &CertifiedPolygonWinding2DBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedPolygonWinding2DEntry {
    case: CertifiedPolygonWinding2DCase,
}

impl CertifiedPolygonWinding2DEntry {
    pub fn case(&self) -> &CertifiedPolygonWinding2DCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<CertifiedPolygonWinding2DQueryDomain>
    for CertifiedPolygonWinding2DEntry
{
    type Family = CertifiedPolygonWinding2DDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        certified_polygon_winding_2d_identity_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn certified_polygon_winding_2d_entry(
    case: CertifiedPolygonWinding2DCase,
) -> CertifiedPolygonWinding2DEntry {
    CertifiedPolygonWinding2DEntry { case }
}
