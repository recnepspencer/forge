use forge_store_authority::StoreCurrentAuthorityWitness;
use forge_store_recovery_physics::{
    RecoveryLayoutReadmissionAdmissionDenial, RecoveryLayoutReadmissionClass,
    RecoveryLayoutReadmissionIdentity, RecoveryLayoutReadmissionWitness,
};

use crate::materialization::S8MaterializationStateClass;
use crate::{LayoutCorruptionClassification, S8LayoutCoverageWitness};

use super::denial::S8CorruptionDenial;
use super::input::S8LayoutCorruptionInput;
use super::outcome::{
    S8LayoutCorruptionOutcome, S8LayoutReadmissionOutcome, S8QuarantineReadmissionRequirement,
    S8ReadmissionRequirement, S8RequiredReadmission, S8UnsupportedCorruptionState,
};
use super::quarantine::S8LayoutQuarantineWitness;
use super::quarantine_authority::{classify_quarantine_authority, LayoutQuarantineAuthorityClass};
use super::readmission::S8NativeReadmissionInput;
use super::S8LayoutReadmissionSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutCorruptionFacade;

impl LayoutCorruptionFacade {
    pub fn classify(&self, input: S8LayoutCorruptionInput) -> S8LayoutCorruptionOutcome {
        match input {
            S8LayoutCorruptionInput::Materialization(coverage) => {
                classify_materialization(coverage)
            }
            S8LayoutCorruptionInput::RebuildClassification(classification) => {
                classify_rebuild(classification)
            }
            S8LayoutCorruptionInput::AuthoritativeQuarantine { family, record } => {
                match classify_quarantine_authority(&record) {
                    LayoutQuarantineAuthorityClass::DerivedProjectionDamage => {
                        S8LayoutCorruptionOutcome::rebuild_required(
                            LayoutCorruptionClassification::DerivedProjectionRebuildToParity,
                        )
                    }
                    LayoutQuarantineAuthorityClass::AuthoritativeQuarantineRequired => {
                        S8LayoutCorruptionOutcome::quarantined(S8LayoutQuarantineWitness::new(
                            family, record,
                        ))
                    }
                }
            }
            S8LayoutCorruptionInput::OfflineEvidence { family, admission } => {
                classify_offline_evidence(family, &admission)
            }
            S8LayoutCorruptionInput::TerminalImport { witness } => {
                classify_terminal_import(witness)
            }
            S8LayoutCorruptionInput::MigrationRequired { family } => {
                S8LayoutCorruptionOutcome::migration_required(family)
            }
        }
    }

    pub fn require_record_backed_recovery_readmission(
        &self,
        required: S8LayoutCorruptionOutcome,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> Result<S8LayoutCorruptionOutcome, S8CorruptionDenial> {
        match required.into_quarantined() {
            Ok(quarantine) => require_quarantine_readmission(quarantine, current_store_authority),
            Err(other) => Err(
                S8CorruptionDenial::ReadmissionInputDoesNotMatchRequiredOutcome {
                    required: other.class(),
                },
            ),
        }
    }

    pub fn readmit_with(
        &self,
        required: S8LayoutCorruptionOutcome,
        input: S8NativeReadmissionInput,
    ) -> S8LayoutReadmissionOutcome {
        let source = required_readmission_source(&required);
        match (required.into_readmission_requirement(), input) {
            (
                Ok(S8RequiredReadmission::Quarantine(required)),
                S8NativeReadmissionInput::RecoveryWitness { witness },
            ) => readmit_quarantine(required.quarantine, required.identity, witness),
            (
                Ok(S8RequiredReadmission::Offline(required)),
                S8NativeReadmissionInput::RecoveryWitness { witness },
            ) => readmit_offline(required.family, required.identity, witness),
            (
                Ok(S8RequiredReadmission::Import(required)),
                S8NativeReadmissionInput::RecoveryWitness { witness },
            ) => readmit_terminal_import(required.family, required.identity, witness),
            (Err(required), _) => S8LayoutReadmissionOutcome::denied(
                source,
                S8CorruptionDenial::ReadmissionInputDoesNotMatchRequiredOutcome {
                    required: required.class(),
                },
            ),
        }
    }
}

fn classify_materialization(coverage: S8LayoutCoverageWitness) -> S8LayoutCorruptionOutcome {
    match coverage.state().class() {
        S8MaterializationStateClass::Absent => {
            S8LayoutCorruptionOutcome::not_found(coverage.family())
        }
        S8MaterializationStateClass::Exact
        | S8MaterializationStateClass::ExactThroughPhysicalBasis
        | S8MaterializationStateClass::EmptyInitialized => {
            S8LayoutCorruptionOutcome::clean(coverage)
        }
        S8MaterializationStateClass::Stale => S8LayoutCorruptionOutcome::stale_binding(coverage),
        S8MaterializationStateClass::RebuildRequired => {
            S8LayoutCorruptionOutcome::rebuild_required(
                LayoutCorruptionClassification::DerivedProjectionRebuildToParity,
            )
        }
        S8MaterializationStateClass::Migrating => {
            S8LayoutCorruptionOutcome::migration_required(coverage.family())
        }
        S8MaterializationStateClass::Quarantined => S8LayoutCorruptionOutcome::quarantined(
            S8LayoutQuarantineWitness::from_materialization(coverage),
        ),
        state => S8LayoutCorruptionOutcome::unsupported(S8UnsupportedCorruptionState::new(
            coverage.family(),
            state,
        )),
    }
}

fn classify_rebuild(classification: LayoutCorruptionClassification) -> S8LayoutCorruptionOutcome {
    match classification {
        LayoutCorruptionClassification::DerivedProjectionRebuildToParity => {
            S8LayoutCorruptionOutcome::rebuild_required(classification)
        }
        LayoutCorruptionClassification::AuthoritativeSourceQuarantineRequired { family } => {
            S8LayoutCorruptionOutcome::quarantined(
                S8LayoutQuarantineWitness::for_authoritative_family(family),
            )
        }
    }
}

fn classify_offline_evidence(
    family: crate::PhysicalArtifactFamily,
    admission: &forge_store_recovery_physics::ReopenedRecoveryArtifactAdmission,
) -> S8LayoutCorruptionOutcome {
    let witness =
        forge_store_recovery_physics::admit_offline_layout_readmission(family.id(), admission);
    S8LayoutCorruptionOutcome::offline_readmission_required(S8ReadmissionRequirement::new(
        family,
        witness.identity().clone(),
    ))
}

fn classify_terminal_import(
    witness: RecoveryLayoutReadmissionWitness,
) -> S8LayoutCorruptionOutcome {
    let family = crate::layout_declarations()
        .declaration(witness.family_id())
        .expect("layout readmission witness should target a declared family")
        .family();
    S8LayoutCorruptionOutcome::import_readmission_required(S8ReadmissionRequirement::new(
        family,
        witness.identity().clone(),
    ))
}

fn require_quarantine_readmission(
    quarantine: S8LayoutQuarantineWitness,
    current_store_authority: &StoreCurrentAuthorityWitness,
) -> Result<S8LayoutCorruptionOutcome, S8CorruptionDenial> {
    let family = quarantine.family();
    let Some(record) = quarantine.record() else {
        return Err(
            S8CorruptionDenial::QuarantineRecordBackedReadmissionEvidenceRequired { family },
        );
    };
    match forge_store_recovery_physics::admit_record_backed_layout_readmission(
        family.id(),
        record,
        current_store_authority,
    ) {
        Ok(witness) => match witness.class() {
            RecoveryLayoutReadmissionClass::QuarantineRecovery => {
                Ok(S8LayoutCorruptionOutcome::quarantine_readmission_required(
                    S8QuarantineReadmissionRequirement::new(quarantine, witness.identity().clone()),
                ))
            }
            RecoveryLayoutReadmissionClass::ImportBoundaryReadmission => {
                Err(S8CorruptionDenial::ImportReadmissionRequired { family })
            }
            RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
                unreachable!("offline readmission class is not produced from quarantine records")
            }
            RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
                Err(S8CorruptionDenial::NoForegroundReadAuthority { family })
            }
        },
        Err(RecoveryLayoutReadmissionAdmissionDenial::NoForegroundAuthority) => {
            Err(S8CorruptionDenial::NoForegroundReadAuthority { family })
        }
        Err(_) => {
            Err(S8CorruptionDenial::QuarantineRecordBackedReadmissionEvidenceRequired { family })
        }
    }
}

fn readmit_quarantine(
    quarantine: S8LayoutQuarantineWitness,
    identity: RecoveryLayoutReadmissionIdentity,
    witness: RecoveryLayoutReadmissionWitness,
) -> S8LayoutReadmissionOutcome {
    match witness.class() {
        RecoveryLayoutReadmissionClass::QuarantineRecovery => {
            if matches_identity(quarantine.family(), &identity, &witness) {
                S8LayoutReadmissionOutcome::readmitted(
                    super::readmission::S8LayoutReadmissionWitness::quarantine_recovery(
                        quarantine.family(),
                    ),
                )
            } else {
                S8LayoutReadmissionOutcome::denied(
                    S8LayoutReadmissionSource::QuarantineRecovery,
                    S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                        family: quarantine.family(),
                        source:
                            super::classification::S8LayoutReadmissionSource::QuarantineRecovery,
                    },
                )
            }
        }
        RecoveryLayoutReadmissionClass::ImportBoundaryReadmission => {
            S8LayoutReadmissionOutcome::denied(
                S8LayoutReadmissionSource::QuarantineRecovery,
                S8CorruptionDenial::ImportReadmissionRequired {
                    family: quarantine.family(),
                },
            )
        }
        RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
            S8LayoutReadmissionOutcome::denied(
                S8LayoutReadmissionSource::QuarantineRecovery,
                S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                    family: quarantine.family(),
                    source: super::classification::S8LayoutReadmissionSource::QuarantineRecovery,
                },
            )
        }
        RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
            S8LayoutReadmissionOutcome::denied(
                S8LayoutReadmissionSource::QuarantineRecovery,
                S8CorruptionDenial::NoForegroundReadAuthority {
                    family: quarantine.family(),
                },
            )
        }
    }
}

fn readmit_offline(
    family: crate::PhysicalArtifactFamily,
    identity: RecoveryLayoutReadmissionIdentity,
    witness: RecoveryLayoutReadmissionWitness,
) -> S8LayoutReadmissionOutcome {
    match witness.class() {
        RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
            if matches_identity(family, &identity, &witness) {
                S8LayoutReadmissionOutcome::readmitted(
                    super::readmission::S8LayoutReadmissionWitness::offline_evidence(family),
                )
            } else {
                S8LayoutReadmissionOutcome::denied(
                    S8LayoutReadmissionSource::OfflineRecoveryEvidence,
                    S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                        family,
                        source:
                            super::classification::S8LayoutReadmissionSource::OfflineRecoveryEvidence,
                    },
                )
            }
        }
        other => S8LayoutReadmissionOutcome::denied(
            S8LayoutReadmissionSource::OfflineRecoveryEvidence,
            S8CorruptionDenial::UnexpectedOfflineReadmissionClass {
                family,
                class: other,
            },
        ),
    }
}

fn readmit_terminal_import(
    family: crate::PhysicalArtifactFamily,
    identity: RecoveryLayoutReadmissionIdentity,
    witness: RecoveryLayoutReadmissionWitness,
) -> S8LayoutReadmissionOutcome {
    match witness.class() {
        RecoveryLayoutReadmissionClass::ImportBoundaryReadmission => {
            if matches_identity(family, &identity, &witness) {
                S8LayoutReadmissionOutcome::readmitted(
                    super::readmission::S8LayoutReadmissionWitness::terminal_import(family),
                )
            } else {
                S8LayoutReadmissionOutcome::denied(
                    S8LayoutReadmissionSource::TerminalImport,
                    S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                        family,
                        source: super::classification::S8LayoutReadmissionSource::TerminalImport,
                    },
                )
            }
        }
        RecoveryLayoutReadmissionClass::QuarantineRecovery => S8LayoutReadmissionOutcome::denied(
            S8LayoutReadmissionSource::TerminalImport,
            S8CorruptionDenial::QuarantineReadmissionRequired { family },
        ),
        RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
            S8LayoutReadmissionOutcome::denied(
                S8LayoutReadmissionSource::TerminalImport,
                S8CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                    family,
                    source: super::classification::S8LayoutReadmissionSource::TerminalImport,
                },
            )
        }
        RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
            S8LayoutReadmissionOutcome::denied(
                S8LayoutReadmissionSource::TerminalImport,
                S8CorruptionDenial::NoForegroundReadAuthority { family },
            )
        }
    }
}

fn required_readmission_source(required: &S8LayoutCorruptionOutcome) -> S8LayoutReadmissionSource {
    match required.view() {
        super::S8LayoutCorruptionView::QuarantineReadmissionRequired(_) => {
            S8LayoutReadmissionSource::QuarantineRecovery
        }
        super::S8LayoutCorruptionView::OfflineReadmissionRequired(_) => {
            S8LayoutReadmissionSource::OfflineRecoveryEvidence
        }
        super::S8LayoutCorruptionView::ImportReadmissionRequired(_) => {
            S8LayoutReadmissionSource::TerminalImport
        }
        _ => S8LayoutReadmissionSource::QuarantineRecovery,
    }
}

fn matches_identity(
    family: crate::PhysicalArtifactFamily,
    expected_identity: &RecoveryLayoutReadmissionIdentity,
    witness: &RecoveryLayoutReadmissionWitness,
) -> bool {
    witness.family_id() == family.id() && witness.identity() == expected_identity
}

pub const fn layout_corruption() -> LayoutCorruptionFacade {
    LayoutCorruptionFacade
}
