#![doc = include_str!("recovery_physics_compile_fail_proofs.md")]
//! S.3 WAL integrity reports cannot satisfy S.4 replay receipts:
//!
//! ```compile_fail
//! use forge_store_physical_integrity::WalFrameIntegrityReport;
//! use forge_store_recovery_physics::WalReplayReceipt;
//!
//! fn requires_replay_receipt(_: WalReplayReceipt) {}
//!
//! let report: WalFrameIntegrityReport = todo!();
//! requires_replay_receipt(report);
//! ```
//!
//! S.3 checkpoint-adjacent reports cannot satisfy checkpoint validity decisions:
//!
//! ```compile_fail
//! use forge_store_physical_integrity::CheckpointRecordIntegrityReport;
//! use forge_store_recovery_physics::CheckpointValidityDecision;
//!
//! fn requires_checkpoint_decision(_: CheckpointValidityDecision) {}
//!
//! let report: CheckpointRecordIntegrityReport = todo!();
//! requires_checkpoint_decision(report);
//! ```

#![forbid(unsafe_code)]

mod integrity_damage_map;
mod integrity_input;
mod integrity_vetted_records;
mod memory_envelope;
mod recovery_blocking_integrity;
mod recovery_integrity_handoff_receipt;
mod replay_receipt;
mod s4_integrity_handoff_denial;
mod s4_integrity_handoff_payload;
mod s4_recovery_physics_integrity_readiness;

pub use integrity_damage_map::{IntegrityDamageMap, QuarantineSummary};
pub use integrity_input::RecoveryPhysicsIntegrityInput;
pub use integrity_vetted_records::{
    IntegrityVettedCheckpointRecord, IntegrityVettedPageFrameKind, IntegrityVettedPageFrameRecord,
    IntegrityVettedRootManifestRecord, IntegrityVettedSegmentManifestRecord,
    IntegrityVettedWalFrame,
};
pub use memory_envelope::{RecoveryMemoryEnvelope, RecoveryMemoryEnvelopeDenial};
pub use recovery_blocking_integrity::{
    RecoveryBlockedByIntegrityDamage, RecoveryBlockingIntegritySource,
};
pub use recovery_integrity_handoff_receipt::RecoveryIntegrityHandoffReceipt;
pub use replay_receipt::{CheckpointValidityDecision, WalReplayReceipt};
pub use s4_integrity_handoff_denial::{S4IntegrityHandoffDenial, S4IntegrityHandoffDenialKind};
pub use s4_integrity_handoff_payload::{
    BoundedInspectionEnvelopeEvidence, RawBytesExcludedFromRecoveryHandoff,
    S4ChecksumAlgorithmScopeBasis, S4IntegrityHandoffCounters, S4IntegrityHandoffPayload,
    S4IntegrityHandoffPayloadDeclaration,
};
pub use s4_recovery_physics_integrity_readiness::S4RecoveryPhysicsIntegrityReadiness;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogSequenceNumber(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoverySource {
    Checkpoint,
    WalTail,
    Manifest,
    Quarantine,
}
