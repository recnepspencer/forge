use super::{
    UiNativeClientShutdownObservation, UiNativeEventLoopApplication, UiNativeEventLoopClient,
    UiNativeEventLoopClientCleanup,
};

pub(super) struct UiNativeEventLoopTerminalEvidence {
    pub client_cleanup: Option<Box<dyn UiNativeEventLoopClientCleanup>>,
    pub client_shutdown: Option<UiNativeClientShutdownObservation>,
    pub peak_census: crate::native::UiNativeResourceCensus,
    pub terminal_census: crate::native::UiNativeResourceCensus,
    pub client_closed: bool,
    pub cleanup_complete: bool,
}

pub(super) fn close<Client: UiNativeEventLoopClient>(
    application: &mut UiNativeEventLoopApplication<Client>,
    host_peak_census: crate::native::UiNativeResourceCensus,
) -> UiNativeEventLoopTerminalEvidence {
    let (client_cleanup, client_shutdown) = application
        .client
        .take()
        .map(|client| client.close().into_parts())
        .unwrap_or((None, None));
    let client_closed = client_cleanup.is_none();
    let peak_census = client_shutdown.as_ref().map_or(host_peak_census, |client| {
        host_peak_census.with_client_peak(client.resources())
    });
    let readiness_owner_count = application.readiness.close();
    drop(application.pointer_input.take());
    let mut shared = application.shared.borrow_mut();
    shared
        .resources
        .release_all(application.loop_resources.drain(..))
        .expect("event-loop owners must remain exact");
    let host_census = shared.close();
    let terminal_census = client_shutdown.as_ref().map_or(host_census, |client| {
        host_census.with_client_terminal(client.resources())
    });
    drop(shared);
    UiNativeEventLoopTerminalEvidence {
        client_cleanup,
        client_shutdown,
        peak_census,
        terminal_census,
        client_closed,
        cleanup_complete: super::terminal_cleanup::terminal_cleanup_complete(
            client_closed,
            readiness_owner_count == 3,
            &terminal_census,
        ),
    }
}
