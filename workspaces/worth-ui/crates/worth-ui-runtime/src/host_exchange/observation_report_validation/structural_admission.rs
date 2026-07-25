use worth_ui_host_contract::{
    UiHostObservationBatch, UiHostObservationCanonicalCore, UiHostObservationReport,
};

use super::UiHostObservationReportDenial;

pub(super) struct UiStructurallyAdmittedObservationBatch {
    batch: UiHostObservationBatch,
}

impl UiStructurallyAdmittedObservationBatch {
    pub(super) fn admit(
        batch: UiHostObservationBatch,
        protocol: worth_ui_host_contract::UiHostProtocolAgreement,
    ) -> Result<Self, UiHostObservationReportDenial> {
        let core = batch.canonical_core();
        if core.protocol() != protocol {
            return Err(UiHostObservationReportDenial::ForeignProtocol);
        }
        batch
            .validate_shape()
            .map_err(|_| UiHostObservationReportDenial::MalformedBatch)?;
        if !batch.integrity().verifies(core, batch.reports()) {
            return Err(UiHostObservationReportDenial::IntegrityMismatch);
        }
        Ok(Self { batch })
    }

    pub(super) const fn core(&self) -> UiHostObservationCanonicalCore {
        self.batch.canonical_core()
    }

    pub(super) fn reports(&self) -> &[UiHostObservationReport] {
        self.batch.reports()
    }

    pub(super) fn into_batch(self) -> UiHostObservationBatch {
        self.batch
    }
}
