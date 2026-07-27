use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
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
}

impl PlatformPulseSourceWatch {
    pub(crate) fn spawn(watcher: WorthUiFilesystemSourceWatcher) -> Self {
        let (stop, stop_requests) = mpsc::channel();
        let (event_publications, events) = mpsc::channel();
        let worker = std::thread::Builder::new()
            .name("worth-ui-platform-pulse-source".to_owned())
            .spawn(move || run(watcher, stop_requests, event_publications))
            .expect("platform pulse source worker should start");
        Self {
            stop,
            events,
            worker,
        }
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
            }
            Err(WorthUiFilesystemWatcherDenial::SettlementTimedOut { .. }) => {}
            Err(denial) => {
                let _ = events.send(PlatformPulseSourceEvent::Failed(denial));
                return watcher.shutdown();
            }
        }
    }
}
