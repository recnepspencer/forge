use crate::{AdmittedRecoveryIntegrityInput, RecoveryMemoryAllocation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryEntryCounters {
    vetted_record_count: u64,
    recovery_blocking_count: u64,
    memory_envelope_admissions: u32,
    replay_plans_started: u32,
    source_precedence_choices: u32,
}

impl RecoveryEntryCounters {
    pub(crate) fn from_entry_inputs(
        integrity_readiness: &AdmittedRecoveryIntegrityInput,
        memory_allocation: &RecoveryMemoryAllocation,
    ) -> Self {
        Self {
            vetted_record_count: integrity_readiness.counters().vetted_record_count(),
            recovery_blocking_count: integrity_readiness.counters().recovery_blocking_count(),
            memory_envelope_admissions: memory_allocation.counters().admitted(),
            replay_plans_started: 0,
            source_precedence_choices: 0,
        }
    }

    pub const fn vetted_record_count(self) -> u64 {
        self.vetted_record_count
    }

    pub const fn recovery_blocking_count(self) -> u64 {
        self.recovery_blocking_count
    }

    pub const fn memory_envelope_admissions(self) -> u32 {
        self.memory_envelope_admissions
    }

    pub const fn replay_plans_started(self) -> u32 {
        self.replay_plans_started
    }

    pub const fn source_precedence_choices(self) -> u32 {
        self.source_precedence_choices
    }
}
