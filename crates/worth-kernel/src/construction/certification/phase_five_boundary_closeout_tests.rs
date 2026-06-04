use super::closeout::{
    prepare_primitive_construction_phase_five_boundary_closeout_report,
    PrimitiveConstructionPhaseFiveBoundaryCloseoutKind,
};

#[test]
fn phase_five_boundary_closeout_report_proves_query_native_construction_boundary() {
    let report = prepare_primitive_construction_phase_five_boundary_closeout_report();

    assert_eq!(report.rows().len(), 8);
    assert_eq!(report.query_runtime_audit().violation_count(), 0);
    assert!(report
        .row_for(
            PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::TopologyRejectsSpatialDependency
        )
        .expect("topology dependency row")
        .verified());
    assert!(report
        .row_for(PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::SpatialRejectsKernelDependency)
        .expect("spatial dependency row")
        .verified());
    assert!(
        report
            .row_for(
                PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::SynopsisOwnedAdmittedHandoffPrecedent
            )
            .expect("topology synopsis row")
            .verified()
    );
    assert!(
        report
            .row_for(
                PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::KernelConsumesSynopsisOwnedAdmittedHandoff
            )
            .expect("kernel synopsis consumer row")
            .verified()
    );
    assert!(report
        .row_for(
            PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::PublicQuerylessHappyPathQuarantined
        )
        .expect("public queryless row")
        .verified());
    assert!(report
        .row_for(PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::QueryRuntimeAuthoringHonesty)
        .expect("authoring honesty row")
        .verified());
    assert!(report
        .row_for(
            PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::FamilyBirthInputBoundaryLocalized
        )
        .expect("family birth input row")
        .verified());
    assert!(report
        .row_for(
            PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::TopologyReadyBirthBoundaryLocalized
        )
        .expect("topology ready birth row")
        .verified());
    assert!(report.closeout_gate_verified());
    assert!(!report.report_digest().is_empty());
}
