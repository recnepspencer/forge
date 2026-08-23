use winit::event_loop::ActiveEventLoop;

use super::{UiNativeEventLoopApplication, UiNativeEventLoopRunDenial};

impl<Client> UiNativeEventLoopApplication<Client> {
    pub(super) fn fail(
        &mut self,
        event_loop: &ActiveEventLoop,
        denial: UiNativeEventLoopRunDenial,
    ) {
        self.failure = Some(denial);
        event_loop.exit();
    }
}
