use forge_store_security::StoreSecurityReadinessVocabularyTerm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S51SecurityScopeReadinessFamily {
    IoQos,
    BlobChunk,
    BackupExportCustody,
    RepairBlastRadius,
    SecurityFoundation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51SecurityScopeReadinessReservation {
    family: S51SecurityScopeReadinessFamily,
    vocabulary: StoreSecurityReadinessVocabularyTerm,
}

impl S51SecurityScopeReadinessReservation {
    pub const fn io_qos() -> Self {
        Self::new(
            S51SecurityScopeReadinessFamily::IoQos,
            StoreSecurityReadinessVocabularyTerm::S6IoQosSecurityScopeReadiness,
        )
    }

    pub const fn blob_chunk() -> Self {
        Self::new(
            S51SecurityScopeReadinessFamily::BlobChunk,
            StoreSecurityReadinessVocabularyTerm::S7BlobSecurityScopeReadiness,
        )
    }

    pub const fn backup_export_custody() -> Self {
        Self::new(
            S51SecurityScopeReadinessFamily::BackupExportCustody,
            StoreSecurityReadinessVocabularyTerm::S10BackupExportCustodyReadiness,
        )
    }

    pub const fn repair_blast_radius() -> Self {
        Self::new(
            S51SecurityScopeReadinessFamily::RepairBlastRadius,
            StoreSecurityReadinessVocabularyTerm::S10RepairBlastRadiusReadiness,
        )
    }

    pub const fn security_foundation() -> Self {
        Self::new(
            S51SecurityScopeReadinessFamily::SecurityFoundation,
            StoreSecurityReadinessVocabularyTerm::S11SecurityFoundationReadiness,
        )
    }

    pub const fn family(self) -> S51SecurityScopeReadinessFamily {
        self.family
    }

    pub const fn vocabulary(self) -> StoreSecurityReadinessVocabularyTerm {
        self.vocabulary
    }

    const fn new(
        family: S51SecurityScopeReadinessFamily,
        vocabulary: StoreSecurityReadinessVocabularyTerm,
    ) -> Self {
        Self { family, vocabulary }
    }
}
