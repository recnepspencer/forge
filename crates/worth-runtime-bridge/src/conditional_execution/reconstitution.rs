/// Owner-separated evidence that a Bridge conditional successor was rebuilt
/// from Signal checkpoint authority and Bridge correspondence authority.
///
/// The report is reconstructive observation only. It does not authorize a
/// delivery, Signal execution, Query impact, or publication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeConditionalRuntimeReconstitutionReport {
    signal: worth_signal::facade::adapters::SignalGraphReconstitutionReport,
    correspondence: crate::correspondence::BridgeCorrespondenceRebuildReport,
}

impl BridgeConditionalRuntimeReconstitutionReport {
    pub(crate) const fn new(
        signal: worth_signal::facade::adapters::SignalGraphReconstitutionReport,
        correspondence: crate::correspondence::BridgeCorrespondenceRebuildReport,
    ) -> Self {
        Self {
            signal,
            correspondence,
        }
    }

    pub const fn signal(self) -> worth_signal::facade::adapters::SignalGraphReconstitutionReport {
        self.signal
    }

    pub const fn correspondence(self) -> crate::correspondence::BridgeCorrespondenceRebuildReport {
        self.correspondence
    }
}
