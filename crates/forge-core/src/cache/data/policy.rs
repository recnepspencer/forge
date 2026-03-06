//! Cache refresh policy types.

use std::collections::BTreeMap;

/// Marker trait for cache-domain enums.
pub trait CacheDomain: Copy + Ord + Eq + 'static {}

impl<T> CacheDomain for T where T: Copy + Ord + Eq + 'static {}

/// Lifecycle checkpoints where cache refresh may be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CacheCheckpoint {
    PerMutation,
    PerOperation,
    PerValidation,
    PerCommit,
    OnDemandRead,
}

/// Refresh mode for a specific cache domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheRefreshMode {
    /// Refresh immediately when a dirty mark is emitted.
    Eager,
    /// Refresh when the specified checkpoint is executed.
    DeferredTo(CacheCheckpoint),
    /// Refresh on first read if dirty.
    LazyOnRead,
}

/// Per-domain refresh policy map.
#[derive(Debug, Clone)]
pub struct CacheRefreshPolicy<D: CacheDomain> {
    defaults_to: CacheRefreshMode,
    per_domain: BTreeMap<D, CacheRefreshMode>,
}

impl<D: CacheDomain> CacheRefreshPolicy<D> {
    /// Create a policy where all domains default to the same refresh mode.
    pub fn new(defaults_to: CacheRefreshMode) -> Self {
        Self {
            defaults_to,
            per_domain: BTreeMap::new(),
        }
    }

    /// Override refresh mode for one domain.
    pub fn set_mode(&mut self, domain: D, mode: CacheRefreshMode) {
        self.per_domain.insert(domain, mode);
    }

    /// Resolve the refresh mode for one domain.
    pub fn mode_for(&self, domain: D) -> CacheRefreshMode {
        self.per_domain
            .get(&domain)
            .copied()
            .unwrap_or(self.defaults_to)
    }
}
