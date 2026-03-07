use std::marker::PhantomData;

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::data::telemetry::RuntimeTelemetry;
use std::time::Instant;

use super::errors::{EventFlushError, SubscriberRegistryError};
use super::ordering::resolve_order;

pub(super) struct SubscriberEntry<E, D, C>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    pub(super) id: SubscriberId,
    pub(super) name: &'static str,
    pub(super) requires: &'static [D],
    pub(super) provides: &'static [D],
    pub(super) sub: Box<dyn EventSubscriber<Event = E, DataId = D, RuntimeContext = C>>,
}

/// Typed deterministic event bus.
///
/// Events are buffered in-order and delivered at checkpoint flush.
pub struct EventBus<E, D, C = ()>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    pending: Vec<E>,
    subscribers: Vec<SubscriberEntry<E, D, C>>,
    order: Vec<usize>,
    context: SubscriberContext<D>,
    telemetry: RuntimeTelemetry,
    finalized: bool,
    context_marker: PhantomData<fn(&mut C)>,
}

impl<E, D, C> Default for EventBus<E, D, C>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<E, D, C> EventBus<E, D, C>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    /// Create an empty bus.
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            subscribers: Vec::new(),
            order: Vec::new(),
            context: SubscriberContext::new(),
            telemetry: RuntimeTelemetry::default(),
            finalized: false,
            context_marker: PhantomData,
        }
    }

    /// Access committed subscriber context.
    pub fn context(&self) -> &SubscriberContext<D> {
        &self.context
    }

    /// Access committed subscriber context mutably.
    pub fn context_mut(&mut self) -> &mut SubscriberContext<D> {
        &mut self.context
    }

    /// Runtime telemetry snapshot.
    pub fn telemetry(&self) -> &RuntimeTelemetry {
        &self.telemetry
    }

    /// Reset runtime telemetry counters.
    pub fn reset_telemetry(&mut self) {
        self.telemetry = RuntimeTelemetry::default();
    }

    /// Register one subscriber (must call `finalize_registration` before use).
    pub fn subscribe(
        &mut self,
        sub: Box<dyn EventSubscriber<Event = E, DataId = D, RuntimeContext = C>>,
    ) -> Result<(), SubscriberRegistryError<D>> {
        let id = sub.id();
        let name = sub.name();
        let requires = sub.requires();
        let provides = sub.provides();

        if let Some(existing) = self.subscribers.iter().find(|e| e.id == id) {
            return Err(SubscriberRegistryError::DuplicateSubscriberId {
                id,
                first: existing.name,
                second: name,
            });
        }
        self.subscribers.push(SubscriberEntry {
            id,
            name,
            requires,
            provides,
            sub,
        });
        self.finalized = false;
        Ok(())
    }

    /// Validate dependency DAG and compute deterministic order.
    pub fn finalize_registration(&mut self) -> Result<(), SubscriberRegistryError<D>> {
        self.order = resolve_order(&self.subscribers)?;
        self.finalized = true;
        Ok(())
    }

    fn ensure_finalized(&mut self) -> Result<(), SubscriberRegistryError<D>> {
        if self.finalized {
            return Ok(());
        }
        self.finalize_registration()
    }

    /// Signal start of an operation lifecycle.
    pub fn begin(&mut self, runtime: &mut C) -> Result<(), SubscriberRegistryError<D>> {
        self.ensure_finalized()?;
        self.context.clear_staged();
        for &idx in &self.order {
            self.subscribers[idx]
                .sub
                .on_begin(&mut self.context, runtime);
        }
        Ok(())
    }

    /// Record one event for later checkpoint delivery.
    pub fn emit(&mut self, event: E) {
        self.pending.push(event);
    }

    /// Flush pending events and run checkpoint lifecycle in deterministic order.
    pub fn flush(
        &mut self,
        barrier: CheckpointBarrier,
        runtime: &mut C,
    ) -> Result<(), EventFlushError<D>> {
        let flush_start = Instant::now();
        self.ensure_finalized().map_err(EventFlushError::Registry)?;

        for &idx in &self.order {
            let entry = &mut self.subscribers[idx];
            for event in &self.pending {
                entry.sub.on_event(event);
            }
            if let Err(source) = entry.sub.on_checkpoint(barrier, &mut self.context, runtime) {
                self.context.clear_staged();
                return Err(EventFlushError::Subscriber {
                    subscriber_id: entry.id,
                    subscriber_name: entry.name,
                    source,
                });
            }
        }

        self.pending.clear();
        self.context.finalize();
        self.telemetry.event_flushes += 1;
        self.telemetry.event_flush_nanos += flush_start.elapsed().as_nanos();
        Ok(())
    }

    /// Roll back lifecycle state in reverse deterministic order.
    pub fn rollback(&mut self, runtime: &mut C) {
        self.pending.clear();
        self.context.clear_staged();
        for &idx in self.order.iter().rev() {
            self.subscribers[idx].sub.on_rollback(runtime);
        }
        self.telemetry.rollback_count += 1;
    }

    /// Deterministic resolved order for diagnostics/tests.
    pub fn resolved_order(&self) -> Vec<SubscriberId> {
        self.order
            .iter()
            .map(|&idx| self.subscribers[idx].id)
            .collect()
    }
}
