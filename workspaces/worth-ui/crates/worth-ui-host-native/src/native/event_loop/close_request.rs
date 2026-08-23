use winit::event_loop::ActiveEventLoop;

use super::{UiNativeEventLoopApplication, UiNativeEventLoopClient, UiNativeEventLoopRunDenial};

impl<Client: UiNativeEventLoopClient> UiNativeEventLoopApplication<Client> {
    pub(super) fn handle_close_requested(&mut self, event_loop: &ActiveEventLoop) {
        let directive = self
            .client
            .as_mut()
            .and_then(|client| client.external_close_requested().ok());
        match directive {
            Some(directive) => {
                self.apply_client_directive(event_loop, directive);
            }
            None => self.fail(event_loop, UiNativeEventLoopRunDenial::ApplicationDriver),
        }
    }
}
