use super::{
    UiNativeClientShutdownObservation, UiNativeEventLoopClient, UiNativeEventLoopClientCleanup,
    UiNativeEventLoopShutdownOverlapObservation,
};

pub(super) struct UiNativeClientCloseEvidence {
    pub(super) cleanup: Option<Box<dyn UiNativeEventLoopClientCleanup>>,
    pub(super) shutdown: Option<UiNativeClientShutdownObservation>,
    pub(super) overlap: UiNativeEventLoopShutdownOverlapObservation,
}

pub(super) fn close<Client: UiNativeEventLoopClient>(
    client: Option<Client>,
    readiness: &crate::native::UiNativeReadinessRegistry,
) -> UiNativeClientCloseEvidence {
    let queued_readiness_before_client_close = readiness.pending_signal_count();
    let (cleanup, shutdown) = client
        .map(|client| client.close().into_parts())
        .unwrap_or((None, None));
    let overlap = UiNativeEventLoopShutdownOverlapObservation::observed(
        queued_readiness_before_client_close,
        shutdown.as_ref(),
    );
    UiNativeClientCloseEvidence {
        cleanup,
        shutdown,
        overlap,
    }
}
