#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessPolicyCounterStrength {
    Exact,
    Bounded,
    Sampled,
    Derived,
    CertificationOnly,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccessPolicyCounterSnapshot {
    buffered_admissions: u64,
    mmap_admissions: u64,
    direct_io_admissions: u64,
    mixed_mode_admissions: u64,
    page_cache_visibility_checks: u64,
    direct_io_alignment_checks: u64,
    mmap_fault_observations: u64,
    mixed_mode_invalidations: u64,
    security_scope_preservations: u64,
    denials: u64,
    violations: u64,
    strength: AccessPolicyCounterStrength,
}

impl AccessPolicyCounterSnapshot {
    pub const fn new(strength: AccessPolicyCounterStrength) -> Self {
        Self {
            buffered_admissions: 0,
            mmap_admissions: 0,
            direct_io_admissions: 0,
            mixed_mode_admissions: 0,
            page_cache_visibility_checks: 0,
            direct_io_alignment_checks: 0,
            mmap_fault_observations: 0,
            mixed_mode_invalidations: 0,
            security_scope_preservations: 0,
            denials: 0,
            violations: 0,
            strength,
        }
    }

    pub const fn buffered_admissions(self) -> u64 {
        self.buffered_admissions
    }
    pub const fn mmap_admissions(self) -> u64 {
        self.mmap_admissions
    }
    pub const fn direct_io_admissions(self) -> u64 {
        self.direct_io_admissions
    }
    pub const fn mixed_mode_admissions(self) -> u64 {
        self.mixed_mode_admissions
    }
    pub const fn page_cache_visibility_checks(self) -> u64 {
        self.page_cache_visibility_checks
    }
    pub const fn direct_io_alignment_checks(self) -> u64 {
        self.direct_io_alignment_checks
    }
    pub const fn mmap_fault_observations(self) -> u64 {
        self.mmap_fault_observations
    }
    pub const fn mixed_mode_invalidations(self) -> u64 {
        self.mixed_mode_invalidations
    }
    pub const fn security_scope_preservations(self) -> u64 {
        self.security_scope_preservations
    }
    pub const fn denials(self) -> u64 {
        self.denials
    }
    pub const fn violations(self) -> u64 {
        self.violations
    }
    pub const fn strength(self) -> AccessPolicyCounterStrength {
        self.strength
    }

    pub(crate) const fn with_buffered_admission(mut self) -> Self {
        self.buffered_admissions += 1;
        self
    }
    pub(crate) const fn with_mmap_admission(mut self) -> Self {
        self.mmap_admissions += 1;
        self
    }
    pub(crate) const fn with_direct_io_admission(mut self) -> Self {
        self.direct_io_admissions += 1;
        self
    }
    pub(crate) const fn with_mixed_mode_admission(mut self) -> Self {
        self.mixed_mode_admissions += 1;
        self
    }
    pub(crate) const fn with_page_cache_visibility_check(mut self) -> Self {
        self.page_cache_visibility_checks += 1;
        self
    }
    pub(crate) const fn with_direct_io_alignment_check(mut self) -> Self {
        self.direct_io_alignment_checks += 1;
        self
    }
    pub(crate) const fn with_mmap_fault_observation(mut self) -> Self {
        self.mmap_fault_observations += 1;
        self
    }
    pub(crate) const fn with_mixed_mode_invalidation(mut self) -> Self {
        self.mixed_mode_invalidations += 1;
        self
    }
    pub(crate) const fn with_security_scope_preservation(mut self) -> Self {
        self.security_scope_preservations += 1;
        self
    }
    pub(crate) const fn with_denial(mut self) -> Self {
        self.denials += 1;
        self
    }
    pub(crate) const fn with_violation(mut self) -> Self {
        self.violations += 1;
        self
    }
}
