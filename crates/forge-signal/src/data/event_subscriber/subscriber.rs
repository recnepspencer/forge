use crate::data::checkpoint::CheckpointBarrier;
use crate::data::error::SignalError;
use crate::data::subscriber_context::SubscriberContext;

use super::SubscriberId;

/// Domain-free subscriber contract for event bus orchestration.
///
/// Contract:
/// - `on_begin` is infallible
/// - all fallible work happens in `on_checkpoint`
/// - `requires`/`provides` define DAG dependencies
pub trait EventSubscriber {
    /// Event payload type routed through this bus.
    type Event;
    /// Typed data keys exchanged through `SubscriberContext`.
    type DataId: Copy + Ord + std::fmt::Debug + 'static;
    /// Runtime context passed to lifecycle hooks.
    ///
    /// `()` for pure subscribers, or a domain draft context for side-effecting
    /// subscribers that must mutate transaction state.
    type RuntimeContext;

    /// Stable deterministic identity for DAG tie-breaking.
    fn id(&self) -> SubscriberId;

    /// Human-readable static subscriber name for diagnostics.
    fn name(&self) -> &'static str;

    /// Data IDs this subscriber reads from prior subscribers.
    fn requires(&self) -> &'static [Self::DataId];

    /// Data IDs this subscriber writes for later subscribers.
    fn provides(&self) -> &'static [Self::DataId];

    /// Called once at the start of operation scope. Infallible by contract.
    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) {
    }

    /// Called once per pending event during flush.
    fn on_event(&mut self, event: &Self::Event);

    /// Called once at checkpoint barrier flush.
    fn on_checkpoint(
        &mut self,
        barrier: CheckpointBarrier,
        ctx: &mut SubscriberContext<Self::DataId>,
        runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError>;

    /// Called on rollback/drop path in reverse deterministic order.
    fn on_rollback(&mut self, _runtime: &mut Self::RuntimeContext) {}
}
