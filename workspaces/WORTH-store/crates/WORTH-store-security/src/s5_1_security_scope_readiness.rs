use crate::{
    StoreAdmittedSecurityScope, StoreCurrentSecurityScopeWitnessSet,
    StoreSecurityReadinessVocabularyTerm, StoreSecurityScopeAdmissionReceipt,
};

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

#[derive(Debug, PartialEq, Eq)]
pub struct S51AdmittedSecurityScopeReadiness {
    reservation: S51SecurityScopeReadinessReservation,
    witnesses: StoreCurrentSecurityScopeWitnessSet,
    receipt: StoreSecurityScopeAdmissionReceipt,
}

impl S51AdmittedSecurityScopeReadiness {
    pub fn from_admitted_security_scope(
        reservation: S51SecurityScopeReadinessReservation,
        admitted_scope: StoreAdmittedSecurityScope,
    ) -> Self {
        let receipt = admitted_scope.receipt();
        Self {
            reservation,
            witnesses: admitted_scope.into_witnesses_for_readiness_handoff(),
            receipt,
        }
    }

    pub const fn reservation(&self) -> S51SecurityScopeReadinessReservation {
        self.reservation
    }

    pub const fn witnesses(&self) -> &StoreCurrentSecurityScopeWitnessSet {
        &self.witnesses
    }

    pub const fn receipt(&self) -> StoreSecurityScopeAdmissionReceipt {
        self.receipt
    }
}

pub fn accept_s5_1_admitted_security_scope_readiness(
    reservation: S51SecurityScopeReadinessReservation,
    admitted_scope: StoreAdmittedSecurityScope,
) -> S51AdmittedSecurityScopeReadiness {
    S51AdmittedSecurityScopeReadiness::from_admitted_security_scope(reservation, admitted_scope)
}
