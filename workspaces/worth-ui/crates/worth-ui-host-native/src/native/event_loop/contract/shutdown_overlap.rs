#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeEventLoopShutdownOverlapObservation {
    queued_readiness_before_client_close: usize,
    held_application_attempts_during_client_close: usize,
}

impl UiNativeEventLoopShutdownOverlapObservation {
    pub(in crate::native::event_loop) fn observed(
        queued_readiness_before_client_close: usize,
        client_shutdown: Option<&super::UiNativeClientShutdownObservation>,
    ) -> Self {
        Self {
            queued_readiness_before_client_close,
            held_application_attempts_during_client_close: client_shutdown
                .map_or(0, |shutdown| shutdown.shutdown_attempts().len()),
        }
    }

    pub const fn queued_readiness_before_client_close(self) -> usize {
        self.queued_readiness_before_client_close
    }

    pub const fn held_application_attempts_during_client_close(self) -> usize {
        self.held_application_attempts_during_client_close
    }

    pub const fn crossed_queued_readiness_with_held_attempt(self) -> bool {
        self.queued_readiness_before_client_close > 0
            && self.held_application_attempts_during_client_close > 0
    }
}
