use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_recovery_physics::{
    RecoveryLayoutReadmissionAdmissionDenial, RecoveryLayoutReadmissionClass,
    RecoveryLayoutReadmissionWitness,
};

use crate::materialization::MaterializationStateClass;
use crate::{LayoutCorruptionClassification, LayoutCoverageWitness};

use super::classification_outcome::{
    ImportReadmissionRequirement, LayoutCorruptionOutcome, OfflineReadmissionRequirement,
    QuarantineReadmissionRequirement, UnsupportedCorruptionState,
};
use super::denial::CorruptionDenial;
use super::entrypoint::LayoutCorruptionFacade;
use super::input::LayoutCorruptionInput;
use super::quarantine::LayoutQuarantineWitness;
use super::quarantine_authority::{classify_quarantine_authority, LayoutQuarantineAuthorityClass};

impl LayoutCorruptionFacade {
    pub fn classify(&self, input: LayoutCorruptionInput) -> LayoutCorruptionOutcome {
        match input {
            LayoutCorruptionInput::Materialization(coverage) => classify_materialization(coverage),
            LayoutCorruptionInput::RebuildClassification(classification) => {
                classify_rebuild(classification)
            }
            LayoutCorruptionInput::AuthoritativeQuarantine { family, record } => {
                match classify_quarantine_authority(&record) {
                    LayoutQuarantineAuthorityClass::DerivedProjectionDamage => {
                        LayoutCorruptionOutcome::rebuild_required(
                            LayoutCorruptionClassification::DerivedProjectionRebuildToParity,
                        )
                    }
                    LayoutQuarantineAuthorityClass::AuthoritativeQuarantineRequired => {
                        LayoutCorruptionOutcome::quarantined(LayoutQuarantineWitness::new(
                            family, record,
                        ))
                    }
                }
            }
            LayoutCorruptionInput::OfflineEvidence { family, admission } => {
                classify_offline_evidence(family, &admission)
            }
            LayoutCorruptionInput::TerminalImport { witness } => classify_terminal_import(witness),
            LayoutCorruptionInput::MigrationRequired { family } => {
                LayoutCorruptionOutcome::migration_required(family)
            }
        }
    }

    pub fn require_record_backed_recovery_readmission(
        &self,
        required: LayoutCorruptionOutcome,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<LayoutCorruptionOutcome, CorruptionDenial> {
        match required.into_quarantined() {
            Ok(quarantine) => require_quarantine_readmission(quarantine, current_store_authority),
            Err(other) => Err(
                CorruptionDenial::ReadmissionInputDoesNotMatchRequiredOutcome {
                    required: other.class(),
                },
            ),
        }
    }
}

fn classify_materialization(coverage: LayoutCoverageWitness) -> LayoutCorruptionOutcome {
    match coverage.state().class() {
        MaterializationStateClass::Absent => LayoutCorruptionOutcome::not_found(coverage.family()),
        MaterializationStateClass::Exact
        | MaterializationStateClass::ExactThroughPhysicalBasis
        | MaterializationStateClass::EmptyInitialized => LayoutCorruptionOutcome::clean(coverage),
        MaterializationStateClass::Stale => LayoutCorruptionOutcome::stale_binding(coverage),
        MaterializationStateClass::RebuildRequired => LayoutCorruptionOutcome::rebuild_required(
            LayoutCorruptionClassification::DerivedProjectionRebuildToParity,
        ),
        MaterializationStateClass::Migrating => {
            LayoutCorruptionOutcome::migration_required(coverage.family())
        }
        MaterializationStateClass::Quarantined => LayoutCorruptionOutcome::quarantined(
            LayoutQuarantineWitness::from_materialization(coverage),
        ),
        state => LayoutCorruptionOutcome::unsupported(UnsupportedCorruptionState::new(
            coverage.family(),
            state,
        )),
    }
}

fn classify_rebuild(classification: LayoutCorruptionClassification) -> LayoutCorruptionOutcome {
    match classification {
        LayoutCorruptionClassification::DerivedProjectionRebuildToParity => {
            LayoutCorruptionOutcome::rebuild_required(classification)
        }
        LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired { family } => {
            LayoutCorruptionOutcome::quarantined(LayoutQuarantineWitness::for_authoritative_family(
                family,
            ))
        }
    }
}

fn classify_offline_evidence(
    family: crate::PhysicalArtifactFamily,
    admission: &forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission,
) -> LayoutCorruptionOutcome {
    let witness =
        forge_store_recovery_physics::admit_offline_layout_readmission(family.id(), admission);
    LayoutCorruptionOutcome::offline_readmission_required(OfflineReadmissionRequirement::new(
        family,
        witness.identity().clone(),
    ))
}

fn classify_terminal_import(witness: RecoveryLayoutReadmissionWitness) -> LayoutCorruptionOutcome {
    let family = crate::layout_declarations()
        .declaration(witness.family_id())
        .expect("layout readmission witness should target a declared family")
        .family();
    LayoutCorruptionOutcome::import_readmission_required(ImportReadmissionRequirement::new(
        family,
        witness.identity().clone(),
    ))
}

fn require_quarantine_readmission(
    quarantine: LayoutQuarantineWitness,
    current_store_authority: &StoreCurrentAuthorityWitness,
) -> Result<LayoutCorruptionOutcome, CorruptionDenial> {
    let family = quarantine.family();
    let Some(record) = quarantine.record() else {
        return Err(CorruptionDenial::QuarantineRecordBackedReadmissionEvidenceRequired { family });
    };
    match forge_store_recovery_physics::admit_record_backed_layout_readmission(
        family.id(),
        record,
        current_store_authority,
    ) {
        Ok(witness) => match witness.class() {
            RecoveryLayoutReadmissionClass::QuarantineRecovery => {
                Ok(LayoutCorruptionOutcome::quarantine_readmission_required(
                    QuarantineReadmissionRequirement::new(quarantine, witness.identity().clone()),
                ))
            }
            RecoveryLayoutReadmissionClass::ImportBoundaryReadmission => {
                Err(CorruptionDenial::ImportReadmissionRequired { family })
            }
            RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
                unreachable!("offline readmission class is not produced from quarantine records")
            }
            RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
                Err(CorruptionDenial::NoForegroundReadAuthority { family })
            }
        },
        Err(RecoveryLayoutReadmissionAdmissionDenial::NoForegroundAuthority) => {
            Err(CorruptionDenial::NoForegroundReadAuthority { family })
        }
        Err(_) => {
            Err(CorruptionDenial::QuarantineRecordBackedReadmissionEvidenceRequired { family })
        }
    }
}
