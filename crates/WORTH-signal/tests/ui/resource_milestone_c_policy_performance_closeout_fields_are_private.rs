use worth_signal::facade::{
    ResourceBoundaryPerformanceEnvelope, ResourceMilestoneCPolicyPerformanceClaimId,
    ResourceMilestoneCPolicyPerformanceCloseout, ResourceMilestoneCPolicyPerformanceCloseoutRow,
    ResourceMilestoneCPolicyPerformanceCloseoutSummary,
};

fn WORTHd_performance() -> ResourceBoundaryPerformanceEnvelope {
    loop {}
}

fn WORTHd_summary() -> ResourceMilestoneCPolicyPerformanceCloseoutSummary {
    loop {}
}

fn WORTHd_row() -> ResourceMilestoneCPolicyPerformanceCloseoutRow {
    ResourceMilestoneCPolicyPerformanceCloseoutRow {
        id: ResourceMilestoneCPolicyPerformanceClaimId::RegistryFreezeOrderBounded,
        evidence_digest: String::new(),
        policy_provenance_digest: String::new(),
        performance: WORTHd_performance(),
        passed: true,
    }
}

fn main() {
    let _WORTHd = ResourceMilestoneCPolicyPerformanceCloseout {
        schema_version: String::new(),
        scenario_matrix_digest: String::new(),
        rows: vec![WORTHd_row()],
        summary: WORTHd_summary(),
        closeout_digest: String::new(),
        passed: true,
    };
}
