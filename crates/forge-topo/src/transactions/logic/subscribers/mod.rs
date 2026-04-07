//! Operation subscribers for deterministic transaction lifecycle orchestration.

mod euler;
mod invariant;
mod journal;
mod lineage;
mod metrics;
mod replay;
mod topology_hash;
mod version;

use forge_core::KernelError;
use forge_signal::facade::SignalError;
use forge_signal::facade::adapters::{
    EventSubscriber, SubscriberContext, SubscriberContextError,
};
use forge_signal::facade::specialist::EventBus;

pub(crate) use euler::EulerSubscriber;
pub(crate) use invariant::InvariantSubscriber;
pub(crate) use journal::JournalSubscriber;
pub(crate) use lineage::LineageSubscriber;
pub(crate) use metrics::MetricsSubscriber;
pub(crate) use replay::ReplaySubscriber;
pub(crate) use topology_hash::TopologyHashSubscriber;
pub(crate) use version::VersionSubscriber;

use crate::transactions::data::operation_event::{TopoOperationEvent, TopoSubscriberDataId};

pub(crate) fn register_operation_subscribers(
    bus: &mut EventBus<
        TopoOperationEvent,
        TopoSubscriberDataId,
        crate::transactions::logic::mutable_draft::MutableDraft,
    >,
) -> Result<(), SignalError> {
    register(bus, JournalSubscriber::new())?;
    register(bus, VersionSubscriber::new())?;
    register(bus, TopologyHashSubscriber::new())?;
    register(bus, ReplaySubscriber::new())?;
    register(bus, EulerSubscriber::new())?;
    register(bus, InvariantSubscriber::new())?;
    register(bus, LineageSubscriber::new())?;
    register(bus, MetricsSubscriber::new())?;
    Ok(())
}

fn register<S>(
    bus: &mut EventBus<
        TopoOperationEvent,
        TopoSubscriberDataId,
        crate::transactions::logic::mutable_draft::MutableDraft,
    >,
    subscriber: S,
) -> Result<(), SignalError>
where
    S: EventSubscriber<
            Event = TopoOperationEvent,
            DataId = TopoSubscriberDataId,
            RuntimeContext = crate::transactions::logic::mutable_draft::MutableDraft,
        > + 'static,
{
    bus.subscribe(Box::new(subscriber)).map_err(|err| {
        SignalError::internal(format!(
            "failed to register topo operation subscriber: {err:?}"
        ))
    })
}

pub(crate) fn stage_or_signal_error(
    stage_result: Result<(), SubscriberContextError<TopoSubscriberDataId>>,
    field: &'static str,
) -> Result<(), SignalError> {
    stage_result.map_err(|err| {
        SignalError::internal(format!(
            "subscriber context staging failed for {field}: {err:?}"
        ))
    })
}

pub(crate) fn stage_output_value<T: 'static>(
    ctx: &mut SubscriberContext<TopoSubscriberDataId>,
    id: TopoSubscriberDataId,
    value: T,
    field: &'static str,
) -> Result<(), SignalError> {
    stage_or_signal_error(ctx.stage(id, value), field)
}

pub(crate) fn kernel_to_signal(err: KernelError) -> SignalError {
    SignalError::internal(err.to_string())
}
