#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityRequirementVocabulary {
    KeyScopeRequired,
    TenantScopeRequired,
    AuthenticityRequirementDeclared,
    CustodyPostureRequired,
    LegacyPostureRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityResultVocabulary {
    AuthenticityObservedResult,
    ScopeAdmissionResult,
    CustodyAdmissionResult,
    LegacyReadmissionResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityWitnessVocabularyTerm {
    KeyScopeWitness,
    TenantScopeWitness,
    AuthenticityWitness,
    CustodyWitness,
    RepairBlastRadiusWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityWitnessVocabulary {
    term: StoreSecurityWitnessVocabularyTerm,
}

impl StoreSecurityWitnessVocabulary {
    pub const fn term(self) -> StoreSecurityWitnessVocabularyTerm {
        self.term
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityEvidenceVocabulary {
    PublishableBoundaryEvidence,
    CertificationEvidence,
    CounterBackedEvidence,
    ReadmissionEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreSecurityReadinessVocabularyTerm {
    IoQosSecurityScopeReadiness,
    BlobSecurityScopeReadiness,
    BackupExportCustodyReadiness,
    RepairBlastRadiusReadiness,
    SecurityFoundationReadiness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreSecurityReadinessVocabulary {
    term: StoreSecurityReadinessVocabularyTerm,
}

impl StoreSecurityReadinessVocabulary {
    pub const fn term(self) -> StoreSecurityReadinessVocabularyTerm {
        self.term
    }
}
