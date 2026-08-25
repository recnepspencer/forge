use winit::event_loop::ActiveEventLoop;

use super::{
    UiNativeApplicationReadinessGrant, UiNativeEventLoopApplication, UiNativeEventLoopClient,
    UiNativeEventLoopRunDenial,
};

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    pub(super) fn progress_application_readiness(&mut self, event_loop: &ActiveEventLoop) {
        self.readiness_signals = self.readiness_signals.saturating_add(1);
        for ordinal in 0..self.application_readiness_owners.len() {
            let owner = self.application_readiness_owners[ordinal];
            let Ok(ready) = self.readiness.take_level(owner) else {
                continue;
            };
            let Ok(owner_ordinal) = u8::try_from(ordinal) else {
                return self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
            };
            let directive = self.client.as_mut().and_then(|client| {
                client
                    .application_readiness_ready(UiNativeApplicationReadinessGrant::issued(
                        owner_ordinal,
                        ready.generation(),
                    ))
                    .ok()
            });
            let Some(directive) = directive else {
                return self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
            };
            if self.apply_client_directive(event_loop, directive) {
                return;
            }
        }
    }
}
