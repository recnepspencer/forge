#[path = "wal_only_tail_fixture/construction.rs"]
mod construction;
#[path = "wal_only_tail_fixture/hostile_denials.rs"]
mod hostile_denials;
#[path = "wal_only_tail_fixture/source_precedence_admission.rs"]
mod source_precedence_admission;
#[path = "wal_only_tail_fixture/wal_tail_evidence.rs"]
mod wal_tail_evidence;

pub(crate) use construction::physical_integrity_readiness;
pub(crate) use hostile_denials::{
    quarantined_torn_wal_tail_handoff, recovery_blocking_torn_wal_frame_damage,
    recovery_blocking_wal_frame_damage, wal_only_tail_denial_from_torn_frame,
    wal_tail_quarantine_record,
};
pub(crate) use source_precedence_admission::wal_only_tail_proof;
pub(crate) use wal_tail_evidence::{
    intact_wal_integrity_evidence, intact_wal_integrity_evidence_for_owner,
};
