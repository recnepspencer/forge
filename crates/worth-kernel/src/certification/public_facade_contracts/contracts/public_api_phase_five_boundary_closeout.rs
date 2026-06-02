use worth_kernel::facade::certification::closeout::{
    prepare_primitive_construction_phase_five_boundary_closeout_report,
    PrimitiveConstructionPhaseFiveBoundaryCloseoutKind,
};

#[test]
fn kernel_public_facade_exports_phase_five_boundary_closeout_surface() {
    let report = prepare_primitive_construction_phase_five_boundary_closeout_report();

    assert!(report.closeout_gate_verified());
    assert_eq!(report.rows().len(), 8);
    assert_eq!(report.query_runtime_audit().violation_count(), 0);
    assert!(report
        .row_for(
            PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::FamilyBirthInputBoundaryLocalized
        )
        .expect("family boundary row")
        .verified());
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
    assert!(report
        .row_for(
            PrimitiveConstructionPhaseFiveBoundaryCloseoutKind::TopologyReadyBirthBoundaryLocalized
        )
        .expect("topology-ready birth row")
        .verified());
}
