use super::ledger::TopologyReadProofReport;
use super::report::TopologyReadAggregateReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyNoNPlusOneContract {
    LoweringBreadth,
    FallbackPosture,
    ViewParity,
    RelationshipProofPosture,
}

impl TopologyNoNPlusOneContract {
    pub const ALL: [Self; 4] = [
        Self::LoweringBreadth,
        Self::FallbackPosture,
        Self::ViewParity,
        Self::RelationshipProofPosture,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::LoweringBreadth => "topology_read_lowering_breadth",
            Self::FallbackPosture => "topology_read_fallback_posture",
            Self::ViewParity => "topology_read_view_parity",
            Self::RelationshipProofPosture => "topology_read_relationship_proof_posture",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyNoNPlusOneContractStatus {
    Satisfied,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyNoNPlusOneContractRow {
    pub(crate) contract: TopologyNoNPlusOneContract,
    pub(crate) status: TopologyNoNPlusOneContractStatus,
    pub(crate) reason: String,
    pub(crate) row_digest: String,
}

pub(crate) fn no_n_plus_one_contract_rows(
    proof_report: &TopologyReadProofReport,
) -> Vec<TopologyNoNPlusOneContractRow> {
    TopologyNoNPlusOneContract::ALL
        .into_iter()
        .map(|contract| no_n_plus_one_contract_row(contract, proof_report))
        .collect()
}

fn no_n_plus_one_contract_row(
    contract: TopologyNoNPlusOneContract,
    proof_report: &TopologyReadProofReport,
) -> TopologyNoNPlusOneContractRow {
    let request_aggregate = &proof_report.request_aggregate;
    let (status, reason) = match contract {
        TopologyNoNPlusOneContract::LoweringBreadth => lowering_breadth_status(request_aggregate),
        TopologyNoNPlusOneContract::FallbackPosture => fallback_posture_status(request_aggregate),
        TopologyNoNPlusOneContract::ViewParity => view_parity_status(proof_report),
        TopologyNoNPlusOneContract::RelationshipProofPosture => contract_from_condition(
            request_aggregate.lowered_traversal_count > 0
                && request_aggregate.relationship_proof_admission_count
                    >= request_aggregate.lowered_traversal_count,
            "relationship-proof admission covered every lowered traversal",
            "lowered traversal breadth is not fully covered by relationship-proof admission",
        ),
    };
    TopologyNoNPlusOneContractRow {
        contract,
        status,
        row_digest: format!(
            "contract={};status={status:?};reason={reason}",
            contract.as_str()
        ),
        reason: reason.to_string(),
    }
}

fn lowering_breadth_status(
    request_aggregate: &TopologyReadAggregateReport,
) -> (TopologyNoNPlusOneContractStatus, &'static str) {
    contract_from_condition(
        request_aggregate.request_count > 0
            && request_aggregate.query_execution_count == request_aggregate.request_count
            && request_aggregate.locality_claim_mismatch_count == 0,
        "all observed topology-domain reads expose query-native execution and exact scope-class breadth",
        "topology-domain read breadth is unobserved, non-query-native, or scope-mismatched",
    )
}

fn view_parity_status(
    proof_report: &TopologyReadProofReport,
) -> (TopologyNoNPlusOneContractStatus, &'static str) {
    let parity_aggregate = &proof_report.parity_aggregate;
    contract_from_condition(
        parity_aggregate.replay_checked_count > 0
            && parity_aggregate.replay_checked_count == parity_aggregate.replay_verified_count
            && parity_aggregate.branch_local_checked_count > 0
            && parity_aggregate.branch_local_checked_count
                == parity_aggregate.branch_local_verified_count
            && parity_aggregate.view_determinism_checked_count
                == parity_aggregate.view_determinism_verified_count,
        "replay and branch-local decoded-view parity were observed and determinism-verified",
        "replay or branch-local decoded-view parity is missing or not fully deterministic",
    )
}

fn fallback_posture_status(
    request_aggregate: &TopologyReadAggregateReport,
) -> (TopologyNoNPlusOneContractStatus, &'static str) {
    contract_from_condition(
        request_aggregate.row_scan_fallback_count == 0
            && request_aggregate.whole_view_fallback_count == 0
            && request_aggregate.repeated_rediscovery_denied_count == 0
            && request_aggregate.debt_rows.is_empty(),
        "no row-scan, whole-view, repeated-rediscovery, or explicit debt fallback was observed",
        "one or more topology-domain reads still expose fallback or rediscovery debt",
    )
}

fn contract_from_condition(
    satisfied: bool,
    satisfied_reason: &'static str,
    blocked_reason: &'static str,
) -> (TopologyNoNPlusOneContractStatus, &'static str) {
    if satisfied {
        (
            TopologyNoNPlusOneContractStatus::Satisfied,
            satisfied_reason,
        )
    } else {
        (TopologyNoNPlusOneContractStatus::Blocked, blocked_reason)
    }
}
