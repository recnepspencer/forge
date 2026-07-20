use std::sync::mpsc::RecvTimeoutError;
use std::time::{Duration, Instant};

use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind};

use crate::runtime::source_ingress::{
    WorthUiReloadDebounce, WorthUiSettledSourceSnapshot, WorthUiWatcherEvent,
};

use super::filesystem_notification_queue::{
    filesystem_notification_queue, WorthUiFilesystemNotificationQueue,
};
use super::filesystem_settlement_wait::settlement_wait;
use super::filesystem_source_acquisition_denial::WorthUiFilesystemSourceAcquisitionDenial;
use super::filesystem_source_provider::WorthUiFilesystemSourceProvider;
use super::filesystem_source_reader::{canonical_source_root, freeze_filesystem_source};
use super::filesystem_watcher_denial::WorthUiFilesystemWatcherDenial;
use super::filesystem_watcher_event_translation::translate_filesystem_event;
use super::filesystem_watcher_readiness::{
    WorthUiFilesystemWatcherBackend, WorthUiFilesystemWatcherReadiness,
};
use super::filesystem_watcher_shutdown::WorthUiFilesystemWatcherShutdownReceipt;

const MAX_SETTLEMENT_EVENTS: usize = 4096;
#[cfg(not(test))]
const INITIAL_SETTLEMENT_TIMEOUT: Duration = Duration::from_secs(5);
#[cfg(test)]
const INITIAL_SETTLEMENT_TIMEOUT: Duration = Duration::from_millis(250);

pub struct WorthUiFilesystemSourceWatcher {
    provider: WorthUiFilesystemSourceProvider,
    watcher: RecommendedWatcher,
    notifications: WorthUiFilesystemNotificationQueue,
    debounce: WorthUiReloadDebounce,
    readiness: WorthUiFilesystemWatcherReadiness,
    initial_snapshot: Option<WorthUiSettledSourceSnapshot>,
    last_package_digest: u64,
    next_sequence: u64,
    pending_resnapshot: bool,
}

impl WorthUiFilesystemSourceWatcher {
    pub fn start(
        provider: WorthUiFilesystemSourceProvider,
    ) -> Result<Self, WorthUiFilesystemWatcherDenial> {
        Self::start_with_debounce(provider, WorthUiReloadDebounce::default())
    }

    pub fn start_with_debounce(
        provider: WorthUiFilesystemSourceProvider,
        debounce: WorthUiReloadDebounce,
    ) -> Result<Self, WorthUiFilesystemWatcherDenial> {
        let root = canonical_source_root(provider.root())?;
        if debounce.settlement_window().is_zero() {
            return Err(WorthUiFilesystemWatcherDenial::EmptySettlementWindow(root));
        }
        let provider = WorthUiFilesystemSourceProvider::new(&root);
        let (notifications, handler) = filesystem_notification_queue();
        let mut watcher = recommended_watcher(handler).map_err(|_| {
            WorthUiFilesystemWatcherDenial::NotificationBackendUnavailable(root.clone())
        })?;
        let backend = native_backend(<RecommendedWatcher as Watcher>::kind())
            .ok_or_else(|| WorthUiFilesystemWatcherDenial::NonNativeBackend(root.clone()))?;
        watcher
            .watch(provider.root(), RecursiveMode::Recursive)
            .map_err(|_| WorthUiFilesystemWatcherDenial::WatchRegistrationFailed(root.clone()))?;
        let initial_snapshot = freeze_initial_filesystem_source(
            &provider,
            &notifications,
            &debounce,
            INITIAL_SETTLEMENT_TIMEOUT,
        )?;
        let last_package_digest = initial_snapshot.source_revision().final_package_digest();
        Ok(Self {
            provider,
            watcher,
            notifications,
            debounce,
            readiness: WorthUiFilesystemWatcherReadiness::new(root, backend),
            initial_snapshot: Some(initial_snapshot),
            last_package_digest,
            next_sequence: 2,
            pending_resnapshot: false,
        })
    }

    pub fn readiness(&self) -> &WorthUiFilesystemWatcherReadiness {
        &self.readiness
    }

    pub fn take_initial_snapshot(
        &mut self,
    ) -> Result<WorthUiSettledSourceSnapshot, WorthUiFilesystemWatcherDenial> {
        self.initial_snapshot.take().ok_or_else(|| {
            WorthUiFilesystemWatcherDenial::InitialSnapshotAlreadyTaken(
                self.provider.root().to_path_buf(),
            )
        })
    }

    pub fn settle(
        &mut self,
        timeout: Duration,
    ) -> Result<WorthUiSettledSourceSnapshot, WorthUiFilesystemWatcherDenial> {
        let deadline = Instant::now().checked_add(timeout).ok_or_else(|| {
            WorthUiFilesystemWatcherDenial::SettlementDeadlineUnrepresentable(
                self.provider.root().to_path_buf(),
            )
        })?;
        let provider_id = self.provider.root().to_string_lossy().into_owned();
        let mut events = Vec::new();
        if std::mem::take(&mut self.pending_resnapshot) {
            events.push(WorthUiWatcherEvent::provider_revision(&provider_id));
        }
        loop {
            if self.notifications.take_backend_failure() {
                return Err(WorthUiFilesystemWatcherDenial::BackendEventFailed(
                    self.provider.root().to_path_buf(),
                ));
            }
            if self.notifications.take_resnapshot_signal() {
                append_bounded_events(
                    &mut events,
                    vec![WorthUiWatcherEvent::provider_revision(&provider_id)],
                    &provider_id,
                );
            }
            let observed_notification_count = self.notifications.observed_notification_count();
            let remaining_duration =
                remaining(deadline, self.provider.root(), observed_notification_count)?;
            let wait = settlement_wait(
                !events.is_empty(),
                self.debounce.settlement_window(),
                remaining_duration,
            );
            match self.notifications.recv_timeout(wait.duration) {
                Ok(Ok(event)) => {
                    let translated = translate_filesystem_event(event, &provider_id);
                    append_bounded_events(&mut events, translated, &provider_id);
                }
                Ok(Err(_)) => {
                    return Err(WorthUiFilesystemWatcherDenial::BackendEventFailed(
                        self.provider.root().to_path_buf(),
                    ));
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(WorthUiFilesystemWatcherDenial::EventStreamDisconnected(
                        self.provider.root().to_path_buf(),
                    ));
                }
                Err(RecvTimeoutError::Timeout)
                    if !events.is_empty() && !wait.permits_snapshot_freeze =>
                {
                    self.pending_resnapshot = true;
                    return Err(WorthUiFilesystemWatcherDenial::SettlementTimedOut {
                        root: self.provider.root().to_path_buf(),
                        observed_notification_count: self
                            .notifications
                            .observed_notification_count(),
                    });
                }
                Err(RecvTimeoutError::Timeout) if !events.is_empty() => {
                    let sequence = self.next_sequence;
                    let snapshot = match freeze_filesystem_source(
                        &self.provider,
                        &self.debounce,
                        &events,
                        sequence,
                    ) {
                        Ok(snapshot) => snapshot,
                        Err(denial) if transient_acquisition_denial(&denial) => {
                            events.clear();
                            events.push(WorthUiWatcherEvent::provider_revision(&provider_id));
                            continue;
                        }
                        Err(denial) => return Err(denial.into()),
                    };
                    if let Err(denial) = remaining(
                        deadline,
                        self.provider.root(),
                        self.notifications.observed_notification_count(),
                    ) {
                        self.pending_resnapshot = true;
                        return Err(denial);
                    }
                    let package_digest = snapshot.source_revision().final_package_digest();
                    if package_digest == self.last_package_digest {
                        events.clear();
                        continue;
                    }
                    self.last_package_digest = package_digest;
                    self.next_sequence += 1;
                    return Ok(snapshot);
                }
                Err(RecvTimeoutError::Timeout) => {
                    return Err(WorthUiFilesystemWatcherDenial::SettlementTimedOut {
                        root: self.provider.root().to_path_buf(),
                        observed_notification_count: self
                            .notifications
                            .observed_notification_count(),
                    });
                }
            }
        }
    }

    pub fn shutdown(
        mut self,
    ) -> Result<WorthUiFilesystemWatcherShutdownReceipt, WorthUiFilesystemWatcherDenial> {
        self.watcher.unwatch(self.provider.root()).map_err(|_| {
            WorthUiFilesystemWatcherDenial::ShutdownFailed(self.provider.root().to_path_buf())
        })?;
        Ok(WorthUiFilesystemWatcherShutdownReceipt::new(
            self.readiness.backend(),
            self.notifications.observed_notification_count(),
        ))
    }
}

fn await_initial_quiet_window(
    notifications: &WorthUiFilesystemNotificationQueue,
    debounce: &WorthUiReloadDebounce,
    root: &std::path::Path,
    deadline: Instant,
) -> Result<u64, WorthUiFilesystemWatcherDenial> {
    loop {
        if notifications.take_backend_failure() {
            return Err(WorthUiFilesystemWatcherDenial::BackendEventFailed(
                root.to_path_buf(),
            ));
        }
        if notifications.take_resnapshot_signal() {
            continue;
        }
        let remaining = deadline
            .checked_duration_since(Instant::now())
            .ok_or_else(|| initial_settlement_timeout(root, notifications))?;
        let quiet_window = debounce.settlement_window();
        match notifications.recv_timeout(quiet_window.min(remaining)) {
            Ok(Ok(_)) => {}
            Ok(Err(_)) => {
                return Err(WorthUiFilesystemWatcherDenial::BackendEventFailed(
                    root.to_path_buf(),
                ));
            }
            Err(RecvTimeoutError::Disconnected) => {
                return Err(WorthUiFilesystemWatcherDenial::EventStreamDisconnected(
                    root.to_path_buf(),
                ));
            }
            Err(RecvTimeoutError::Timeout) if remaining <= quiet_window => {
                return Err(initial_settlement_timeout(root, notifications));
            }
            Err(RecvTimeoutError::Timeout) => {
                return Ok(notifications.observed_notification_count());
            }
        }
    }
}

fn freeze_initial_filesystem_source(
    provider: &WorthUiFilesystemSourceProvider,
    notifications: &WorthUiFilesystemNotificationQueue,
    debounce: &WorthUiReloadDebounce,
    timeout: Duration,
) -> Result<WorthUiSettledSourceSnapshot, WorthUiFilesystemWatcherDenial> {
    let root = provider.root();
    let deadline = Instant::now().checked_add(timeout).ok_or_else(|| {
        WorthUiFilesystemWatcherDenial::SettlementDeadlineUnrepresentable(root.to_path_buf())
    })?;
    let events = [WorthUiWatcherEvent::provider_revision(
        root.to_string_lossy(),
    )];

    loop {
        let quiet_notification_count =
            await_initial_quiet_window(notifications, debounce, root, deadline)?;
        let snapshot = match freeze_filesystem_source(provider, debounce, &events, 1) {
            Ok(snapshot) => snapshot,
            Err(denial) if transient_acquisition_denial(&denial) => continue,
            Err(denial) => return Err(denial.into()),
        };
        if notifications.take_backend_failure() {
            return Err(WorthUiFilesystemWatcherDenial::BackendEventFailed(
                root.to_path_buf(),
            ));
        }
        if notifications.take_resnapshot_signal()
            || notifications.observed_notification_count() != quiet_notification_count
        {
            continue;
        }
        let _ = deadline
            .checked_duration_since(Instant::now())
            .ok_or_else(|| initial_settlement_timeout(root, notifications))?;
        return Ok(snapshot);
    }
}

fn initial_settlement_timeout(
    root: &std::path::Path,
    notifications: &WorthUiFilesystemNotificationQueue,
) -> WorthUiFilesystemWatcherDenial {
    WorthUiFilesystemWatcherDenial::InitialSettlementTimedOut {
        root: root.to_path_buf(),
        observed_notification_count: notifications.observed_notification_count(),
    }
}

fn transient_acquisition_denial(denial: &WorthUiFilesystemSourceAcquisitionDenial) -> bool {
    matches!(
        denial,
        WorthUiFilesystemSourceAcquisitionDenial::RootMetadataUnavailable(_)
            | WorthUiFilesystemSourceAcquisitionDenial::DirectoryReadFailed(_)
            | WorthUiFilesystemSourceAcquisitionDenial::SourceReadFailed(_)
            | WorthUiFilesystemSourceAcquisitionDenial::EmptySourceRoot(_)
            | WorthUiFilesystemSourceAcquisitionDenial::UnstableSourceTree(_)
    )
}

fn remaining(
    deadline: Instant,
    root: &std::path::Path,
    observed_notification_count: u64,
) -> Result<Duration, WorthUiFilesystemWatcherDenial> {
    deadline
        .checked_duration_since(Instant::now())
        .ok_or_else(|| WorthUiFilesystemWatcherDenial::SettlementTimedOut {
            root: root.to_path_buf(),
            observed_notification_count,
        })
}

fn append_bounded_events(
    events: &mut Vec<WorthUiWatcherEvent>,
    translated: Vec<WorthUiWatcherEvent>,
    provider_id: &str,
) {
    if events.len().saturating_add(translated.len()) > MAX_SETTLEMENT_EVENTS {
        events.clear();
        events.push(WorthUiWatcherEvent::provider_revision(provider_id));
    } else {
        events.extend(translated);
    }
}

fn native_backend(kind: WatcherKind) -> Option<WorthUiFilesystemWatcherBackend> {
    match kind {
        WatcherKind::Fsevent => Some(WorthUiFilesystemWatcherBackend::Fsevent),
        WatcherKind::Inotify => Some(WorthUiFilesystemWatcherBackend::Inotify),
        WatcherKind::Kqueue => Some(WorthUiFilesystemWatcherBackend::Kqueue),
        WatcherKind::ReadDirectoryChangesWatcher => {
            Some(WorthUiFilesystemWatcherBackend::ReadDirectoryChanges)
        }
        WatcherKind::PollWatcher | WatcherKind::NullWatcher => None,
        _ => Some(WorthUiFilesystemWatcherBackend::OtherNative),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::{Duration, Instant};

    use super::{await_initial_quiet_window, filesystem_notification_queue};
    use crate::runtime::source_ingress::{WorthUiFilesystemWatcherDenial, WorthUiReloadDebounce};

    #[test]
    fn expired_initial_settlement_deadline_is_typed_and_bounded() {
        let (notifications, _handler) = filesystem_notification_queue();
        let denial = await_initial_quiet_window(
            &notifications,
            &WorthUiReloadDebounce::stable_window(Duration::from_millis(25)),
            Path::new("expired-initial-settlement"),
            Instant::now(),
        )
        .expect_err("an expired initial deadline must deny before waiting");

        assert!(matches!(
            denial,
            WorthUiFilesystemWatcherDenial::InitialSettlementTimedOut { .. }
        ));
    }
}
