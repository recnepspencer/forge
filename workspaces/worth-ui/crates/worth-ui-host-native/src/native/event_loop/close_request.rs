use winit::event_loop::ActiveEventLoop;

use super::{
    apply_directive, UiNativeEventLoopApplication, UiNativeEventLoopClient,
    UiNativeEventLoopRunDenial,
};

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    pub(super) fn handle_close_requested(&mut self, event_loop: &ActiveEventLoop) {
        let directive = self
            .client
            .as_mut()
            .and_then(|client| client.external_close_requested().ok());
        match directive {
            Some(directive) if apply_directive(&self.shared, event_loop, directive) => {
                event_loop.exit()
            }
            Some(_) => self.request_physical_signal_redraw(),
            None => self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver),
        }
    }
}
