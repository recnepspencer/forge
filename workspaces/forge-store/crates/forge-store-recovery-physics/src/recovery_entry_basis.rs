use crate::{RecoveryMemoryEnvelope, S4RecoveryPhysicsIntegrityReadiness};
use forge_store_buffer_pool::{AllocationScope, BackgroundEnvelopeCounterSnapshot};
use forge_store_contracts::StableDigest;
use forge_store_readiness::PhysicalAuthorityRecap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEntryBasis {
    integrity_handoff_identity: StableDigest,
    integrity_damage_basis: StableDigest,
    memory_allocation_scope: AllocationScope,
    memory_counters: BackgroundEnvelopeCounterSnapshot,
    physical_reference_count: u32,
    header_decode_witness_count: u32,
    payload_admission_witness_count: u32,
}

impl RecoveryEntryBasis {
    pub(crate) fn from_entry_inputs(
        integrity_readiness: &S4RecoveryPhysicsIntegrityReadiness,
        memory_envelope: RecoveryMemoryEnvelope,
        physical_authority: PhysicalAuthorityRecap,
    ) -> Self {
        Self {
            integrity_handoff_identity: integrity_readiness.payload().identity().clone(),
            integrity_damage_basis: integrity_readiness.payload().damage_map().basis(),
            memory_allocation_scope: memory_envelope.allocation_scope(),
            memory_counters: memory_envelope.counters(),
            physical_reference_count: physical_authority.physical_reference_count(),
            header_decode_witness_count: physical_authority.header_decode_witness_count(),
            payload_admission_witness_count: physical_authority.payload_admission_witness_count(),
        }
    }

    pub fn integrity_handoff_identity(&self) -> &StableDigest {
        &self.integrity_handoff_identity
    }

    pub fn integrity_damage_basis(&self) -> &StableDigest {
        &self.integrity_damage_basis
    }

    pub const fn memory_allocation_scope(&self) -> AllocationScope {
        self.memory_allocation_scope
    }

    pub const fn memory_counters(&self) -> BackgroundEnvelopeCounterSnapshot {
        self.memory_counters
    }

    pub const fn physical_reference_count(&self) -> u32 {
        self.physical_reference_count
    }

    pub const fn header_decode_witness_count(&self) -> u32 {
        self.header_decode_witness_count
    }

    pub const fn payload_admission_witness_count(&self) -> u32 {
        self.payload_admission_witness_count
    }
}
