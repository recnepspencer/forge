use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::{
    prepare_primitive_construction_motion_dx_surface_report, PrimitiveConstructionMotionDxSurface,
    PrimitiveConstructionMotionResolutionPolicyCase,
};

#[test]
fn kernel_public_facade_exports_motion_dx_surface_report() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-dx-surface".to_string(),
    )
    .expect("workspace");
    let report =
        prepare_primitive_construction_motion_dx_surface_report(&mut workspace).expect("dx report");

    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::DirectMove)
            .expect("direct move")
            .dx_surface(),
        PrimitiveConstructionMotionDxSurface::CommonPath
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::CarrierReorient)
            .expect("carrier reorient")
            .dx_surface(),
        PrimitiveConstructionMotionDxSurface::AdvancedPath
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::AmbiguousMove)
            .expect("ambiguous move")
            .dx_surface(),
        PrimitiveConstructionMotionDxSurface::UnsafeOrDegradedPath
    );
    assert!(!report.report_digest().is_empty());
}
