#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedRunTerminalKind {
    Completed,
    Cancelled,
    TimedOut,
    Exhausted,
    Degraded,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedRunCleanupDisposition {
    CleanupComplete,
    CleanupPending,
    RecoveryRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryManagedRunCleanupFailureKind {
    ProviderMemoryRetained,
    QueueRelease(worth_runtime_bridge::facade::BridgeManagedQueueFailureKind),
    BridgeFinalization(worth_runtime_bridge::facade::BridgeExecutionBasisFinalizationFailureKind),
}
