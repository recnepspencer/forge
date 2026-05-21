use super::{
    prepare_primitive_construction_motion_dx_surface_report,
    prepare_primitive_construction_motion_resolution_policy_report,
    PrimitiveConstructionMotionDxSurface, PrimitiveConstructionMotionResolutionPolicyCase,
};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};

fn workspace(name: &str) -> forge_query::facade::ForgeQueryWorkspace {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        name.to_string(),
    )
    .expect("workspace")
}

#[test]
fn motion_dx_surface_report_distinguishes_common_advanced_and_degraded_paths() {
    let mut workspace = workspace("worth-kernel.motion-dx-surface");
    let report =
        prepare_primitive_construction_motion_dx_surface_report(&mut workspace).expect("dx report");
    let policy_report =
        prepare_primitive_construction_motion_resolution_policy_report(&mut workspace)
            .expect("policy report");

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
            .row(PrimitiveConstructionMotionResolutionPolicyCase::FallbackPointsToward)
            .expect("fallback")
            .dx_surface(),
        PrimitiveConstructionMotionDxSurface::UnsafeOrDegradedPath
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::AmbiguousMove)
            .expect("ambiguous")
            .dx_surface(),
        PrimitiveConstructionMotionDxSurface::UnsafeOrDegradedPath
    );
    assert_ne!(report.report_digest(), policy_report.report_digest());
}
