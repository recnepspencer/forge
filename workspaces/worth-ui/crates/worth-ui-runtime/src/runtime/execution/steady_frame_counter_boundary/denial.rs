use crate::runtime::WorthUiMeasurementCertificationDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSteadyFrameCounterDenial {
    reason: WorthUiSteadyFrameCounterDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSteadyFrameCounterDenialReason {
    EmptySteadyFrameReceipt,
    DuplicateLaneFrameReceipt,
    LaneFrameReceiptMismatch,
    ForeignGeneration,
    ExecutedBreadthExceedsRequest,
    ForbiddenFramePathWork,
    DiagnosticMaterializationOnMinimalPolicy,
    MissingRequiredCounterRow,
    UnexpectedCounterRow,
    DuplicateCounterRow,
    MeasurementCertification(WorthUiMeasurementCertificationDenial),
    FoundationalLowering(WorthUiMeasurementCertificationDenial),
    FoundationalReportPlanning,
}

impl WorthUiSteadyFrameCounterDenial {
    pub(crate) fn new(reason: WorthUiSteadyFrameCounterDenialReason) -> Self {
        Self { reason }
    }

    pub fn reason(&self) -> WorthUiSteadyFrameCounterDenialReason {
        self.reason
    }
}
