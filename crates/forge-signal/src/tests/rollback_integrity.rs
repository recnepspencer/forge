use std::sync::{Arc, Mutex};

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::facade::*;
use crate::tests::support::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Audit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[derive(Clone)]
struct RecordingSubscriber {
    id: u32,
    name: &'static str,
    log: Arc<Mutex<Vec<&'static str>>>,
    fail_on_checkpoint: bool,
    requires_audit: bool,
    provides_audit: bool,
}

impl EventSubscriber for RecordingSubscriber {
    type Event = Ev;
    type DataId = Domain;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        SubscriberId::new(self.id)
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn requires(&self) -> &'static [Self::DataId] {
        if self.requires_audit {
            &[Domain::Audit]
        } else {
            &[]
        }
    }

    fn provides(&self) -> &'static [Self::DataId] {
        if self.provides_audit {
            &[Domain::Audit]
        } else {
            &[]
        }
    }

    fn on_event(&mut self, _event: &Self::Event) {}

    fn on_checkpoint(
        &mut self,
        _barrier: CheckpointBarrier,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        self.log.lock().unwrap().push(self.name);
        if self.fail_on_checkpoint {
            Err(SignalError::internal("checkpoint failure"))
        } else {
            Ok(())
        }
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.log.lock().unwrap().push(match self.name {
            "first" => "first-rollback",
            "second" => "second-rollback",
            other => other,
        });
    }
}

#[test]
fn failed_event_flush_triggers_compensating_rollbacks_for_flushed_subscribers() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph)
        .with_domains::<Domain>()
        .with_events::<Ev>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .build();
    let node = runtime.graph_mut().node().build();
    let log = Arc::new(Mutex::new(Vec::new()));
    runtime
        .event_bus_mut()
        .subscribe(Box::new(RecordingSubscriber {
            id: 1,
            name: "first",
            log: log.clone(),
            fail_on_checkpoint: false,
            requires_audit: false,
            provides_audit: true,
        }))
        .unwrap();
    runtime
        .event_bus_mut()
        .subscribe(Box::new(RecordingSubscriber {
            id: 2,
            name: "second",
            log: log.clone(),
            fail_on_checkpoint: true,
            requires_audit: true,
            provides_audit: false,
        }))
        .unwrap();

    let before = runtime.graph().get_state(node).unwrap();
    let mut ctx = ();
    let mut tx = runtime.begin();
    tx.mark_dirty(node, ASPECT_A).unwrap();
    tx.emit_event(Ev::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();

    let err = tx.commit(&mut ctx).unwrap_err();
    assert!(format!("{err}").contains("event bus flush failed"));
    assert_eq!(runtime.graph().get_state(node).unwrap(), before);

    let log = log.lock().unwrap().clone();
    assert!(log.starts_with(&["first", "second"]));
    assert!(log.contains(&"first-rollback"));
    assert!(log.contains(&"second-rollback"));
}
