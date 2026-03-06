//! Deterministic batched dirty tracking for entity-tier scheduling.

use std::collections::{BTreeMap, BTreeSet};

/// Dirty impact for one domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainImpact<I: Copy + Ord> {
    global: bool,
    scoped: BTreeSet<I>,
}

impl<I: Copy + Ord> DomainImpact<I> {
    /// Empty, clean impact.
    pub fn empty() -> Self {
        Self {
            global: false,
            scoped: BTreeSet::new(),
        }
    }

    /// Whether this domain is fully dirty.
    pub fn is_global(&self) -> bool {
        self.global
    }

    /// Whether no dirty signal is recorded.
    pub fn is_empty(&self) -> bool {
        !self.global && self.scoped.is_empty()
    }

    /// Mark this domain globally dirty and drop scoped keys.
    pub fn mark_global(&mut self) {
        self.global = true;
        self.scoped.clear();
    }

    /// Add one scoped impact key.
    pub fn add_scoped(&mut self, impact: I) {
        if !self.global {
            self.scoped.insert(impact);
        }
    }

    /// Add multiple scoped impact keys.
    pub fn add_scoped_many<T>(&mut self, impacts: T)
    where
        T: IntoIterator<Item = I>,
    {
        if self.global {
            return;
        }
        self.scoped.extend(impacts);
    }

    /// Iterate scoped impact keys in deterministic order.
    pub fn scoped(&self) -> impl Iterator<Item = I> + '_ {
        self.scoped.iter().copied()
    }
}

impl<I: Copy + Ord> Default for DomainImpact<I> {
    fn default() -> Self {
        Self::empty()
    }
}

/// Batched dirty map from domain -> impact.
#[derive(Debug, Clone)]
pub struct BatchedDirtySet<D: Copy + Ord, I: Copy + Ord> {
    by_domain: BTreeMap<D, DomainImpact<I>>,
}

impl<D: Copy + Ord, I: Copy + Ord> BatchedDirtySet<D, I> {
    /// Create an empty dirty set.
    pub fn new() -> Self {
        Self {
            by_domain: BTreeMap::new(),
        }
    }

    /// Whether all domains are clean.
    pub fn is_empty(&self) -> bool {
        self.by_domain.values().all(DomainImpact::is_empty)
    }

    /// Mark one domain globally dirty.
    pub fn mark_domain_global(&mut self, domain: D) {
        self.by_domain.entry(domain).or_default().mark_global();
    }

    /// Mark one scoped impact on one domain.
    pub fn mark_domain_scoped(&mut self, domain: D, impact: I) {
        self.by_domain.entry(domain).or_default().add_scoped(impact);
    }

    /// Mark multiple scoped impacts on one domain.
    pub fn mark_domain_scoped_many<T>(&mut self, domain: D, impacts: T)
    where
        T: IntoIterator<Item = I>,
    {
        self.by_domain
            .entry(domain)
            .or_default()
            .add_scoped_many(impacts);
    }

    /// Merge a precomputed impact for one domain.
    pub fn merge_domain_impact(&mut self, domain: D, impact: DomainImpact<I>) {
        let current = self.by_domain.entry(domain).or_default();
        if impact.is_global() {
            current.mark_global();
            return;
        }
        current.add_scoped_many(impact.scoped());
    }

    /// Whether a domain currently has dirty impact.
    pub fn is_domain_dirty(&self, domain: D) -> bool {
        self.by_domain
            .get(&domain)
            .map(|impact| !impact.is_empty())
            .unwrap_or(false)
    }

    /// Deterministic iterator of dirty domains.
    pub fn dirty_domains(&self) -> impl Iterator<Item = D> + '_ {
        self.by_domain
            .iter()
            .filter_map(|(domain, impact)| (!impact.is_empty()).then_some(*domain))
    }

    /// Read-only impact for a domain.
    pub fn impact_for(&self, domain: D) -> Option<&DomainImpact<I>> {
        self.by_domain.get(&domain)
    }

    /// Take and clear impact for one domain.
    pub fn take_domain_impact(&mut self, domain: D) -> Option<DomainImpact<I>> {
        self.by_domain.remove(&domain)
    }

    /// Clear all dirty impacts.
    pub fn clear(&mut self) {
        self.by_domain.clear();
    }
}

impl<D: Copy + Ord, I: Copy + Ord> Default for BatchedDirtySet<D, I> {
    fn default() -> Self {
        Self::new()
    }
}
