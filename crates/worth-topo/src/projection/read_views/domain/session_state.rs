use std::cell::RefCell;

use super::error::TopologyReadError;
use crate::projection::diagnostic_surfaces::read_proof::fallback::TopologyReadFallbackPosture;
use crate::projection::diagnostic_surfaces::read_proof::ledger::TopologyReadProofLedger;
use crate::projection::diagnostic_surfaces::read_proof::report::{
    TopologyReadAggregateReport, TopologyReadRequestFamily, TopologyReadRequestReport,
};

pub(crate) struct TopologyReadLedger {
    pub(crate) request_reports: RefCell<Vec<TopologyReadRequestReport>>,
    pub(crate) proof_ledger: TopologyReadProofLedger,
}

impl TopologyReadLedger {
    const MAX_SUPPORTED_TRAVERSAL_DEPTH: usize = 64;

    pub(crate) fn record_report(
        &self,
        report: TopologyReadRequestReport,
    ) -> TopologyReadRequestReport {
        self.request_reports.borrow_mut().push(report.clone());
        report
    }

    pub(crate) fn require_supported_traversal_depth(
        request_family: TopologyReadRequestFamily,
        requested_depth: usize,
    ) -> Result<(), TopologyReadError> {
        if requested_depth == 0 || requested_depth > Self::MAX_SUPPORTED_TRAVERSAL_DEPTH {
            return Err(TopologyReadError::unsupported_traversal_depth(
                request_family,
                requested_depth,
                Self::MAX_SUPPORTED_TRAVERSAL_DEPTH,
            ));
        }
        Ok(())
    }

    pub(crate) fn new() -> Self {
        Self {
            request_reports: RefCell::new(Vec::new()),
            proof_ledger: TopologyReadProofLedger::default(),
        }
    }

    pub(crate) fn fallback_posture(&self) -> TopologyReadFallbackPosture {
        TopologyReadFallbackPosture::None
    }

    pub(crate) fn supported_request_families(&self) -> Vec<TopologyReadRequestFamily> {
        TopologyReadRequestFamily::ALL.to_vec()
    }

    pub(crate) fn aggregate_report(&self) -> TopologyReadAggregateReport {
        TopologyReadAggregateReport::from_request_reports(self.request_reports.borrow().as_slice())
    }
}
