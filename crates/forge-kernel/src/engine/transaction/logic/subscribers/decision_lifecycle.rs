//! Subscriber that owns modeling-context reset and drain lifecycle.

use crate::engine::transaction::data::feature_event::KernelFeatureEvent;
use crate::engine::transaction::data::operation_outputs::DecisionDrainOutput;
use crate::engine::transaction::data::subscriber_data_id::KernelSubscriberDataId;
use crate::engine::transaction::logic::feature_event_runtime::FeatureEventRuntimeContext;
use forge_signal::facade::adapters::{EventSubscriber, SubscriberContext, SubscriberId};
use forge_signal::facade::runtime::CheckpointBarrier;
use forge_signal::facade::SignalError;

use super::stage_output_value;

pub(crate) struct DecisionLifecycleSubscriber;

impl DecisionLifecycleSubscriber {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl EventSubscriber for DecisionLifecycleSubscriber {
    type Event = KernelFeatureEvent;
    type DataId = KernelSubscriberDataId;
    type RuntimeContext = FeatureEventRuntimeContext;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(10)
    }

    fn name(&self) -> &'static str {
        "DecisionLifecycleSubscriber"
    }

    fn requires(&self) -> &'static [Self::DataId] {
        &[]
    }

    fn provides(&self) -> &'static [Self::DataId] {
        &[KernelSubscriberDataId::DecisionDrain]
    }

    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<Self::DataId>,
        runtime: &mut Self::RuntimeContext,
    ) {
        runtime.modeling_context.reset_for_new_operation();
    }

    fn on_event(&mut self, _event: &Self::Event) {}

    fn on_checkpoint(
        &mut self,
        barrier: CheckpointBarrier,
        ctx: &mut SubscriberContext<Self::DataId>,
        runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        if barrier != CheckpointBarrier::PerOperation {
            return Ok(());
        }
        let output = DecisionDrainOutput {
            decision_log: runtime.modeling_context.take_decision_log(),
            trace_adjuncts: runtime.modeling_context.take_trace_adjuncts(),
            sub_metadata: runtime.modeling_context.take_sub_metadata(),
        };
        stage_output_value(
            ctx,
            KernelSubscriberDataId::DecisionDrain,
            output,
            "decision_drain",
        )
    }
}
