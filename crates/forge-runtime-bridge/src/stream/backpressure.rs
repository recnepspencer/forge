use std::sync::Arc;

use crate::identity::{BackpressureDecisionIdentityTag, BridgeIdentity};
use crate::routing::canonicalization::digest_string;

use super::counters::StreamProtocolCounters;
use super::protocol::ConsumerContractIdentity;
use super::window::{PlannedChangeStreamWindow, StreamWindowIdentity};

type BackpressureDecisionIdentity = BridgeIdentity<BackpressureDecisionIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackpressureDecisionRecord {
    backpressure_decision_identity: BackpressureDecisionIdentity,
    consumer_contract_identity: ConsumerContractIdentity,
    stream_window_identity: StreamWindowIdentity,
    pressure_class: Arc<str>,
    pressure_reason_family: Arc<str>,
    counters: StreamProtocolCounters,
}

impl BackpressureDecisionRecord {
    pub(crate) fn classify(window: &PlannedChangeStreamWindow) -> Self {
        let (pressure_class, pressure_reason_family) =
            match (window.members().len(), window.coalescing_family()) {
                (0 | 1, _) => ("no-pressure", "none"),
                (2, super::declaration::StreamCoalescingFamily::None) => {
                    ("elevated-pressure", "consumer-lag-risk")
                }
                (2, _) => ("elevated-pressure", "coalesced-window-width"),
                (_, super::declaration::StreamCoalescingFamily::None) => {
                    ("saturated-pressure", "uncoalesced-burst-width")
                }
                (_, _) => ("saturated-pressure", "coalesced-burst-width"),
            };
        Self::new(window, pressure_class, pressure_reason_family)
    }

    fn new(
        window: &PlannedChangeStreamWindow,
        pressure_class: &str,
        pressure_reason_family: &str,
    ) -> Self {
        let basis = format!(
            "backpressure-decision-record|contract={}|window={}|pressure-class={}|pressure-reason-family={}",
            window.consumer_contract_identity().as_str(),
            window.stream_window_identity().as_str(),
            pressure_class,
            pressure_reason_family,
        );
        let digest = digest_string("backpressure-decision-record", &basis);
        Self {
            backpressure_decision_identity: BackpressureDecisionIdentity::new(digest),
            consumer_contract_identity: window.consumer_contract_identity().clone(),
            stream_window_identity: window.stream_window_identity().clone(),
            pressure_class: Arc::from(pressure_class),
            pressure_reason_family: Arc::from(pressure_reason_family),
            counters: window.counters().clone().with_backpressure(
                pressure_class != "no-pressure",
                pressure_class == "saturated-pressure",
            ),
        }
    }

    pub fn backpressure_decision_identity(&self) -> &str {
        self.backpressure_decision_identity.as_str()
    }

    pub fn consumer_contract_identity(&self) -> &ConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn stream_window_identity(&self) -> &StreamWindowIdentity {
        &self.stream_window_identity
    }

    pub fn pressure_class(&self) -> &str {
        self.pressure_class.as_ref()
    }

    pub fn pressure_reason_family(&self) -> &str {
        self.pressure_reason_family.as_ref()
    }

    pub fn counters(&self) -> &StreamProtocolCounters {
        &self.counters
    }
}
