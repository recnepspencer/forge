use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};

use worth_kernel::facade::certification::closeout::prepare_primitive_construction_milestone_four_kernel_closeout_evidence_report;

#[test]
fn kernel_public_facade_exports_milestone_four_kernel_closeout_evidence_surface() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api-milestone-four-kernel-closeout".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_milestone_four_kernel_closeout_evidence_report(
        &mut workspace,
    )
    .expect("milestone four kernel closeout evidence report");

    assert_eq!(
        report.proof_substrate_closeout().proof_grade().as_str(),
        "proof_substrate_closeout"
    );
    assert_eq!(
        report.proof_substrate_closeout().proof_subject().as_str(),
        "proof_substrate_closeout"
    );
    assert!(report.query_boundary_gap_register().unresolved_gap_count() >= 1);
    assert_eq!(report.motion_policy_report().rows().len(), 9);
    assert_eq!(report.preview_surface_report().rows().len(), 5);
    assert_eq!(report.continuity_surface_report().rows().len(), 6);
    assert_eq!(report.policy_profile_report().rows().len(), 5);
    assert_eq!(
        report.realization_exhaustion_witness_report().rows().len(),
        3
    );
    assert_ne!(
        report.report_digest(),
        report.phase_five_six_closeout().report_digest()
    );
}
