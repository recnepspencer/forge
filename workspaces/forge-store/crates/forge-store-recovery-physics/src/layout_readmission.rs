use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_contracts::{DurableArtifactFamilyId, StableDigest};
use forge_store_physical_integrity::{
    DamageClassification, PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceDenial,
    PhysicalIntegrityEvidenceProfile, QuarantineHandoffPosture, QuarantineRecord,
    StoreExecutedIntegrityEvidence,
};

use crate::{
    verify_store_authority_for_readmission, IntegrityHandoffDenial,
    PersistedRecoveryArtifactDigest, RecoveryIntegrityHandoffReceipt,
    ReopenedRecoveryArtifactAdmission,
};

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
    replay_frontier: Option<crate::LogSequenceNumber>,
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
    pub const fn replay_frontier(&self) -> Option<crate::LogSequenceNumber> {
        self.replay_frontier
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryLayoutReadmissionAdmissionDenial {
    NoForegroundAuthority,
    QuarantineReceiptEvidence(PhysicalIntegrityEvidenceDenial),
    QuarantineHandoff(IntegrityHandoffDenial),
    StoreAuthority(crate::BlobReplayAdmissionDenial),
    DerivedProjectionDamageCannotReadmit,
}

pub fn admit_record_backed_layout_readmission(
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
    Ok(RecoveryLayoutReadmissionWitness {
        family_id,
        class: classify_record(record)?,
        identity: RecoveryLayoutReadmissionIdentity::QuarantineReceipt(
            record.receipt().foundational_basis().digest().clone(),
        ),
        replay_frontier: None,
    })
}

pub fn admit_offline_layout_readmission(
    family_id: DurableArtifactFamilyId,
    admission: &ReopenedRecoveryArtifactAdmission,
) -> RecoveryLayoutReadmissionWitness {
    let replay_frontier = admission
        .replay_cursor()
        .pages()
        .iter()
        .map(|page| page.eligibility().redo_frontier().lsn())
        .max();
    RecoveryLayoutReadmissionWitness {
        family_id,
        class: RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact,
        identity: RecoveryLayoutReadmissionIdentity::OfflineArtifactDigest(
            admission.artifact_digest().clone(),
        ),
        replay_frontier,
    }
}

fn classify_record(
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
            classify_posture(record.handoff_posture())
        }
    }
}

fn classify_posture(
    posture: QuarantineHandoffPosture,
) -> Result<RecoveryLayoutReadmissionClass, RecoveryLayoutReadmissionAdmissionDenial> {
    match posture {
        QuarantineHandoffPosture::S4RecoveryOwnerRequired
        | QuarantineHandoffPosture::S10RepairOwnerRequired => {
            Ok(RecoveryLayoutReadmissionClass::QuarantineRecovery)
        }
        QuarantineHandoffPosture::AuditRetentionOwnerRequired
        | QuarantineHandoffPosture::RootChangeRevalidationRequired => {
            Err(RecoveryLayoutReadmissionAdmissionDenial::NoForegroundAuthority)
        }
    }
}
