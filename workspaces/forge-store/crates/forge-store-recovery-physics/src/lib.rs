#![forbid(unsafe_code)]

mod memory_envelope;

pub use memory_envelope::{RecoveryMemoryEnvelope, RecoveryMemoryEnvelopeDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogSequenceNumber(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRecoverySource {
    Checkpoint,
    WalTail,
    Manifest,
    Quarantine,
}
