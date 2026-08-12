use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_contracts::{DurableArtifactFamilyId, StableDigest};
use worth_store_physical_integrity::{
    DamageClassification, PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceDenial,
    PhysicalIntegrityEvidenceProfile, QuarantineHandoffPosture, QuarantineRecord,
    StoreExecutedIntegrityEvidence,
};
use worth_store_security::StoreCurrentSecurityScopeWitnessSet;

use crate::{
    verify_store_authority_for_readmission, IntegrityHandoffDenial, RecoveryIntegrityHandoffReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryLayoutReadmissionClass {
    QuarantineRecovery,
    ImportBoundaryReadmission,
    NoForegroundAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryLayoutReadmissionIdentity {
    QuarantineReceipt(StableDigest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLayoutReadmissionWitness {
    family_id: DurableArtifactFamilyId,
    class: RecoveryLayoutReadmissionClass,
    identity: RecoveryLayoutReadmissionIdentity,
    source_store_authority_identity: Option<worth_store_authority::StoreCurrentAuthorityIdentity>,
    source_security_scope_identity: Option<worth_store_security::StoreSecurityScopeIdentity>,
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
    pub const fn source_store_authority_identity(
        &self,
    ) -> Option<worth_store_authority::StoreCurrentAuthorityIdentity> {
        self.source_store_authority_identity
    }

    pub const fn source_security_scope_identity(
        &self,
    ) -> Option<worth_store_security::StoreSecurityScopeIdentity> {
        self.source_security_scope_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryLayoutReadmissionAdmissionDenial {
    NoForegroundAuthority,
    QuarantineReceiptEvidence(PhysicalIntegrityEvidenceDenial),
    QuarantineHandoff(IntegrityHandoffDenial),
    StoreAuthority(crate::BlobReplayAdmissionDenial),
    SecurityScopeAuthorityMismatch {
        store: worth_store_authority::StoreCurrentAuthorityIdentity,
        security: worth_store_authority::StoreCurrentAuthorityIdentity,
    },
    DerivedProjectionDamageCannotReadmit,
    UnexpectedReadmissionClass {
        expected: RecoveryLayoutReadmissionClass,
        actual: RecoveryLayoutReadmissionClass,
    },
}

fn admit_record_backed_layout_readmission(
    family_id: DurableArtifactFamilyId,
    record: &QuarantineRecord,
    current_store_authority: &StoreCurrentAuthorityWitness,
    current_security_scope: &StoreCurrentSecurityScopeWitnessSet,
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
    if current_security_scope.authority_identity() != current_store_authority.authority_identity() {
        return Err(
            RecoveryLayoutReadmissionAdmissionDenial::SecurityScopeAuthorityMismatch {
                store: current_store_authority.authority_identity(),
                security: current_security_scope.authority_identity(),
            },
        );
    }
    Ok(RecoveryLayoutReadmissionWitness {
        family_id,
        class: classify_record(record)?,
        identity: RecoveryLayoutReadmissionIdentity::QuarantineReceipt(
            record.receipt().foundational_basis().digest().clone(),
        ),
        source_store_authority_identity: Some(current_store_authority.authority_identity()),
        source_security_scope_identity: Some(current_security_scope.key_scope().identity()),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutReadmissionAuthority;

pub const fn layout_readmission() -> LayoutReadmissionAuthority {
    LayoutReadmissionAuthority
}

macro_rules! readmission_outcome {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            result:
                Result<RecoveryLayoutReadmissionWitness, RecoveryLayoutReadmissionAdmissionDenial>,
        }

        impl $name {
            fn issue(
                result: Result<
                    RecoveryLayoutReadmissionWitness,
                    RecoveryLayoutReadmissionAdmissionDenial,
                >,
            ) -> Self {
                Self { result }
            }

            pub const fn view(&self) -> RecoveryLayoutReadmissionOutcomeView<'_> {
                match &self.result {
                    Ok(witness) => RecoveryLayoutReadmissionOutcomeView::Readmitted(witness),
                    Err(denial) => RecoveryLayoutReadmissionOutcomeView::Denied(denial),
                }
            }

            pub fn into_result(
                self,
            ) -> Result<RecoveryLayoutReadmissionWitness, RecoveryLayoutReadmissionAdmissionDenial>
            {
                self.result
            }

            pub fn expect(self, message: &str) -> RecoveryLayoutReadmissionWitness {
                self.result.expect(message)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryLayoutReadmissionOutcomeView<'a> {
    Readmitted(&'a RecoveryLayoutReadmissionWitness),
    Denied(&'a RecoveryLayoutReadmissionAdmissionDenial),
}

readmission_outcome!(QuarantineLayoutReadmissionOutcome);
readmission_outcome!(ImportLayoutReadmissionOutcome);

impl LayoutReadmissionAuthority {
    pub fn admit_quarantine(
        self,
        family_id: DurableArtifactFamilyId,
        record: &QuarantineRecord,
        current_store_authority: &StoreCurrentAuthorityWitness,
        current_security_scope: &StoreCurrentSecurityScopeWitnessSet,
    ) -> QuarantineLayoutReadmissionOutcome {
        QuarantineLayoutReadmissionOutcome::issue(require_record_class(
            admit_record_backed_layout_readmission(
                family_id,
                record,
                current_store_authority,
                current_security_scope,
            ),
            RecoveryLayoutReadmissionClass::QuarantineRecovery,
        ))
    }

    pub fn admit_import(
        self,
        family_id: DurableArtifactFamilyId,
        record: &QuarantineRecord,
        current_store_authority: &StoreCurrentAuthorityWitness,
        current_security_scope: &StoreCurrentSecurityScopeWitnessSet,
    ) -> ImportLayoutReadmissionOutcome {
        ImportLayoutReadmissionOutcome::issue(require_record_class(
            admit_record_backed_layout_readmission(
                family_id,
                record,
                current_store_authority,
                current_security_scope,
            ),
            RecoveryLayoutReadmissionClass::ImportBoundaryReadmission,
        ))
    }

}

fn require_record_class(
    result: Result<RecoveryLayoutReadmissionWitness, RecoveryLayoutReadmissionAdmissionDenial>,
    expected: RecoveryLayoutReadmissionClass,
) -> Result<RecoveryLayoutReadmissionWitness, RecoveryLayoutReadmissionAdmissionDenial> {
    let witness = result?;
    if witness.class() == expected {
        Ok(witness)
    } else {
        Err(
            RecoveryLayoutReadmissionAdmissionDenial::UnexpectedReadmissionClass {
                expected,
                actual: witness.class(),
            },
        )
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
        QuarantineHandoffPosture::RecoveryOwnerRequired
        | QuarantineHandoffPosture::RepairOwnerRequired => {
            Ok(RecoveryLayoutReadmissionClass::QuarantineRecovery)
        }
        QuarantineHandoffPosture::AuditRetentionOwnerRequired
        | QuarantineHandoffPosture::RootChangeRevalidationRequired => {
            Err(RecoveryLayoutReadmissionAdmissionDenial::NoForegroundAuthority)
        }
    }
}
