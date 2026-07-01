use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::projection::read_views::domain::{
    TopologyCurrentHeadReadSession, TopologyReadAggregateReport, TopologyReadCloseoutStatus,
    TopologyReadRequestFamily,
};
use crate::query_domain::TopologyCurrentHeadQueryBasisEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TopologyObservedQueryBackedReadFamilyRow {
    request_family: TopologyReadRequestFamily,
    closeout_status: TopologyReadCloseoutStatus,
    closeout_row_digest: String,
    request_count: usize,
    query_execution_count: usize,
    locality_claim_mismatch_count: usize,
    debt_row_count: usize,
    row_scan_fallback_count: usize,
    whole_view_fallback_count: usize,
    repeated_rediscovery_denied_count: usize,
}

pub(crate) struct TopologyQueryBackedReadFamilyRouteInput<'a> {
    observed_family_rows: Vec<TopologyObservedQueryBackedReadFamilyRow>,
    query_basis_evidence: TopologyCurrentHeadQueryBasisEvidence,
    operating_context_identity_digest: String,
    parity_verified_count: usize,
    query_executed_debt_free_family_count: usize,
    debt_family_count: usize,
    equivalence_contract: &'a DerivedEquivalenceContractReport,
}

impl<'a> TopologyQueryBackedReadFamilyRouteInput<'a> {
    pub(crate) fn new(
        session: &TopologyCurrentHeadReadSession<'_>,
        query_basis_evidence: &TopologyCurrentHeadQueryBasisEvidence,
        equivalence_contract: &'a DerivedEquivalenceContractReport,
    ) -> Self {
        let proof_report = session.proof_report();
        let observed_family_rows = observed_family_rows(proof_report.request_aggregate());
        let query_executed_debt_free_family_count = observed_family_rows
            .iter()
            .filter(|row| row.closeout_status() == TopologyReadCloseoutStatus::QueryExecutedDebtFree)
            .count();
        let debt_family_count = observed_family_rows
            .iter()
            .filter(|row| row.debt_row_count() > 0)
            .count();
        Self {
            observed_family_rows,
            query_basis_evidence: query_basis_evidence.clone(),
            operating_context_identity_digest: session
                .operating_context_identity_digest()
                .to_string(),
            parity_verified_count: proof_report
                .parity_aggregate()
                .view_determinism_verified_count(),
            query_executed_debt_free_family_count,
            debt_family_count,
            equivalence_contract,
        }
    }

    pub(crate) fn observed_family_rows(&self) -> &[TopologyObservedQueryBackedReadFamilyRow] {
        &self.observed_family_rows
    }

    pub(crate) fn handle_identity_digest(&self) -> &str {
        self.query_basis_evidence.handle_identity_digest()
    }

    pub(crate) fn support_snapshot_digest(&self) -> &str {
        self.query_basis_evidence.support_snapshot_digest()
    }

    pub(crate) fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub(crate) fn parity_verified_count(&self) -> usize {
        self.parity_verified_count
    }

    pub(crate) fn query_executed_debt_free_family_count(&self) -> usize {
        self.query_executed_debt_free_family_count
    }

    pub(crate) fn debt_family_count(&self) -> usize {
        self.debt_family_count
    }

    pub(crate) fn equivalence_contract(&self) -> &'a DerivedEquivalenceContractReport {
        self.equivalence_contract
    }
}

impl TopologyObservedQueryBackedReadFamilyRow {
    pub(crate) fn request_family(&self) -> TopologyReadRequestFamily {
        self.request_family
    }

    pub(crate) fn closeout_status(&self) -> TopologyReadCloseoutStatus {
        self.closeout_status
    }

    pub(crate) fn closeout_row_digest(&self) -> &str {
        &self.closeout_row_digest
    }

    pub(crate) fn request_count(&self) -> usize {
        self.request_count
    }

    pub(crate) fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub(crate) fn locality_claim_mismatch_count(&self) -> usize {
        self.locality_claim_mismatch_count
    }

    pub(crate) fn debt_row_count(&self) -> usize {
        self.debt_row_count
    }

    pub(crate) fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub(crate) fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub(crate) fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }
}

fn observed_family_rows(
    request_aggregate: &TopologyReadAggregateReport,
) -> Vec<TopologyObservedQueryBackedReadFamilyRow> {
    TopologyReadRequestFamily::ALL
        .into_iter()
        .map(|request_family| observed_family_row(request_aggregate, request_family))
        .collect()
}

fn observed_family_row(
    request_aggregate: &TopologyReadAggregateReport,
    request_family: TopologyReadRequestFamily,
) -> TopologyObservedQueryBackedReadFamilyRow {
    let family_aggregate = request_aggregate
        .family_rows()
        .iter()
        .find(|row| row.request_family() == request_family);
    let request_count = family_aggregate.map_or(0, |row| row.request_count());
    let query_execution_count = family_aggregate.map_or(0, |row| row.query_execution_count());
    let row_scan_fallback_count =
        family_aggregate.map_or(0, |row| row.row_scan_fallback_count());
    let whole_view_fallback_count =
        family_aggregate.map_or(0, |row| row.whole_view_fallback_count());
    let repeated_rediscovery_denied_count = family_aggregate
        .map_or(0, |row| row.repeated_rediscovery_denied_count());
    let debt_row_count = request_aggregate
        .debt_rows()
        .iter()
        .filter(|row| row.request_family() == request_family)
        .count();
    let locality_claim_mismatch_count = request_aggregate
        .execution_rows()
        .iter()
        .filter(|row| {
            row.request_family() == request_family
                && row.executed_scope_class() != Some(row.claimed_scope_class())
        })
        .map(|row| row.request_count())
        .sum();
    let closeout_status = if request_count == 0 {
        TopologyReadCloseoutStatus::Unobserved
    } else if query_execution_count != request_count {
        TopologyReadCloseoutStatus::ExecutionGap
    } else if debt_row_count > 0
        || row_scan_fallback_count > 0
        || whole_view_fallback_count > 0
        || repeated_rediscovery_denied_count > 0
        || locality_claim_mismatch_count > 0
    {
        TopologyReadCloseoutStatus::QueryExecutedWithDebt
    } else {
        TopologyReadCloseoutStatus::QueryExecutedDebtFree
    };
    TopologyObservedQueryBackedReadFamilyRow {
        request_family,
        closeout_status,
        closeout_row_digest: format!(
            "request_family={request_family:?};status={closeout_status:?};request_count={request_count};query_execution_count={query_execution_count};locality_claim_mismatch_count={locality_claim_mismatch_count};debt_row_count={debt_row_count};row_scan_fallback_count={row_scan_fallback_count};whole_view_fallback_count={whole_view_fallback_count};repeated_rediscovery_denied_count={repeated_rediscovery_denied_count}",
        ),
        request_count,
        query_execution_count,
        locality_claim_mismatch_count,
        debt_row_count,
        row_scan_fallback_count,
        whole_view_fallback_count,
        repeated_rediscovery_denied_count,
    }
}
