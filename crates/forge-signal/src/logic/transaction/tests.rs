use crate::data::checkpoint::CheckpointBarrier;
use crate::data::checkpoint_policy::CheckpointPolicy;
use crate::data::comparator::{DefaultComparatorResolver, VersionComparatorPolicy};
use crate::data::error::SignalError;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
use crate::logic::transaction::{SignalRuntimeState, TransactionOutcome};
use crate::tests::support::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    A,
}

struct FailingSubscriber;
impl EventSubscriber for FailingSubscriber {
    type Event = Ev;
    type DataId = Domain;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        SubscriberId::new(99)
    }
    fn name(&self) -> &'static str {
        "failing"
    }
    fn requires(&self) -> &'static [Self::DataId] {
        &[]
    }
    fn provides(&self) -> &'static [Self::DataId] {
        &[]
    }
    fn on_event(&mut self, _event: &Self::Event) {}
    fn on_checkpoint(
        &mut self,
        _barrier: CheckpointBarrier,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        Err(SignalError::internal("injected subscriber failure"))
    }
}

#[test]
fn begin_commit_applies_staged_state_once() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.create_node();
    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));

    let mut ctx = ();
    let mut tx = runtime.begin();
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();
    assert_eq!(tx.commit(&mut ctx).unwrap(), TransactionOutcome::Committed);
    assert_eq!(runtime.telemetry().transaction_commit_count, 1);
}

#[test]
fn begin_rollback_preserves_committed_state() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.create_node();
    let before = graph.get_state(a).unwrap();

    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));

    let mut ctx = ();
    let mut tx = runtime.begin();
    tx.mark_dirty(a, ASPECT_B).unwrap();
    assert_eq!(tx.rollback(&mut ctx).unwrap(), TransactionOutcome::RolledBack);
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
}

#[test]
fn failed_event_flush_does_not_commit_graph_state() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.create_node();
    let before = graph.get_state(a).unwrap();

    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));
    runtime
        .event_bus_mut()
        .subscribe(Box::new(FailingSubscriber))
        .unwrap();

    let mut ctx = ();
    let mut tx = runtime.begin();
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();

    let err = tx.commit(&mut ctx).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("event bus flush failed"));
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
    assert!(runtime.telemetry().transaction_poison_count >= 1);
}

#[test]
fn commit_failure_discards_checkpoint_state() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.create_node();
    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));
    runtime
        .event_bus_mut()
        .subscribe(Box::new(FailingSubscriber))
        .unwrap();

    let mut ctx = ();
    let mut tx = runtime.begin();
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.record_effect::<TestEffectMap>(&TestEffect::CacheOne);
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();

    let _ = tx.commit(&mut ctx).unwrap_err();
    assert!(runtime.checkpoint().dirty().is_empty());
}

#[derive(Debug, Clone, Copy)]
enum TestEffect {
    CacheOne,
}

struct TestEffectMap;
impl crate::data::effect_mapping::EffectMapping for TestEffectMap {
    type Domain = Domain;
    type Impact = Impact;
    type Effect = TestEffect;

    fn route(effect: &Self::Effect, out: &mut crate::data::dirty_set::BatchedDirtySet<Self::Domain, Self::Impact>) {
        match effect {
            TestEffect::CacheOne => out.mark_domain_scoped(Domain::Cache, Impact::One),
        }
    }
}

#[test]
fn poisoned_transaction_returns_poisoned_outcome() {
    let graph = crate::data::graph::SignalGraph::new();
    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));
    let mut ctx = ();
    let mut tx = runtime.begin();

    // Invalid handle poisons transaction.
    let invalid = crate::data::handle::NodeId::new(999_999, 0);
    assert!(tx.mark_dirty(invalid, ASPECT_B).is_err());
    assert_eq!(tx.commit(&mut ctx).unwrap(), TransactionOutcome::Poisoned);
}

#[test]
fn poisoned_rollback_rewinds_graph() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.create_node();
    let before = graph.get_state(a).unwrap();
    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));
    let mut ctx = ();
    let mut tx = runtime.begin();

    tx.mark_dirty(a, ASPECT_B).unwrap();
    let invalid = crate::data::handle::NodeId::new(999_999, 0);
    assert!(tx.mark_dirty(invalid, ASPECT_B).is_err());
    assert_eq!(tx.rollback(&mut ctx).unwrap(), TransactionOutcome::Poisoned);
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
}

#[test]
fn failure_during_event_begin_rewinds_graph() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.create_node();
    let before = graph.get_state(a).unwrap();
    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));

    runtime
        .event_bus_mut()
        .subscribe(Box::new(NeedsMissingProviderSubscriber))
        .unwrap();

    let mut ctx = ();
    let mut tx = runtime.begin();
    tx.mark_dirty(a, ASPECT_B).unwrap();
    let err = tx.commit(&mut ctx).unwrap_err();
    assert!(format!("{err}").contains("event bus begin failed"));
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
}

struct NeedsMissingProviderSubscriber;
impl EventSubscriber for NeedsMissingProviderSubscriber {
    type Event = Ev;
    type DataId = Domain;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        SubscriberId::new(100)
    }
    fn name(&self) -> &'static str {
        "missing-provider"
    }
    fn requires(&self) -> &'static [Self::DataId] {
        &[Domain::Cache]
    }
    fn provides(&self) -> &'static [Self::DataId] {
        &[]
    }
    fn on_event(&mut self, _event: &Self::Event) {}
    fn on_checkpoint(
        &mut self,
        _barrier: CheckpointBarrier,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        Ok(())
    }
}

#[test]
fn tier_comparator_inheritance_uses_tier_default() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.create_node();
    let b = graph.create_node();
    let c = graph.create_node();
    graph.add_dependency(b, a, ASPECT_B).unwrap();
    graph.add_dependency(c, b, ASPECT_B).unwrap();

    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));
    runtime.set_node_tier(c, Tier::A);
    runtime.set_tier_policy(
        TierPolicy::new(
            Tier::A,
            DependencyMode::AutoDiscovered,
            DirtyPropagation::Immediate,
            EvaluationTrigger::LazyPull,
        )
        .with_default_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 }),
    );

    let mut ctx = ();
    let mut tx = runtime.begin();

    let mut compute_a_10 = |_id: crate::data::handle::NodeId, _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 10))
    };
    let mut compute_a_12 = |_id: crate::data::handle::NodeId, _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 12))
    };
    let mut compute_b = |_id: crate::data::handle::NodeId, _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 100))
    };
    let mut compute_c = |_id: crate::data::handle::NodeId, _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 1_000))
    };

    tx.evaluate(a, &mut compute_a_10, DefaultComparatorResolver).unwrap();
    tx.evaluate(b, &mut compute_b, DefaultComparatorResolver).unwrap();
    tx.evaluate(c, &mut compute_c, DefaultComparatorResolver).unwrap();
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.evaluate(a, &mut compute_a_12, DefaultComparatorResolver).unwrap();
    tx.evaluate(b, &mut compute_b, DefaultComparatorResolver).unwrap();
    tx.evaluate(c, &mut compute_c, DefaultComparatorResolver).unwrap();

    assert!(
        tx.staged_graph().telemetry().skipped_by_comparator >= 1,
        "tier tolerance comparator should skip small delta"
    );
    assert_eq!(tx.commit(&mut ctx).unwrap(), TransactionOutcome::Committed);
}

#[test]
fn node_tier_metadata_is_generation_safe_on_slot_reuse() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let first = graph.create_node();
    graph.unregister_node(first).unwrap();
    let reused = graph.create_node();
    let mut runtime: SignalRuntimeState<Domain, Impact, Ev, (), Tier> =
        SignalRuntimeState::with_policy(graph, CheckpointPolicy::new(CheckpointBarrier::PerOperation));
    runtime.set_node_tier(first, Tier::A);

    assert!(
        runtime.config().node_meta().tier_for_node(reused).is_none(),
        "reused slot must not inherit stale tier metadata from prior generation"
    );
}
