use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use worth_ui::facade::source::{
    WorthUiFilesystemSourceWatcher, WorthUiFilesystemWatcherDenial,
    WorthUiFilesystemWatcherShutdownReceipt, WorthUiSettledSourceSnapshot,
};

pub(crate) enum PlatformPulseSourceEvent {
    Settled(Box<WorthUiSettledSourceSnapshot>),
    Failed(WorthUiFilesystemWatcherDenial),
}

#[derive(Debug)]
pub(crate) enum PlatformPulseSourceWatchShutdownDenial {
    WorkerPanicked,
    Watcher(WorthUiFilesystemWatcherDenial),
}

pub(crate) struct PlatformPulseSourceWatch {
    stop: Sender<()>,
    events: Receiver<PlatformPulseSourceEvent>,
    worker:
        JoinHandle<Result<WorthUiFilesystemWatcherShutdownReceipt, WorthUiFilesystemWatcherDenial>>,
    readiness: Arc<Mutex<Option<worth_ui_platform_pulse::PlatformPulseApplicationReadinessSignal>>>,
}

impl PlatformPulseSourceWatch {
    pub(crate) fn spawn(watcher: WorthUiFilesystemSourceWatcher) -> Self {
        let (stop, stop_requests) = mpsc::channel();
        let (event_publications, events) = mpsc::channel();
        let readiness = Arc::new(Mutex::new(None));
        let worker_readiness = Arc::clone(&readiness);
        let worker = std::thread::Builder::new()
            .name("worth-ui-platform-pulse-source".to_owned())
            .spawn(move || run(watcher, stop_requests, event_publications, worker_readiness))
            .expect("platform pulse source worker should start");
        Self {
            stop,
            events,
            worker,
            readiness,
        }
    }

    pub(crate) fn install_readiness(
        &self,
        readiness: worth_ui_platform_pulse::PlatformPulseApplicationReadinessSignal,
    ) {
        *self
            .readiness
            .lock()
            .expect("source readiness installation is not poisoned") = Some(readiness);
    }

    pub(crate) fn try_next(&self) -> Option<PlatformPulseSourceEvent> {
        self.events.try_recv().ok()
    }

    pub(crate) fn shutdown(
        self,
    ) -> Result<WorthUiFilesystemWatcherShutdownReceipt, PlatformPulseSourceWatchShutdownDenial>
    {
        let _ = self.stop.send(());
        self.worker
            .join()
            .map_err(|_| PlatformPulseSourceWatchShutdownDenial::WorkerPanicked)?
            .map_err(PlatformPulseSourceWatchShutdownDenial::Watcher)
    }
}

fn run(
    mut watcher: WorthUiFilesystemSourceWatcher,
    stop: Receiver<()>,
    events: Sender<PlatformPulseSourceEvent>,
    readiness: Arc<Mutex<Option<worth_ui_platform_pulse::PlatformPulseApplicationReadinessSignal>>>,
) -> Result<WorthUiFilesystemWatcherShutdownReceipt, WorthUiFilesystemWatcherDenial> {
    loop {
        match stop.try_recv() {
            Ok(()) | Err(TryRecvError::Disconnected) => return watcher.shutdown(),
            Err(TryRecvError::Empty) => {}
        }
        match watcher.settle(Duration::from_millis(250)) {
            Ok(snapshot) => {
                if events
                    .send(PlatformPulseSourceEvent::Settled(Box::new(snapshot)))
                    .is_err()
                {
                    return watcher.shutdown();
                }
                signal_readiness(&readiness);
            }
            Err(WorthUiFilesystemWatcherDenial::SettlementTimedOut { .. }) => {}
            Err(denial) => {
                let _ = events.send(PlatformPulseSourceEvent::Failed(denial));
                signal_readiness(&readiness);
                return watcher.shutdown();
            }
        }
    }
}

fn signal_readiness(
    readiness: &Mutex<Option<worth_ui_platform_pulse::PlatformPulseApplicationReadinessSignal>>,
) {
    if let Some(readiness) = readiness
        .lock()
        .expect("source readiness signal is not poisoned")
        .as_ref()
    {
        readiness.signal();
    }
}
