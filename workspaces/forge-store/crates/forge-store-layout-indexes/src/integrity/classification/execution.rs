use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_recovery_physics::{
    RecoveryLayoutReadmissionAdmissionDenial, RecoveryLayoutReadmissionClass,
    RecoveryLayoutReadmissionWitness,
};

use crate::LayoutCorruptionClassification;

use super::super::denial::CorruptionDenial;
use super::super::quarantine::{
    classify_quarantine_authority, LayoutQuarantineAuthorityClass, LayoutQuarantineWitness,
};
use super::super::readmission::{
    ImportReadmissionRequirement, OfflineReadmissionRequirement, QuarantineReadmissionRequirement,
};
use super::outcome::LayoutCorruptionOutcome;
use super::LayoutCorruptionAssessment;

impl LayoutCorruptionAssessment {
    pub fn assess_derived_projection(
        &self,
        classification: LayoutCorruptionClassification,
    ) -> LayoutCorruptionOutcome {
        LayoutCorruptionOutcome::rebuild_required(
            classification,
            super::LayoutCorruptionCounterSnapshot::rebuild_classification(),
        )
    }

    pub fn assess_physical_quarantine(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        record: forge_store_physical_integrity::QuarantineRecord,
    ) -> LayoutCorruptionOutcome {
        match classify_quarantine_authority(&record) {
            LayoutQuarantineAuthorityClass::DerivedProjectionDamage => self
                .assess_derived_projection(
                    LayoutCorruptionClassification::derived_projection_rebuild_to_parity(),
                ),
            LayoutQuarantineAuthorityClass::AuthoritativeQuarantineRequired => {
                LayoutCorruptionOutcome::quarantined(
                    LayoutQuarantineWitness::from_record(family, record),
                    super::LayoutCorruptionCounterSnapshot::quarantine_record(),
                )
            }
        }
    }

    pub fn require_offline_readmission(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        admission: &forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission,
    ) -> LayoutCorruptionOutcome {
        let witness = forge_store_recovery_physics::layout_readmission()
            .admit_offline(family.family_id(), admission)
            .expect("reopened recovery admission issues offline readmission");
        LayoutCorruptionOutcome::offline_readmission_required(
            OfflineReadmissionRequirement::new(family, witness.identity().clone()),
            super::LayoutCorruptionCounterSnapshot::offline_admission(),
        )
    }

    pub fn require_import_readmission(
        &self,
        target: crate::AdmittedPhysicalArtifactFamily,
        witness: RecoveryLayoutReadmissionWitness,
    ) -> LayoutCorruptionOutcome {
        LayoutCorruptionOutcome::import_readmission_required(
            ImportReadmissionRequirement::new(target, witness.identity().clone()),
            super::LayoutCorruptionCounterSnapshot::terminal_import(),
        )
    }

    pub fn require_record_backed_recovery_readmission(
        &self,
        required: LayoutCorruptionOutcome,
        current_store_authority: &StoreCurrentAuthorityWitness,
        current_security_scope: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<LayoutCorruptionOutcome, CorruptionDenial> {
        match required.into_quarantined() {
            Ok((quarantine, counters)) => require_quarantine_readmission(
                quarantine,
                current_store_authority,
                current_security_scope,
                counters,
            ),
            Err(other) => Err(
                CorruptionDenial::ReadmissionInputDoesNotMatchRequiredOutcome {
                    required: other.class(),
                },
            ),
        }
    }
}

fn require_quarantine_readmission(
    quarantine: LayoutQuarantineWitness,
    current_store_authority: &StoreCurrentAuthorityWitness,
    current_security_scope: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
    counters: super::LayoutCorruptionCounterSnapshot,
) -> Result<LayoutCorruptionOutcome, CorruptionDenial> {
    let family = quarantine.family();
    let record = quarantine.record();
    let admitted_family = quarantine.admitted_family();
    let current_security_identity = current_security_scope.key_scope().identity();
    if admitted_family.security_identity() != current_security_identity {
        return Err(CorruptionDenial::SecurityScopeReadmissionMismatch {
            family,
            required: admitted_family.security_identity(),
            current: current_security_identity,
        });
    }
    match forge_store_recovery_physics::layout_readmission()
        .admit_quarantine(
            admitted_family.family_id(),
            record,
            current_store_authority,
            current_security_scope,
        )
        .into_result()
    {
        Ok(witness) => match witness.class() {
            RecoveryLayoutReadmissionClass::QuarantineRecovery => {
                Ok(LayoutCorruptionOutcome::quarantine_readmission_required(
                    QuarantineReadmissionRequirement::new(quarantine, witness.identity().clone()),
                    counters.with_record_backed_readmission(),
                ))
            }
            RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
                Err(CorruptionDenial::NoForegroundReadAuthority { family })
            }
            class => Err(CorruptionDenial::UnexpectedQuarantineReadmissionClass { family, class }),
        },
        Err(RecoveryLayoutReadmissionAdmissionDenial::NoForegroundAuthority) => {
            Err(CorruptionDenial::NoForegroundReadAuthority { family })
        }
        Err(_) => {
            Err(CorruptionDenial::QuarantineRecordBackedReadmissionEvidenceRequired { family })
        }
    }
}
