use crate::runtime::{WorthQueryReadDenial, WorthQueryRuntimeError};

use super::super::{
    WorthQueryReadContextDenial, WorthQueryReadContextReceipt, WorthQueryReadJourneyCounters,
};
use super::next_action::{
    classify_context_next_action, classify_planning_next_action, classify_runtime_next_action,
};
use super::WorthQueryReadNextAction;

#[derive(Debug)]
pub enum WorthQueryReadStopSource {
    Context(WorthQueryReadContextDenial),
    Planning(WorthQueryReadDenial),
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
            WorthQueryReadStopSource::Planning(_) | WorthQueryReadStopSource::Runtime(_) => None,
        }
    }

    pub fn planning_denial(&self) -> Option<&WorthQueryReadDenial> {
        match &self.source {
            WorthQueryReadStopSource::Planning(denial) => Some(denial),
            WorthQueryReadStopSource::Context(_) | WorthQueryReadStopSource::Runtime(_) => None,
        }
    }

    pub fn runtime_error(&self) -> Option<&WorthQueryRuntimeError> {
        match &self.source {
            WorthQueryReadStopSource::Runtime(error) => Some(error),
            WorthQueryReadStopSource::Context(_) | WorthQueryReadStopSource::Planning(_) => None,
        }
    }

    pub(crate) fn planning(
        source: WorthQueryReadDenial,
        context_receipt: WorthQueryReadContextReceipt,
        journey_counters: WorthQueryReadJourneyCounters,
    ) -> Self {
        let next_action = classify_planning_next_action(&source);
        Self {
            next_action,
            source: WorthQueryReadStopSource::Planning(source),
            context_receipt: Some(context_receipt),
            journey_counters,
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
