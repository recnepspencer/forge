use topology::facade::TopologyDomainQueryCloseoutReport;

fn main() {
    let _ = TopologyDomainQueryCloseoutReport {
        proof_report: todo!(),
        query_executed_family_count: 0,
        query_executed_debt_free_family_count: 0,
        query_executed_debt_backed_family_count: 0,
        debt_family_count: 0,
        whole_view_debt_request_count: 0,
        row_scan_fallback_request_count: 0,
        repeated_rediscovery_denied_count: 0,
        phase_three_ready: false,
    };
}
