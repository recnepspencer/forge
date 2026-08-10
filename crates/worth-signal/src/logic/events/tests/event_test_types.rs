use crate::data::checkpoint::CheckpointBarrier;
use crate::data::error::SignalError;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Data {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Ev {
    Tick(u32),
    Alarm(u32),
}

pub(super) struct RecSub {
    pub(super) id: SubscriberId,
    pub(super) name: &'static str,
    pub(super) req: &'static [Data],
    pub(super) prov: &'static [Data],
    pub(super) out: std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>,
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
