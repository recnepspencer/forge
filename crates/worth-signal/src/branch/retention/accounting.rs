use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::state::SignalBranchId;

use super::outcome::{SignalBranchRetentionOwnerPosture, SignalBranchRetentionTerminalCounts};

/// Why one external obligation reached its terminal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SignalRetentionTerminalCause {
    ExplicitRelease,
    DroppedLease,
}

/// Owner accounting produced by exactly one terminal external release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SignalRetentionReleaseAccounting {
    pub(crate) branch_id: SignalBranchId,
    pub(crate) remaining_target_leases: u32,
    pub(crate) remaining_branch_leases: u32,
}

/// Terminality the narrow retention owner has recorded.
///
/// It lives in the cleanup ledger rather than the owner, so it stays readable
/// through an outstanding obligation after the owning runtime is gone.
#[derive(Debug, Default)]
pub(super) struct SignalRetentionTerminalAccounting {
    explicit_releases: AtomicU64,
    dropped_releases: AtomicU64,
    owner_loss_releases: AtomicU64,
    unknown_lease_defenses: AtomicU64,
}

impl SignalRetentionTerminalAccounting {
    /// Record one terminal attempt.
    ///
    /// The cause and the owner posture are independent axes: a dropped release
    /// after owner loss is both a dropped release and an owner-loss release, so
    /// both are recorded rather than collapsed. An attempt the ledger did not
    /// recognise is recorded only as a defended unknown lease.
    pub(super) fn record(
        &self,
        cause: SignalRetentionTerminalCause,
        posture: SignalBranchRetentionOwnerPosture,
        released: bool,
    ) {
        if !released {
            self.unknown_lease_defenses.fetch_add(1, Ordering::Relaxed);
            return;
        }
        match cause {
            SignalRetentionTerminalCause::ExplicitRelease => {
                self.explicit_releases.fetch_add(1, Ordering::Relaxed);
            }
            SignalRetentionTerminalCause::DroppedLease => {
                self.dropped_releases.fetch_add(1, Ordering::Relaxed);
            }
        }
        if posture == SignalBranchRetentionOwnerPosture::Lost {
            self.owner_loss_releases.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub(super) fn counts(&self) -> SignalBranchRetentionTerminalCounts {
        SignalBranchRetentionTerminalCounts::owner_observed(
            self.explicit_releases.load(Ordering::Relaxed),
            self.dropped_releases.load(Ordering::Relaxed),
            self.owner_loss_releases.load(Ordering::Relaxed),
            self.unknown_lease_defenses.load(Ordering::Relaxed),
        )
    }
}

pub(super) fn obligation_count<K>(counts: &HashMap<K, u32>, key: &K) -> u32
where
    K: Eq + Hash,
{
    counts.get(key).copied().unwrap_or(0)
}

pub(super) fn increment_obligation_count<K>(counts: &mut HashMap<K, u32>, key: K)
where
    K: Eq + Hash,
{
    let count = counts.entry(key).or_default();
    *count = count.saturating_add(1);
}

/// Decrement one obligation count and drop the entry once it reaches zero, so
/// the ledger never accumulates keys for targets nothing retains.
pub(super) fn decrement_obligation_count<K>(counts: &mut HashMap<K, u32>, key: &K) -> u32
where
    K: Eq + Hash,
{
    let remaining = counts
        .get_mut(key)
        .map(|count| {
            *count = count.saturating_sub(1);
            *count
        })
        .unwrap_or(0);
    if remaining == 0 {
        counts.remove(key);
    }
    remaining
}
