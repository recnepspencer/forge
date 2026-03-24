use crate::data::checkpoint::CheckpointBarrier;
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::error::SignalError;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::data::tier::{DependencyMode, DirtyPropagation, EvaluationTrigger, TierPolicy};
use crate::facade::*;
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
        .with_kernel_defaults()
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
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();
    assert_eq!(tx.commit().unwrap().outcome, TransactionOutcome::Committed);
    assert_eq!(runtime.telemetry().transaction.transaction_commit_count, 1);
}

#[test]
fn commit_result_without_evaluation_reports_empty_execution_summary() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let mut runtime = build_runtime(graph);

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();

    let result = tx.commit().unwrap();
    assert_eq!(result.outcome, TransactionOutcome::Committed);
    assert!(result.execution_report.is_none());
    assert_eq!(result.evaluation_summary.nodes_evaluated, 0);
    assert_eq!(result.evaluation_summary.nodes_recomputed, 0);
    assert_eq!(result.evaluation_summary.nodes_suppressed, 0);
    assert_eq!(result.evaluation_summary.plans_built, 0);
    assert_eq!(result.evaluation_summary.stages_executed, 0);
    assert_eq!(result.rollback, None);
    assert!(result.decision_summary.committed);
    assert!(!result.decision_summary.rollback_recorded);
    assert!(result.integrity_markers.event_epochs_attached);
    assert_eq!(result.event_epochs.len(), 1);
    assert!(result.timing.total_nanos >= result.timing.commit_nanos);
}

#[test]
fn commit_result_with_evaluation_carries_execution_summary() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let source = graph
        .node()
        .authority_policy(AuthorityPolicy::AuthoritativeOnly)
        .build();
    let mut runtime = build_runtime(graph);

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        source,
        &|view| Ok(view.finish(version_ab(1, 0))),
        EvaluationRequestMode::Default,
    )
    .unwrap();

    let result = tx.commit().unwrap();
    assert_eq!(result.outcome, TransactionOutcome::Committed);
    assert!(result.execution_report.is_some());
    assert!(result.evaluation_summary.nodes_evaluated >= 1);
    assert!(result.evaluation_summary.nodes_recomputed >= 1);
    assert!(result.evaluation_summary.plans_built >= 1);
    assert!(result.evaluation_summary.stages_executed >= 1);
    assert!(result.timing.evaluation_nanos > 0);
    assert!(result.decision_summary.stage_authority_decisions >= 1);
    assert!(result.integrity_markers.execution_report_attached);
    assert_eq!(result.decision_log.records.len(), 3);
    assert!(matches!(
        result.decision_log.records[0].detail,
        DecisionDetail::TransactionOutcome {
            outcome: TransactionOutcome::Committed
        }
    ));
    assert!(matches!(
        result.decision_log.records[1],
        DecisionRecord {
            stage_index: Some(0),
            detail: DecisionDetail::StageAuthorityPolicy {
                authority_policy: AuthorityPolicy::AuthoritativeOnly
            }
        }
    ));
    assert!(matches!(
        result.decision_log.records[2],
        DecisionRecord {
            stage_index: Some(0),
            detail: DecisionDetail::StageParallelAdmission { .. }
        }
    ));
}

#[test]
fn begin_rollback_preserves_committed_state() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let before = graph.get_state(a).unwrap();
    let mut runtime = build_runtime(graph);

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();
    assert_eq!(
        tx.rollback().unwrap().outcome,
        TransactionOutcome::RolledBack
    );
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
}

#[test]
fn rollback_result_carries_rollback_diagnostic() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let mut runtime = build_runtime(graph);

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();

    let result = tx.rollback().unwrap();
    assert_eq!(result.outcome, TransactionOutcome::RolledBack);
    assert!(result.rollback.is_some());
    assert!(result.execution_report.is_none());
    assert!(result.decision_summary.rollback_recorded);
    assert!(result.decision_summary.rolled_back);
}

#[test]
fn read_only_rollback_emits_zero_rollback_packets() {
    let graph = crate::data::graph::SignalGraph::new();
    let mut runtime = build_runtime(graph);
    let mut ctx = ();

    let result = runtime.begin(&mut ctx).rollback().unwrap();

    assert_eq!(result.outcome, TransactionOutcome::RolledBack);
    assert_eq!(runtime.telemetry().transaction.rollback_packet_breadth, 0);
    assert_eq!(
        runtime.telemetry().transaction.rollback_packet_config_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_diagnostics_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_graph_patch_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_created_node_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_subscriber_repair_count,
        0
    );
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
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();

    let err = tx.commit().unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("event bus flush failed"));
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
    assert!(runtime.telemetry().transaction.transaction_poison_count >= 1);
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
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.record_effect::<TestEffectMap>(&TestEffect::CacheOne);
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();

    let _ = tx.commit().unwrap_err();
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
    let mut tx = runtime.begin(&mut ctx);

    // Invalid handle poisons transaction.
    let invalid = crate::data::handle::NodeId::new(999_999, 0);
    assert!(tx.mark_dirty(invalid, ASPECT_B).is_err());
    let result = tx.commit().unwrap();
    assert_eq!(result.outcome, TransactionOutcome::Poisoned);
    assert!(result.decision_summary.poisoned);
    assert!(result.decision_summary.rollback_recorded);
    assert!(result.decision_summary.failure_recorded);
    assert!(result.integrity_markers.rollback_attached);
    assert!(result.integrity_markers.failure_attached);
    assert!(result
        .warnings
        .iter()
        .any(|warning| warning.code == "rollback"));
    assert!(result
        .warnings
        .iter()
        .any(|warning| warning.code == "failure"));
    assert!(result.decision_log.records.iter().any(|record| {
        matches!(
            record.detail,
            DecisionDetail::Rollback { ref reason }
                if reason == "poisoned transaction rollback"
        )
    }));
    assert!(result.decision_log.records.iter().any(|record| {
        matches!(
            record.detail,
            DecisionDetail::Failure {
                phase: ExecutionFailurePhase::Rollback,
                ..
            }
        )
    }));
}

#[test]
fn poisoned_rollback_rewinds_graph() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let before = graph.get_state(a).unwrap();
    let mut runtime = build_runtime(graph);
    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);

    tx.mark_dirty(a, ASPECT_B).unwrap();
    let invalid = crate::data::handle::NodeId::new(999_999, 0);
    assert!(tx.mark_dirty(invalid, ASPECT_B).is_err());
    assert_eq!(tx.rollback().unwrap().outcome, TransactionOutcome::Poisoned);
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
}

#[test]
fn poisoned_rollback_does_not_increment_explicit_rollback_metric() {
    let graph = crate::data::graph::SignalGraph::new();
    let mut runtime = build_runtime(graph);
    let mut ctx = ();
    let rollback_before = runtime.telemetry().transaction.transaction_rollback_count;
    let poison_before = runtime.telemetry().transaction.transaction_poison_count;
    let mut tx = runtime.begin(&mut ctx);

    let invalid = crate::data::handle::NodeId::new(999_999, 0);
    assert!(tx.mark_dirty(invalid, ASPECT_B).is_err());
    assert_eq!(tx.rollback().unwrap().outcome, TransactionOutcome::Poisoned);

    assert_eq!(
        runtime.telemetry().transaction.transaction_rollback_count,
        rollback_before,
        "poisoned rollback should not inflate explicit rollback telemetry"
    );
    assert_eq!(
        runtime.telemetry().transaction.transaction_poison_count,
        poison_before + 1
    );
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
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();
    let err = tx.commit().unwrap_err();
    assert!(format!("{err}").contains("event bus begin failed"));
    assert_eq!(runtime.graph().get_state(a).unwrap(), before);
}

#[test]
fn commit_failure_reports_rolled_back_patch_count() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let mut runtime = build_runtime(graph);

    runtime
        .event_bus_mut()
        .subscribe(Box::new(FailingSubscriber))
        .unwrap();

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();

    let _ = tx.commit().unwrap_err();
    let rollback = runtime
        .observe()
        .diagnostics()
        .latest_rollback()
        .expect("rollback diagnostics should be recorded");
    assert!(
        rollback.staged_node_patch_count > 0,
        "rollback diagnostics should retain the staged patch count after rewinding"
    );
}

#[test]
fn hostile_rollback_and_commit_cycles_do_not_leak_semantic_events() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let mut runtime = build_runtime(graph);
    let mut ctx = ();

    let baseline_events = runtime.graph().replay_events().len();
    for cycle in 0..12 {
        let before_state = runtime.graph().get_state(a).unwrap();
        let mut tx = runtime.begin(&mut ctx);
        tx.mark_dirty(a, ASPECT_B).unwrap();
        tx.emit_event(Ev::Tick);
        if cycle % 2 == 0 {
            assert_eq!(
                tx.rollback().unwrap().outcome,
                TransactionOutcome::RolledBack
            );
            assert_eq!(runtime.graph().get_state(a).unwrap(), before_state);
        } else {
            tx.flush_events(CheckpointBarrier::PerOperation).unwrap();
            assert_eq!(tx.commit().unwrap().outcome, TransactionOutcome::Committed);
        }
    }

    let replay_events = runtime.graph().replay_events();
    assert!(replay_events.len() > baseline_events);
    let mut last_sequence = None;
    for event in replay_events {
        if let Some(previous) = last_sequence {
            assert!(
                previous < event.cursor,
                "replay sequences must be strictly increasing"
            );
        }
        last_sequence = Some(event.cursor);
    }
}

#[test]
fn hostile_commit_failure_does_not_leak_committed_semantic_outcome() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let a = graph.node().build();
    let mut runtime = build_runtime(graph);
    runtime
        .event_bus_mut()
        .subscribe(Box::new(FailingSubscriber))
        .unwrap();

    let replay_len_before = runtime.graph().replay_events().len();
    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(a, ASPECT_B).unwrap();
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();

    let err = tx.commit().unwrap_err();
    assert!(format!("{err}").contains("event bus flush failed"));
    let replay_events = runtime.graph().replay_events();
    assert!(replay_events.len() >= replay_len_before + 2);
    let last = replay_events
        .back()
        .expect("rollback/failure replay should be recorded");
    assert_ne!(
        last.detail.as_ref().and_then(|detail| detail.as_message()),
        Some("transaction committed"),
        "failed transaction must not leak committed replay outcome"
    );
    assert!(
        replay_events.iter().rev().take(2).any(|event| {
            event.detail.as_ref().and_then(|detail| detail.as_message())
                == Some("event bus flush failed")
        }),
        "failed transaction should surface flush failure in replay events"
    );
}

#[test]
fn transaction_created_keyed_nodes_are_removed_on_rollback() {
    let graph = crate::data::graph::SignalGraph::new();
    let mut runtime = build_runtime(graph);
    let positions = define_keyed_computation(&mut runtime, "positions", Tier::A);
    let arena_before = runtime.graph().arena_capacity();
    let active_before = runtime.graph().active_node_count();
    let mut ctx = ();

    let mut tx = runtime.begin(&mut ctx);
    let created = positions.keyed("wing-root").node_in_transaction(&mut tx);
    assert!(tx.staged_graph().is_alive(created));
    assert_eq!(
        tx.rollback().unwrap().outcome,
        TransactionOutcome::RolledBack
    );

    assert_eq!(runtime.graph().arena_capacity(), arena_before);
    assert_eq!(runtime.graph().active_node_count(), active_before);
    assert!(!runtime.graph().is_alive(created));
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_created_node_count,
        1
    );
}

#[test]
fn repeated_created_node_rollbacks_do_not_accumulate_storage_debris() {
    let graph = crate::data::graph::SignalGraph::new();
    let mut runtime = build_runtime(graph);
    let positions = define_keyed_computation(&mut runtime, "positions", Tier::A);
    let mut ctx = ();

    let (
        (dep_edges_before, dep_segments_before),
        (sub_edges_before, sub_segments_before),
        snapshot_before,
    ) = runtime.graph().storage_counts();

    for _ in 0..12 {
        let mut tx = runtime.begin(&mut ctx);
        let created = positions.keyed("wing-root").node_in_transaction(&mut tx);
        tx.evaluate_with_plan(
            created,
            &|view| Ok(view.finish(version_ab(1, 0))),
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
        assert_eq!(
            tx.rollback().unwrap().outcome,
            TransactionOutcome::RolledBack
        );
    }

    let (
        (dep_edges_after, dep_segments_after),
        (sub_edges_after, sub_segments_after),
        snapshot_after,
    ) = runtime.graph().storage_counts();

    assert_eq!(dep_edges_after, dep_edges_before);
    assert_eq!(dep_segments_after, dep_segments_before);
    assert_eq!(sub_edges_after, sub_edges_before);
    assert_eq!(sub_segments_after, sub_segments_before);
    assert_eq!(snapshot_after, snapshot_before);
}

#[test]
fn mark_dirty_after_evaluate_staging_still_stages_downstream_rollback_coverage() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let source = graph.node().build();
    let downstream = graph.node().build();
    graph
        .append_dependency(downstream, source, ASPECT_A)
        .unwrap();
    let mut runtime = build_runtime(graph);
    let mut ctx = ();

    let mut seed = |_id: crate::data::handle::NodeId, _graph: &crate::data::graph::SignalGraph| {
        Ok(version_ab(1, 0))
    };
    evaluate(runtime.graph_mut(), source, &mut seed).unwrap();
    evaluate(runtime.graph_mut(), downstream, &mut seed).unwrap();

    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        source,
        &|view| Ok(view.finish(version_ab(2, 0))),
        EvaluationRequestMode::Default,
    )
    .unwrap();
    tx.mark_dirty(source, ASPECT_A).unwrap();

    assert_eq!(
        tx.staged_graph().get_state(downstream).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        tx.rollback().unwrap().outcome,
        TransactionOutcome::RolledBack
    );
    assert_eq!(
        runtime.graph().get_state(downstream).unwrap(),
        NodeState::Clean
    );
}

#[test]
fn evaluate_dirty_rollback_restores_preexisting_dirty_nodes() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let source = graph.node().build();
    graph
        .get_entry_mut(source)
        .unwrap()
        .set_state(NodeState::Dirty);
    let before_version = graph.get_entry(source).unwrap().get_aspect_version();
    let mut runtime = build_runtime(graph);
    let mut ctx = ();

    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_dirty(&|view| {
        let current = view
            .graph()
            .get_entry(view.node())?
            .get_aspect_version()
            .get(ASPECT_A);
        Ok(crate::logic::evaluation::EvaluationOutput::from_result(
            NodeEvaluationResult::from_version(version_ab(current + 1, 0)),
        ))
    })
    .unwrap();
    assert_eq!(
        tx.rollback().unwrap().outcome,
        TransactionOutcome::RolledBack
    );

    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version(),
        before_version
    );
    assert_eq!(runtime.graph().get_state(source).unwrap(), NodeState::Dirty);
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
    graph.append_dependency(b, a, ASPECT_B).unwrap();
    graph.append_dependency(c, b, ASPECT_B).unwrap();
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
        runtime.graph().telemetry().evaluation.skipped_by_comparator >= 1,
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

#[test]
fn unregister_clears_tier_metadata_tombstones() {
    let mut runtime = build_runtime(crate::data::graph::SignalGraph::new());
    let node = runtime.graph_mut().node().build();
    runtime.set_node_tier(node, Tier::A);

    assert_eq!(runtime.config().node_meta().occupied_slot_count(), 1);

    runtime.graph_mut().unregister_node(node).unwrap();

    assert_eq!(
        runtime.config().node_meta().occupied_slot_count(),
        0,
        "unregister should clear retained tier metadata for dead slots"
    );
}

#[test]
fn runtime_builder_applies_seeded_tier_policy() {
    let policy = TierPolicy::new(
        Tier::A,
        DependencyMode::AutoDiscovered,
        DirtyPropagation::Immediate,
        EvaluationTrigger::LazyPull,
    );
    let runtime = SignalRuntime::builder(crate::data::graph::SignalGraph::new())
        .with_kernel_defaults()
        .with_tiers::<Tier>()
        .tier_policy(policy.clone())
        .build();

    assert_eq!(runtime.config().tier_policies().get(Tier::A), Some(&policy));
}

#[test]
fn rollback_removes_dynamic_dependency_capture_ghost_subscribers() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let target = graph.node().build();
    graph.append_dependency(target, source_a, ASPECT_A).unwrap();

    let mut runtime = build_runtime(graph);
    let mut ctx = ();

    evaluate(runtime.graph_mut(), source_a, &mut |_id, _graph| {
        Ok(version_ab(1, 0))
    })
    .unwrap();
    evaluate(runtime.graph_mut(), source_b, &mut |_id, _graph| {
        Ok(version_ab(2, 0))
    })
    .unwrap();
    evaluate(runtime.graph_mut(), target, &mut |_id, _graph| {
        Ok(version_ab(10, 0))
    })
    .unwrap();

    assert!(runtime.graph().subscribers_of(source_b).unwrap().is_empty());

    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        target,
        &|view| {
            let _ = view.read_aspect_version(source_a, ASPECT_A)?;
            let _ = view.read_aspect_version(source_b, ASPECT_A)?;
            Ok(view.finish(version_ab(11, 0)))
        },
        crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand,
    )
    .unwrap();
    assert_eq!(
        tx.staged_graph().subscribers_of(source_b).unwrap(),
        &[target],
        "transactional graph should see the newly captured dependency before rollback"
    );

    assert_eq!(
        tx.rollback().unwrap().outcome,
        TransactionOutcome::RolledBack
    );
    assert!(
        runtime.graph().subscribers_of(source_b).unwrap().is_empty(),
        "rollback must clear subscriber edges introduced by abandoned dynamic dependency capture"
    );
    let dependencies = runtime.graph().dependencies_of(target).unwrap();
    assert_eq!(dependencies.len(), 1);
    assert_eq!(dependencies[0].source(), source_a);
}

#[test]
fn rollback_restores_original_source_subscriber_membership_after_rewire() {
    let mut graph = crate::data::graph::SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let target = graph.node().build();
    graph.append_dependency(target, source_a, ASPECT_A).unwrap();

    let mut runtime = build_runtime(graph);
    let mut ctx = ();

    evaluate(runtime.graph_mut(), source_a, &mut |_id, _graph| {
        Ok(version_ab(1, 0))
    })
    .unwrap();
    evaluate(runtime.graph_mut(), source_b, &mut |_id, _graph| {
        Ok(version_ab(2, 0))
    })
    .unwrap();
    evaluate(runtime.graph_mut(), target, &mut |_id, _graph| {
        Ok(version_ab(10, 0))
    })
    .unwrap();

    assert_eq!(runtime.graph().subscribers_of(source_a).unwrap(), &[target]);
    assert!(runtime.graph().subscribers_of(source_b).unwrap().is_empty());

    let mut tx = runtime.begin(&mut ctx);
    tx.evaluate_with_plan(
        target,
        &|view| {
            let _ = view.read_aspect_version(source_b, ASPECT_A)?;
            Ok(view.finish(version_ab(11, 0)))
        },
        EvaluationRequestMode::ForceOnDemand,
    )
    .unwrap();

    assert!(tx
        .staged_graph()
        .subscribers_of(source_a)
        .unwrap()
        .is_empty());
    assert_eq!(
        tx.staged_graph().subscribers_of(source_b).unwrap(),
        &[target]
    );

    assert_eq!(
        tx.rollback().unwrap().outcome,
        TransactionOutcome::RolledBack
    );

    assert_eq!(runtime.graph().subscribers_of(source_a).unwrap(), &[target]);
    assert!(runtime.graph().subscribers_of(source_b).unwrap().is_empty());
    runtime
        .graph()
        .assert_bidirectional_consistency()
        .expect("rollback should restore bidirectional dependency/subscriber topology");
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_graph_patch_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_subscriber_repair_count,
        1
    );
}
