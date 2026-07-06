#![forbid(unsafe_code)]

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
};
pub use durable_publication::{
    CheckpointDurablePublicationScope, DurablePublicationDeclaration, DurablePublicationScope,
    WalFrameDurablePublicationScope,
};
pub use s6_queue_work::{
    WalQueueExecutionDeclaration, WalQueueExecutionKind, WalQueueGroupingScope,
};
pub use security_metadata::{
    CheckpointRecordSecurityMetadataEnvelope, StoreCheckpointRecordIdentity,
    StoreWalRecordIdentity, WalRecordSecurityMetadataEnvelope, WalSecurityMetadataCarrier,
    WalSecurityMetadataEnvelope,
};
