use crate::EvictionProtectionSummary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvictionCounterSnapshot {
    plan_attempt_count: u64,
    resident_frame_scan_count: u64,
    candidate_count: u64,
    protected_exclusion_count: u64,
    pinned_exclusion_count: u64,
    dirty_unpublished_exclusion_count: u64,
    verifier_protected_exclusion_count: u64,
    recovery_protected_exclusion_count: u64,
    streaming_protected_exclusion_count: u64,
    policy_rank_count: u64,
    all_protected_denial_count: u64,
    no_resident_candidate_denial_count: u64,
    stale_plan_denial_count: u64,
    receipt_count: u64,
}

impl EvictionCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            plan_attempt_count: 0,
            resident_frame_scan_count: 0,
            candidate_count: 0,
            protected_exclusion_count: 0,
            pinned_exclusion_count: 0,
            dirty_unpublished_exclusion_count: 0,
            verifier_protected_exclusion_count: 0,
            recovery_protected_exclusion_count: 0,
            streaming_protected_exclusion_count: 0,
            policy_rank_count: 0,
            all_protected_denial_count: 0,
            no_resident_candidate_denial_count: 0,
            stale_plan_denial_count: 0,
            receipt_count: 0,
        }
    }

    pub(crate) const fn with_plan_attempt(self) -> Self {
        Self {
            plan_attempt_count: self.plan_attempt_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_resident_frame_scanned(self) -> Self {
        Self {
            resident_frame_scan_count: self.resident_frame_scan_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_candidate(self) -> Self {
        Self {
            candidate_count: self.candidate_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_protected_exclusion(
        self,
        protection: EvictionProtectionSummary,
    ) -> Self {
        Self {
            protected_exclusion_count: self.protected_exclusion_count + 1,
            pinned_exclusion_count: self.pinned_exclusion_count + protection.pinned_count(),
            dirty_unpublished_exclusion_count: self.dirty_unpublished_exclusion_count
                + protection.dirty_unpublished_count(),
            verifier_protected_exclusion_count: self.verifier_protected_exclusion_count
                + protection.verifier_protected_count(),
            recovery_protected_exclusion_count: self.recovery_protected_exclusion_count
                + protection.recovery_protected_count(),
            streaming_protected_exclusion_count: self.streaming_protected_exclusion_count
                + protection.streaming_protected_count(),
            ..self
        }
    }

    pub(crate) const fn with_policy_rank(self) -> Self {
        Self {
            policy_rank_count: self.policy_rank_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_all_protected_denial(self) -> Self {
        Self {
            all_protected_denial_count: self.all_protected_denial_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_no_resident_candidate_denial(self) -> Self {
        Self {
            no_resident_candidate_denial_count: self.no_resident_candidate_denial_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_stale_plan_denial(self) -> Self {
        Self {
            stale_plan_denial_count: self.stale_plan_denial_count + 1,
            ..self
        }
    }

    pub(crate) const fn with_receipt(self) -> Self {
        Self {
            receipt_count: self.receipt_count + 1,
            ..self
        }
    }

    pub const fn plan_attempt_count(self) -> u64 {
        self.plan_attempt_count
    }

    pub const fn resident_frame_scan_count(self) -> u64 {
        self.resident_frame_scan_count
    }

    pub const fn candidate_count(self) -> u64 {
        self.candidate_count
    }

    pub const fn protected_exclusion_count(self) -> u64 {
        self.protected_exclusion_count
    }

    pub const fn pinned_exclusion_count(self) -> u64 {
        self.pinned_exclusion_count
    }

    pub const fn dirty_unpublished_exclusion_count(self) -> u64 {
        self.dirty_unpublished_exclusion_count
    }

    pub const fn verifier_protected_exclusion_count(self) -> u64 {
        self.verifier_protected_exclusion_count
    }

    pub const fn recovery_protected_exclusion_count(self) -> u64 {
        self.recovery_protected_exclusion_count
    }

    pub const fn streaming_protected_exclusion_count(self) -> u64 {
        self.streaming_protected_exclusion_count
    }

    pub const fn policy_rank_count(self) -> u64 {
        self.policy_rank_count
    }

    pub const fn all_protected_denial_count(self) -> u64 {
        self.all_protected_denial_count
    }

    pub const fn no_resident_candidate_denial_count(self) -> u64 {
        self.no_resident_candidate_denial_count
    }

    pub const fn stale_plan_denial_count(self) -> u64 {
        self.stale_plan_denial_count
    }

    pub const fn receipt_count(self) -> u64 {
        self.receipt_count
    }
}
