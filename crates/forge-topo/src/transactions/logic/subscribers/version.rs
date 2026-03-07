use forge_core::KernelError;
use forge_signal::facade::{CheckpointBarrier, EventSubscriber, SubscriberContext, SignalError, SubscriberId};

use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::transactions::data::operation_outputs::VersionCounters;

use super::stage_output_value;

#[derive(Debug, Default)]
pub(crate) struct VersionSubscriber {
    counters: VersionCounters,
}

impl VersionSubscriber {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.counters = VersionCounters::default();
    }
}

impl EventSubscriber for VersionSubscriber {
    type Event = TopoOperationEvent;
    type DataId = TopoSubscriberDataId;
    type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(110)
    }

    fn name(&self) -> &'static str {
        "version_operation_subscriber"
    }

    fn requires(&self) -> &'static [TopoSubscriberDataId] {
        &[]
    }

    fn provides(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::VersionCounters]
    }

    fn on_begin(&mut self, _ctx: &mut SubscriberContext<TopoSubscriberDataId>, _runtime: &mut Self::RuntimeContext) {
        self.clear();
    }

    fn on_event(&mut self, event: &Self::Event) {
        match event {
            TopoOperationEvent::TopologyChanged => {
                self.counters.topology_bumps += 1;
            }
            TopoOperationEvent::GeometryChanged => {
                self.counters.geometry_bumps += 1;
            }
            _ => {}
        }
    }

    fn on_checkpoint(
        &mut self,
        barrier: CheckpointBarrier,
        ctx: &mut SubscriberContext<TopoSubscriberDataId>,
        runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        if barrier != CheckpointBarrier::PerOperation {
            return Ok(());
        }

        for _ in 0..self.counters.topology_bumps {
            runtime.bump_topology_version();
        }
        for _ in 0..self.counters.geometry_bumps {
            runtime.bump_geometry_version();
        }
        stage_output_value(
            ctx,
            TopoSubscriberDataId::VersionCounters,
            self.counters.clone(),
            "VersionCounters",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.clear();
    }
}
