use forge_store_physical_integrity::{
    ExecutedQuarantineFinding, PhysicalIntegrityEvidenceAuthority,
    PhysicalIntegrityEvidenceProfile, PhysicalQuarantineAuthority, QuarantineRecord,
    QuarantineSealRequest, StoreExecutedIntegrityEvidence,
};
use forge_store_recovery_physics::{
    RecoveryBlockedByIntegrityDamage, RecoveryIntegrityHandoffReceipt, WalLsnRange,
    WalOnlyTailProof, WalOnlyTailProofDenial, WalSegmentGeneration, WalSegmentId,
    WalTailIntegrityQuarantineHandoff,
};
use forge_store_wal::{admit_replay_cursor, WalSegmentScanRecord, WalTopologyScan};

use super::construction::{inspect_wal_payload, torn_wal_payload, wal_payload};

pub(crate) fn wal_only_tail_denial_from_torn_frame(
    range: WalLsnRange,
) -> WalOnlyTailProofDenial {
    let cursor = admit_replay_cursor(
        WalTopologyScan::from_segment_scan([WalSegmentScanRecord::current(
            WalSegmentId::new(100).unwrap(),
            WalSegmentGeneration::new(1).unwrap(),
            range,
        )]),
        WalSegmentGeneration::new(1).unwrap(),
    )
    .unwrap();
    let handoff = quarantined_torn_wal_tail_handoff(range);
    WalOnlyTailProof::from_quarantined_wal_tail(&handoff, &cursor).unwrap_err()
}

pub(crate) fn quarantined_torn_wal_tail_handoff(
    range: WalLsnRange,
) -> WalTailIntegrityQuarantineHandoff {
    let denial = inspect_wal_payload(&torn_wal_payload(range)).unwrap_err();
    let record = wal_tail_quarantine_record(range);
    let evidence = PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(
            StoreExecutedIntegrityEvidence::receipt_evidence(&record),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    WalTailIntegrityQuarantineHandoff::from_wal_tail_damage_quarantine(
        &denial,
        &record,
        RecoveryIntegrityHandoffReceipt::from_quarantine_receipt_evidence(&evidence).unwrap(),
    )
    .unwrap()
}

pub(crate) fn wal_tail_quarantine_record(range: WalLsnRange) -> QuarantineRecord {
    let denial = inspect_wal_payload(&torn_wal_payload(range)).unwrap_err();
    let finding = ExecutedQuarantineFinding::from_wal_frame_denial(&denial).unwrap();
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .unwrap()
}

pub(crate) fn recovery_blocking_wal_frame_damage(
    range: WalLsnRange,
) -> RecoveryBlockedByIntegrityDamage {
    recovery_blocking_damage_from_payload(&wal_payload(range, 0, "checksum-fail"))
}

pub(crate) fn recovery_blocking_torn_wal_frame_damage(
    range: WalLsnRange,
) -> RecoveryBlockedByIntegrityDamage {
    recovery_blocking_damage_from_payload(&torn_wal_payload(range))
}

fn recovery_blocking_damage_from_payload(payload: &[u8]) -> RecoveryBlockedByIntegrityDamage {
    let denial = inspect_wal_payload(payload).unwrap_err();
    RecoveryBlockedByIntegrityDamage::damaged_wal_frame(&denial)
}
