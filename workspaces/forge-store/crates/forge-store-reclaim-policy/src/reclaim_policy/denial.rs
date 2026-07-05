use super::ReclaimPolicyCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReclaimPolicyDenial {
    kind: ReclaimPolicyDenialKind,
    counters: ReclaimPolicyCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReclaimPolicyDenialKind {
    UnsupportedBackendPosture,
    MissingPosture,
    MissingPhysicalRegion,
    MissingProtectedReachability,
    MissingSecurityScope,
    MissingReclaimPermit,
    ProtectedReachabilityBlocked,
    PlatformGradeDenied,
    LaterLifecycleClaimAttempted,
}

impl ReclaimPolicyDenial {
    pub const fn new(
        kind: ReclaimPolicyDenialKind,
        counters: ReclaimPolicyCounterSnapshot,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(&self) -> &ReclaimPolicyDenialKind {
        &self.kind
    }

    pub const fn counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.counters
    }
}
