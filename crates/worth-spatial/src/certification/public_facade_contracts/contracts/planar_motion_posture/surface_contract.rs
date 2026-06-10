use forge_query::facade::ForgeQueryDeclarationFamilyMarker;
use worth_spatial::certification::geometry_support_posture::geometry_public_support_matrix;
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionPosture, PlanarMotionPostureContracts, PlanarMotionPostureDeclarationFamily,
    PlanarMotionPostureQueryDomain,
};
use worth_spatial::facade::support::{
    geometry_applicability_matrix, GeometryApplicabilityStatus, GeometryPublicSurface,
    GeometryRuntimeConcern,
};

use super::contract_subject::boolean_readiness_receipt;
use super::runtime_handles::motion_posture_handle;

#[test]
fn spatial_public_facade_exports_readable_motion_posture_surface() {
    let family = <PlanarMotionPostureDeclarationFamily as ForgeQueryDeclarationFamilyMarker<
        PlanarMotionPostureQueryDomain,
    >>::semantic_family_key();
    assert_eq!(family, "PlanarMotionPosture");
    assert!(std::any::type_name::<PlanarMotionPosture>().contains("PlanarMotionPosture"));
}

#[test]
fn planar_motion_posture_is_registered_with_query_support_posture() {
    let support_matrix = geometry_public_support_matrix();
    let support = support_matrix
        .row_for_surface(GeometryPublicSurface::PlanarMotionPosture)
        .expect("motion posture support row");
    assert_eq!(support.declared_family_key(), Some("PlanarMotionPosture"));
    assert!(support.admission_rule().contains("signal compatibility"));

    let applicability = geometry_applicability_matrix();
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarMotionPosture,
                GeometryRuntimeConcern::SignalContinuation,
            )
            .expect("motion posture signal continuation row")
            .status(),
        GeometryApplicabilityStatus::RequiredNow
    );
    assert_eq!(
        applicability
            .row(
                GeometryPublicSurface::PlanarMotionPosture,
                GeometryRuntimeConcern::RecoveryAction,
            )
            .expect("motion posture recovery row")
            .status(),
        GeometryApplicabilityStatus::DeniedForThisRuntime
    );
}

#[test]
fn planar_motion_posture_plan_exposes_retained_inspection_breadth() {
    let contracts =
        PlanarMotionPostureContracts::new(motion_posture_handle("motion-plan-inspection"));
    let plan = PlanarMotionPosture::from_boolean_readiness(boolean_readiness_receipt(
        "motion-plan-inspection",
    ))
    .after_exact_translation("motion:translate")
    .after_exact_rotation("motion:rotate")
    .compile(&contracts)
    .expect("motion posture plan");

    assert_eq!(plan.inspected_motion_rows(), 5);
}
