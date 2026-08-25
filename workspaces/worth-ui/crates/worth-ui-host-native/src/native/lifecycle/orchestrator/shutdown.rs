use crate::native::lifecycle::protocol_world::resources::UiProtocolResources;
use crate::native::lifecycle::shutdown::orchestrator::{self, UiNativeShutdownPort};
use crate::native::{UiNativeRecoveryRegistry, UiNativeResourceCensus, UiNativeShutdownPhase};

impl super::UiNativeLifecycleOrchestrator {
    pub(super) fn close_protocol_resources(&mut self, resources: &mut UiProtocolResources) {
        let mut port = UiProtocolShutdownPort {
            resources,
            recovery: &mut self.recovery,
        };
        while self.shutdown_phase != UiNativeShutdownPhase::Closed {
            orchestrator::progress(&mut self.shutdown_phase, &mut port);
        }
    }
}

struct UiProtocolShutdownPort<'world> {
    resources: &'world mut UiProtocolResources,
    recovery: &'world mut UiNativeRecoveryRegistry,
}

impl UiNativeShutdownPort for UiProtocolShutdownPort<'_> {
    type Census = UiNativeResourceCensus;

    fn begin_close(&mut self) {}

    fn settle_external_effects(&mut self) -> bool {
        self.resources.settle_external()
    }

    fn release_derived_state(&mut self) {
        self.recovery.clear();
    }

    fn release_native_resources(&mut self) -> bool {
        self.resources.release_all();
        true
    }

    fn census(&self) -> Self::Census {
        self.resources.current()
    }

    fn terminal_zero(&self) -> bool {
        self.resources.current().is_zero() && self.recovery.len() == 0
    }
}
