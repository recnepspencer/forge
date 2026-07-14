use crate::runtime::WorthQueryReadResult;

use super::super::projection::WorthQueryReadProjectionBinding;
use super::super::{WorthQueryProjectionDeclaration, WorthQueryProjectionOutcome};
use super::super::{WorthQueryReadContextReceipt, WorthQueryReadJourneyCounters};

#[derive(Debug)]
pub struct WorthQueryReadCompletion {
    result: WorthQueryReadResult,
    context_receipt: WorthQueryReadContextReceipt,
    journey_counters: WorthQueryReadJourneyCounters,
    projection_binding: WorthQueryReadProjectionBinding,
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

    /// Extract typed projection facts through the authority sealed into this
    /// completed read. The operational receipt alone cannot invoke this lane.
    pub fn consume_projection(
        &self,
        declaration: WorthQueryProjectionDeclaration,
    ) -> WorthQueryProjectionOutcome {
        self.projection_binding.consume(&self.result, declaration)
    }

    pub fn into_result(self) -> WorthQueryReadResult {
        self.result
    }

    pub(crate) fn new(
        result: WorthQueryReadResult,
        context_receipt: WorthQueryReadContextReceipt,
        journey_counters: WorthQueryReadJourneyCounters,
        projection_binding: WorthQueryReadProjectionBinding,
    ) -> Self {
        Self {
            result,
            context_receipt,
            journey_counters,
            projection_binding,
        }
    }
}
