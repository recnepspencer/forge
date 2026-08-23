use worth_store_physical_format::PhysicalGenerationOwner;
use worth_store_physical_integrity::{
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceBundle,
    PhysicalIntegrityEvidenceProfile, StoreExecutedIntegrityEvidence, WalFrameIntegrityReport,
};
use worth_store_wal::{LogSequenceNumber, WalLsnRange};

use super::construction::{inspect_wal_payload, inspect_wal_payload_for_owner, intact_wal_payload};

pub fn intact_wal_integrity_evidence() -> PhysicalIntegrityEvidenceBundle {
    let range = WalLsnRange::new(LogSequenceNumber::new(40), LogSequenceNumber::new(50)).unwrap();
    let report = inspect_wal_payload(&intact_wal_payload(range)).unwrap();
    integrity_evidence_from_report(&report)
}

pub fn intact_wal_integrity_evidence_for_owner(
    owner: PhysicalGenerationOwner,
) -> PhysicalIntegrityEvidenceBundle {
    let range = WalLsnRange::new(LogSequenceNumber::new(40), LogSequenceNumber::new(50)).unwrap();
    let report = inspect_wal_payload_for_owner(&intact_wal_payload(range), owner).unwrap();
    integrity_evidence_from_report(&report)
}

fn integrity_evidence_from_report(
    report: &WalFrameIntegrityReport,
) -> PhysicalIntegrityEvidenceBundle {
    PhysicalIntegrityEvidenceAuthority::store_local()
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_wal_frame(report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap()
}
