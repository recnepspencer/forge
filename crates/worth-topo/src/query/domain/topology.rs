use std::cell::RefCell;

use super::error::TopologyDomainQueryError;
use super::proof::fallback::TopologyDomainQueryFallbackPosture;
use super::proof::ledger::TopologyDomainQueryProofLedger;
use super::proof::report::{
    TopologyDomainQueryAggregateReport, TopologyDomainQueryRequestFamily,
    TopologyDomainQueryRequestReport,
};

pub struct TopologyDomainQuery {
    pub(super) request_reports: RefCell<Vec<TopologyDomainQueryRequestReport>>,
    pub(super) proof_ledger: TopologyDomainQueryProofLedger,
}

impl TopologyDomainQuery {
    const MAX_SUPPORTED_TRAVERSAL_DEPTH: usize = 64;

    pub(super) fn record_report(
        &self,
        report: TopologyDomainQueryRequestReport,
    ) -> TopologyDomainQueryRequestReport {
        self.request_reports.borrow_mut().push(report.clone());
        report
    }

    pub(super) fn require_supported_traversal_depth(
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

    pub fn load() -> Self {
        Self {
            request_reports: RefCell::new(Vec::new()),
            proof_ledger: TopologyDomainQueryProofLedger::default(),
        }
    }

    #[allow(dead_code)]
    pub fn fallback_posture(&self) -> TopologyDomainQueryFallbackPosture {
        TopologyDomainQueryFallbackPosture::None
    }

    #[allow(dead_code)]
    pub fn supported_request_families(&self) -> Vec<TopologyDomainQueryRequestFamily> {
        TopologyDomainQueryRequestFamily::ALL.to_vec()
    }

    #[allow(dead_code)]
    pub fn aggregate_report(&self) -> TopologyDomainQueryAggregateReport {
        TopologyDomainQueryAggregateReport::from_request_reports(
            self.request_reports.borrow().as_slice(),
        )
    }
}
