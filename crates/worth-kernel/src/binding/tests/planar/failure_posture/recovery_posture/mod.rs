use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryAction, PlanarRecoveryPosture, PlanarRecoveryPostureContracts,
    PlanarRecoveryPostureDenialKind, PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld, PlanarRecoverySource, PlanarRecoveryTruthEffect,
};

#[test]
fn kernel_consumes_planar_recovery_posture_without_repairing_truth() {
    let receipt = PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::dirty_input("kernel-dirty:self-intersection"),
    )
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(
        "kernel-planar-recovery-dirty",
    )))
    .expect("kernel recovery plan")
    .certify()
    .expect("kernel recovery receipt");

    assert_eq!(
        receipt.recovery_action(),
        PlanarRecoveryAction::InspectTopologyAndInputCleanliness
    );
    assert_eq!(
        receipt.truth_effect(),
        PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth
    );
    assert_eq!(receipt.counters().basis_receipts_consumed(), 0);
    assert_eq!(receipt.counters().recovery_rows_emitted(), 1);
}

#[test]
fn kernel_rejects_summary_only_planar_recovery() {
    let denial = match PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_kernel_summary("kernel thinks this is probably recoverable"),
    )
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(
        "kernel-planar-recovery-summary",
    ))) {
        Ok(_) => panic!("kernel summary must not become planar recovery authority"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarRecoveryPostureDenialKind::SummarySourceNotAuthority
    );
}

fn recovery_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarRecoveryPostureQueryDomain)
        .with_operating_context(PlanarRecoveryPostureQueryWorld::new(world))
        .validate()
        .expect("validated planar recovery test domain")
        .admit()
        .expect("admitted planar recovery test domain")
}
