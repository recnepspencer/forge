use crate::{
    AdmittedRecoveryIntegrityInput, RecoveryEntryAdmissionDenialKind, RecoveryEntryBasis,
    RecoveryEntryBlockedByIntegrityDamage, RecoveryEntryCounters, RecoveryMemoryEnvelope,
};
use worth_store_contracts::PhysicalAuthorityRecap;

pub(crate) enum RecoveryEntryInputClassification {
    Admissible(Box<RecoveryEntryBasis>, RecoveryEntryCounters),
    Blocked(RecoveryEntryBlockedByIntegrityDamage),
    Denied(RecoveryEntryAdmissionDenialKind),
}

pub(crate) fn classify_recovery_entry_inputs(
    integrity_readiness: &AdmittedRecoveryIntegrityInput,
    memory_envelope: RecoveryMemoryEnvelope,
    physical_authority: PhysicalAuthorityRecap,
) -> RecoveryEntryInputClassification {
    if integrity_readiness.claims_recovery() {
        return denied(RecoveryEntryAdmissionDenialKind::IntegrityReadinessClaimsRecovery);
    }
    if !integrity_readiness.proves_no_raw_bytes_crossed() {
        return denied(RecoveryEntryAdmissionDenialKind::RawBytesCrossedIntegrityBoundary);
    }
    if memory_envelope.proves_wal_recovery() {
        return denied(RecoveryEntryAdmissionDenialKind::RecoveryMemoryEnvelopeClaimsWalRecovery);
    }
    if memory_envelope.proves_checkpoint_safety() {
        return denied(
            RecoveryEntryAdmissionDenialKind::RecoveryMemoryEnvelopeClaimsCheckpointSafety,
        );
    }
    if memory_envelope.proves_repair_behavior() {
        return denied(
            RecoveryEntryAdmissionDenialKind::RecoveryMemoryEnvelopeClaimsRepairBehavior,
        );
    }
    if physical_authority_recap_missing(physical_authority) {
        return denied(RecoveryEntryAdmissionDenialKind::MissingPhysicalAuthorityRecap);
    }
    if !integrity_readiness
        .payload()
        .damage_map()
        .recovery_blocking_findings()
        .is_empty()
    {
        return RecoveryEntryInputClassification::Blocked(
            RecoveryEntryBlockedByIntegrityDamage::before_replay_planning(
                integrity_readiness.payload().damage_map(),
            ),
        );
    }

    RecoveryEntryInputClassification::Admissible(
        Box::new(RecoveryEntryBasis::from_entry_inputs(
            integrity_readiness,
            memory_envelope,
            physical_authority,
        )),
        RecoveryEntryCounters::from_entry_inputs(integrity_readiness, memory_envelope),
    )
}

fn physical_authority_recap_missing(physical_authority: PhysicalAuthorityRecap) -> bool {
    physical_authority.physical_reference_count() == 0
        || physical_authority.header_decode_witness_count() == 0
        || physical_authority.payload_admission_witness_count() == 0
}

fn denied(kind: RecoveryEntryAdmissionDenialKind) -> RecoveryEntryInputClassification {
    RecoveryEntryInputClassification::Denied(kind)
}
