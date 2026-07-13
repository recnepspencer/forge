use crate::runtime::WorthQueryRuntimeError;

use super::super::{
    WorthQueryReadContextDenial, WorthQueryReadContextReceipt, WorthQueryReadJourneyCounters,
};
use super::next_action::{classify_context_next_action, classify_runtime_next_action};
use super::WorthQueryReadNextAction;

#[derive(Debug)]
pub enum WorthQueryReadStopSource {
    Context(WorthQueryReadContextDenial),
    Runtime(WorthQueryRuntimeError),
}

#[derive(Debug)]
pub struct WorthQueryReadStop {
    next_action: WorthQueryReadNextAction,
    source: WorthQueryReadStopSource,
    context_receipt: Option<WorthQueryReadContextReceipt>,
    journey_counters: WorthQueryReadJourneyCounters,
}

impl WorthQueryReadStop {
    pub fn next_action(&self) -> WorthQueryReadNextAction {
        self.next_action
    }

    pub fn source(&self) -> &WorthQueryReadStopSource {
        &self.source
    }

    pub fn context_denial(&self) -> Option<&WorthQueryReadContextDenial> {
        match &self.source {
            WorthQueryReadStopSource::Context(denial) => Some(denial),
            WorthQueryReadStopSource::Runtime(_) => None,
        }
    }

    pub fn runtime_error(&self) -> Option<&WorthQueryRuntimeError> {
        match &self.source {
            WorthQueryReadStopSource::Runtime(error) => Some(error),
            WorthQueryReadStopSource::Context(_) => None,
        }
    }

    pub fn context_receipt(&self) -> Option<&WorthQueryReadContextReceipt> {
        self.context_receipt.as_ref()
    }

    pub fn journey_counters(&self) -> &WorthQueryReadJourneyCounters {
        &self.journey_counters
    }

    pub(crate) fn context(
        source: WorthQueryReadContextDenial,
        journey_counters: WorthQueryReadJourneyCounters,
    ) -> Self {
        let next_action = classify_context_next_action(&source);
        Self {
            next_action,
            source: WorthQueryReadStopSource::Context(source),
            context_receipt: None,
            journey_counters,
        }
    }

    pub(crate) fn runtime(
        source: WorthQueryRuntimeError,
        context_receipt: WorthQueryReadContextReceipt,
        journey_counters: WorthQueryReadJourneyCounters,
    ) -> Self {
        let next_action = classify_runtime_next_action(&source);
        Self {
            next_action,
            source: WorthQueryReadStopSource::Runtime(source),
            context_receipt: Some(context_receipt),
            journey_counters,
        }
    }
}
