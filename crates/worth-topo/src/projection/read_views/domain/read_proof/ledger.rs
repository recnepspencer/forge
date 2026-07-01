use std::cell::RefCell;

use super::parity::{
    compare_topology_read_view_parity, TopologyReadParityAggregateReport, TopologyReadParityKind,
    TopologyReadViewParityArtifact, TopologyReadViewParityReport,
};
use super::report::TopologyReadAggregateReport;
use crate::projection::read_views::domain::TopologyReadLedger;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadProofReport {
    pub(crate) request_aggregate: TopologyReadAggregateReport,
    pub(crate) parity_aggregate: TopologyReadParityAggregateReport,
}

#[derive(Debug, Default)]
pub(crate) struct TopologyReadProofLedger {
    parity_reports: RefCell<Vec<TopologyReadViewParityReport>>,
}

impl TopologyReadProofLedger {
    pub(crate) fn record_parity_report(&self, report: TopologyReadViewParityReport) {
        self.parity_reports.borrow_mut().push(report);
    }

    pub(crate) fn parity_aggregate_report(&self) -> TopologyReadParityAggregateReport {
        TopologyReadParityAggregateReport::from_reports(self.parity_reports.borrow().as_slice())
    }

    pub(crate) fn build_report(
        &self,
        request_aggregate: TopologyReadAggregateReport,
    ) -> TopologyReadProofReport {
        TopologyReadProofReport {
            request_aggregate,
            parity_aggregate: self.parity_aggregate_report(),
        }
    }
}

impl TopologyReadLedger {
    pub(crate) fn record_view_parity(
        &self,
        parity_kind: TopologyReadParityKind,
        left: &TopologyReadViewParityArtifact,
        right: &TopologyReadViewParityArtifact,
    ) -> TopologyReadViewParityReport {
        assert!(
            self.request_reports
                .borrow()
                .iter()
                .any(|report| report.request_family == left.request_family()),
            "topology read parity must be recorded only for a request family observed on this boundary"
        );
        let report = compare_topology_read_view_parity(parity_kind, left, right);
        self.proof_ledger.record_parity_report(report.clone());
        report
    }

    pub fn proof_report(&self) -> TopologyReadProofReport {
        let request_reports = self.request_reports.borrow();
        self.proof_ledger
            .build_report(TopologyReadAggregateReport::from_request_reports(
                request_reports.as_slice(),
            ))
    }
}
