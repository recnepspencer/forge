#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HazardLeaseCounterSnapshot {
    acquired_leases: u64,
    released_leases: u64,
    revoked_leases: u64,
    owned_copy_conversions: u64,
    stale_release_denials: u64,
    expired_without_authority_denials: u64,
    range_bucket_lookups: u64,
    live_lookup_ranges: u64,
    range_comparisons: u64,
    overlapping_ranges: u64,
}

impl HazardLeaseCounterSnapshot {
    pub(crate) const fn with_acquire(mut self) -> Self {
        self.acquired_leases += 1;
        self
    }

    pub(crate) const fn with_release(mut self) -> Self {
        self.released_leases += 1;
        self
    }

    pub(crate) const fn with_revocation(mut self) -> Self {
        self.revoked_leases += 1;
        self
    }

    pub(crate) const fn with_owned_copy(mut self) -> Self {
        self.owned_copy_conversions += 1;
        self
    }

    pub(crate) const fn with_stale_release_denial(mut self) -> Self {
        self.stale_release_denials += 1;
        self
    }

    pub(crate) const fn with_lookup(
        mut self,
        live_lookup_ranges: u64,
        range_comparisons: u64,
        overlapping_ranges: u64,
    ) -> Self {
        self.live_lookup_ranges += live_lookup_ranges;
        self.range_comparisons += range_comparisons;
        self.overlapping_ranges += overlapping_ranges;
        self
    }

    pub(crate) const fn with_range_bucket_lookup(mut self) -> Self {
        self.range_bucket_lookups += 1;
        self
    }

    pub const fn acquired_leases(self) -> u64 {
        self.acquired_leases
    }

    pub const fn released_leases(self) -> u64 {
        self.released_leases
    }

    pub const fn revoked_leases(self) -> u64 {
        self.revoked_leases
    }

    pub const fn owned_copy_conversions(self) -> u64 {
        self.owned_copy_conversions
    }

    pub const fn stale_release_denials(self) -> u64 {
        self.stale_release_denials
    }

    pub const fn expired_without_authority_denials(self) -> u64 {
        self.expired_without_authority_denials
    }

    pub const fn range_bucket_lookups(self) -> u64 {
        self.range_bucket_lookups
    }

    pub const fn live_lookup_ranges(self) -> u64 {
        self.live_lookup_ranges
    }

    pub const fn range_comparisons(self) -> u64 {
        self.range_comparisons
    }

    pub const fn overlapping_ranges(self) -> u64 {
        self.overlapping_ranges
    }
}
