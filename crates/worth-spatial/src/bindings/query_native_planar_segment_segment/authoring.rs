use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_segment_segment::domain::{
    CertifiedSegmentSegment2DDeclarationFamily, CertifiedSegmentSegment2DQueryDomain,
};
use crate::planar_contracts::segment_segment_2d::{
    certified_segment_segment_2d_identity_entries, CertifiedSegmentSegment2DBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSegmentSegment2DCase {
    basis: CertifiedSegmentSegment2DBasis,
}

impl CertifiedSegmentSegment2DCase {
    pub fn from_projected_segments(basis: CertifiedSegmentSegment2DBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &CertifiedSegmentSegment2DBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSegmentSegment2DEntry {
    case: CertifiedSegmentSegment2DCase,
}

impl CertifiedSegmentSegment2DEntry {
    pub fn case(&self) -> &CertifiedSegmentSegment2DCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<CertifiedSegmentSegment2DQueryDomain>
    for CertifiedSegmentSegment2DEntry
{
    type Family = CertifiedSegmentSegment2DDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        certified_segment_segment_2d_identity_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn certified_segment_segment_2d_entry(
    case: CertifiedSegmentSegment2DCase,
) -> CertifiedSegmentSegment2DEntry {
    CertifiedSegmentSegment2DEntry { case }
}
