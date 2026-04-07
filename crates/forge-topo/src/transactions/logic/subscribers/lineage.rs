use forge_core::EntityRef;
use forge_core::KernelError;
use forge_signal::facade::SignalError;
use forge_signal::facade::adapters::{EventSubscriber, SubscriberContext, SubscriberId};
use forge_signal::facade::runtime::CheckpointBarrier;

use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::transactions::data::operation_outputs::LineageSummary;

use super::stage_output_value;

#[derive(Debug, Default)]
pub(crate) struct LineageSubscriber {
    summary: LineageSummary,
    destroyed: Vec<EntityRef>,
}

impl LineageSubscriber {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl EventSubscriber for LineageSubscriber {
    type Event = TopoOperationEvent;
    type DataId = TopoSubscriberDataId;
    type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(160)
    }

    fn name(&self) -> &'static str {
        "lineage_operation_subscriber"
    }

    fn requires(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::MutationCounts]
    }

    fn provides(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::LineageEvents]
    }

    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<TopoSubscriberDataId>,
        _runtime: &mut Self::RuntimeContext,
    ) {
        self.summary = LineageSummary::default();
        self.destroyed.clear();
    }

    fn on_event(&mut self, event: &Self::Event) {
        if let TopoOperationEvent::EntityDestroyed(entity) = event {
            self.destroyed.push(*entity);
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

        self.summary = LineageSummary::default();
        self.destroyed.sort_unstable();
        self.destroyed.dedup();

        for entity in &self.destroyed {
            self.summary.deletions_seen += 1;
            if runtime.lineage_store_mut().record_deletion(*entity).is_ok() {
                self.summary.deletions_stamped += 1;
            }
        }

        stage_output_value(
            ctx,
            TopoSubscriberDataId::LineageEvents,
            self.summary.clone(),
            "LineageEvents",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.summary = LineageSummary::default();
        self.destroyed.clear();
    }
}
