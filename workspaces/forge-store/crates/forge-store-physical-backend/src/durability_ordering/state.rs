#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreDurabilityState {
    WriteSubmitted,
    WriteAcceptedByBackend,
    WriteReachedDurabilityBoundary,
    ParentNamespaceDurable,
    RenameDurable,
    OrderingBarrierDurable,
    DurabilityUnsupported,
    DurabilityUnknown,
    Denied,
    Stale,
    RebindRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreDurabilityOperation {
    Flush,
    Fdatasync,
    Fsync,
    DirectorySync,
    Rename,
    WalPublication,
    CheckpointPublication,
    ManifestPublication,
}
