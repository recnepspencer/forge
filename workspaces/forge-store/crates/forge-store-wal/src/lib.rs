#![forbid(unsafe_code)]

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

pub use security_metadata::{
    CheckpointRecordSecurityMetadataEnvelope, StoreCheckpointRecordIdentity,
    StoreWalRecordIdentity, WalRecordSecurityMetadataEnvelope, WalSecurityMetadataCarrier,
    WalSecurityMetadataEnvelope,
};
