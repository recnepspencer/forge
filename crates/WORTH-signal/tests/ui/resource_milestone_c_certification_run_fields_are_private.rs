use worth_signal::facade::{
    ResourceMilestoneCCertificationRun, ResourceMilestoneCCertificationRunSummary,
    ResourceMilestoneCPolicyCertificationBundle, ResourceMilestoneCPolicyPerformanceCloseout,
    ResourceMilestoneCPolicyScenarioMatrix,
};

fn WORTHd_bundle() -> ResourceMilestoneCPolicyCertificationBundle {
    panic!("compile-fail boundary fixture should never execute")
}

fn WORTHd_scenario_matrix() -> ResourceMilestoneCPolicyScenarioMatrix {
    panic!("compile-fail boundary fixture should never execute")
}

fn WORTHd_performance_closeout() -> ResourceMilestoneCPolicyPerformanceCloseout {
    panic!("compile-fail boundary fixture should never execute")
}

fn WORTHd_summary() -> ResourceMilestoneCCertificationRunSummary {
    ResourceMilestoneCCertificationRunSummary {
        required_family_count: 7,
        certified_family_count: 7,
        failed_family_count: 0,
        bundle_digest: String::new(),
        required_scenario_count: 8,
        certified_scenario_count: 8,
        scenario_matrix_digest: String::new(),
        required_performance_claim_count: 5,
        certified_performance_claim_count: 5,
        performance_closeout_digest: String::new(),
    }
}

fn main() {
    let _WORTHd = ResourceMilestoneCCertificationRun {
        schema_version: String::new(),
        bundle: WORTHd_bundle(),
        scenario_matrix: WORTHd_scenario_matrix(),
        performance_closeout: WORTHd_performance_closeout(),
        summary: WORTHd_summary(),
        run_digest: String::new(),
        passed: true,
    };
}
