use crate::ordinary::read::{
    WorthQueryReadContextReceipt, WorthQueryReadJourneyCounters, WorthQueryReadNextAction,
    WorthQueryReadStop, WorthQueryReadStopSource,
};

use super::WorthQueryManagedLiveHandle;

#[derive(Debug)]
#[must_use = "live resource opening may stop and successful handles require lifecycle ownership"]
pub enum WorthQueryLiveOpenOutcome {
    Opened(WorthQueryLiveOpenCompletion),
    Stopped(WorthQueryLiveOpenStop),
}

#[derive(Debug)]
pub struct WorthQueryLiveOpenCompletion {
    handle: WorthQueryManagedLiveHandle,
    context_receipt: WorthQueryReadContextReceipt,
    journey_counters: WorthQueryReadJourneyCounters,
}

impl WorthQueryLiveOpenCompletion {
    pub fn handle(&self) -> &WorthQueryManagedLiveHandle {
        &self.handle
    }

    pub fn context_receipt(&self) -> &WorthQueryReadContextReceipt {
        &self.context_receipt
    }

    pub fn journey_counters(&self) -> &WorthQueryReadJourneyCounters {
        &self.journey_counters
    }

    pub fn into_handle(self) -> WorthQueryManagedLiveHandle {
        self.handle
    }

    pub(crate) fn new(
        handle: WorthQueryManagedLiveHandle,
        context_receipt: WorthQueryReadContextReceipt,
        journey_counters: WorthQueryReadJourneyCounters,
    ) -> Self {
        Self {
            handle,
            context_receipt,
            journey_counters,
        }
    }
}

#[derive(Debug)]
pub struct WorthQueryLiveOpenStop {
    read_stop: WorthQueryReadStop,
}

impl WorthQueryLiveOpenStop {
    pub fn next_action(&self) -> WorthQueryReadNextAction {
        self.read_stop.next_action()
    }

    pub fn source(&self) -> &WorthQueryReadStopSource {
        self.read_stop.source()
    }

    pub fn read_stop(&self) -> &WorthQueryReadStop {
        &self.read_stop
    }

    pub(crate) fn new(read_stop: WorthQueryReadStop) -> Self {
        Self { read_stop }
    }
}
