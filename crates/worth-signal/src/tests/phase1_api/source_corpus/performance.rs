pub(in crate::tests::phase1_api) const PERFORMANCE_SUPPORT_SOURCE: &str = concat!(
    include_str!("../../performance_support.rs"),
    include_str!("../../performance_support/measurement_capture.rs"),
    include_str!("../../performance_support/profile_comparison.rs"),
    include_str!("../../performance_support/workload_construction.rs"),
);
pub(in crate::tests::phase1_api) const PERFORMANCE_PROFILES_SOURCE: &str = concat!(
    include_str!("../../performance_profiles.rs"),
    include_str!("../../performance_profiles/chain_bootstrap.rs"),
    include_str!("../../performance_profiles/dependency_reconciliation_rotating.rs"),
    include_str!("../../performance_profiles/dependency_reconciliation_stable_shape.rs"),
    include_str!("../../performance_profiles/dependency_reconciliation_staged.rs"),
    include_str!("../../performance_profiles/fintech_fanout.rs"),
    include_str!("../../performance_profiles/observability_profile.rs"),
    include_str!("../../performance_profiles/suppression_fanout.rs"),
    include_str!("../../performance_profiles/topology_rewiring.rs"),
);
pub(in crate::tests::phase1_api) const PERFORMANCE_BASELINE_SOURCE: &str =
    include_str!("../../performance_baseline.json");
