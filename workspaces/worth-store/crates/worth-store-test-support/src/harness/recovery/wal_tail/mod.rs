mod construction;
mod hostile_denials;
mod source_precedence_admission;
mod wal_tail_evidence;

pub use construction::physical_integrity_model_payload;
pub use hostile_denials::{
    quarantined_torn_wal_tail_handoff, recovery_blocking_torn_wal_frame_damage,
    recovery_blocking_wal_frame_damage, wal_only_tail_denial_from_torn_frame,
    wal_tail_quarantine_record,
};
pub use source_precedence_admission::wal_only_tail_proof;
pub use wal_tail_evidence::{
    intact_wal_integrity_evidence, intact_wal_integrity_evidence_for_owner,
};
