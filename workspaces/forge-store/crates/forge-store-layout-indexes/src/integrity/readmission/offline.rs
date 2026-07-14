use forge_store_recovery_physics::{
    RecoveryLayoutReadmissionClass, RecoveryLayoutReadmissionWitness,
};

use super::matching::{family_bound_denial, matches_identity};
use super::{
    LayoutReadmissionSource, LayoutReadmissionWitness, OfflineReadmissionCaseId,
    OfflineReadmissionOutcome, OfflineReadmissionRequirement,
};
use crate::integrity::{CorruptionDenial, LayoutReadmissionCounterSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineReadmission;

pub const fn offline_readmission() -> OfflineReadmission {
    OfflineReadmission
}

impl OfflineReadmission {
    pub fn admit(
        self,
        required: OfflineReadmissionRequirement,
        witness: RecoveryLayoutReadmissionWitness,
    ) -> OfflineReadmissionOutcome {
        let family = required.family;
        let family_declaration = required.family();
        if witness.class() != RecoveryLayoutReadmissionClass::OfflineVerifiedArtifact {
            return OfflineReadmissionOutcome::denied(
                CorruptionDenial::UnexpectedOfflineReadmissionClass {
                    family: family_declaration,
                    class: witness.class(),
                },
                OfflineReadmissionCaseId::WRONG_CLASS,
                LayoutReadmissionCounterSnapshot::new(0, 0, 0),
            );
        }
        let Some(frontier) = witness.replay_frontier() else {
            return OfflineReadmissionOutcome::denied(
                family_bound_denial(
                    family_declaration,
                    LayoutReadmissionSource::OfflineRecoveryEvidence,
                ),
                OfflineReadmissionCaseId::FAMILY_IDENTITY,
                LayoutReadmissionCounterSnapshot::new(0, 1, 0),
            );
        };
        if !matches_identity(family, &required.identity, &witness) {
            return OfflineReadmissionOutcome::denied(
                family_bound_denial(
                    family_declaration,
                    LayoutReadmissionSource::OfflineRecoveryEvidence,
                ),
                OfflineReadmissionCaseId::FAMILY_IDENTITY,
                LayoutReadmissionCounterSnapshot::new(1, 1, 0),
            );
        }
        OfflineReadmissionOutcome::readmitted(
            LayoutReadmissionWitness::issue(
                family,
                LayoutReadmissionSource::OfflineRecoveryEvidence,
                witness.identity(),
                Some(frontier),
            ),
            LayoutReadmissionCounterSnapshot::new(1, 1, 1),
        )
    }
}
