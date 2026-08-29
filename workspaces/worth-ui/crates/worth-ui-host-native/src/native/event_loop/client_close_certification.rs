use super::{
    UiNativeClientShutdownObservation, UiNativeEventLoopClient,
    UiNativeEventLoopShutdownOverlapObservation,
};

pub struct UiNativeQueuedReadinessCloseCertification {
    client_cleanup_complete: bool,
    readiness_closure_complete: bool,
    client_shutdown: Option<UiNativeClientShutdownObservation>,
    overlap: UiNativeEventLoopShutdownOverlapObservation,
}

impl UiNativeQueuedReadinessCloseCertification {
    pub const fn client_cleanup_complete(&self) -> bool {
        self.client_cleanup_complete
    }

    pub const fn readiness_closure_complete(&self) -> bool {
        self.readiness_closure_complete
    }

    pub const fn client_shutdown(&self) -> Option<&UiNativeClientShutdownObservation> {
        self.client_shutdown.as_ref()
    }

    pub const fn overlap(&self) -> UiNativeEventLoopShutdownOverlapObservation {
        self.overlap
    }
}

#[doc(hidden)]
pub fn certify_client_close_with_queued_readiness<Client: UiNativeEventLoopClient>(
    client: Client,
) -> UiNativeQueuedReadinessCloseCertification {
    let readiness = crate::native::UiNativeReadinessRegistry::new();
    let owner = readiness
        .register()
        .expect("certification readiness owner must fit the production registry");
    readiness
        .commit_latest(owner, 1_000, [160, 96])
        .expect("certification readiness work must be admitted");
    readiness
        .signal(owner)
        .expect("certification readiness signal must be queued");

    let evidence = super::client_close::close(Some(client), &readiness);
    let closure = readiness.close_exact(&[owner]);
    UiNativeQueuedReadinessCloseCertification {
        client_cleanup_complete: evidence.cleanup.is_none(),
        readiness_closure_complete: closure.is_complete(),
        client_shutdown: evidence.shutdown,
        overlap: evidence.overlap,
    }
}
