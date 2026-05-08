use super::super::topology::TopologyDomainQuery;
use super::ledger::TopologyDomainQueryProofReport;
use super::no_n_plus_one::{
    no_n_plus_one_contract_rows, TopologyNoNPlusOneContractRow, TopologyNoNPlusOneContractStatus,
};
use super::report::TopologyDomainQueryAggregateReport;
use super::report::TopologyDomainQueryRequestFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyDomainQueryCloseoutStatus {
    Unobserved,
    ExecutionGap,
    QueryExecutedWithDebt,
    QueryExecutedDebtFree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyDomainQueryCloseoutRow {
    pub(crate) request_family: TopologyDomainQueryRequestFamily,
    pub(crate) status: TopologyDomainQueryCloseoutStatus,
    pub(crate) reason: String,
    pub(crate) row_digest: String,
    pub(crate) request_count: usize,
    pub(crate) query_execution_count: usize,
    pub(crate) locality_claim_mismatch_count: usize,
    pub(crate) debt_row_count: usize,
    pub(crate) row_scan_fallback_count: usize,
    pub(crate) whole_view_fallback_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyDomainQueryPhaseThreeBlocker {
    NoObservedRequests,
    NonQueryRuntimeExecution,
    LocalityClaimMismatch,
    RowScanFallback,
    WholeViewDebt,
    RepeatedRediscoveryDenial,
    OutstandingDebtRows,
    ParityDeterminismGap,
}

impl TopologyDomainQueryPhaseThreeBlocker {
    pub const ALL: [Self; 8] = [
        Self::NoObservedRequests,
        Self::NonQueryRuntimeExecution,
        Self::LocalityClaimMismatch,
        Self::RowScanFallback,
        Self::WholeViewDebt,
        Self::RepeatedRediscoveryDenial,
        Self::OutstandingDebtRows,
        Self::ParityDeterminismGap,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyDomainQueryPhaseThreeBlockerStatus {
    Clear,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyDomainQueryPhaseThreeBlockerRow {
    pub(crate) blocker: TopologyDomainQueryPhaseThreeBlocker,
    pub(crate) status: TopologyDomainQueryPhaseThreeBlockerStatus,
    pub(crate) reason: String,
    pub(crate) row_digest: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyDomainQueryCloseoutReport {
    pub(crate) proof_report: TopologyDomainQueryProofReport,
    pub(crate) query_executed_family_count: usize,
    pub(crate) query_executed_debt_free_family_count: usize,
    pub(crate) query_executed_debt_backed_family_count: usize,
    pub(crate) debt_family_count: usize,
    pub(crate) whole_view_debt_request_count: usize,
    pub(crate) row_scan_fallback_request_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
    pub(crate) family_rows: Vec<TopologyDomainQueryCloseoutRow>,
    pub(crate) phase_three_blocker_rows: Vec<TopologyDomainQueryPhaseThreeBlockerRow>,
    pub(crate) no_n_plus_one_contract_rows: Vec<TopologyNoNPlusOneContractRow>,
    pub(crate) phase_three_ready: bool,
}

impl TopologyDomainQueryCloseoutReport {
    #[allow(dead_code)]
    pub(crate) fn from_proof_report(proof_report: TopologyDomainQueryProofReport) -> Self {
        let request_aggregate = &proof_report.request_aggregate;
        let family_rows = closeout_family_rows(request_aggregate);
        let query_executed_family_count = request_aggregate
            .family_rows
            .iter()
            .filter(|row| row.query_execution_count == row.request_count)
            .count();
        let query_executed_debt_free_family_count = family_rows
            .iter()
            .filter(|row| row.status == TopologyDomainQueryCloseoutStatus::QueryExecutedDebtFree)
            .count();
        let query_executed_debt_backed_family_count =
            query_executed_family_count - query_executed_debt_free_family_count;
        let debt_family_count = request_aggregate.debt_rows.len();
        let whole_view_debt_request_count = request_aggregate.whole_view_fallback_count;
        let row_scan_fallback_request_count = request_aggregate.row_scan_fallback_count;
        let repeated_rediscovery_denied_count = request_aggregate.repeated_rediscovery_denied_count;
        let phase_three_blocker_rows = phase_three_blocker_rows(request_aggregate, &proof_report);
        let no_n_plus_one_contract_rows = no_n_plus_one_contract_rows(&proof_report);
        let phase_three_ready =
            phase_three_ready(&phase_three_blocker_rows, &no_n_plus_one_contract_rows);
        Self {
            proof_report,
            query_executed_family_count,
            query_executed_debt_free_family_count,
            query_executed_debt_backed_family_count,
            debt_family_count,
            whole_view_debt_request_count,
            row_scan_fallback_request_count,
            repeated_rediscovery_denied_count,
            family_rows,
            phase_three_blocker_rows,
            no_n_plus_one_contract_rows,
            phase_three_ready,
        }
    }
}

fn phase_three_ready(
    blocker_rows: &[TopologyDomainQueryPhaseThreeBlockerRow],
    no_n_plus_one_rows: &[TopologyNoNPlusOneContractRow],
) -> bool {
    blocker_rows
        .iter()
        .all(|row| row.status == TopologyDomainQueryPhaseThreeBlockerStatus::Clear)
        && no_n_plus_one_rows
            .iter()
            .all(|row| row.status == TopologyNoNPlusOneContractStatus::Satisfied)
}

impl TopologyDomainQuery {
    #[allow(dead_code)]
    pub fn closeout_report(&self) -> TopologyDomainQueryCloseoutReport {
        TopologyDomainQueryCloseoutReport::from_proof_report(self.proof_report())
    }
}

fn closeout_family_rows(
    request_aggregate: &TopologyDomainQueryAggregateReport,
) -> Vec<TopologyDomainQueryCloseoutRow> {
    TopologyDomainQueryRequestFamily::ALL
        .into_iter()
        .map(|request_family| closeout_family_row(request_aggregate, request_family))
        .collect()
}

fn closeout_family_row(
    request_aggregate: &TopologyDomainQueryAggregateReport,
    request_family: TopologyDomainQueryRequestFamily,
) -> TopologyDomainQueryCloseoutRow {
    let family_aggregate = request_aggregate
        .family_rows
        .iter()
        .find(|row| row.request_family == request_family);
    let request_count = family_aggregate.map_or(0, |row| row.request_count);
    let query_execution_count = family_aggregate.map_or(0, |row| row.query_execution_count);
    let row_scan_fallback_count = family_aggregate.map_or(0, |row| row.row_scan_fallback_count);
    let whole_view_fallback_count = family_aggregate.map_or(0, |row| row.whole_view_fallback_count);
    let repeated_rediscovery_denied_count =
        family_aggregate.map_or(0, |row| row.repeated_rediscovery_denied_count);
    let debt_row_count = request_aggregate
        .debt_rows
        .iter()
        .filter(|row| row.request_family == request_family)
        .count();
    let locality_claim_mismatch_count = request_aggregate
        .execution_rows
        .iter()
        .filter(|row| {
            row.request_family == request_family
                && row.executed_scope_class != Some(row.claimed_scope_class.clone())
        })
        .map(|row| row.request_count)
        .sum();
    let (status, reason) = if request_count == 0 {
        (
            TopologyDomainQueryCloseoutStatus::Unobserved,
            "no executed requests were observed for this public topology-domain family".to_string(),
        )
    } else if query_execution_count != request_count {
        (
            TopologyDomainQueryCloseoutStatus::ExecutionGap,
            format!(
                "observed requests for this public topology-domain family outnumber query-runtime executions ({query_execution_count}/{request_count})"
            ),
        )
    } else if debt_row_count > 0
        || row_scan_fallback_count > 0
        || whole_view_fallback_count > 0
        || repeated_rediscovery_denied_count > 0
        || locality_claim_mismatch_count > 0
    {
        (
            TopologyDomainQueryCloseoutStatus::QueryExecutedWithDebt,
            format!(
                "this public topology-domain family executed through the query runtime but still carries debt signals (debt_rows={debt_row_count};row_scan_fallback={row_scan_fallback_count};whole_view_fallback={whole_view_fallback_count};repeated_rediscovery_denied={repeated_rediscovery_denied_count};locality_claim_mismatch={locality_claim_mismatch_count})"
            ),
        )
    } else {
        (
            TopologyDomainQueryCloseoutStatus::QueryExecutedDebtFree,
            format!(
                "this public topology-domain family executed through the query runtime without observed debt signals ({query_execution_count}/{request_count} executions)"
            ),
        )
    };
    TopologyDomainQueryCloseoutRow {
        request_family,
        status,
        row_digest: closeout_family_row_digest(
            request_family,
            status,
            request_count,
            query_execution_count,
            locality_claim_mismatch_count,
            debt_row_count,
            row_scan_fallback_count,
            whole_view_fallback_count,
            repeated_rediscovery_denied_count,
        ),
        reason,
        request_count,
        query_execution_count,
        locality_claim_mismatch_count,
        debt_row_count,
        row_scan_fallback_count,
        whole_view_fallback_count,
        repeated_rediscovery_denied_count,
    }
}

fn closeout_family_row_digest(
    request_family: TopologyDomainQueryRequestFamily,
    status: TopologyDomainQueryCloseoutStatus,
    request_count: usize,
    query_execution_count: usize,
    locality_claim_mismatch_count: usize,
    debt_row_count: usize,
    row_scan_fallback_count: usize,
    whole_view_fallback_count: usize,
    repeated_rediscovery_denied_count: usize,
) -> String {
    format!(
        "request_family={request_family:?};status={status:?};request_count={request_count};query_execution_count={query_execution_count};locality_claim_mismatch_count={locality_claim_mismatch_count};debt_row_count={debt_row_count};row_scan_fallback_count={row_scan_fallback_count};whole_view_fallback_count={whole_view_fallback_count};repeated_rediscovery_denied_count={repeated_rediscovery_denied_count}",
    )
}

fn phase_three_blocker_rows(
    request_aggregate: &TopologyDomainQueryAggregateReport,
    proof_report: &TopologyDomainQueryProofReport,
) -> Vec<TopologyDomainQueryPhaseThreeBlockerRow> {
    TopologyDomainQueryPhaseThreeBlocker::ALL
        .into_iter()
        .map(|blocker| phase_three_blocker_row(blocker, request_aggregate, proof_report))
        .collect()
}

fn phase_three_blocker_row(
    blocker: TopologyDomainQueryPhaseThreeBlocker,
    request_aggregate: &TopologyDomainQueryAggregateReport,
    proof_report: &TopologyDomainQueryProofReport,
) -> TopologyDomainQueryPhaseThreeBlockerRow {
    let (status, reason) = match blocker {
        TopologyDomainQueryPhaseThreeBlocker::NoObservedRequests => {
            blocker_from_condition(
                request_aggregate.request_count == 0,
                "no executed topology-domain read requests were observed on this boundary",
                "at least one executed topology-domain read request was observed on this boundary",
            )
        }
        TopologyDomainQueryPhaseThreeBlocker::NonQueryRuntimeExecution => {
            blocker_from_condition(
                request_aggregate.query_execution_count != request_aggregate.request_count,
                "one or more observed topology-domain requests were not executed through the query runtime",
                "all observed topology-domain requests executed through the query runtime",
            )
        }
        TopologyDomainQueryPhaseThreeBlocker::LocalityClaimMismatch => blocker_from_condition(
            request_aggregate.locality_claim_mismatch_count > 0,
            "one or more observed topology-domain requests executed under a different scope class than their claimed family posture",
            "all observed topology-domain requests executed under their claimed scope class",
        ),
        TopologyDomainQueryPhaseThreeBlocker::RowScanFallback => blocker_from_condition(
            request_aggregate.row_scan_fallback_count > 0,
            "one or more observed topology-domain requests incurred row-scan fallback debt",
            "no observed topology-domain requests incurred row-scan fallback debt",
        ),
        TopologyDomainQueryPhaseThreeBlocker::WholeViewDebt => blocker_from_condition(
            request_aggregate.whole_view_fallback_count > 0,
            "one or more observed topology-domain requests incurred whole-view fallback debt",
            "no observed topology-domain requests incurred whole-view fallback debt",
        ),
        TopologyDomainQueryPhaseThreeBlocker::RepeatedRediscoveryDenial => {
            blocker_from_condition(
                request_aggregate.repeated_rediscovery_denied_count > 0,
                "one or more observed topology-domain requests were denied by repeated-rediscovery debt",
                "no observed topology-domain requests were denied by repeated-rediscovery debt",
            )
        }
        TopologyDomainQueryPhaseThreeBlocker::OutstandingDebtRows => blocker_from_condition(
            !request_aggregate.debt_rows.is_empty(),
            "the executed topology-domain aggregate still exposes outstanding debt rows",
            "the executed topology-domain aggregate exposes no outstanding debt rows",
        ),
        TopologyDomainQueryPhaseThreeBlocker::ParityDeterminismGap => blocker_from_condition(
            proof_report.parity_aggregate.view_determinism_checked_count
                != proof_report.parity_aggregate.view_determinism_verified_count,
            "one or more checked topology-domain parity views have not been determinism-verified",
            "all checked topology-domain parity views were determinism-verified",
        ),
    };
    TopologyDomainQueryPhaseThreeBlockerRow {
        blocker,
        status,
        row_digest: format!("blocker={blocker:?};status={status:?};reason={reason}"),
        reason: reason.to_string(),
    }
}

fn blocker_from_condition(
    blocked: bool,
    blocked_reason: &'static str,
    clear_reason: &'static str,
) -> (TopologyDomainQueryPhaseThreeBlockerStatus, &'static str) {
    if blocked {
        (
            TopologyDomainQueryPhaseThreeBlockerStatus::Blocked,
            blocked_reason,
        )
    } else {
        (
            TopologyDomainQueryPhaseThreeBlockerStatus::Clear,
            clear_reason,
        )
    }
}
