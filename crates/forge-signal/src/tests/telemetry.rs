use crate::facade::*;

#[test]
fn evaluation_telemetry_records_activity() {
    let mut graph = SignalGraph::new();
    let a = graph.create_node();
    let mut compute = |_id: NodeId, _g: &SignalGraph| Ok(AspectVersion::new(1, 1));

    evaluate(&mut graph, a, &mut compute).unwrap();

    let t = graph.telemetry();
    assert!(t.evaluation_calls >= 1);
    assert!(t.nodes_evaluated >= 1);
    assert!(t.nodes_recomputed >= 1);
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
