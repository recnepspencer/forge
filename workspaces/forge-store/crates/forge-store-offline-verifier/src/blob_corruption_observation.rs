use forge_store_security::{
    StoreRawSecurityScopeDeclaration, StoreSecurityScopeDeclarationProvenance,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineBlobCorruptionObservation {
    raw_declaration: StoreRawSecurityScopeDeclaration,
    evidence_kind: OfflineBlobCorruptionEvidenceKind,
}

impl OfflineBlobCorruptionObservation {
    pub fn from_offline_corruption_report(
        raw_declaration: StoreRawSecurityScopeDeclaration,
        evidence_kind: OfflineBlobCorruptionEvidenceKind,
    ) -> Result<Self, OfflineBlobCorruptionObservationDenial> {
        match raw_declaration.provenance() {
            StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => Ok(Self {
                raw_declaration,
                evidence_kind,
            }),
            _ => Err(OfflineBlobCorruptionObservationDenial::NotRawReportInput),
        }
    }

    pub const fn raw_declaration(&self) -> StoreRawSecurityScopeDeclaration {
        self.raw_declaration
    }

    pub const fn evidence_kind(&self) -> OfflineBlobCorruptionEvidenceKind {
        self.evidence_kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineBlobCorruptionEvidenceKind {
    Read,
    Scrub,
    ColdFetch,
    Import,
    PartialCapsuleMaterialization,
    QuarantineReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineBlobCorruptionObservationDenial {
    NotRawReportInput,
}
