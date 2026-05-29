use std::cell::RefCell;

use super::parity::{
    compare_domain_query_view_parity, TopologyDomainQueryParityAggregateReport,
    TopologyDomainQueryParityKind, TopologyDomainQueryViewParityArtifact,
    TopologyDomainQueryViewParityReport,
};
use super::report::TopologyDomainQueryAggregateReport;
use crate::projection::read_views::domain::TopologyDomainQuery;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyDomainQueryProofReport {
    pub(crate) request_aggregate: TopologyDomainQueryAggregateReport,
    pub(crate) parity_aggregate: TopologyDomainQueryParityAggregateReport,
}

#[derive(Debug, Default)]
pub(crate) struct TopologyDomainQueryProofLedger {
    parity_reports: RefCell<Vec<TopologyDomainQueryViewParityReport>>,
}

impl TopologyDomainQueryProofLedger {
    pub(crate) fn record_parity_report(&self, report: TopologyDomainQueryViewParityReport) {
        self.parity_reports.borrow_mut().push(report);
    }

    pub(crate) fn parity_aggregate_report(&self) -> TopologyDomainQueryParityAggregateReport {
        TopologyDomainQueryParityAggregateReport::from_reports(
            self.parity_reports.borrow().as_slice(),
        )
    }

    pub(crate) fn build_report(
        &self,
        request_aggregate: TopologyDomainQueryAggregateReport,
    ) -> TopologyDomainQueryProofReport {
        TopologyDomainQueryProofReport {
            request_aggregate,
            parity_aggregate: self.parity_aggregate_report(),
        }
    }
}

impl TopologyDomainQuery {
    pub(crate) fn record_view_parity(
        &self,
        parity_kind: TopologyDomainQueryParityKind,
        left: &TopologyDomainQueryViewParityArtifact,
        right: &TopologyDomainQueryViewParityArtifact,
    ) -> TopologyDomainQueryViewParityReport {
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

    pub fn proof_report(&self) -> TopologyDomainQueryProofReport {
        self.proof_ledger
            .build_report(TopologyDomainQueryAggregateReport::from_request_reports(
                self.request_reports.borrow().as_slice(),
            ))
    }
}




