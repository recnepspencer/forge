use crate::data::checkpoint::CheckpointBarrier;
use crate::data::error::SignalError;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;

use super::errors::SubscriberRegistryError;
use super::runtime::EventBus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Data {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick(u32),
    Alarm(u32),
}

struct RecSub {
    id: SubscriberId,
    name: &'static str,
    req: &'static [Data],
    prov: &'static [Data],
    out: std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>,
}

impl EventSubscriber for RecSub {
    type Event = Ev;
    type DataId = Data;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        self.id
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn requires(&self) -> &'static [Data] {
        self.req
    }
    fn provides(&self) -> &'static [Data] {
        self.prov
    }
    fn on_event(&mut self, event: &Ev) {
        match event {
            Ev::Tick(_) | Ev::Alarm(_) => {}
        }
    }
    fn on_checkpoint(
        &mut self,
        _barrier: CheckpointBarrier,
        _ctx: &mut SubscriberContext<Data>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        self.out.lock().unwrap().push(self.name);
        Ok(())
    }
    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.out.lock().unwrap().push(self.name);
    }
}

#[test]
fn deterministic_order_independent_of_registration() {
    let out1 = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let out2 = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mk = |id, name, req, prov, out| RecSub {
        id: SubscriberId::new(id),
        name,
        req,
        prov,
        out,
    };

    let mut b1: EventBus<Ev, Data> = EventBus::new();
    b1.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C], out1.clone())))
        .unwrap();
    b1.subscribe(Box::new(mk(10, "a", &[], &[Data::A], out1.clone())))
        .unwrap();
    b1.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B], out1.clone())))
        .unwrap();
    b1.finalize_registration().unwrap();

    let mut b2: EventBus<Ev, Data> = EventBus::new();
    b2.subscribe(Box::new(mk(10, "a", &[], &[Data::A], out2.clone())))
        .unwrap();
    b2.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B], out2.clone())))
        .unwrap();
    b2.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C], out2.clone())))
        .unwrap();
    b2.finalize_registration().unwrap();

    assert_eq!(b1.resolved_order(), b2.resolved_order());
    assert_eq!(
        b1.resolved_order(),
        vec![
            SubscriberId::new(10),
            SubscriberId::new(20),
            SubscriberId::new(30)
        ]
    );
}

#[test]
fn cycle_error_contains_chain() {
    let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mk = |id, name, req, prov| RecSub {
        id: SubscriberId::new(id),
        name,
        req,
        prov,
        out: out.clone(),
    };

    let mut bus: EventBus<Ev, Data> = EventBus::new();
    bus.subscribe(Box::new(mk(10, "a", &[Data::C], &[Data::A])))
        .unwrap();
    bus.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B])))
        .unwrap();
    bus.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C])))
        .unwrap();

    let err = bus.finalize_registration().unwrap_err();
    match err {
        SubscriberRegistryError::CycleDetected { cycle_chain } => {
            assert!(!cycle_chain.is_empty());
            assert!(cycle_chain.contains(&"a"));
            assert!(cycle_chain.contains(&"b"));
            assert!(cycle_chain.contains(&"c"));
        }
        _ => panic!("expected cycle error"),
    }
}

#[test]
fn duplicate_provider_and_missing_provider_errors() {
    let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mk = |id, name, req, prov| RecSub {
        id: SubscriberId::new(id),
        name,
        req,
        prov,
        out: out.clone(),
    };

    let mut dup: EventBus<Ev, Data> = EventBus::new();
    dup.subscribe(Box::new(mk(10, "a", &[], &[Data::A])))
        .unwrap();
    dup.subscribe(Box::new(mk(20, "b", &[], &[Data::A])))
        .unwrap();
    let err = dup.finalize_registration().unwrap_err();
    assert!(matches!(
        err,
        SubscriberRegistryError::DuplicateProvider { .. }
    ));

    let mut miss: EventBus<Ev, Data> = EventBus::new();
    miss.subscribe(Box::new(mk(10, "a", &[Data::C], &[Data::A])))
        .unwrap();
    let err = miss.finalize_registration().unwrap_err();
    assert!(matches!(
        err,
        SubscriberRegistryError::MissingProvider { .. }
    ));
}

#[test]
fn rollback_runs_reverse_order() {
    let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mk = |id, name, req, prov| RecSub {
        id: SubscriberId::new(id),
        name,
        req,
        prov,
        out: out.clone(),
    };

    let mut bus: EventBus<Ev, Data> = EventBus::new();
    bus.subscribe(Box::new(mk(10, "a", &[], &[Data::A])))
        .unwrap();
    bus.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B])))
        .unwrap();
    bus.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C])))
        .unwrap();
    bus.finalize_registration().unwrap();
    let mut runtime = ();
    bus.begin(&mut runtime).unwrap();
    bus.emit(Ev::Tick(1));
    bus.flush(CheckpointBarrier::PerOperation, &mut runtime)
        .unwrap();
    out.lock().unwrap().clear();
    bus.rollback(&mut runtime);
    assert_eq!(&*out.lock().unwrap(), &["c", "b", "a"]);
}

#[test]
fn flush_auto_finalizes_and_delivers_events() {
    let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mk = |id, name, req, prov| RecSub {
        id: SubscriberId::new(id),
        name,
        req,
        prov,
        out: out.clone(),
    };

    let mut bus: EventBus<Ev, Data> = EventBus::new();
    bus.subscribe(Box::new(mk(10, "a", &[], &[Data::A])))
        .unwrap();
    bus.emit(Ev::Tick(1));
    let mut runtime = ();
    bus.flush(CheckpointBarrier::PerOperation, &mut runtime)
        .unwrap();

    assert_eq!(&*out.lock().unwrap(), &["a"]);
    assert_eq!(bus.resolved_order(), vec![SubscriberId::new(10)]);
}

#[test]
fn rollback_unwinds_begin_only_lifecycle_before_any_flush() {
    let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mk = |id, name, req, prov| RecSub {
        id: SubscriberId::new(id),
        name,
        req,
        prov,
        out: out.clone(),
    };

    let mut bus: EventBus<Ev, Data> = EventBus::new();
    bus.subscribe(Box::new(mk(10, "a", &[], &[Data::A])))
        .unwrap();
    bus.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B])))
        .unwrap();
    bus.finalize_registration().unwrap();

    let mut runtime = ();
    bus.begin(&mut runtime).unwrap();
    bus.rollback(&mut runtime);

    assert_eq!(&*out.lock().unwrap(), &["b", "a"]);
}

#[test]
fn routed_subscribers_only_receive_matching_events() {
    let event_hits = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    struct RoutedSub {
        id: SubscriberId,
        key: &'static str,
        hits: std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>,
    }
    impl EventSubscriber for RoutedSub {
        type Event = Ev;
        type DataId = Data;
        type RuntimeContext = ();
        fn id(&self) -> SubscriberId {
            self.id
        }
        fn name(&self) -> &'static str {
            self.key
        }
        fn requires(&self) -> &'static [Data] {
            &[]
        }
        fn provides(&self) -> &'static [Data] {
            &[]
        }
        fn on_event(&mut self, _event: &Ev) {
            self.hits.lock().unwrap().push(self.key);
        }
        fn on_checkpoint(
            &mut self,
            _barrier: CheckpointBarrier,
            _ctx: &mut SubscriberContext<Data>,
            _runtime: &mut (),
        ) -> Result<(), SignalError> {
            Ok(())
        }
    }

    let mut bus: EventBus<Ev, Data> = EventBus::new();
    bus.set_event_router(|event| match event {
        Ev::Tick(_) => 1,
        Ev::Alarm(_) => 2,
    });
    bus.subscribe_routed(
        &[1],
        Box::new(RoutedSub {
            id: SubscriberId::new(10),
            key: "tick",
            hits: event_hits.clone(),
        }),
    )
    .unwrap();
    bus.subscribe_routed(
        &[2],
        Box::new(RoutedSub {
            id: SubscriberId::new(20),
            key: "alarm",
            hits: event_hits.clone(),
        }),
    )
    .unwrap();
    bus.subscribe(Box::new(RoutedSub {
        id: SubscriberId::new(30),
        key: "all",
        hits: event_hits.clone(),
    }))
    .unwrap();

    bus.emit(Ev::Tick(1));
    bus.emit(Ev::Alarm(7));
    let mut runtime = ();
    bus.flush(CheckpointBarrier::PerOperation, &mut runtime)
        .unwrap();

    let hits = event_hits.lock().unwrap().clone();
    assert_eq!(hits.iter().filter(|hit| **hit == "tick").count(), 1);
    assert_eq!(hits.iter().filter(|hit| **hit == "alarm").count(), 1);
    assert_eq!(hits.iter().filter(|hit| **hit == "all").count(), 2);
}

struct FailCheckpointSub {
    id: SubscriberId,
    name: &'static str,
    req: &'static [Data],
    prov: &'static [Data],
    events: std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>,
    fail_on_checkpoint: bool,
}

impl EventSubscriber for FailCheckpointSub {
    type Event = Ev;
    type DataId = Data;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        self.id
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn requires(&self) -> &'static [Data] {
        self.req
    }

    fn provides(&self) -> &'static [Data] {
        self.prov
    }

    fn on_event(&mut self, _event: &Ev) {}

    fn on_checkpoint(
        &mut self,
        _barrier: CheckpointBarrier,
        _ctx: &mut SubscriberContext<Data>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        self.events.lock().unwrap().push(self.name);
        if self.fail_on_checkpoint {
            Err(SignalError::internal("checkpoint failure"))
        } else {
            Ok(())
        }
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.events.lock().unwrap().push(match self.name {
            "a" => "rollback-a",
            "b" => "rollback-b",
            "c" => "rollback-c",
            other => other,
        });
    }
}

#[test]
fn rollback_only_unwinds_successfully_checkpointed_subscribers_after_partial_flush_failure() {
    let events = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mk = |id, name, req, prov, fail_on_checkpoint| FailCheckpointSub {
        id: SubscriberId::new(id),
        name,
        req,
        prov,
        events: events.clone(),
        fail_on_checkpoint,
    };

    let mut bus: EventBus<Ev, Data> = EventBus::new();
    bus.subscribe(Box::new(mk(10, "a", &[], &[Data::A], false)))
        .unwrap();
    bus.subscribe(Box::new(mk(20, "b", &[Data::A], &[Data::B], true)))
        .unwrap();
    bus.subscribe(Box::new(mk(30, "c", &[Data::B], &[Data::C], false)))
        .unwrap();

    let mut runtime = ();
    bus.begin(&mut runtime).unwrap();
    bus.emit(Ev::Tick(1));
    let err = bus
        .flush(CheckpointBarrier::PerOperation, &mut runtime)
        .unwrap_err();
    assert!(format!("{err:?}").contains("checkpoint failure"));

    bus.rollback(&mut runtime);

    assert_eq!(
        &*events.lock().unwrap(),
        &["a", "b", "rollback-c", "rollback-b", "rollback-a"],
        "rollback should unwind every subscriber that entered the lifecycle, not only those that checkpointed successfully"
    );
}

#[test]
fn failed_flush_preserves_committed_context_and_does_not_replay_stale_pending_events() {
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    struct ContextWriter {
        id: SubscriberId,
        seen_events: Arc<Mutex<Vec<u32>>>,
        staged_value: Option<u32>,
    }

    impl EventSubscriber for ContextWriter {
        type Event = Ev;
        type DataId = Data;
        type RuntimeContext = ();

        fn id(&self) -> SubscriberId {
            self.id
        }

        fn name(&self) -> &'static str {
            "writer"
        }

        fn requires(&self) -> &'static [Data] {
            &[]
        }

        fn provides(&self) -> &'static [Data] {
            &[Data::A]
        }

        fn on_begin(
            &mut self,
            _ctx: &mut SubscriberContext<Self::DataId>,
            _runtime: &mut Self::RuntimeContext,
        ) {
            self.staged_value = None;
        }

        fn on_event(&mut self, event: &Self::Event) {
            if let Ev::Tick(value) = event {
                self.seen_events.lock().unwrap().push(*value);
                self.staged_value = Some(*value);
            }
        }

        fn on_checkpoint(
            &mut self,
            _barrier: CheckpointBarrier,
            ctx: &mut SubscriberContext<Self::DataId>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), SignalError> {
            ctx.stage(Data::A, self.staged_value.unwrap_or_default())
                .map_err(|err| SignalError::internal(format!("{err:?}")))
        }
    }

    #[derive(Clone)]
    struct FailGate {
        should_fail: Arc<Mutex<bool>>,
    }

    impl EventSubscriber for FailGate {
        type Event = Ev;
        type DataId = Data;
        type RuntimeContext = ();

        fn id(&self) -> SubscriberId {
            SubscriberId::new(2)
        }

        fn name(&self) -> &'static str {
            "gate"
        }

        fn requires(&self) -> &'static [Data] {
            &[Data::A]
        }

        fn provides(&self) -> &'static [Data] {
            &[Data::B]
        }

        fn on_event(&mut self, _event: &Self::Event) {}

        fn on_checkpoint(
            &mut self,
            _barrier: CheckpointBarrier,
            _ctx: &mut SubscriberContext<Self::DataId>,
            _runtime: &mut Self::RuntimeContext,
        ) -> Result<(), SignalError> {
            if *self.should_fail.lock().unwrap() {
                Err(SignalError::internal("checkpoint failure"))
            } else {
                Ok(())
            }
        }
    }

    let seen_events = Arc::new(Mutex::new(Vec::new()));
    let should_fail = Arc::new(Mutex::new(false));
    let mut bus: EventBus<Ev, Data> = EventBus::new();
    bus.subscribe(Box::new(ContextWriter {
        id: SubscriberId::new(1),
        seen_events: seen_events.clone(),
        staged_value: None,
    }))
    .unwrap();
    bus.subscribe(Box::new(FailGate {
        should_fail: should_fail.clone(),
    }))
    .unwrap();

    let mut runtime = ();

    bus.begin(&mut runtime).unwrap();
    bus.emit(Ev::Tick(1));
    bus.flush(CheckpointBarrier::PerOperation, &mut runtime)
        .unwrap();
    assert_eq!(bus.context().committed::<u32>(Data::A), Some(&1));

    *should_fail.lock().unwrap() = true;
    bus.begin(&mut runtime).unwrap();
    bus.emit(Ev::Tick(7));
    let err = bus
        .flush(CheckpointBarrier::PerOperation, &mut runtime)
        .unwrap_err();
    assert!(format!("{err:?}").contains("checkpoint failure"));
    assert_eq!(
        bus.context().committed::<u32>(Data::A),
        Some(&1),
        "failed flush must not overwrite committed subscriber context",
    );
    assert!(
        bus.context().staged::<u32>(Data::A).is_none(),
        "failed flush must clear staged subscriber context",
    );
    bus.rollback(&mut runtime);

    *should_fail.lock().unwrap() = false;
    bus.begin(&mut runtime).unwrap();
    bus.emit(Ev::Tick(11));
    bus.flush(CheckpointBarrier::PerOperation, &mut runtime)
        .unwrap();

    assert_eq!(bus.context().committed::<u32>(Data::A), Some(&11));
    assert_eq!(
        &*seen_events.lock().unwrap(),
        &[1, 7, 11],
        "pending events from the failed flush must not replay on the next successful flush",
    );
}
