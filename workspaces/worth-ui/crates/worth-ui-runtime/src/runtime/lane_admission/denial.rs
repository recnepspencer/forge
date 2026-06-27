use crate::runtime::{
    WorthUiExecutionLane, WorthUiLaneAdmissionCounters, WorthUiLaneSupportDiagnostic,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLaneAdmissionDenialReason {
    UnsupportedLaneReference,
    PrivateComponentLaneClaim,
    MissingQuerySupportLinks,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneAdmissionDenial {
    reason: WorthUiLaneAdmissionDenialReason,
    lane: Option<WorthUiExecutionLane>,
    diagnostic: Option<WorthUiLaneSupportDiagnostic>,
    counters: WorthUiLaneAdmissionCounters,
}

impl WorthUiLaneAdmissionDenial {
    pub(crate) fn new(
        reason: WorthUiLaneAdmissionDenialReason,
        lane: Option<WorthUiExecutionLane>,
        diagnostic: Option<WorthUiLaneSupportDiagnostic>,
        mut counters: WorthUiLaneAdmissionCounters,
    ) -> Self {
        counters.record_denial();
        Self {
            reason,
            lane,
            diagnostic,
            counters,
        }
    }

    pub fn reason(&self) -> WorthUiLaneAdmissionDenialReason {
        self.reason
    }

    pub fn lane(&self) -> Option<WorthUiExecutionLane> {
        self.lane
    }

    pub fn diagnostic(&self) -> Option<&WorthUiLaneSupportDiagnostic> {
        self.diagnostic.as_ref()
    }

    pub fn counters(&self) -> WorthUiLaneAdmissionCounters {
        self.counters
    }
}
