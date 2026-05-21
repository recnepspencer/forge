use super::{
    prepare_primitive_construction_motion_resolution_policy_report,
    PrimitiveConstructionMotionResolutionPolicyCase,
};
use crate::construction::{
    PrimitiveConstructionFamily, PrimitiveConstructionMotionRuntimeSurfaceStatus,
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionStatus,
};
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_spatial::facade::{
    SpatialPlacementConstraintError, SpatialWitnessFailureClass, SpatialWitnessResolutionClass,
};

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
fn motion_resolution_policy_report_distinguishes_resolution_and_failure_classes() {
    let mut workspace = workspace("worth-kernel.motion-policy-report");
    let report = prepare_primitive_construction_motion_resolution_policy_report(&mut workspace)
        .expect("policy report");

    assert_eq!(report.rows().len(), 9);
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::DirectMove)
            .expect("direct move")
            .subject_family(),
        PrimitiveConstructionFamily::WireBody
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::DirectMove)
            .expect("direct move")
            .anchor(),
        &worth_spatial::facade::SpatialAnchorRef::shape_origin()
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::DirectMove)
            .expect("direct move")
            .resolution_class(),
        Some(SpatialWitnessResolutionClass::DirectWorld)
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::FrameReorient)
            .expect("frame reorient")
            .kind(),
        PrimitiveConstructionMotionWitnessResolutionKind::Reorient
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::FrameReorient)
            .expect("frame reorient")
            .resolution_class(),
        Some(SpatialWitnessResolutionClass::FrameDerived)
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::CarrierReorient)
            .expect("carrier reorient")
            .resolution_class(),
        Some(SpatialWitnessResolutionClass::CarrierDerived)
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::FallbackPointsToward)
            .expect("fallback points")
            .resolution_class(),
        Some(SpatialWitnessResolutionClass::FallbackDerived)
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::AmbiguousMove)
            .expect("ambiguous move")
            .status(),
        PrimitiveConstructionMotionWitnessResolutionStatus::Rejected
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
            .row(PrimitiveConstructionMotionResolutionPolicyCase::UndefinedReorient)
            .expect("undefined reorient")
            .failure_kind(),
        Some(
            PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(
                SpatialWitnessFailureClass::Undefined
            )
        )
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::UnsupportedReorient)
            .expect("unsupported reorient")
            .failure_kind(),
        Some(
            PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(
                SpatialWitnessFailureClass::Unsupported
            )
        )
    );
    assert_eq!(
        report
            .row(PrimitiveConstructionMotionResolutionPolicyCase::ExhaustedRotate)
            .expect("exhausted rotate")
            .failure_kind(),
        Some(
            PrimitiveConstructionMotionWitnessResolutionFailureKind::Witness(
                SpatialWitnessFailureClass::Exhausted
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
