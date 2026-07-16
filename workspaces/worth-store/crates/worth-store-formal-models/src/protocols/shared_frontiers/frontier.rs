#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedDurabilityFrontier {
    Pending,
    Admitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedVisibilityFrontier {
    Stable,
    CompactionCutover,
    Reopened,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedReachabilityFrontier {
    Reachable,
    LiveLease,
    ReleaseEligible,
    Reused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedQuarantineFrontier {
    Clear,
    Sealed,
    VerificationPending,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SharedAdmissionFrontier {
    None,
    ImportPending,
    ReplicationPending,
    ExternalDurable,
    Divergence,
    Published,
}
