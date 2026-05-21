use super::prepare_primitive_construction_phase_five_six_closeout_report;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

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
    for scenario_id in report.required_simplex_scenarios() {
        assert!(report.simplex_ladder().row_for(scenario_id).is_some());
    }
    assert_eq!(report.simplex_exhaustion().rows().len(), 2);
    assert!(report
        .simplex_exhaustion()
        .row_for(PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse)
        .is_some());
    assert_eq!(report.policy_pressure().required_direct_cases().len(), 7);
    assert_eq!(report.policy_pressure().required_delta_cases().len(), 5);
    assert_ne!(
        report.report_digest(),
        report.compound_closeout().report_digest()
    );
    assert_ne!(
        report.report_digest(),
        report.policy_pressure().report_digest()
    );
}
