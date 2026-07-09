use worth_store_security::{
    StoreKeyScope, StoreRawSecurityScopeDeclaration, StoreSecurityScopeDeclarationProvenance,
    StoreTenantScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineBlobDamageCaseHint {
    ChecksumMismatch,
    AuthenticityFailure,
    MissingChunk,
    StaleGeneration,
    CrossScopeImport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineBlobCorruptionClassification {
    observation: OfflineBlobCorruptionObservation,
    damage_case_hint: OfflineBlobDamageCaseHint,
}

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
        verify_raw_unadmitted_provenance(&raw_declaration)?;
        Ok(Self {
            raw_declaration,
            evidence_kind,
        })
    }

    pub fn admit_and_classify_offline_corruption_report(
        raw_declaration: StoreRawSecurityScopeDeclaration,
        evidence_kind: OfflineBlobCorruptionEvidenceKind,
    ) -> Result<OfflineBlobCorruptionClassification, OfflineBlobCorruptionObservationDenial> {
        let observation = Self::from_offline_corruption_report(raw_declaration, evidence_kind)?;
        let damage_case_hint = classify_offline_damage_case(
            observation.evidence_kind(),
            observation.raw_declaration().tenant_scope(),
            observation.raw_declaration().key_scope(),
        );
        Ok(OfflineBlobCorruptionClassification {
            observation,
            damage_case_hint,
        })
    }

    pub const fn raw_declaration(&self) -> StoreRawSecurityScopeDeclaration {
        self.raw_declaration
    }

    pub const fn evidence_kind(&self) -> OfflineBlobCorruptionEvidenceKind {
        self.evidence_kind
    }
}

impl OfflineBlobCorruptionClassification {
    pub const fn observation(&self) -> &OfflineBlobCorruptionObservation {
        &self.observation
    }

    pub const fn damage_case_hint(&self) -> OfflineBlobDamageCaseHint {
        self.damage_case_hint
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

fn verify_raw_unadmitted_provenance(
    raw_declaration: &StoreRawSecurityScopeDeclaration,
) -> Result<(), OfflineBlobCorruptionObservationDenial> {
    match raw_declaration.provenance() {
        StoreSecurityScopeDeclarationProvenance::DeserializedUnadmitted => Ok(()),
        _ => Err(OfflineBlobCorruptionObservationDenial::NotRawReportInput),
    }
}

pub fn classify_offline_damage_case(
    evidence_kind: OfflineBlobCorruptionEvidenceKind,
    tenant_scope: StoreTenantScope,
    _key_scope: StoreKeyScope,
) -> OfflineBlobDamageCaseHint {
    match evidence_kind {
        OfflineBlobCorruptionEvidenceKind::Import
            if tenant_scope == StoreTenantScope::ImportReadmissionBoundary =>
        {
            OfflineBlobDamageCaseHint::CrossScopeImport
        }
        OfflineBlobCorruptionEvidenceKind::ColdFetch => OfflineBlobDamageCaseHint::MissingChunk,
        OfflineBlobCorruptionEvidenceKind::QuarantineReport => {
            OfflineBlobDamageCaseHint::StaleGeneration
        }
        OfflineBlobCorruptionEvidenceKind::Read | OfflineBlobCorruptionEvidenceKind::Scrub => {
            OfflineBlobDamageCaseHint::ChecksumMismatch
        }
        OfflineBlobCorruptionEvidenceKind::PartialCapsuleMaterialization => {
            OfflineBlobDamageCaseHint::AuthenticityFailure
        }
        _ => OfflineBlobDamageCaseHint::ChecksumMismatch,
    }
}