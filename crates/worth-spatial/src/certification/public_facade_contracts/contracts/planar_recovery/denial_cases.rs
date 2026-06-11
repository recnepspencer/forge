use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureDenialKind,
    PlanarRecoverySource,
};

use super::contract_subject::{
    planar_recovery_parts, projection_basis_source, retained_projection_basis_source,
};
use super::runtime_handles::recovery_handle;

#[test]
fn planar_recovery_rejects_missing_retained_or_projection_basis() {
    let world = "planar-recovery-missing-basis";
    let parts = planar_recovery_parts(world);
    let contracts = PlanarRecoveryPostureContracts::new(recovery_handle(world));

    let missing_retained =
        match PlanarRecoveryPosture::from_blocked_planar_source(projection_basis_source())
            .with_projection_consumed_facts(parts.projected.clone())
            .prepare_next_step()
            .compile(&contracts)
        {
            Ok(_) => panic!("missing retained basis must deny recovery posture"),
            Err(error) => error,
        };
    assert_eq!(
        missing_retained.kind(),
        PlanarRecoveryPostureDenialKind::MissingRetainedPlanarFacts
    );

    let missing_projection =
        match PlanarRecoveryPosture::from_blocked_planar_source(projection_basis_source())
            .with_retained_planar_facts(parts.retained)
            .prepare_next_step()
            .compile(&contracts)
        {
            Ok(_) => panic!("missing projection-consumed basis must deny recovery posture"),
            Err(error) => error,
        };
    assert_eq!(
        missing_projection.kind(),
        PlanarRecoveryPostureDenialKind::MissingProjectionConsumedPlanarFacts
    );
}

#[test]
fn planar_recovery_rejects_blank_source_before_basis_recovery() {
    let world = "planar-recovery-blank-source";
    let parts = planar_recovery_parts(world);
    let contracts = PlanarRecoveryPostureContracts::new(recovery_handle(world));

    let denial = match PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_projection_denial("  "),
    )
    .with_retained_planar_facts(parts.retained)
    .with_projection_consumed_facts(parts.projected)
    .prepare_next_step()
    .compile(&contracts)
    {
        Ok(_) => panic!("blank source digest must deny before recovery posture classification"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarRecoveryPostureDenialKind::MissingRecoverySource
    );
}

#[test]
fn planar_recovery_rejects_mismatched_retained_and_projection_basis() {
    let retained_parts = planar_recovery_parts("planar-recovery-mismatch-retained");
    let projected_parts = planar_recovery_parts("planar-recovery-mismatch-projected");
    let contracts =
        PlanarRecoveryPostureContracts::new(recovery_handle("planar-recovery-mismatch"));

    let denial =
        match PlanarRecoveryPosture::from_blocked_planar_source(retained_projection_basis_source())
            .with_retained_planar_facts(retained_parts.retained)
            .with_projection_consumed_facts(projected_parts.projected)
            .prepare_next_step()
            .compile(&contracts)
        {
            Ok(_) => panic!("mismatched retained and projection basis must deny recovery posture"),
            Err(error) => error,
        };

    assert_eq!(
        denial.kind(),
        PlanarRecoveryPostureDenialKind::MismatchedRetainedProjectionBasis
    );
}

#[test]
fn planar_recovery_rejects_kernel_summary_sources() {
    let world = "planar-recovery-summary";
    let contracts = PlanarRecoveryPostureContracts::new(recovery_handle(world));
    let denial = match PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_kernel_summary("probably dirty"),
    )
    .prepare_next_step()
    .compile(&contracts)
    {
        Ok(_) => panic!("kernel summary must not become planar recovery authority"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarRecoveryPostureDenialKind::SummarySourceNotAuthority
    );
}
