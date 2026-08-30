use std::path::{Path, PathBuf};

use super::{
    OfflineIntegrityObservationLimits, OfflineIntegrityReportBoundary,
    OfflineIntegrityReportBoundaryDenial, OfflineIntegrityReportDestination,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineIntegrityObservationRequest {
    store_root: PathBuf,
    limits: OfflineIntegrityObservationLimits,
    report: OfflineIntegrityReportBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineIntegrityObservationRequestDenial {
    EmptyStoreRoot,
    ReportBoundary(OfflineIntegrityReportBoundaryDenial),
}

impl OfflineIntegrityObservationRequest {
    pub fn new(
        store_root: PathBuf,
        limits: OfflineIntegrityObservationLimits,
        report_destination: OfflineIntegrityReportDestination,
    ) -> Result<Self, OfflineIntegrityObservationRequestDenial> {
        if store_root.as_os_str().is_empty() {
            return Err(OfflineIntegrityObservationRequestDenial::EmptyStoreRoot);
        }
        let report = OfflineIntegrityReportBoundary::new(
            &store_root,
            report_destination,
            limits.maximum_report_bytes(),
        )
        .map_err(OfflineIntegrityObservationRequestDenial::ReportBoundary)?;
        Ok(Self {
            store_root,
            limits,
            report,
        })
    }

    pub fn store_root(&self) -> &Path {
        &self.store_root
    }

    pub const fn limits(&self) -> OfflineIntegrityObservationLimits {
        self.limits
    }

    pub const fn report(&self) -> &OfflineIntegrityReportBoundary {
        &self.report
    }
}
