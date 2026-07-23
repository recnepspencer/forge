use super::filesystem_watcher_readiness::WorthUiFilesystemWatcherBackend;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiFilesystemWatcherShutdownReceipt {
    backend: WorthUiFilesystemWatcherBackend,
    observed_notification_count: u64,
}

impl WorthUiFilesystemWatcherShutdownReceipt {
    pub(super) fn new(
        backend: WorthUiFilesystemWatcherBackend,
        observed_notification_count: u64,
    ) -> Self {
        Self {
            backend,
            observed_notification_count,
        }
    }

    pub fn backend(&self) -> WorthUiFilesystemWatcherBackend {
        self.backend
    }

    pub fn observed_notification_count(&self) -> u64 {
        self.observed_notification_count
    }
}
