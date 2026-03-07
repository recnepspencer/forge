use crate::facade::*;

#[test]
fn tier_policy_supports_caller_defined_n_granularity() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum AppTier {
        Component,
        System,
        Frame,
        Scene,
    }

    let policies = [
        TierPolicy::new(
            AppTier::Component,
            DependencyMode::Static,
            DirtyPropagation::Batched,
            EvaluationTrigger::Checkpoint(CheckpointBarrier::PerOperation),
        ),
        TierPolicy::new(
            AppTier::System,
            DependencyMode::AutoDiscovered,
            DirtyPropagation::Immediate,
            EvaluationTrigger::LazyPull,
        ),
        TierPolicy::new(
            AppTier::Frame,
            DependencyMode::AutoDiscovered,
            DirtyPropagation::Immediate,
            EvaluationTrigger::OnDemand,
        ),
        TierPolicy::new(
            AppTier::Scene,
            DependencyMode::AutoDiscovered,
            DirtyPropagation::Immediate,
            EvaluationTrigger::Async,
        ),
    ];

    assert_eq!(policies.len(), 4);
    assert!(matches!(
        policies[0].evaluation_trigger,
        EvaluationTrigger::Checkpoint(CheckpointBarrier::PerOperation)
    ));
    assert_eq!(
        policies[0].default_comparator,
        VersionComparatorPolicy::Exact
    );
}
