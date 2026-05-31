use std::cell::RefCell;

use super::error::TopologyDomainQueryError;
use crate::projection::diagnostic_surfaces::read_proof::fallback::TopologyDomainQueryFallbackPosture;
use crate::projection::diagnostic_surfaces::read_proof::ledger::TopologyDomainQueryProofLedger;
use crate::projection::diagnostic_surfaces::read_proof::report::{
    TopologyDomainQueryAggregateReport, TopologyDomainQueryRequestFamily,
    TopologyDomainQueryRequestReport,
};

pub(crate) struct TopologyReadLedger {
    pub(crate) request_reports: RefCell<Vec<TopologyDomainQueryRequestReport>>,
    pub(crate) proof_ledger: TopologyDomainQueryProofLedger,
}

impl TopologyReadLedger {
    const MAX_SUPPORTED_TRAVERSAL_DEPTH: usize = 64;

    pub(crate) fn record_report(
        &self,
        report: TopologyDomainQueryRequestReport,
    ) -> TopologyDomainQueryRequestReport {
        self.request_reports.borrow_mut().push(report.clone());
        report
    }

    pub(crate) fn require_supported_traversal_depth(
        request_family: TopologyDomainQueryRequestFamily,
        requested_depth: usize,
    ) -> Result<(), TopologyDomainQueryError> {
        if requested_depth == 0 || requested_depth > Self::MAX_SUPPORTED_TRAVERSAL_DEPTH {
            return Err(TopologyDomainQueryError::unsupported_traversal_depth(
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
            proof_ledger: TopologyDomainQueryProofLedger::default(),
        }
    }

    pub(crate) fn fallback_posture(&self) -> TopologyDomainQueryFallbackPosture {
        TopologyDomainQueryFallbackPosture::None
    }

    pub(crate) fn supported_request_families(&self) -> Vec<TopologyDomainQueryRequestFamily> {
        TopologyDomainQueryRequestFamily::ALL.to_vec()
    }

    pub(crate) fn aggregate_report(&self) -> TopologyDomainQueryAggregateReport {
        TopologyDomainQueryAggregateReport::from_request_reports(
            self.request_reports.borrow().as_slice(),
        )
    }
}
