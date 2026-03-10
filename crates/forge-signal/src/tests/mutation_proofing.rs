use crate::data::aspect::AspectMask;
use crate::data::checkpoint::CheckpointBarrier;
use crate::data::output::{OutputChange, PartitionSubscription};
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::facade::*;
use crate::tests::support::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Audit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

struct FailingSubscriber;

impl EventSubscriber for FailingSubscriber {
    type Event = Ev;
    type DataId = Domain;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        SubscriberId::new(77)
    }

    fn name(&self) -> &'static str {
        "tripwire-failing"
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
        Err(SignalError::internal("tripwire subscriber failure"))
    }
}

#[test]
fn tripwire_failed_commit_cannot_leak_key_registry_growth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_domains::<Domain>()
        .with_events::<Ev>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .build();
    runtime
        .event_bus_mut()
        .subscribe(Box::new(FailingSubscriber))
        .unwrap();
    let before = runtime.config().test_registry_counts();
    let mut ctx = ();

    let err = runtime
        .transaction(&mut ctx, |tx| {
            let family = tx.register_computation_family("tripwire-family");
            let keyed = tx.keyed_node(&family, "tripwire-key");
            let computation =
                KeyedComputation::new(family.clone(), "tripwire-key").with_memo_key("tripwire");
            tx.evaluate_keyed(keyed, &computation, &|_node, view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            tx.emit_event(Ev::Tick);
            tx.flush_events(CheckpointBarrier::PerOperation)?;
            Ok(())
        })
        .unwrap_err();

    assert!(format!("{err}").contains("event bus flush failed"));
    assert_eq!(runtime.config().test_registry_counts(), before);
}

#[test]
fn tripwire_partition_scoped_dirty_nodes_cannot_fast_validate_clean() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .add_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    {
        let entry = graph.get_entry_mut(dependent).unwrap();
        entry.set_state(NodeState::MaybeStale);
        entry.set_dirty_aspects(AspectMask::from_aspect(ASPECT_A));
        entry.add_dirty_partition_scope(
            ASPECT_A,
            PartitionSubscription::partition_and_detail("wing", "rib-12"),
        );
        entry.set_trace_summary(Some(crate::data::trace::TraceSummary {
            output_change: OutputChange::Unchanged,
            ..crate::data::trace::TraceSummary::default()
        }));
    }

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let report = graph
        .execute_prepared_plan(&plan, &|_node, view| {
            Ok(view.finish(NodeEvaluationResult::from_version(version_ab(2, 0))))
        })
        .unwrap();

    assert_eq!(report.tasks_validated_clean, 0);
    assert_eq!(report.tasks_executed, 1);
}

#[test]
fn tripwire_slot_reuse_cannot_resurrect_subscribers() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let dependent = graph.create_node();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    graph.unregister_node(source).unwrap();
    let replacement = graph.create_node();
    graph.rebuild_subscriber_index_from_dependencies().unwrap();

    assert_eq!(replacement.index(), source.index());
    assert!(graph.subscribers_of(replacement).unwrap().is_empty());
}

#[cfg(feature = "parallel")]
#[test]
fn tripwire_parallel_runtime_usage_must_not_increment_serial_counter() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).build();
    let nodes = (0..16)
        .map(|_| runtime.graph_mut().node().build())
        .collect::<Vec<_>>();
    let mut ctx = ();

    let mut tx = runtime.begin();
    for &node in &nodes {
        tx.mark_dirty(node, ASPECT_A).unwrap();
    }
    let report = tx
        .evaluate_dirty_with_executor(
            &|_node, view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
            StageExecutor::aggressive_parallel(),
        )
        .unwrap();
    tx.commit(&mut ctx).unwrap();

    assert!(report
        .stages
        .iter()
        .any(|stage| matches!(stage.outcome, StageExecutionOutcome::CompletedParallel)));
    assert_eq!(runtime.metrics().serial_executor_usage_count, 0);
    assert_eq!(runtime.metrics().parallel_executor_usage_count, 1);
}
