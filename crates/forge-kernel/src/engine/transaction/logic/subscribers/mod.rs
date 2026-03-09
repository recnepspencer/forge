//! Feature-pipeline lifecycle subscribers.

mod audit;
mod decision_lifecycle;
mod finalization;

use crate::engine::transaction::data::feature_event::KernelFeatureEvent;
use crate::engine::transaction::data::subscriber_data_id::KernelSubscriberDataId;
use crate::engine::transaction::logic::feature_event_runtime::FeatureEventRuntimeContext;
use forge_core::KernelError;
use forge_signal::facade::{
    EventBus, EventSubscriber, SignalError, SubscriberContext, SubscriberContextError,
};

pub(crate) use audit::AuditSubscriber;
pub(crate) use decision_lifecycle::DecisionLifecycleSubscriber;
pub(crate) use finalization::FinalizationSubscriber;

type KernelEventBus =
    EventBus<KernelFeatureEvent, KernelSubscriberDataId, FeatureEventRuntimeContext>;

pub(crate) fn register_feature_subscribers(bus: &mut KernelEventBus) -> Result<(), KernelError> {
    register(bus, DecisionLifecycleSubscriber::new())?;
    register(bus, FinalizationSubscriber::new())?;
    register(bus, AuditSubscriber::new())?;
    Ok(())
}

fn register<S>(bus: &mut KernelEventBus, subscriber: S) -> Result<(), KernelError>
where
    S: EventSubscriber<
            Event = KernelFeatureEvent,
            DataId = KernelSubscriberDataId,
            RuntimeContext = FeatureEventRuntimeContext,
        > + 'static,
{
    bus.subscribe(Box::new(subscriber))
        .map_err(|err| KernelError::InternalError {
            message: format!("failed to register feature event subscriber: {err:?}"),
            context: None,
        })
}

pub(crate) fn stage_or_signal_error(
    stage_result: Result<(), SubscriberContextError<KernelSubscriberDataId>>,
    field: &'static str,
) -> Result<(), SignalError> {
    stage_result.map_err(|err| {
        SignalError::internal(format!(
            "subscriber context staging failed for {field}: {err:?}"
        ))
    })
}

pub(crate) fn stage_output_value<T: 'static>(
    ctx: &mut SubscriberContext<KernelSubscriberDataId>,
    id: KernelSubscriberDataId,
    value: T,
    field: &'static str,
) -> Result<(), SignalError> {
    stage_or_signal_error(ctx.stage(id, value), field)
}

pub(crate) fn kernel_to_signal(err: KernelError) -> SignalError {
    SignalError::internal(err.to_string())
}
