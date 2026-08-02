use std::collections::BTreeSet;

use worth_store_formal_models::{
    DurabilityRecoveryAction as Action, DurabilityRecoveryDenial, DurabilityRecoveryFrontier,
};

use super::scenario::{
    execute_ordinary_durability_recovery, execute_ordinary_durability_recovery_traces,
    replay_acknowledgment_ordering_guard,
};

#[test]
fn ordinary_execution_covers_every_production_owned_durability_action() {
    let required = Action::production_owned()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let observed = execute_ordinary_durability_recovery()
        .into_iter()
        .filter(|action| required.contains(action))
        .collect::<BTreeSet<_>>();
    assert_eq!(observed, required);
}

#[test]
fn production_and_policy_actions_are_a_disjoint_complete_partition() {
    let production = Action::production_owned()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let policy = Action::policy_only().into_iter().collect::<BTreeSet<_>>();
    assert!(production.is_disjoint(&policy));
    assert_eq!(
        production.union(&policy).copied().collect::<BTreeSet<_>>(),
        Action::all().into_iter().collect::<BTreeSet<_>>()
    );
}

#[test]
fn canonical_physical_acknowledgment_is_last_in_its_trace() {
    let traces = execute_ordinary_durability_recovery_traces();
    let completed = traces
        .iter()
        .find(|trace| trace.contains(&Action::PhysicalMutationAcknowledged))
        .expect("ordinary execution includes one completed mutation trace");
    assert_eq!(
        completed.last(),
        Some(&Action::PhysicalMutationAcknowledged)
    );
}

#[test]
fn physical_acknowledgment_requires_wal_data_and_namespace_durability() {
    let mut frontier = DurabilityRecoveryFrontier::initial();
    assert_eq!(
        frontier.apply(Action::PhysicalMutationAcknowledged),
        Err(DurabilityRecoveryDenial::IncompletePhysicalDurability)
    );

    for action in [
        Action::WalAppendProposed,
        Action::WalAppendCompletedInMemory,
        Action::WalFenceRequested,
        Action::WalFenceCompleted,
        Action::PageFlushRequested,
        Action::PageFlushCompleted,
        Action::CheckpointBegun,
        Action::CheckpointDurable,
        Action::DirectorySyncCompleted,
        Action::CheckpointPublished,
        Action::PhysicalMutationAcknowledged,
    ] {
        frontier.apply(action).unwrap();
    }
    assert!(frontier.physical_mutation_acknowledged());
    assert_eq!(
        frontier.apply(Action::PhysicalMutationAcknowledged),
        Err(DurabilityRecoveryDenial::PhysicalMutationAlreadyAcknowledged)
    );
}

#[test]
fn page_flush_dispatch_is_denied_before_the_wal_fence() {
    let mut frontier = DurabilityRecoveryFrontier::initial();
    assert_eq!(
        frontier.apply(Action::PageFlushRequested),
        Err(DurabilityRecoveryDenial::PageFlushAheadOfWal)
    );
}

#[test]
fn failed_wal_fence_never_yields_fence_completion_or_physical_acknowledgment() {
    let trace = replay_acknowledgment_ordering_guard(91);
    assert_eq!(trace.last(), Some(&Action::WalFenceRequested));
    assert!(!trace.contains(&Action::WalFenceCompleted));
    assert!(!trace.contains(&Action::PhysicalMutationAcknowledged));
}
