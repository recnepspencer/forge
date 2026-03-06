//! Deterministic dirty-state tracking for cache domains.

use std::collections::{BTreeMap, BTreeSet};

use super::policy::CacheDomain;

/// Dirty scope for one domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainImpact<Target: Ord + Copy> {
    global: bool,
    targets: BTreeSet<Target>,
}

impl<Target: Ord + Copy> DomainImpact<Target> {
    /// Empty impact (not dirty).
    pub fn empty() -> Self {
        Self {
            global: false,
            targets: BTreeSet::new(),
        }
    }

    /// Mark as globally dirty.
    pub fn mark_global(&mut self) {
        self.global = true;
        self.targets.clear();
    }

    /// Add targeted dirty keys.
    pub fn add_targets<I>(&mut self, targets: I)
    where
        I: IntoIterator<Item = Target>,
    {
        if self.global {
            return;
        }
        self.targets.extend(targets);
    }

    /// Whether this domain is globally dirty.
    pub fn is_global(&self) -> bool {
        self.global
    }

    /// Targeted dirty keys, deterministic order.
    pub fn targets(&self) -> impl Iterator<Item = Target> + '_ {
        self.targets.iter().copied()
    }

    /// Whether this impact is empty.
    pub fn is_empty(&self) -> bool {
        !self.global && self.targets.is_empty()
    }
}

impl<Target: Ord + Copy> Default for DomainImpact<Target> {
    fn default() -> Self {
        Self::empty()
    }
}

/// Full dirty-state map across domains.
#[derive(Debug, Clone)]
pub struct CacheDirtyState<Domain: CacheDomain, Target: Ord + Copy> {
    by_domain: BTreeMap<Domain, DomainImpact<Target>>,
}

impl<Domain: CacheDomain, Target: Ord + Copy> CacheDirtyState<Domain, Target> {
    /// Mark an entire domain as dirty.
    pub fn mark_domain_global(&mut self, domain: Domain) {
        self.by_domain.entry(domain).or_default().mark_global();
    }

    /// Mark targeted keys dirty for a domain.
    pub fn mark_domain_targets<I>(&mut self, domain: Domain, targets: I)
    where
        I: IntoIterator<Item = Target>,
    {
        self.by_domain
            .entry(domain)
            .or_default()
            .add_targets(targets);
    }

    /// Current impact for one domain.
    pub fn impact_for(&self, domain: Domain) -> Option<&DomainImpact<Target>> {
        self.by_domain.get(&domain)
    }

    /// Whether a domain is dirty.
    pub fn is_dirty(&self, domain: Domain) -> bool {
        self.by_domain
            .get(&domain)
            .map(|i| !i.is_empty())
            .unwrap_or(false)
    }

    /// Drain and return impact for one domain.
    pub fn take_impact(&mut self, domain: Domain) -> Option<DomainImpact<Target>> {
        self.by_domain.remove(&domain)
    }

    /// Deterministic iterator of currently dirty domains.
    pub fn dirty_domains(&self) -> impl Iterator<Item = Domain> + '_ {
        self.by_domain
            .iter()
            .filter(|(_, impact)| !impact.is_empty())
            .map(|(domain, _)| *domain)
    }
}

impl<Domain: CacheDomain, Target: Ord + Copy> Default for CacheDirtyState<Domain, Target> {
    fn default() -> Self {
        Self {
            by_domain: BTreeMap::new(),
        }
    }
}
