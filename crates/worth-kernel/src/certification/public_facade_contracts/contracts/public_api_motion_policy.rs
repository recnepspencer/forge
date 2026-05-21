use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_kernel::facade::diagnostics::motion::*;
use worth_spatial::facade::{SpatialPlacementConstraintError, SpatialWitnessFailureClass};

#[test]
fn kernel_public_facade_exports_motion_resolution_policy_report() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-api.motion-policy".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_motion_resolution_policy_report(&mut workspace)
        .expect("policy report");

    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::DirectMove)
            .expect("direct move")
            .runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::Available
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::CarrierReorient)
            .expect("carrier reorient")
            .resolution_class(),
        Some(worth_spatial::facade::SpatialWitnessResolutionClass::CarrierDerived)
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::AmbiguousMove)
            .expect("ambiguous move")
            .failure_kind(),
        Some(
            PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(
                SpatialWitnessFailureClass::Ambiguous
            )
        )
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::CoincidentPointsToward)
            .expect("coincident points")
            .runtime_surface_status(),
        PrimitiveConstructionMotionRuntimeSurfaceStatus::ConstraintLoweringBlocked(
            SpatialPlacementConstraintError::CoincidentTarget
        )
    );
    assert_ne!(
        report.report_digest(),
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::DirectMove)
            .expect("direct move")
            .row_digest()
    );
}
