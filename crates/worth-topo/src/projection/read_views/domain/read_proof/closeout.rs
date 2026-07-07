use super::ledger::TopologyReadProofReport;
use super::no_n_plus_one::{
    no_n_plus_one_contract_rows, TopologyNoNPlusOneContractRow, TopologyNoNPlusOneContractStatus,
};
use super::report::TopologyReadAggregateReport;
use super::report::TopologyReadRequestFamily;
use crate::projection::read_views::domain::TopologyReadLedger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyReadCloseoutStatus {
    Unobserved,
    ExecutionGap,
    QueryExecutedWithDebt,
    QueryExecutedDebtFree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadCloseoutRow {
    pub(crate) request_family: TopologyReadRequestFamily,
    pub(crate) status: TopologyReadCloseoutStatus,
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
pub enum TopologyReadPhaseThreeBlocker {
    NoObservedRequests,
    NonQueryRuntimeExecution,
    LocalityClaimMismatch,
    RowScanFallback,
    WholeViewDebt,
    RepeatedRediscoveryDenial,
    OutstandingDebtRows,
    ParityDeterminismGap,
}

impl TopologyReadPhaseThreeBlocker {
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
pub enum TopologyReadPhaseThreeBlockerStatus {
    Clear,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadPhaseThreeBlockerRow {
    pub(crate) blocker: TopologyReadPhaseThreeBlocker,
    pub(crate) status: TopologyReadPhaseThreeBlockerStatus,
    pub(crate) reason: String,
    pub(crate) row_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadCloseoutReport {
    pub(crate) proof_report: TopologyReadProofReport,
    pub(crate) query_executed_family_count: usize,
    pub(crate) query_executed_debt_free_family_count: usize,
    pub(crate) query_executed_debt_backed_family_count: usize,
    pub(crate) debt_family_count: usize,
    pub(crate) whole_view_debt_request_count: usize,
    pub(crate) row_scan_fallback_request_count: usize,
    pub(crate) repeated_rediscovery_denied_count: usize,
    pub(crate) family_rows: Vec<TopologyReadCloseoutRow>,
    pub(crate) phase_three_blocker_rows: Vec<TopologyReadPhaseThreeBlockerRow>,
    pub(crate) no_n_plus_one_contract_rows: Vec<TopologyNoNPlusOneContractRow>,
    pub(crate) phase_three_ready: bool,
}

impl TopologyReadCloseoutReport {
    pub(crate) fn from_proof_report(proof_report: TopologyReadProofReport) -> Self {
        let request_aggregate = &proof_report.request_aggregate;
        let family_rows = closeout_family_rows(request_aggregate);
        let query_executed_family_count = request_aggregate
            .family_rows
            .iter()
            .filter(|row| row.query_execution_count == row.request_count)
            .count();
        let query_executed_debt_free_family_count = family_rows
            .iter()
            .filter(|row| row.status == TopologyReadCloseoutStatus::QueryExecutedDebtFree)
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
    blocker_rows: &[TopologyReadPhaseThreeBlockerRow],
    no_n_plus_one_rows: &[TopologyNoNPlusOneContractRow],
) -> bool {
    blocker_rows
        .iter()
        .all(|row| row.status == TopologyReadPhaseThreeBlockerStatus::Clear)
        && no_n_plus_one_rows
            .iter()
            .all(|row| row.status == TopologyNoNPlusOneContractStatus::Satisfied)
}

impl TopologyReadLedger {
    pub fn closeout_report(&self) -> TopologyReadCloseoutReport {
        TopologyReadCloseoutReport::from_proof_report(self.proof_report())
    }
}

fn closeout_family_rows(
    request_aggregate: &TopologyReadAggregateReport,
) -> Vec<TopologyReadCloseoutRow> {
    TopologyReadRequestFamily::ALL
        .into_iter()
        .map(|request_family| closeout_family_row(request_aggregate, request_family))
        .collect()
}

fn closeout_family_row(
    request_aggregate: &TopologyReadAggregateReport,
    request_family: TopologyReadRequestFamily,
) -> TopologyReadCloseoutRow {
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
            TopologyReadCloseoutStatus::Unobserved,
            "no executed requests were observed for this public topology-domain family".to_string(),
        )
    } else if query_execution_count != request_count {
        (
            TopologyReadCloseoutStatus::ExecutionGap,
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
            TopologyReadCloseoutStatus::QueryExecutedWithDebt,
            format!(
                "this public topology-domain family executed through the query runtime but still carries debt signals (debt_rows={debt_row_count};row_scan_fallback={row_scan_fallback_count};whole_view_fallback={whole_view_fallback_count};repeated_rediscovery_denied={repeated_rediscovery_denied_count};locality_claim_mismatch={locality_claim_mismatch_count})"
            ),
        )
    } else {
        (
            TopologyReadCloseoutStatus::QueryExecutedDebtFree,
            format!(
                "this public topology-domain family executed through the query runtime without observed debt signals ({query_execution_count}/{request_count} executions)"
            ),
        )
    };
    TopologyReadCloseoutRow {
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
    request_family: TopologyReadRequestFamily,
    status: TopologyReadCloseoutStatus,
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
    request_aggregate: &TopologyReadAggregateReport,
    proof_report: &TopologyReadProofReport,
) -> Vec<TopologyReadPhaseThreeBlockerRow> {
    TopologyReadPhaseThreeBlocker::ALL
        .into_iter()
        .map(|blocker| phase_three_blocker_row(blocker, request_aggregate, proof_report))
        .collect()
}

fn phase_three_blocker_row(
    blocker: TopologyReadPhaseThreeBlocker,
    request_aggregate: &TopologyReadAggregateReport,
    proof_report: &TopologyReadProofReport,
) -> TopologyReadPhaseThreeBlockerRow {
    let (status, reason) = match blocker {
        TopologyReadPhaseThreeBlocker::NoObservedRequests => {
            blocker_from_condition(
                request_aggregate.request_count == 0,
                "no executed topology-domain read requests were observed on this boundary",
                "at least one executed topology-domain read request was observed on this boundary",
            )
        }
        TopologyReadPhaseThreeBlocker::NonQueryRuntimeExecution => {
            blocker_from_condition(
                request_aggregate.query_execution_count != request_aggregate.request_count,
                "one or more observed topology-domain requests were not executed through the query runtime",
                "all observed topology-domain requests executed through the query runtime",
            )
        }
        TopologyReadPhaseThreeBlocker::LocalityClaimMismatch => blocker_from_condition(
            request_aggregate.locality_claim_mismatch_count > 0,
            "one or more observed topology-domain requests executed under a different scope class than their claimed family posture",
            "all observed topology-domain requests executed under their claimed scope class",
        ),
        TopologyReadPhaseThreeBlocker::RowScanFallback => blocker_from_condition(
            request_aggregate.row_scan_fallback_count > 0,
            "one or more observed topology-domain requests incurred row-scan fallback debt",
            "no observed topology-domain requests incurred row-scan fallback debt",
        ),
        TopologyReadPhaseThreeBlocker::WholeViewDebt => blocker_from_condition(
            request_aggregate.whole_view_fallback_count > 0,
            "one or more observed topology-domain requests incurred whole-view fallback debt",
            "no observed topology-domain requests incurred whole-view fallback debt",
        ),
        TopologyReadPhaseThreeBlocker::RepeatedRediscoveryDenial => {
            blocker_from_condition(
                request_aggregate.repeated_rediscovery_denied_count > 0,
                "one or more observed topology-domain requests were denied by repeated-rediscovery debt",
                "no observed topology-domain requests were denied by repeated-rediscovery debt",
            )
        }
        TopologyReadPhaseThreeBlocker::OutstandingDebtRows => blocker_from_condition(
            !request_aggregate.debt_rows.is_empty(),
            "the executed topology-domain aggregate still exposes outstanding debt rows",
            "the executed topology-domain aggregate exposes no outstanding debt rows",
        ),
        TopologyReadPhaseThreeBlocker::ParityDeterminismGap => blocker_from_condition(
            proof_report.parity_aggregate.view_determinism_checked_count
                != proof_report.parity_aggregate.view_determinism_verified_count,
            "one or more checked topology-domain parity views have not been determinism-verified",
            "all checked topology-domain parity views were determinism-verified",
        ),
    };
    TopologyReadPhaseThreeBlockerRow {
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
) -> (TopologyReadPhaseThreeBlockerStatus, &'static str) {
    if blocked {
        (TopologyReadPhaseThreeBlockerStatus::Blocked, blocked_reason)
    } else {
        (TopologyReadPhaseThreeBlockerStatus::Clear, clear_reason)
    }
}
