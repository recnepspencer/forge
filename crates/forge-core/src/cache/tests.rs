use super::{CacheCheckpoint, CacheDirtyState, CacheRefreshMode, CacheRefreshPolicy, DomainImpact};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TestDomain {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TestTarget {
    X,
    Y,
}

#[test]
fn domain_impact_global_supersedes_targets() {
    let mut impact = DomainImpact::empty();
    impact.add_targets([TestTarget::X, TestTarget::Y]);
    assert!(!impact.is_global());
    assert_eq!(impact.targets().count(), 2);

    impact.mark_global();
    assert!(impact.is_global());
    assert_eq!(impact.targets().count(), 0);
}

#[test]
fn dirty_state_tracks_domains_deterministically() {
    let mut dirty = CacheDirtyState::<TestDomain, TestTarget>::default();
    dirty.mark_domain_targets(TestDomain::B, [TestTarget::X]);
    dirty.mark_domain_targets(TestDomain::A, [TestTarget::Y]);

    let domains: Vec<_> = dirty.dirty_domains().collect();
    assert_eq!(domains, vec![TestDomain::A, TestDomain::B]);
}

#[test]
fn refresh_policy_resolves_domain_override() {
    let mut policy = CacheRefreshPolicy::<TestDomain>::new(CacheRefreshMode::DeferredTo(
        CacheCheckpoint::PerOperation,
    ));
    policy.set_mode(TestDomain::A, CacheRefreshMode::Eager);

    assert_eq!(policy.mode_for(TestDomain::A), CacheRefreshMode::Eager);
    assert_eq!(
        policy.mode_for(TestDomain::B),
        CacheRefreshMode::DeferredTo(CacheCheckpoint::PerOperation)
    );
}
