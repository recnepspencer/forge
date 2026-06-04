use super::closeout::prepare_primitive_construction_phase_five_six_closeout_report;
use crate::construction::certification::corpus::{
    required_simplex_exhaustion_witness_kinds, required_simplex_ladder_scenarios,
};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};

#[test]
fn phase_five_six_closeout_report_binds_compound_simplex_and_policy_pressure_evidence() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.phase-five-six-closeout".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_phase_five_six_closeout_report(&mut workspace)
        .expect("phase 5.6 closeout report");

    assert!(report.phase_five_boundary().closeout_gate_verified());
    assert!(report.compound_closeout().closeout_gate_verified());
    assert!(report.policy_pressure().parity_verified());
    assert_eq!(report.policy_pressure().required_direct_cases().len(), 7);
    assert_eq!(report.policy_pressure().required_delta_cases().len(), 5);
    assert_eq!(
        report.required_simplex_scenarios(),
        required_simplex_ladder_scenarios()
    );
    for scenario_id in report.required_simplex_scenarios() {
        assert!(report.simplex_ladder().row_for(scenario_id).is_some());
    }
    assert_eq!(
        report.simplex_exhaustion().rows().len(),
        required_simplex_exhaustion_witness_kinds().len()
    );
    for witness_kind in required_simplex_exhaustion_witness_kinds() {
        assert!(report.simplex_exhaustion().row_for(*witness_kind).is_some());
    }
    assert_ne!(
        report.report_digest(),
        report.phase_five_boundary().report_digest()
    );
    assert_ne!(
        report.report_digest(),
        report.compound_closeout().report_digest()
    );
    assert_ne!(
        report.report_digest(),
        report.policy_pressure().report_digest()
    );
}
