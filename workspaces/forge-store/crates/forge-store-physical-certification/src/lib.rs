#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalCertificationLane {
    PowerLoss,
    TornWrite,
    ByteFlip,
    BoundedMemory,
    RecoveryTime,
    ForegroundLatency,
    BlobScale,
}
