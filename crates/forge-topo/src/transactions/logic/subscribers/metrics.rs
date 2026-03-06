use forge_core::KernelError;
use forge_signal::facade::{CheckpointBarrier, EventSubscriber, SubscriberContext, SubscriberId};

use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};
use crate::transactions::data::operation_outputs::OperationArtifacts;

use super::stage_output_value;

#[derive(Debug, Default)]
pub(crate) struct MetricsSubscriber {
    latest: Option<OperationArtifacts>,
}

impl MetricsSubscriber {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl EventSubscriber for MetricsSubscriber {
    type Event = TopoOperationEvent;
    type DataId = TopoSubscriberDataId;
    type RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(170)
    }

    fn name(&self) -> &'static str {
        "metrics_operation_subscriber"
    }

    fn requires(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::LineageEvents]
    }

    fn provides(&self) -> &'static [TopoSubscriberDataId] {
        &[TopoSubscriberDataId::OperationMetrics]
    }

    fn on_begin(&mut self, _ctx: &mut SubscriberContext<TopoSubscriberDataId>, _runtime: &mut Self::RuntimeContext) {
        self.latest = None;
    }

    fn on_event(&mut self, event: &Self::Event) {
        if let TopoOperationEvent::OperationArtifactsBuilt {
            created,
            destroyed,
            lineage_delta,
        } = event
        {
            self.latest = Some(OperationArtifacts {
                entities_created: created.total(),
                entities_deleted: destroyed.total(),
                lineage_delta: lineage_delta.clone(),
            });
        }
    }

    fn on_checkpoint(
        &mut self,
        barrier: CheckpointBarrier,
        ctx: &mut SubscriberContext<TopoSubscriberDataId>,
            _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), KernelError> {
        if barrier != CheckpointBarrier::PerOperation {
            return Ok(());
        }

        let value = self.latest.clone().unwrap_or(OperationArtifacts {
            entities_created: 0,
            entities_deleted: 0,
            lineage_delta: Default::default(),
        });
        stage_output_value(
            ctx,
            TopoSubscriberDataId::OperationMetrics,
            value,
            "OperationMetrics",
        )
    }

    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {
        self.latest = None;
    }
}
