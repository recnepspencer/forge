use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::certification::closeout::prepare_primitive_construction_phase_five_six_closeout_report;

#[test]
fn kernel_public_facade_exports_phase_five_six_closeout_surface() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-phase-five-six-closeout".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_phase_five_six_closeout_report(&mut workspace)
        .expect("phase 5.6 closeout");

    assert!(report.phase_five_boundary().closeout_gate_verified());
    assert!(report.compound_closeout().closeout_gate_verified());
    assert!(report.policy_pressure().parity_verified());
    assert_eq!(
        report.simplex_ladder().rows().len(),
        report.required_simplex_scenarios().len()
    );
    for scenario_id in report.required_simplex_scenarios() {
        assert!(report.simplex_ladder().row_for(scenario_id).is_some());
    }
    assert_eq!(
        report.policy_pressure().direct_report().rows().len(),
        report.policy_pressure().required_direct_cases().len()
    );
    assert_eq!(
        report.policy_pressure().delta_report().rows().len(),
        report.policy_pressure().required_delta_cases().len()
    );
    assert_eq!(
        report.simplex_exhaustion().rows().len(),
        report.required_exhaustion_kinds().len()
    );
    for witness_kind in report.required_exhaustion_kinds() {
        assert!(report.simplex_exhaustion().row_for(*witness_kind).is_some());
    }
}
