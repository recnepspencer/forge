use super::prepare_primitive_construction_milestone_four_kernel_closeout_evidence_report;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};

#[test]
fn milestone_four_kernel_closeout_evidence_report_binds_query_spatial_intent_realization_and_phase_five_six_truth(
) {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.milestone-four-kernel-closeout".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_milestone_four_kernel_closeout_evidence_report(
        &mut workspace,
    )
    .expect("milestone four kernel closeout evidence report");

    assert!(report.phase_five_six_closeout().closeout_verified());
    assert!(report.query_closeout_verified());
    assert!(report.spatial_intent_closeout_verified());
    assert!(report.realization_closeout_verified());
    assert!(report.kernel_evidence_verified());
    assert_ne!(
        report.report_digest(),
        report.phase_five_six_closeout().report_digest()
    );
}
