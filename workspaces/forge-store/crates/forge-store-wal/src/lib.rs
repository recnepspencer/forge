#![forbid(unsafe_code)]

pub mod layout_access;
pub mod wal_topology;

mod blob_records;
mod durable_publication;
mod s6_queue_work;
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

pub use blob_records::{
    durable_phase_for_record_kind, record_kind_admits_recovery_replay, BlobWalRecordEnvelope,
    BlobWalRecordIdentity, BlobWalRecordKind, BlobWalRecordScopeDenial,
    BlobWalReplayRebuildWitness,
};
pub use durable_publication::{
    CheckpointDurablePublicationScope, DurablePublicationDeclaration, DurablePublicationScope,
    WalFrameDurablePublicationScope,
};
pub use layout_access::{
    AdmittedCheckpointLayoutFamily, AdmittedCheckpointLayoutRule,
    AdmittedCheckpointPublicationReceipt, AdmittedDurableMutationLayoutFamily,
    AdmittedReplayTailCursor, AdmittedReplayTailLayoutFamily, AdmittedWalAppendLayoutRule,
    AdmittedWalAppendReceipt, AdmittedWalTailLayoutRule, CheckpointLayoutFamilyHome,
    CheckpointPublicationLayoutReport, DurableMutationLayoutFamilyHome, ReplayTailLayoutFamilyHome,
    WalAppendLayoutReport, WalLayoutAccess, WalLayoutAccessDenial, WalLayoutAccessDenialKind,
    WalReplayTailCursorReport, WalReplayTailRecordReport,
};
pub use s6_queue_work::{
    WalQueueExecutionDeclaration, WalQueueExecutionKind, WalQueueGroupingScope,
};
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
