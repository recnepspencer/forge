use forge_store_recovery_physics::{
    RecoveryLayoutReadmissionClass, RecoveryLayoutReadmissionIdentity,
    RecoveryLayoutReadmissionWitness,
};

use super::classification::LayoutReadmissionSource;
use super::classification_outcome::{
    ImportReadmissionRequirement, OfflineReadmissionRequirement, QuarantineReadmissionRequirement,
};
use super::denial::CorruptionDenial;
use super::entrypoint::LayoutCorruptionFacade;
use super::quarantine::LayoutQuarantineWitness;
use super::readmission::LayoutReadmissionWitness;
use super::readmission_outcome::{
    ImportReadmissionOutcome, OfflineReadmissionOutcome, QuarantineReadmissionOutcome,
};

impl LayoutCorruptionFacade {
    pub fn readmit_quarantine(
        &self,
        required: QuarantineReadmissionRequirement,
        witness: RecoveryLayoutReadmissionWitness,
    ) -> QuarantineReadmissionOutcome {
        readmit_quarantine(required.quarantine, required.identity, witness)
    }

    pub fn readmit_offline(
        &self,
        required: OfflineReadmissionRequirement,
        witness: RecoveryLayoutReadmissionWitness,
    ) -> OfflineReadmissionOutcome {
        readmit_offline(required.family, required.identity, witness)
    }

    pub fn readmit_import(
        &self,
        required: ImportReadmissionRequirement,
        witness: RecoveryLayoutReadmissionWitness,
    ) -> ImportReadmissionOutcome {
        readmit_terminal_import(required.family, required.identity, witness)
    }
}

fn readmit_quarantine(
    quarantine: LayoutQuarantineWitness,
    identity: RecoveryLayoutReadmissionIdentity,
    witness: RecoveryLayoutReadmissionWitness,
) -> QuarantineReadmissionOutcome {
    match witness.class() {
        RecoveryLayoutReadmissionClass::QuarantineRecovery => {
            if matches_identity(quarantine.family(), &identity, &witness) {
                QuarantineReadmissionOutcome::readmitted(
                    LayoutReadmissionWitness::quarantine_recovery(
                        quarantine.family(),
                        witness.identity(),
                    ),
                )
            } else {
                QuarantineReadmissionOutcome::denied(
                    CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                        family: quarantine.family(),
                        source: LayoutReadmissionSource::QuarantineRecovery,
                    },
                )
            }
        }
        RecoveryLayoutReadmissionClass::ImportBoundaryReadmission => {
            QuarantineReadmissionOutcome::denied(CorruptionDenial::ImportReadmissionRequired {
                family: quarantine.family(),
            })
        }
        RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
            QuarantineReadmissionOutcome::denied(
                CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                    family: quarantine.family(),
                    source: LayoutReadmissionSource::QuarantineRecovery,
                },
            )
        }
        RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
            QuarantineReadmissionOutcome::denied(CorruptionDenial::NoForegroundReadAuthority {
                family: quarantine.family(),
            })
        }
    }
}

fn readmit_offline(
    family: crate::PhysicalArtifactFamily,
    identity: RecoveryLayoutReadmissionIdentity,
    witness: RecoveryLayoutReadmissionWitness,
) -> OfflineReadmissionOutcome {
    match witness.class() {
        RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
            if matches_identity(family, &identity, &witness) {
                match witness.replay_frontier() {
                    Some(frontier) => OfflineReadmissionOutcome::readmitted(
                        LayoutReadmissionWitness::offline_evidence(
                            family,
                            witness.identity(),
                            frontier,
                        ),
                    ),
                    None => OfflineReadmissionOutcome::denied(
                        CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                            family,
                            source: LayoutReadmissionSource::OfflineRecoveryEvidence,
                        },
                    ),
                }
            } else {
                OfflineReadmissionOutcome::denied(
                    CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                        family,
                        source: LayoutReadmissionSource::OfflineRecoveryEvidence,
                    },
                )
            }
        }
        other => {
            OfflineReadmissionOutcome::denied(CorruptionDenial::UnexpectedOfflineReadmissionClass {
                family,
                class: other,
            })
        }
    }
}

fn readmit_terminal_import(
    family: crate::PhysicalArtifactFamily,
    identity: RecoveryLayoutReadmissionIdentity,
    witness: RecoveryLayoutReadmissionWitness,
) -> ImportReadmissionOutcome {
    match witness.class() {
        RecoveryLayoutReadmissionClass::ImportBoundaryReadmission => {
            if matches_identity(family, &identity, &witness) {
                ImportReadmissionOutcome::readmitted(LayoutReadmissionWitness::terminal_import(
                    family,
                    witness.identity(),
                ))
            } else {
                ImportReadmissionOutcome::denied(
                    CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                        family,
                        source: LayoutReadmissionSource::TerminalImport,
                    },
                )
            }
        }
        RecoveryLayoutReadmissionClass::QuarantineRecovery => {
            ImportReadmissionOutcome::denied(CorruptionDenial::QuarantineReadmissionRequired {
                family,
            })
        }
        RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact => {
            ImportReadmissionOutcome::denied(
                CorruptionDenial::FamilyBoundReadmissionWitnessRequired {
                    family,
                    source: LayoutReadmissionSource::TerminalImport,
                },
            )
        }
        RecoveryLayoutReadmissionClass::NoForegroundAuthority => {
            ImportReadmissionOutcome::denied(CorruptionDenial::NoForegroundReadAuthority { family })
        }
    }
}

fn matches_identity(
    family: crate::PhysicalArtifactFamily,
    expected_identity: &RecoveryLayoutReadmissionIdentity,
    witness: &RecoveryLayoutReadmissionWitness,
) -> bool {
    witness.family_id() == family.id() && witness.identity() == expected_identity
}
