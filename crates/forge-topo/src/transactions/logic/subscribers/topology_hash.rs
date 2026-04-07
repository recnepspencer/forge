use forge_core::KernelError;
use forge_signal::facade::SignalError;
use forge_signal::facade::adapters::{EventSubscriber, SubscriberContext, SubscriberId};
use forge_signal::facade::runtime::CheckpointBarrier;

use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};

use super::stage_output_value;

#[derive(Debug, Default)]
pub(crate) struct TopologyHashSubscriber {
    op_seen: bool,
}

impl TopologyHashSubscriber {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.op_seen = false;
    }
}

impl EventSubscriber for TopologyHashSubscriber {
    type Event = TopoOperationEvent;
    type DataId = TopoSubscriberDataId;
    type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(120)
    }

    fn name(&self) -> &'static str {
        "topology_hash_operation_subscriber"
    }

    fn requires(&self) -> &'static [TopoSubscriberDataId] {
        &[]
    }

    fn provides(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::TopologyHash]
    }

    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
        _runtime: &mut Self::RuntimeContext,
    ) {
        self.clear();
    }

    fn on_event(&mut self, event: &Self::Event) {
        if let TopoOperationEvent::OperationStarted { .. } = event {
            self.op_seen = true;
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

        let computed = if self.op_seen && runtime.config().per_op_hashing {
            let post_hash = runtime.compute_topology_hash();
            runtime.set_topology_hash(post_hash);
            true
        } else {
            false
        };
        stage_output_value(
            ctx,
            TopoSubscriberDataId::TopologyHash,
            computed,
            "TopologyHash",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.clear();
    }
}
