use super::closeout::TopologyDomainQueryCloseoutReport;
use super::closeout::{
    TopologyDomainQueryCloseoutRow, TopologyDomainQueryCloseoutStatus,
    TopologyDomainQueryPhaseThreeBlocker, TopologyDomainQueryPhaseThreeBlockerRow,
    TopologyDomainQueryPhaseThreeBlockerStatus,
};
use super::ledger::TopologyDomainQueryProofReport;
use super::no_n_plus_one::{
    TopologyNoNPlusOneContract, TopologyNoNPlusOneContractRow, TopologyNoNPlusOneContractStatus,
};
use super::parity::{
    TopologyDomainQueryParityAggregateReport, TopologyDomainQueryParityAggregateRow,
    TopologyDomainQueryParityKind,
};
use super::report::{TopologyDomainQueryAggregateReport, TopologyDomainQueryRequestFamily};

impl TopologyDomainQueryParityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Replay => "replay",
            Self::BranchLocal => "branch_local",
        }
    }
}

impl TopologyDomainQueryParityAggregateRow {
    pub fn parity_kind(&self) -> TopologyDomainQueryParityKind {
        self.parity_kind
    }

    pub fn request_family(&self) -> TopologyDomainQueryRequestFamily {
        self.request_family
    }

    pub fn checked_count(&self) -> usize {
        self.checked_count
    }

    pub fn verified_count(&self) -> usize {
        self.verified_count
    }
}

impl TopologyDomainQueryParityAggregateReport {
    pub fn domain_query_parity_count(&self) -> usize {
        self.domain_query_parity_count
    }

    pub fn view_determinism_checked_count(&self) -> usize {
        self.view_determinism_checked_count
    }

    pub fn view_determinism_verified_count(&self) -> usize {
        self.view_determinism_verified_count
    }

    pub fn replay_checked_count(&self) -> usize {
        self.replay_checked_count
    }

    pub fn replay_verified_count(&self) -> usize {
        self.replay_verified_count
    }

    pub fn branch_local_checked_count(&self) -> usize {
        self.branch_local_checked_count
    }

    pub fn branch_local_verified_count(&self) -> usize {
        self.branch_local_verified_count
    }

    pub fn parity_rows(&self) -> &[TopologyDomainQueryParityAggregateRow] {
        self.parity_rows.as_slice()
    }
}

impl TopologyDomainQueryProofReport {
    pub fn request_aggregate(&self) -> &TopologyDomainQueryAggregateReport {
        &self.request_aggregate
    }

    pub fn parity_aggregate(&self) -> &TopologyDomainQueryParityAggregateReport {
        &self.parity_aggregate
    }
}

impl TopologyDomainQueryCloseoutReport {
    pub fn proof_report(&self) -> &TopologyDomainQueryProofReport {
        &self.proof_report
    }

    pub fn query_executed_family_count(&self) -> usize {
        self.query_executed_family_count
    }

    pub fn query_executed_debt_free_family_count(&self) -> usize {
        self.query_executed_debt_free_family_count
    }

    pub fn query_executed_debt_backed_family_count(&self) -> usize {
        self.query_executed_debt_backed_family_count
    }

    pub fn debt_family_count(&self) -> usize {
        self.debt_family_count
    }

    pub fn whole_view_debt_request_count(&self) -> usize {
        self.whole_view_debt_request_count
    }

    pub fn row_scan_fallback_request_count(&self) -> usize {
        self.row_scan_fallback_request_count
    }

    pub fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }

    pub fn family_rows(&self) -> &[TopologyDomainQueryCloseoutRow] {
        self.family_rows.as_slice()
    }

    pub fn status(
        &self,
        request_family: TopologyDomainQueryRequestFamily,
    ) -> TopologyDomainQueryCloseoutStatus {
        self.family_rows
            .iter()
            .find(|row| row.request_family == request_family)
            .map(TopologyDomainQueryCloseoutRow::status)
            .unwrap_or_else(|| {
                panic!("domain query closeout rows should cover every declared request family")
            })
    }

    pub fn phase_three_blocker_rows(&self) -> &[TopologyDomainQueryPhaseThreeBlockerRow] {
        self.phase_three_blocker_rows.as_slice()
    }

    pub fn phase_three_blocker_status(
        &self,
        blocker: TopologyDomainQueryPhaseThreeBlocker,
    ) -> TopologyDomainQueryPhaseThreeBlockerStatus {
        self.phase_three_blocker_rows
            .iter()
            .find(|row| row.blocker == blocker)
            .map(TopologyDomainQueryPhaseThreeBlockerRow::status)
            .unwrap_or_else(|| {
                panic!("domain query closeout blocker rows should cover every declared blocker")
            })
    }

    pub fn no_n_plus_one_contract_rows(&self) -> &[TopologyNoNPlusOneContractRow] {
        self.no_n_plus_one_contract_rows.as_slice()
    }

    pub fn no_n_plus_one_contract_status(
        &self,
        contract: TopologyNoNPlusOneContract,
    ) -> TopologyNoNPlusOneContractStatus {
        self.no_n_plus_one_contract_rows
            .iter()
            .find(|row| row.contract == contract)
            .map(TopologyNoNPlusOneContractRow::status)
            .unwrap_or_else(|| {
                panic!("domain query no-N-plus-one rows should cover every declared contract")
            })
    }

    pub fn phase_three_ready(&self) -> bool {
        self.phase_three_ready
    }
}

impl TopologyDomainQueryCloseoutStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unobserved => "unobserved",
            Self::ExecutionGap => "execution_gap",
            Self::QueryExecutedWithDebt => "query_executed_with_debt",
            Self::QueryExecutedDebtFree => "query_executed_debt_free",
        }
    }
}

impl TopologyDomainQueryCloseoutRow {
    pub fn request_family(&self) -> TopologyDomainQueryRequestFamily {
        self.request_family
    }

    pub fn status(&self) -> TopologyDomainQueryCloseoutStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        self.reason.as_str()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }

    pub fn request_count(&self) -> usize {
        self.request_count
    }

    pub fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub fn locality_claim_mismatch_count(&self) -> usize {
        self.locality_claim_mismatch_count
    }

    pub fn debt_row_count(&self) -> usize {
        self.debt_row_count
    }

    pub fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }
}

impl TopologyDomainQueryPhaseThreeBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoObservedRequests => "no_observed_requests",
            Self::NonQueryRuntimeExecution => "non_query_runtime_execution",
            Self::LocalityClaimMismatch => "locality_claim_mismatch",
            Self::RowScanFallback => "row_scan_fallback",
            Self::WholeViewDebt => "whole_view_debt",
            Self::RepeatedRediscoveryDenial => "repeated_rediscovery_denial",
            Self::OutstandingDebtRows => "outstanding_debt_rows",
            Self::ParityDeterminismGap => "parity_determinism_gap",
        }
    }
}

impl TopologyDomainQueryPhaseThreeBlockerStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Blocked => "blocked",
        }
    }
}

impl TopologyDomainQueryPhaseThreeBlockerRow {
    pub fn blocker(&self) -> TopologyDomainQueryPhaseThreeBlocker {
        self.blocker
    }

    pub fn status(&self) -> TopologyDomainQueryPhaseThreeBlockerStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        self.reason.as_str()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl TopologyNoNPlusOneContractStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Blocked => "blocked",
        }
    }
}

impl TopologyNoNPlusOneContractRow {
    pub fn contract(&self) -> TopologyNoNPlusOneContract {
        self.contract
    }

    pub fn status(&self) -> TopologyNoNPlusOneContractStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        self.reason.as_str()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
