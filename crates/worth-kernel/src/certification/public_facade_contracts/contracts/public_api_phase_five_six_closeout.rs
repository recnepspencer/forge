use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::prepare_primitive_construction_phase_five_six_closeout_report;

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

    assert!(report.closeout_verified());
    assert!(report.compound_closeout().closeout_gate_verified());
    assert!(report.policy_pressure().parity_verified());
    assert!(report.required_simplex_rows_present());
    assert_eq!(
        report.required_simplex_scenarios(),
        &[
            "simplex_world_collapsed_admitted_local_or_exact",
            "simplex_world_collapsed_threshold_rejected",
            "simplex_world_collapsed_explicit_exhaustion",
        ]
    );
    assert_eq!(report.policy_pressure().required_direct_cases().len(), 7);
    assert_eq!(report.policy_pressure().required_delta_cases().len(), 5);
}
