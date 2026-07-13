use crate::runtime::WorthQueryReadResult;

use super::super::{WorthQueryReadContextReceipt, WorthQueryReadJourneyCounters};

#[derive(Debug)]
pub struct WorthQueryReadCompletion {
    result: WorthQueryReadResult,
    context_receipt: WorthQueryReadContextReceipt,
    journey_counters: WorthQueryReadJourneyCounters,
}

impl WorthQueryReadCompletion {
    pub fn result(&self) -> &WorthQueryReadResult {
        &self.result
    }

    pub fn context_receipt(&self) -> &WorthQueryReadContextReceipt {
        &self.context_receipt
    }

    pub fn journey_counters(&self) -> &WorthQueryReadJourneyCounters {
        &self.journey_counters
    }

    pub fn into_result(self) -> WorthQueryReadResult {
        self.result
    }

    pub(crate) fn new(
        result: WorthQueryReadResult,
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
