use forge_store_security::{
    StoreRawSecurityScopeDeclaration, StoreRepairPhysicalRegionDeclaration,
    StoreSecurityScopeDeclarationProvenance,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineRepairBlastRadiusObservation {
    raw_declaration: StoreRawSecurityScopeDeclaration,
    physical_region: StoreRepairPhysicalRegionDeclaration,
    evidence_kind: OfflineRepairEvidenceKind,
}

impl OfflineRepairBlastRadiusObservation {
    pub fn from_offline_repair_report(
        raw_declaration: StoreRawSecurityScopeDeclaration,
        physical_region: StoreRepairPhysicalRegionDeclaration,
        evidence_kind: OfflineRepairEvidenceKind,
    ) -> Result<Self, OfflineRepairBlastRadiusObservationDenial> {
        match raw_declaration.provenance() {
            StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => Ok(Self {
                raw_declaration,
                physical_region,
                evidence_kind,
            }),
            _ => Err(OfflineRepairBlastRadiusObservationDenial::NotRawReportInput),
        }
    }

    pub const fn raw_declaration(&self) -> StoreRawSecurityScopeDeclaration {
        self.raw_declaration
    }

    pub fn physical_region(&self) -> &StoreRepairPhysicalRegionDeclaration {
        &self.physical_region
    }

    pub const fn evidence_kind(&self) -> OfflineRepairEvidenceKind {
        self.evidence_kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineRepairEvidenceKind {
    SupportTruth,
    DegradedRecovery,
    QuarantineReport,
    RepairReadCloseout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineRepairBlastRadiusObservationDenial {
    NotRawReportInput,
}
