use crate::facade::*;
use crate::tests::support::*;

#[test]
fn evaluation_telemetry_records_activity() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let mut compute = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(1, 1));

    evaluate(&mut graph, a, &mut compute).unwrap();

    let t = graph.telemetry();
    assert!(t.evaluation_calls >= 1);
    assert!(t.nodes_evaluated >= 1);
    assert!(t.nodes_recomputed >= 1);
    assert!(t.evaluation_stack_peak >= 1);
}

#[test]
fn condition_telemetry_records_deferrals() {
    let mut graph = SignalGraph::new();
    let node = graph.node().on_demand().build();
    let mut compute = |_id: NodeId, _g: &SignalGraph| Ok(version_ab(1, 1));

    evaluate(&mut graph, node, &mut compute).unwrap();

    let t = graph.telemetry();
    assert_eq!(t.condition_skip_count, 1);
    assert_eq!(t.ondemand_deferred_count, 1);
}

#[test]
fn event_bus_telemetry_counts_flush_and_rollback() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum D {
        A,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum E {
        Tick,
    }

    struct Sub;
    impl EventSubscriber for Sub {
        type Event = E;
        type DataId = D;
        type RuntimeContext = ();
        fn id(&self) -> SubscriberId {
            SubscriberId::new(1)
        }
        fn name(&self) -> &'static str {
            "sub"
        }
        fn requires(&self) -> &'static [D] {
            &[]
        }
        fn provides(&self) -> &'static [D] {
            &[D::A]
        }
        fn on_event(&mut self, _event: &E) {}
        fn on_checkpoint(
            &mut self,
            _barrier: CheckpointBarrier,
            _ctx: &mut SubscriberContext<D>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), SignalError> {
            Ok(())
        }
    }

    let mut bus: EventBus<E, D> = EventBus::new();
    bus.subscribe(Box::new(Sub)).unwrap();
    let mut runtime = ();
    bus.begin(&mut runtime).unwrap();
    bus.emit(E::Tick);
    bus.flush(CheckpointBarrier::PerOperation, &mut runtime)
        .unwrap();
    bus.rollback(&mut runtime);

    assert_eq!(bus.telemetry().event_flushes, 1);
    assert_eq!(bus.telemetry().rollback_count, 1);
}

#[test]
fn invalidation_and_gc_telemetry_record_activity() {
    let mut graph = SignalGraph::with_gc_threshold(1);
    let a = graph.node().build();
    let b = graph.node().build();
    graph.add_dependency(b, a, ASPECT_B).unwrap();

    mark_dirty(&mut graph, a, ASPECT_B).unwrap();
    graph.unregister_node(a).unwrap();
    graph.run_gc_epoch();

    let t = graph.telemetry();
    assert!(t.invalidation_nodes_visited >= 1);
    assert_eq!(t.gc_epoch_count, 1);
    assert!(t.gc_epoch_nanos > 0);
}
