use crate::runtime::{
    WorthUiFailedActivationReport, WorthUiReloadDenial, WorthUiReloadFailureCounters,
    WorthUiReloadPreservationReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiReloadFailure {
    denial: WorthUiReloadDenial,
    preservation_receipt: WorthUiReloadPreservationReceipt,
    failed_activation_report: WorthUiFailedActivationReport,
    counters: WorthUiReloadFailureCounters,
}

impl WorthUiReloadFailure {
    pub(crate) fn new(
        denial: WorthUiReloadDenial,
        preservation_receipt: WorthUiReloadPreservationReceipt,
        failed_activation_report: WorthUiFailedActivationReport,
        counters: WorthUiReloadFailureCounters,
    ) -> Self {
        Self {
            denial,
            preservation_receipt,
            failed_activation_report,
            counters,
        }
    }

    pub fn denial(self) -> WorthUiReloadDenial {
        self.denial
    }

    pub fn preservation_receipt(self) -> WorthUiReloadPreservationReceipt {
        self.preservation_receipt
    }

    pub fn failed_activation_report(self) -> WorthUiFailedActivationReport {
        self.failed_activation_report
    }

    pub fn counters(self) -> WorthUiReloadFailureCounters {
        self.counters
    }
}
