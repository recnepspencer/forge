use crate::runtime::WorthUiMeasurementCertificationDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadCounterBoundaryDenial {
    reason: WorthUiReloadCounterBoundaryDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiReloadCounterBoundaryDenialReason {
    EmptyCounterReceipt,
    FullArtifactScanDetected,
    MissingRequiredCounterRow,
    UnexpectedCounterRow,
    DuplicateCounterRow,
    DuplicateCounterPacket,
    MeasurementCertification(WorthUiMeasurementCertificationDenial),
    FoundationalLowering(WorthUiMeasurementCertificationDenial),
}

impl WorthUiReloadCounterBoundaryDenial {
    pub(crate) fn new(reason: WorthUiReloadCounterBoundaryDenialReason) -> Self {
        Self { reason }
    }

    pub fn reason(&self) -> WorthUiReloadCounterBoundaryDenialReason {
        self.reason
    }
}
