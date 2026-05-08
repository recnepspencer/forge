use topology::facade::{
    TopologyDomainQueryCloseoutRow, TopologyDomainQueryCloseoutStatus,
    TopologyDomainQueryRequestFamily,
};

fn main() {
    let _ = TopologyDomainQueryCloseoutRow {
        request_family: TopologyDomainQueryRequestFamily::LoopCycleNeighborhood,
        status: TopologyDomainQueryCloseoutStatus::QueryExecutedDebtFree,
        request_count: 1,
        query_execution_count: 1,
        locality_claim_mismatch_count: 0,
        debt_row_count: 0,
        row_scan_fallback_count: 0,
        whole_view_fallback_count: 0,
        repeated_rediscovery_denied_count: 0,
    };
}
