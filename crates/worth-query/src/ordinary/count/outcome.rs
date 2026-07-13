use crate::ordinary::read::{
    WorthQueryReadContextReceipt, WorthQueryReadJourneyCounters, WorthQueryReadStop,
};
use crate::runtime::WorthQueryCountResult;

#[derive(Debug)]
pub struct WorthQueryCountCompletion {
    result: WorthQueryCountResult,
    context_receipt: WorthQueryReadContextReceipt,
    journey_counters: WorthQueryReadJourneyCounters,
}

impl WorthQueryCountCompletion {
    pub fn result(&self) -> &WorthQueryCountResult {
        &self.result
    }

    pub fn context_receipt(&self) -> &WorthQueryReadContextReceipt {
        &self.context_receipt
    }

    pub fn journey_counters(&self) -> &WorthQueryReadJourneyCounters {
        &self.journey_counters
    }

    pub fn into_result(self) -> WorthQueryCountResult {
        self.result
    }

    pub(crate) fn new(
        result: WorthQueryCountResult,
        context_receipt: WorthQueryReadContextReceipt,
        journey_counters: WorthQueryReadJourneyCounters,
    ) -> Self {
        Self {
            result,
            context_receipt,
            journey_counters,
        }
    }
}

#[derive(Debug)]
pub enum WorthQueryCountOutcome {
    Completed(WorthQueryCountCompletion),
    Stopped(WorthQueryReadStop),
}

impl WorthQueryCountOutcome {
    pub fn completed(&self) -> Option<&WorthQueryCountCompletion> {
        match self {
            Self::Completed(completion) => Some(completion),
            Self::Stopped(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryReadStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
        }
    }

    pub fn into_result(self) -> Result<WorthQueryCountCompletion, WorthQueryReadStop> {
        match self {
            Self::Completed(completion) => Ok(completion),
            Self::Stopped(stop) => Err(stop),
        }
    }
}
