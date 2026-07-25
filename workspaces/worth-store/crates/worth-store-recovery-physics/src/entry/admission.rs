use crate::{
    classify_recovery_entry_inputs, AdmittedRecoveryIntegrityInput, RecoveryEntryAdmissionDecision,
    RecoveryEntryAdmissionDenial, RecoveryEntryBasis, RecoveryEntryCounters, RecoveryEntryIdentity,
    RecoveryEntryInputClassification, RecoveryMemoryAllocation, RecoveryMemoryObservation,
};
use worth_store_contracts::PhysicalAuthorityRecap;

#[derive(Debug)]
pub struct RecoveryEntryAdmission {
    entry_identity: RecoveryEntryIdentity,
    recovery_basis: RecoveryEntryBasis,
    counters: RecoveryEntryCounters,
    integrity_readiness: AdmittedRecoveryIntegrityInput,
    memory_allocation: RecoveryMemoryAllocation,
    physical_authority: PhysicalAuthorityRecap,
}

impl RecoveryEntryAdmission {
    pub fn admit(
        integrity_readiness: AdmittedRecoveryIntegrityInput,
        memory_allocation: RecoveryMemoryAllocation,
        physical_authority: PhysicalAuthorityRecap,
    ) -> RecoveryEntryAdmissionDecision {
        match classify_recovery_entry_inputs(
            &integrity_readiness,
            &memory_allocation,
            physical_authority,
        ) {
            RecoveryEntryInputClassification::Admissible(basis, counters) => {
                RecoveryEntryAdmissionDecision::Admitted(Box::new(Self {
                    entry_identity: RecoveryEntryIdentity::from_basis(&basis),
                    recovery_basis: *basis,
                    counters,
                    integrity_readiness,
                    memory_allocation,
                    physical_authority,
                }))
            }
            RecoveryEntryInputClassification::Blocked(blocked) => {
                RecoveryEntryAdmissionDecision::Blocked(blocked)
            }
            RecoveryEntryInputClassification::Denied(denial) => {
                RecoveryEntryAdmissionDecision::Denied(RecoveryEntryAdmissionDenial::new(denial))
            }
        }
    }

    pub const fn entry_identity(&self) -> &RecoveryEntryIdentity {
        &self.entry_identity
    }

    pub const fn recovery_basis(&self) -> &RecoveryEntryBasis {
        &self.recovery_basis
    }

    pub const fn counters(&self) -> RecoveryEntryCounters {
        self.counters
    }

    pub const fn integrity_readiness(&self) -> &AdmittedRecoveryIntegrityInput {
        &self.integrity_readiness
    }

    pub const fn memory_allocation(&self) -> RecoveryMemoryObservation {
        self.memory_allocation.observation()
    }

    pub const fn physical_authority(&self) -> PhysicalAuthorityRecap {
        self.physical_authority
    }

    pub const fn claims_replay_plan(&self) -> bool {
        false
    }

    pub const fn source_precedence_chosen(&self) -> bool {
        false
    }
}
