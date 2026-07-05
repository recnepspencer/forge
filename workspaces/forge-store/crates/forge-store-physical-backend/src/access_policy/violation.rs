use super::AccessPolicyCounterSnapshot;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessPolicyViolationKind {
    None,
    MmapLazyFault,
    PageCacheVisibilityLost,
    DirectIoAlignmentContradicted,
    MixedModeInvalidationMissed,
    BackendContradictedWitness,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyExecutionObservation {
    violation: AccessPolicyViolationKind,
    page_cache_visibility_observed: bool,
    direct_io_alignment_observed: bool,
    mixed_mode_invalidation_observed: bool,
    security_scope_preserved: bool,
}

impl AccessPolicyExecutionObservation {
    pub const fn completed_without_violation() -> Self {
        Self {
            violation: AccessPolicyViolationKind::None,
            page_cache_visibility_observed: false,
            direct_io_alignment_observed: false,
            mixed_mode_invalidation_observed: false,
            security_scope_preserved: false,
        }
    }

    pub const fn mmap_lazy_fault() -> Self {
        Self {
            violation: AccessPolicyViolationKind::MmapLazyFault,
            page_cache_visibility_observed: false,
            direct_io_alignment_observed: false,
            mixed_mode_invalidation_observed: false,
            security_scope_preserved: false,
        }
    }

    pub const fn mixed_mode_invalidation_missed() -> Self {
        Self {
            violation: AccessPolicyViolationKind::MixedModeInvalidationMissed,
            page_cache_visibility_observed: false,
            direct_io_alignment_observed: false,
            mixed_mode_invalidation_observed: false,
            security_scope_preserved: false,
        }
    }

    pub const fn with_page_cache_visibility_observed(mut self) -> Self {
        self.page_cache_visibility_observed = true;
        self
    }

    pub const fn with_direct_io_alignment_observed(mut self) -> Self {
        self.direct_io_alignment_observed = true;
        self
    }

    pub const fn with_mixed_mode_invalidation_observed(mut self) -> Self {
        self.mixed_mode_invalidation_observed = true;
        self
    }

    pub const fn with_security_scope_preserved(mut self) -> Self {
        self.security_scope_preserved = true;
        self
    }

    pub const fn violation(self) -> AccessPolicyViolationKind {
        self.violation
    }
    pub const fn page_cache_visibility_observed(self) -> bool {
        self.page_cache_visibility_observed
    }
    pub const fn direct_io_alignment_observed(self) -> bool {
        self.direct_io_alignment_observed
    }
    pub const fn mixed_mode_invalidation_observed(self) -> bool {
        self.mixed_mode_invalidation_observed
    }
    pub const fn security_scope_preserved(self) -> bool {
        self.security_scope_preserved
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyViolation {
    kind: AccessPolicyViolationKind,
    counters: AccessPolicyCounterSnapshot,
}

impl AccessPolicyViolation {
    pub(crate) const fn new(
        kind: AccessPolicyViolationKind,
        counters: AccessPolicyCounterSnapshot,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> AccessPolicyViolationKind {
        self.kind
    }
    pub const fn counters(self) -> AccessPolicyCounterSnapshot {
        self.counters
    }
}
