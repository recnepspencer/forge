//! Store-owned WAL identities consumed by fresh-process recovery cleanup.
//!
//! The recovery runtime reaches WAL facts through this physical-runtime
//! facade. The runtime therefore depends on the Store owner of the lifecycle,
//! while the WAL crate remains an implementation dependency of that owner.

pub use worth_store_wal::{
    LogSequenceNumber, VerifiedWalArtifact, WalLsnRange, WalSegmentArtifactIdentity,
    WalSegmentGeneration, WalSegmentId,
};
