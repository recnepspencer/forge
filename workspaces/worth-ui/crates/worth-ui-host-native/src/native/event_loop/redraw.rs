use winit::event_loop::ActiveEventLoop;

use super::{
    physical_progression, UiNativeEventLoopApplication, UiNativeEventLoopClient,
    UiNativeEventLoopRunDenial, UiNativeReadinessGrant,
};

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    pub(super) fn redraw(&mut self, event_loop: &ActiveEventLoop) {
        let physical = physical_progression::progress_ready_physical_work(
            &mut self.readiness,
            self.physical_readiness_owner,
            &self.shared,
        );
        if let Some(grant) = physical.application_progress_grant() {
            if self.progress_physical_client(event_loop, grant) {
                return;
            }
        }
        self.request_physical_signal_redraw();
        let Ok(work) = self.readiness.take(self.readiness_owner) else {
            self.notify_native_observations_ready(event_loop);
            return;
        };
        self.redraw_turns += 1;
        let readiness = UiNativeReadinessGrant::issued(
            work.generation,
            work.scale_factor_milli,
            work.client_physical_size,
        );
        let directive = self
            .client
            .as_mut()
            .and_then(|client| client.redraw_ready(readiness).ok());
        self.request_physical_signal_redraw();
        if self.finish_client_progress(event_loop, directive) {
            return;
        }
        self.first_frame_presented = true;
    }

    fn progress_physical_client(
        &mut self,
        event_loop: &ActiveEventLoop,
        grant: super::UiNativePhysicalProgressGrant,
    ) -> bool {
        let directive = self
            .client
            .as_mut()
            .and_then(|client| client.physical_work_progressed(grant).ok());
        self.finish_client_progress(event_loop, directive)
    }

    fn finish_client_progress(
        &mut self,
        event_loop: &ActiveEventLoop,
        directive: Option<super::UiNativeEventLoopDirective>,
    ) -> bool {
        let Some(directive) = directive else {
            self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver);
            return true;
        };
        self.apply_qualified_surface_basis_successor(event_loop)
            || self.apply_client_directive(event_loop, directive)
            || self.notify_native_observations_ready(event_loop)
    }
}
