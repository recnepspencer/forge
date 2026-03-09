//! Feature-tier lifecycle event runtime.

use crate::context::facade::ModelingContext;
use crate::engine::transaction::data::feature_event::KernelFeatureEvent;
use crate::engine::transaction::data::subscriber_data_id::KernelSubscriberDataId;
use crate::engine::transaction::logic::subscribers::register_feature_subscribers;
use forge_core::KernelError;
use forge_signal::facade::{CheckpointBarrier, EventBus, EventFlushError, SubscriberRegistryError};

/// Runtime context passed to feature event subscribers.
pub(crate) struct FeatureEventRuntimeContext {
    pub(crate) modeling_context: ModelingContext,
}

impl FeatureEventRuntimeContext {
    pub(crate) fn from_config(config: crate::configuration::facade::KernelConfig) -> Self {
        Self {
            modeling_context: ModelingContext::from_config(config),
        }
    }
}

/// Wrapper around the deterministic event bus used by feature pipeline execution.
pub(crate) struct FeatureEventRuntime {
    event_bus: EventBus<KernelFeatureEvent, KernelSubscriberDataId, FeatureEventRuntimeContext>,
}

impl FeatureEventRuntime {
    pub(crate) fn new() -> Result<Self, KernelError> {
        let mut event_bus: EventBus<
            KernelFeatureEvent,
            KernelSubscriberDataId,
            FeatureEventRuntimeContext,
        > = EventBus::new();
        register_feature_subscribers(&mut event_bus)?;
        Ok(Self { event_bus })
    }

    pub(crate) fn begin(
        &mut self,
        runtime_ctx: &mut FeatureEventRuntimeContext,
    ) -> Result<(), KernelError> {
        self.event_bus
            .begin(runtime_ctx)
            .map_err(registry_error_to_kernel)
    }

    pub(crate) fn emit(&mut self, event: KernelFeatureEvent) {
        self.event_bus.emit(event);
    }

    pub(crate) fn flush(
        &mut self,
        barrier: CheckpointBarrier,
        runtime_ctx: &mut FeatureEventRuntimeContext,
    ) -> Result<(), KernelError> {
        self.event_bus
            .flush(barrier, runtime_ctx)
            .map_err(flush_error_to_kernel)
    }

    pub(crate) fn rollback(&mut self, runtime_ctx: &mut FeatureEventRuntimeContext) {
        self.event_bus.rollback(runtime_ctx);
    }

    pub(crate) fn event_bus(
        &self,
    ) -> &EventBus<KernelFeatureEvent, KernelSubscriberDataId, FeatureEventRuntimeContext> {
        &self.event_bus
    }
}

fn registry_error_to_kernel(err: SubscriberRegistryError<KernelSubscriberDataId>) -> KernelError {
    KernelError::InternalError {
        message: format!("feature subscriber registry error: {err:?}"),
        context: None,
    }
}

fn flush_error_to_kernel(err: EventFlushError<KernelSubscriberDataId>) -> KernelError {
    KernelError::InternalError {
        message: format!("feature event flush failed: {err}"),
        context: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::facade::KernelConfig;
    use forge_signal::facade::SubscriberId;

    #[test]
    fn runtime_registration_is_deterministic() {
        let mut runtime = FeatureEventRuntime::new().expect("runtime should build");
        let mut ctx = FeatureEventRuntimeContext::from_config(KernelConfig::default());
        runtime
            .begin(&mut ctx)
            .expect("begin should finalize registration");
        assert_eq!(
            runtime.event_bus().resolved_order(),
            vec![
                SubscriberId::new(10),
                SubscriberId::new(20),
                SubscriberId::new(30),
            ]
        );
    }

    #[test]
    fn rollback_path_is_safe_to_call_without_events() {
        let mut runtime = FeatureEventRuntime::new().expect("runtime should build");
        let mut ctx = FeatureEventRuntimeContext::from_config(KernelConfig::default());
        runtime.begin(&mut ctx).expect("begin should succeed");
        runtime.rollback(&mut ctx);
    }
}
