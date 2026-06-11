use worth_spatial::facade::planar_contract_bundle::{
    PlanarM7ReadinessBundle, PlanarM7ReadinessDenialKind, PlanarM7ReadinessSupportPosture,
};

use super::fixture::{bundle_contracts, m7_readiness_parts};

#[test]
fn boolean_readiness_bundle_rejects_partial_or_kernel_synthesized_facts() {
    let world = "m7-readiness-denial";
    let parts = m7_readiness_parts(world);
    let contracts = bundle_contracts(world);
    let missing_support =
        PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness.clone())
            .with_structural_identity(parts.structural.clone())
            .with_motion_posture(parts.motion.clone())
            .with_retained_planar_facts(parts.retained.clone())
            .with_projection_consumed_facts(parts.projected.clone())
            .with_recovery_posture(parts.recovery.clone())
            .with_diagnostics(parts.diagnostics.clone())
            .compile(&contracts);
    let missing_support = match missing_support {
        Ok(_) => panic!("support posture is mandatory"),
        Err(denial) => denial,
    };

    assert_eq!(
        missing_support.kind(),
        PlanarM7ReadinessDenialKind::MissingSupportPosture
    );
    assert_eq!(missing_support.counters().rejected_rows(), 1);

    let missing_diagnostics =
        PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness.clone())
            .with_structural_identity(parts.structural.clone())
            .with_motion_posture(parts.motion.clone())
            .with_retained_planar_facts(parts.retained.clone())
            .with_projection_consumed_facts(parts.projected.clone())
            .with_recovery_posture(parts.recovery)
            .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
                "M7 lanes stay support-gated",
            ))
            .compile(&contracts);
    let missing_diagnostics = match missing_diagnostics {
        Ok(_) => panic!("diagnostics are mandatory"),
        Err(denial) => denial,
    };

    assert_eq!(
        missing_diagnostics.kind(),
        PlanarM7ReadinessDenialKind::MissingCloseoutFamily
    );

    let stale_projected = m7_readiness_parts("m7-readiness-stale-projection").projected;
    let stale_projection = PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness)
        .with_structural_identity(parts.structural)
        .with_motion_posture(parts.retained.basis().motion_posture_receipt().clone())
        .with_retained_planar_facts(parts.retained)
        .with_projection_consumed_facts(stale_projected)
        .with_recovery_posture(m7_readiness_parts("m7-readiness-stale-recovery").recovery)
        .with_diagnostics(parts.diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "M7 lanes stay support-gated",
        ))
        .compile(&contracts);
    let stale_projection = match stale_projection {
        Ok(_) => panic!("stale projection-consumed facts cannot close out readiness"),
        Err(denial) => denial,
    };

    assert_eq!(
        stale_projection.kind(),
        PlanarM7ReadinessDenialKind::MismatchedRetainedFacts
    );

    let parts = m7_readiness_parts("m7-readiness-query-boundary");
    let wrong_query_boundary = bundle_contracts("m7-readiness-wrong-query-boundary");
    let wrong_query_boundary_plan =
        PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness)
            .with_structural_identity(parts.structural)
            .with_motion_posture(parts.motion)
            .with_retained_planar_facts(parts.retained)
            .with_projection_consumed_facts(parts.projected)
            .with_recovery_posture(parts.recovery)
            .with_diagnostics(parts.diagnostics)
            .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
                "M7 lanes stay support-gated",
            ))
            .compile(&wrong_query_boundary)
            .expect("wrong Query boundary is detected during certification");
    let wrong_query_boundary_denial = match wrong_query_boundary_plan.certify() {
        Ok(_) => panic!("M7 readiness must reject a mismatched Query boundary"),
        Err(denial) => denial,
    };

    assert_eq!(
        wrong_query_boundary_denial.kind(),
        PlanarM7ReadinessDenialKind::QueryBoundaryMismatch
    );
}
