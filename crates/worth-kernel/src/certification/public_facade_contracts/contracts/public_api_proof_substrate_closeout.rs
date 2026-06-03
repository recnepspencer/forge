use worth_kernel::facade::certification::closeout::{
    prepare_primitive_construction_digest_protocol_report,
    prepare_primitive_construction_proof_boundary_compile_fail_report,
    prepare_primitive_construction_proof_substrate_closeout_report,
    prepare_primitive_construction_truth_projection_matrix,
    prepare_primitive_construction_verified_artifact_surface_report,
};

#[test]
fn kernel_public_facade_exports_proof_substrate_closeout_surface() {
    let digest_protocol_report = prepare_primitive_construction_digest_protocol_report();
    let verified_artifact_surface_report =
        prepare_primitive_construction_verified_artifact_surface_report();
    let truth_projection_matrix = prepare_primitive_construction_truth_projection_matrix();
    let proof_boundary_compile_fail_report =
        prepare_primitive_construction_proof_boundary_compile_fail_report();
    let substrate_closeout = prepare_primitive_construction_proof_substrate_closeout_report()
        .expect("proof substrate closeout");

    assert_eq!(digest_protocol_report.version_prefix(), "worth-kernel.v1");
    assert_eq!(verified_artifact_surface_report.rows().len(), 3);
    assert_eq!(truth_projection_matrix.rows().len(), 1);
    assert_eq!(proof_boundary_compile_fail_report.fixtures().len(), 6);
    assert_eq!(
        substrate_closeout.digest_protocol_report().report_digest(),
        digest_protocol_report.report_digest()
    );
    assert_eq!(
        substrate_closeout
            .verified_artifact_surface_report()
            .report_digest(),
        verified_artifact_surface_report.report_digest()
    );
    assert_eq!(
        substrate_closeout.truth_projection_matrix().report_digest(),
        truth_projection_matrix.report_digest()
    );
    assert_eq!(
        substrate_closeout
            .proof_boundary_compile_fail_report()
            .report_digest(),
        proof_boundary_compile_fail_report.report_digest()
    );
    assert_eq!(
        substrate_closeout.proof_grade().as_str(),
        "proof_substrate_closeout"
    );
    assert_eq!(
        substrate_closeout.proof_subject().as_str(),
        "proof_substrate_closeout"
    );
}
