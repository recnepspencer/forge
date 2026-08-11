use crate::data::checkpoint::CheckpointBarrier;
use crate::data::error::SignalError;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;

use super::super::runtime::EventBus;
use super::event_test_types::{Data, Ev, RecSub};

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
