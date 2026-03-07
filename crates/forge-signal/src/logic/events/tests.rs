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
        let Ev::Tick(_v) = event;
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
