use super::runtime_world::{build_runtime, Domain, Tier};
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
use crate::facade::{DiagnosticsTier, SignalRuntimePolicy};
use crate::logic::transaction::SignalRuntime;

#[test]
fn runtime_reset_runtime_policy_to_tier_restores_stock_posture() {
    let mut runtime = SignalRuntime::builder(crate::data::graph::SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime.set_runtime_policy(
        SignalRuntimePolicy::forensic()
            .with_history_limit(19)
            .with_detail_limit(37)
            .with_history_details(false),
    );

    runtime.reset_runtime_policy_to_tier(DiagnosticsTier::Operational);

    assert_eq!(runtime.runtime_policy(), SignalRuntimePolicy::operational());
}

#[test]
fn builder_adjust_helpers_keep_advanced_policy_changes_grouped() {
    let runtime = SignalRuntime::builder(crate::data::graph::SignalGraph::new())
        .with_kernel_defaults()
        .with_domains::<Domain>()
        .adjust_runtime_policy(|policy| policy.with_history_limit(9).with_detail_limit(3))
        .adjust_fallback_comparator(|_| VersionComparatorPolicy::Tolerance { epsilon: 4 })
        .adjust_checkpoints(|policy| {
            policy.set_barrier(Domain::Cache, CheckpointBarrier::PerCommit)
        })
        .build();

    assert_eq!(runtime.runtime_policy().retention_budget.history_limit, 9);
    assert_eq!(runtime.runtime_policy().retention_budget.detail_limit, 3);
    assert_eq!(
        *runtime.config().fallback_comparator(),
        VersionComparatorPolicy::Tolerance { epsilon: 4 }
    );
    assert_eq!(
        runtime.checkpoint().policy().barrier_for(Domain::Cache),
        CheckpointBarrier::PerCommit
    );
}

#[test]
fn runtime_adjust_helpers_update_existing_policy_owners() {
    let mut runtime = build_runtime(crate::data::graph::SignalGraph::new());

    runtime.adjust_runtime_policy(|policy| policy.with_history_limit(11).with_detail_limit(5));
    runtime.adjust_fallback_comparator(|_| VersionComparatorPolicy::Tolerance { epsilon: 7 });
    runtime.set_tier_policy(TierPolicy::new(
        Tier::A,
        DependencyMode::AutoDiscovered,
        DirtyPropagation::Immediate,
        EvaluationTrigger::LazyPull,
    ));
    assert!(runtime.adjust_tier_policy(Tier::A, |policy| {
        policy.with_default_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 })
    }));
    runtime.set_domain_checkpoint_barrier(Domain::Cache, CheckpointBarrier::PerCommit);

    assert_eq!(runtime.runtime_policy().retention_budget.history_limit, 11);
    assert_eq!(runtime.runtime_policy().retention_budget.detail_limit, 5);
    assert_eq!(
        *runtime.config().fallback_comparator(),
        VersionComparatorPolicy::Tolerance { epsilon: 7 }
    );
    assert_eq!(
        runtime
            .config()
            .tier_policies()
            .get(Tier::A)
            .unwrap()
            .default_comparator,
        VersionComparatorPolicy::Tolerance { epsilon: 2 }
    );
    assert_eq!(
        runtime.checkpoint().policy().barrier_for(Domain::Cache),
        CheckpointBarrier::PerCommit
    );
}
