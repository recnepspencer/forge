use std::path::PathBuf;

use super::filesystem_source_acquisition_denial::WorthUiFilesystemSourceAcquisitionDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiFilesystemWatcherDenial {
    Acquisition(WorthUiFilesystemSourceAcquisitionDenial),
    NotificationBackendUnavailable(PathBuf),
    NonNativeBackend(PathBuf),
    WatchRegistrationFailed(PathBuf),
    BackendEventFailed(PathBuf),
    EventStreamDisconnected(PathBuf),
    SettlementTimedOut {
        root: PathBuf,
        observed_notification_count: u64,
    },
    SettlementDeadlineUnrepresentable(PathBuf),
    EmptySettlementWindow(PathBuf),
    ShutdownFailed(PathBuf),
    InitialSnapshotAlreadyTaken(PathBuf),
    InitialSettlementTimedOut {
        root: PathBuf,
        observed_notification_count: u64,
    },
}

impl From<WorthUiFilesystemSourceAcquisitionDenial> for WorthUiFilesystemWatcherDenial {
    fn from(denial: WorthUiFilesystemSourceAcquisitionDenial) -> Self {
        Self::Acquisition(denial)
    }
}
