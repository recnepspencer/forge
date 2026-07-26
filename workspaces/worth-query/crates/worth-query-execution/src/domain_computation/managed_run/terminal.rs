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
