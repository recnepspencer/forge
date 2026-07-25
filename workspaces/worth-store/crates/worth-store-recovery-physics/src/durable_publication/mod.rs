mod checkpoint_publication;
mod replay;
mod source_precedence;
mod wal_publication;

pub use checkpoint_publication::{
    DurableCheckpointPublication, DurableManifestPublication, StoreDurablePublicationDenial,
    StoreDurablePublicationDenialKind,
};
pub use replay::{DurabilityReplayIdentity, DurabilityReplayKind};
pub use source_precedence::{
    CheckpointCrashDurabilityPosture, DurabilityRecoveryReplaySource,
    DurabilityRecoverySourcePrecedence,
};
pub use wal_publication::DurableWalPublication;

#[cfg(feature = "certification-test-authority")]
mod test_support;

#[cfg(feature = "certification-test-authority")]
pub use test_support::certified_durable_wal_publication_for_test;

#[cfg(all(test, feature = "certification-test-authority"))]
mod tests;
