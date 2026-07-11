#![forbid(unsafe_code)]

pub mod append;
pub mod checkpoint;
pub mod layout_access;
pub mod recovery_read;
pub mod wal_topology;

mod blob_records;
mod durability;
mod durable_publication;
mod operation_denial;
mod security_metadata;
#[cfg(test)]
mod security_metadata_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurablePublicationPhase {
    Prepared,
    Logged,
    Acknowledged,
    Recovered,
}

pub use append::admit_durable_append;
pub use blob_records::{
    durable_phase_for_record_kind, record_kind_admits_recovery_replay, BlobWalRecordEnvelope,
    BlobWalRecordIdentity, BlobWalRecordKind, BlobWalRecordScopeDenial,
    BlobWalReplayRebuildWitness,
};
pub use checkpoint::{admit_checkpoint_cutover, admit_checkpoint_publication};
pub use durability::{WalQueueExecutionDeclaration, WalQueueExecutionKind, WalQueueGroupingScope};
pub use durable_publication::{
    CheckpointDurablePublicationScope, DurablePublicationDeclaration, DurablePublicationScope,
    WalFrameDurablePublicationScope,
};
pub use layout_access::{
    AdmittedCheckpointPublicationReceipt, AdmittedReplayTailCursor, AdmittedWalAppendReceipt,
    CheckpointPublicationLayoutReport, WalAppendLayoutReport, WalReplayTailCursorReport,
    WalReplayTailRecordReport,
};
pub use operation_denial::{WalOperationDenial, WalOperationDenialKind};
pub use recovery_read::{admit_replay_cursor, inspect_replay_tail_record};
pub use security_metadata::{
    CheckpointRecordSecurityMetadataEnvelope, StoreCheckpointRecordIdentity,
    StoreWalRecordIdentity, WalRecordSecurityMetadataEnvelope, WalSecurityMetadataCarrier,
    WalSecurityMetadataEnvelope,
};
pub use wal_topology::{
    LogSequenceNumber, ReplayCursor, ReplayCursorSegment, WalFrameOrderingProof, WalLsnRange,
    WalSegmentGeneration, WalSegmentId, WalSegmentScanRecord, WalTopologyDenial,
    WalTopologyDenialKind, WalTopologyScan,
};
