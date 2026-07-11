use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_contracts::{DurableArtifactFamilyId, StableDigest};
use forge_store_physical_integrity::{
    DamageClassification, PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceDenial,
    PhysicalIntegrityEvidenceProfile, QuarantineHandoffPosture, QuarantineRecord,
    StoreExecutedIntegrityEvidence,
};

use crate::{
    verify_store_authority_for_readmission, PersistedRecoveryArtifactDigest,
    RecoveryIntegrityHandoffReceipt, ReopenedRecoveryArtifactAdmission, IntegrityHandoffDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedReadmissionLayoutRule {
    _private: (),
}

impl AdmittedReadmissionLayoutRule {
    pub(crate) const fn internal_phase22() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase22-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase22() -> Self {
        Self::internal_phase22()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryLayoutReadmissionClass {
    QuarantineRecovery,
    ImportBoundaryReadmission,
    OfflineVerifiedArtifact,
    NoForegroundAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryLayoutReadmissionIdentity {
    QuarantineReceipt(StableDigest),
    OfflineArtifactDigest(PersistedRecoveryArtifactDigest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLayoutReadmissionWitness {
    family_id: DurableArtifactFamilyId,
    class: RecoveryLayoutReadmissionClass,
    identity: RecoveryLayoutReadmissionIdentity,
}

impl RecoveryLayoutReadmissionWitness {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn class(&self) -> RecoveryLayoutReadmissionClass {
        self.class
    }

    pub const fn identity(&self) -> &RecoveryLayoutReadmissionIdentity {
        &self.identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryLayoutReadmissionAdmissionDenial {
    NoForegroundAuthority,
    UnexpectedOfflineClassification,
    QuarantineReceiptEvidence(PhysicalIntegrityEvidenceDenial),
    QuarantineHandoff(IntegrityHandoffDenial),
    StoreAuthority(crate::BlobReplayAdmissionDenial),
    DerivedProjectionDamageCannotReadmit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadmissionLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadmissionLayoutAdmission {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryReadmissionLayoutFamilyHome;

impl ReadmissionLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        self,
        _rule: &AdmittedReadmissionLayoutRule,
    ) -> Result<ReadmissionLayoutAdmission, RecoveryLayoutReadmissionAdmissionDenial> {
        Ok(ReadmissionLayoutAdmission { _private: () })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedReadmissionLayoutFamily {
    _admission: ReadmissionLayoutAdmission,
}

impl AdmittedReadmissionLayoutFamily {
    pub(crate) const fn new(admission: ReadmissionLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn admit_record_backed_witness(
        &self,
        family_id: DurableArtifactFamilyId,
        record: &QuarantineRecord,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<RecoveryLayoutReadmissionWitness, RecoveryLayoutReadmissionAdmissionDenial> {
        recovery_readmission_layout_family().admit_record_backed_witness(
            family_id,
            record,
            current_store_authority,
        )
    }

    pub fn admit_offline_witness(
        &self,
        family_id: DurableArtifactFamilyId,
        admission: &ReopenedRecoveryArtifactAdmission,
    ) -> Result<RecoveryLayoutReadmissionWitness, RecoveryLayoutReadmissionAdmissionDenial> {
        recovery_readmission_layout_family().admit_offline_witness(family_id, admission)
    }

    pub fn report_for(
        &self,
        witness: &RecoveryLayoutReadmissionWitness,
    ) -> RecoveryReadmissionLayoutReport {
        RecoveryReadmissionLayoutReport::from_witness(witness)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryReadmissionLayoutReport {
    family_id: DurableArtifactFamilyId,
    class: RecoveryLayoutReadmissionClass,
    identity: RecoveryLayoutReadmissionIdentity,
}

impl RecoveryReadmissionLayoutReport {
    fn from_witness(witness: &RecoveryLayoutReadmissionWitness) -> Self {
        Self {
            family_id: witness.family_id(),
            class: witness.class(),
            identity: witness.identity().clone(),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn class(&self) -> RecoveryLayoutReadmissionClass {
        self.class
    }

    pub const fn identity(&self) -> &RecoveryLayoutReadmissionIdentity {
        &self.identity
    }
}

impl RecoveryReadmissionLayoutFamilyHome {
    pub const fn classify_offline_admission(
        &self,
        _admission: &ReopenedRecoveryArtifactAdmission,
    ) -> RecoveryLayoutReadmissionClass {
        RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact
    }

    pub fn admit_record_backed_witness(
        &self,
        family_id: DurableArtifactFamilyId,
        record: &QuarantineRecord,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<RecoveryLayoutReadmissionWitness, RecoveryLayoutReadmissionAdmissionDenial> {
        let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
            .materialize(
                StoreExecutedIntegrityEvidence::receipt_evidence(record),
                PhysicalIntegrityEvidenceProfile::reduced(),
            )
            .map_err(RecoveryLayoutReadmissionAdmissionDenial::QuarantineReceiptEvidence)?;
        let receipt = RecoveryIntegrityHandoffReceipt::from_quarantine_receipt_evidence(&evidence)
            .map_err(RecoveryLayoutReadmissionAdmissionDenial::QuarantineHandoff)?;
        receipt
            .require_quarantine_record_basis(record)
            .map_err(RecoveryLayoutReadmissionAdmissionDenial::QuarantineHandoff)?;
        verify_store_authority_for_readmission(current_store_authority)
            .map_err(RecoveryLayoutReadmissionAdmissionDenial::StoreAuthority)?;

        let class = classify_record_for_readmission(record)?;
        match class {
            RecoveryLayoutReadmissionClass::QuarantineRecovery
            | RecoveryLayoutReadmissionClass::ImportBoundaryReadmission => {
                Ok(RecoveryLayoutReadmissionWitness {
                    family_id,
                    class,
                    identity: RecoveryLayoutReadmissionIdentity::QuarantineReceipt(
                        record.receipt().foundational_basis().digest().clone(),
                    ),
                })
            }
            RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
                unreachable!("offline artifact class is not produced from quarantine records")
            }
            RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
                Err(RecoveryLayoutReadmissionAdmissionDenial::NoForegroundAuthority)
            }
        }
    }

    pub fn admit_offline_witness(
        &self,
        family_id: DurableArtifactFamilyId,
        admission: &ReopenedRecoveryArtifactAdmission,
    ) -> Result<RecoveryLayoutReadmissionWitness, RecoveryLayoutReadmissionAdmissionDenial> {
        let class = self.classify_offline_admission(admission);
        match class {
            RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
                Ok(RecoveryLayoutReadmissionWitness {
                    family_id,
                    class,
                    identity: RecoveryLayoutReadmissionIdentity::OfflineArtifactDigest(
                        admission.artifact_digest().clone(),
                    ),
                })
            }
            RecoveryLayoutReadmissionClass::QuarantineRecovery
            | RecoveryLayoutReadmissionClass::ImportBoundaryReadmission
            | RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
                Err(RecoveryLayoutReadmissionAdmissionDenial::UnexpectedOfflineClassification)
            }
        }
    }
}

fn classify_record_for_readmission(
    record: &QuarantineRecord,
) -> Result<RecoveryLayoutReadmissionClass, RecoveryLayoutReadmissionAdmissionDenial> {
    match record.damage_classification() {
        DamageClassification::UnrecoverableAuthorityDamage(_) => {
            Ok(RecoveryLayoutReadmissionClass::ImportBoundaryReadmission)
        }
        DamageClassification::RebuildableDerivedDamage(_) => {
            Err(RecoveryLayoutReadmissionAdmissionDenial::DerivedProjectionDamageCannotReadmit)
        }
        DamageClassification::IntactPhysicalBoundary(_)
        | DamageClassification::QuarantinedPhysicalDamage(_)
        | DamageClassification::IndeterminatePhysicalDamage(_) => {
            Ok(classify_posture(record.handoff_posture()))
        }
    }
}

const fn classify_posture(posture: QuarantineHandoffPosture) -> RecoveryLayoutReadmissionClass {
    match posture {
        QuarantineHandoffPosture::S4RecoveryOwnerRequired
        | QuarantineHandoffPosture::S10RepairOwnerRequired => {
            RecoveryLayoutReadmissionClass::QuarantineRecovery
        }
        QuarantineHandoffPosture::AuditRetentionOwnerRequired
        | QuarantineHandoffPosture::RootChangeRevalidationRequired => {
            RecoveryLayoutReadmissionClass::NoForegroundAuthority
        }
    }
}

pub const fn recovery_readmission_layout_family() -> RecoveryReadmissionLayoutFamilyHome {
    RecoveryReadmissionLayoutFamilyHome
}
