//! Barrier policy for domain refresh scheduling.

use std::collections::BTreeMap;

use crate::data::checkpoint::CheckpointBarrier;

/// Per-domain barrier schedule.
#[derive(Debug, Clone)]
pub struct CheckpointPolicy<D: Copy + Ord> {
    default_barrier: CheckpointBarrier,
    per_domain: BTreeMap<D, CheckpointBarrier>,
}

impl<D: Copy + Ord> CheckpointPolicy<D> {
    /// Create a new policy with one default barrier for all domains.
    pub fn new(default_barrier: CheckpointBarrier) -> Self {
        Self {
            default_barrier,
            per_domain: BTreeMap::new(),
        }
    }

    /// Override barrier for one domain.
    pub fn set_barrier(&mut self, domain: D, barrier: CheckpointBarrier) {
        self.per_domain.insert(domain, barrier);
    }

    /// Resolve barrier for one domain.
    pub fn barrier_for(&self, domain: D) -> CheckpointBarrier {
        self.per_domain
            .get(&domain)
            .copied()
            .unwrap_or(self.default_barrier)
    }
}
