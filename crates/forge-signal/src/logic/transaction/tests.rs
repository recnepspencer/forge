use crate::data::checkpoint::CheckpointBarrier;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::error::SignalError;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
use crate::facade::mark_dirty;
use crate::logic::transaction::{SignalRuntime, TransactionOutcome};
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

fn build_runtime(
    graph: crate::data::graph::SignalGraph,
) -> SignalRuntime<Domain, Impact, Ev, (), Tier> {
    SignalRuntime::builder(graph)
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .build()
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
    let a = graph.node().build();
    let mut runtime = build_runtime(graph);

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
    let a = graph.node().build();
    let before = graph.get_state(a).unwrap();
    let mut runtime = build_runtime(graph);

    let mut ctx = ();
    let mut tx = runtime.begin();
    tx.mark_dirty(a, ASPECT_B).unwrap();
    assert_eq!(
        tx.rollback(&mut ctx).unwrap(),
        TransactionOutcome::RolledBack
    );
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
}

#[test]
fn failed_event_flush_does_not_commit_graph_state() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let before = graph.get_state(a).unwrap();
    let mut runtime = build_runtime(graph);
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
    let a = graph.node().build();
    let mut runtime = build_runtime(graph);
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

    fn route(
        effect: &Self::Effect,
        out: &mut crate::data::dirty_set::BatchedDirtySet<Self::Domain, Self::Impact>,
    ) {
        match effect {
            TestEffect::CacheOne => out.mark_domain_scoped(Domain::Cache, Impact::One),
        }
    }
}

#[test]
fn poisoned_transaction_returns_poisoned_outcome() {
    let graph = crate::data::graph::SignalGraph::new();
    let mut runtime = build_runtime(graph);
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
    let a = graph.node().build();
    let before = graph.get_state(a).unwrap();
    let mut runtime = build_runtime(graph);
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
    let a = graph.node().build();
    let before = graph.get_state(a).unwrap();
    let mut runtime = build_runtime(graph);

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
    let a = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();
    graph.add_dependency(b, a, ASPECT_B).unwrap();
    graph.add_dependency(c, b, ASPECT_B).unwrap();
    let mut runtime = build_runtime(graph);
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
    let mut compute_a_10 = |_id: crate::data::handle::NodeId,
                        _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 10))
    };
    let mut compute_a_12 = |_id: crate::data::handle::NodeId,
                        _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 12))
    };
    let mut compute_b = |_id: crate::data::handle::NodeId, _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 100))
    };
    let mut compute_c = |_id: crate::data::handle::NodeId, _g: &crate::data::graph::SignalGraph| {
        Ok(version_ab(0, 1_000))
    };

    evaluate(runtime.graph_mut(), a, &mut compute_a_10).unwrap();
    evaluate(runtime.graph_mut(), b, &mut compute_b).unwrap();
    evaluate(runtime.graph_mut(), c, &mut compute_c).unwrap();
    mark_dirty(runtime.graph_mut(), a, ASPECT_B).unwrap();
    evaluate(runtime.graph_mut(), a, &mut compute_a_12).unwrap();
    evaluate(runtime.graph_mut(), b, &mut compute_b).unwrap();
    evaluate(runtime.graph_mut(), c, &mut compute_c).unwrap();

    assert!(
        runtime.graph().telemetry().skipped_by_comparator >= 1,
        "tier tolerance comparator should skip small delta"
    );
}

#[test]
fn node_tier_metadata_is_generation_safe_on_slot_reuse() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let first = graph.node().build();
    graph.unregister_node(first).unwrap();
    let reused = graph.node().build();
    let mut runtime = build_runtime(graph);
    runtime.set_node_tier(first, Tier::A);

    assert!(
        runtime.config().node_meta().tier_for_node(reused).is_none(),
        "reused slot must not inherit stale tier metadata from prior generation"
    );
}
