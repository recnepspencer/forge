use worth_store_physical_integrity::{
    ExecutedQuarantineFinding, PhysicalQuarantineAuthority, QuarantineRecord,
    QuarantineSealRequest, RecoveryBlockedByIntegrityDamage,
};
use worth_store_wal::WalLsnRange;

use super::construction::{inspect_wal_payload, torn_wal_payload, wal_payload};

pub fn wal_tail_quarantine_record(range: WalLsnRange) -> QuarantineRecord {
    let denial = inspect_wal_payload(&torn_wal_payload(range)).unwrap_err();
    let finding = ExecutedQuarantineFinding::from_wal_frame_denial(&denial).unwrap();
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .unwrap()
}

pub fn recovery_blocking_wal_frame_damage(range: WalLsnRange) -> RecoveryBlockedByIntegrityDamage {
    recovery_blocking_damage_from_payload(&wal_payload(range, 0, "checksum-fail"))
}

pub fn recovery_blocking_torn_wal_frame_damage(
    range: WalLsnRange,
) -> RecoveryBlockedByIntegrityDamage {
    recovery_blocking_damage_from_payload(&torn_wal_payload(range))
}

fn recovery_blocking_damage_from_payload(payload: &[u8]) -> RecoveryBlockedByIntegrityDamage {
    let denial = inspect_wal_payload(payload).unwrap_err();
    RecoveryBlockedByIntegrityDamage::damaged_wal_frame(&denial)
}
