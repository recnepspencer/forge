use std::cell::RefCell;

use super::parity::{
    compare_domain_query_view_parity, WorthTopologyDomainQueryParityAggregateReport,
    WorthTopologyDomainQueryParityKind, WorthTopologyDomainQueryViewParityArtifact,
    WorthTopologyDomainQueryViewParityReport,
};
use super::report::WorthTopologyDomainQueryAggregateReport;
use super::topology::WorthTopologyDomainQuery;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyDomainQueryProofReport {
    pub(crate) request_aggregate: WorthTopologyDomainQueryAggregateReport,
    pub(crate) parity_aggregate: WorthTopologyDomainQueryParityAggregateReport,
}

#[derive(Debug, Default)]
pub(crate) struct WorthTopologyDomainQueryProofLedger {
    parity_reports: RefCell<Vec<WorthTopologyDomainQueryViewParityReport>>,
}

impl WorthTopologyDomainQueryProofLedger {
    pub(crate) fn record_parity_report(&self, report: WorthTopologyDomainQueryViewParityReport) {
        self.parity_reports.borrow_mut().push(report);
    }

    pub(crate) fn parity_aggregate_report(&self) -> WorthTopologyDomainQueryParityAggregateReport {
        WorthTopologyDomainQueryParityAggregateReport::from_reports(
            self.parity_reports.borrow().as_slice(),
        )
    }

    pub(crate) fn build_report(
        &self,
        request_aggregate: WorthTopologyDomainQueryAggregateReport,
    ) -> WorthTopologyDomainQueryProofReport {
        WorthTopologyDomainQueryProofReport {
            request_aggregate,
            parity_aggregate: self.parity_aggregate_report(),
        }
    }
}

impl WorthTopologyDomainQuery {
    pub(crate) fn record_view_parity(
        &self,
        parity_kind: WorthTopologyDomainQueryParityKind,
        left: &WorthTopologyDomainQueryViewParityArtifact,
        right: &WorthTopologyDomainQueryViewParityArtifact,
    ) -> WorthTopologyDomainQueryViewParityReport {
        assert!(
            self.request_reports
                .borrow()
                .iter()
                .any(|report| report.request_family == left.request_family()),
            "domain query parity must be recorded only for a request family observed on this boundary"
        );
        let report = compare_domain_query_view_parity(parity_kind, left, right);
        self.proof_ledger.record_parity_report(report.clone());
        report
    }

    pub(crate) fn proof_report(&self) -> WorthTopologyDomainQueryProofReport {
        self.proof_ledger.build_report(
            WorthTopologyDomainQueryAggregateReport::from_request_reports(
                self.request_reports.borrow().as_slice(),
            ),
        )
    }
}
