use super::{PlatformPulseNativeFrame, PlatformPulseTerminalError};

mod clock;
mod evidence_index;
mod execution;
mod native_ingress;
mod product_action;
mod product_input;

pub(super) use clock::{PlatformPulseIntentClock, PlatformPulseIntentClockDenial};
pub(super) use evidence_index::PlatformPulseIntentEvidenceIndex;

pub(super) enum PlatformPulseIntentPosturePublicationDenial {
    Indeterminate,
    RemainedInFlight,
    Stopped(worth_ui::facade::app::WorthUiNativeIntentPosturePublicationStop),
    InternalDefect(worth_ui::facade::rebind::UiRebindInternalDefectOutcome),
}

impl std::fmt::Display for PlatformPulseIntentPosturePublicationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Indeterminate => formatter.write_str("became indeterminate"),
            Self::RemainedInFlight => formatter.write_str("remained in flight"),
            Self::Stopped(stop) => write!(formatter, "stopped: {:?}", stop.reason()),
            Self::InternalDefect(defect) => {
                write!(formatter, "reached internal defect: {:?}", defect.kind())
            }
        }
    }
}

impl PlatformPulseNativeFrame {
    fn fail_intent_clock(&mut self, denial: PlatformPulseIntentClockDenial) {
        let observation = self.publisher.intent_preparation_failure();
        self.fail(PlatformPulseTerminalError::IntentClock(denial), observation);
    }

    fn fail_intent_settlement(&mut self, detail: impl Into<String>) {
        let observation = self.publisher.intent_preparation_failure();
        self.fail(
            PlatformPulseTerminalError::IntentExecution(detail.into()),
            observation,
        );
    }
}
