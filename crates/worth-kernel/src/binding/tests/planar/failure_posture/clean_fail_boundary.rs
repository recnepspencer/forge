use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarBoundedConversion, PlanarCleanFailBoundary, PlanarCleanFailBoundaryContracts,
    PlanarCleanFailBoundaryDenialKind, PlanarCleanFailClass, PlanarCleanFailInput,
    PlanarRepairAttempt,
};
use worth_spatial::facade::planar_contracts::{
    planar_admission_matrix, PlanarAdmissionFamily, PlanarRuntimeConcern,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleReceipt,
    PlanarDiagnosticSubject,
};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarMotionPostureReceipt, PlanarReorientation,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureReceipt,
    PlanarRecoverySource,
};

use super::super::bundle_closeout::contract_bundle::readiness_receipt;
use super::super::bundle_closeout::runtime_handles::motion_posture_handle;

#[test]
fn kernel_consumes_dirty_clean_fail_boundary_without_repair_or_summary_upgrade() {
    let source = "dirty:kernel-clean-fail-self-intersection";
    let receipt = PlanarCleanFailBoundary::from_planar_input(dirty_input(source))
        .recovery_posture(dirty_recovery(source))
        .diagnostics(diagnostic(PlanarDiagnosticSubject::policy_required(source)))
        .certify_clean_fail_boundary()
        .compile(&PlanarCleanFailBoundaryContracts::new(clean_fail_handle()))
        .expect("clean-fail boundary plan")
        .certify()
        .expect("clean-fail boundary receipt");

    assert_eq!(receipt.class(), PlanarCleanFailClass::DirtyInput);
    assert_eq!(receipt.repair_attempt(), PlanarRepairAttempt::NotAttempted);
    assert_eq!(
        receipt.bounded_conversion(),
        PlanarBoundedConversion::NotAttempted
    );
    assert!(receipt.basis().input().transform_posture_digest().is_some());
    assert_eq!(receipt.counters().clean_fail_sources(), 1);
}

#[test]
fn kernel_denies_dirty_repair_attempt_even_with_motion_and_diagnostics() {
    let source = "dirty:kernel-clean-fail-repair-pressure";
    let contracts = PlanarCleanFailBoundaryContracts::new(clean_fail_handle());
    let denial = PlanarCleanFailBoundary::from_planar_input(dirty_input(source))
        .recovery_posture(dirty_recovery(source))
        .diagnostics(diagnostic(PlanarDiagnosticSubject::policy_required(source)))
        .with_heuristic_repair_attempt()
        .certify_clean_fail_boundary()
        .compile(&contracts);
    let denial = match denial {
        Ok(_) => panic!("kernel must not repair dirty planar input"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarCleanFailBoundaryDenialKind::HeuristicRepairAttempted
    );
    assert_eq!(denial.counters().repair_attempts_denied(), 1);
}

#[test]
fn kernel_denies_unbounded_bounded_conversion_under_transform_pressure() {
    let source = "unbounded:kernel-clean-fail-half-space";
    let contracts = PlanarCleanFailBoundaryContracts::new(clean_fail_handle());
    let denial = PlanarCleanFailBoundary::from_planar_input(unbounded_input(source))
        .recovery_posture(unbounded_recovery(source))
        .diagnostics(diagnostic(
            PlanarDiagnosticSubject::unsupported_planar_class(source),
        ))
        .with_bounded_conversion_attempt()
        .certify_clean_fail_boundary()
        .compile(&contracts);
    let denial = match denial {
        Ok(_) => panic!("kernel must not convert unbounded input to bounded topology"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarCleanFailBoundaryDenialKind::BoundedConversionAttempted
    );
    assert_eq!(denial.counters().bounded_conversions_denied(), 1);
}

fn dirty_input(source: &'static str) -> PlanarCleanFailInput {
    PlanarCleanFailInput::dirty_planar_loop(source)
        .with_topology_identity("topology:kernel-clean-fail-dirty")
        .with_transform_posture(transform_posture())
        .with_admission_row(admission_row(
            PlanarAdmissionFamily::DirtyPlanarInput,
            PlanarRuntimeConcern::DiagnosticsLocalization,
        ))
}

fn unbounded_input(source: &'static str) -> PlanarCleanFailInput {
    PlanarCleanFailInput::unbounded_half_space(source)
        .with_topology_identity("topology:kernel-clean-fail-unbounded")
        .with_transform_posture(transform_posture())
        .with_admission_row(admission_row(
            PlanarAdmissionFamily::UnboundedPlanarDomain,
            PlanarRuntimeConcern::BooleanReadinessBundle,
        ))
}

fn admission_row(
    family: PlanarAdmissionFamily,
    concern: PlanarRuntimeConcern,
) -> worth_spatial::facade::planar_contracts::PlanarAdmissionRow {
    planar_admission_matrix()
        .row(family, concern)
        .expect("planar admission row")
        .clone()
}

fn transform_posture() -> PlanarMotionPostureReceipt {
    PlanarMotionPosture::from_boolean_readiness(readiness_receipt())
        .after_exact_translation("motion:kernel-clean-fail-translate")
        .after_exact_rotation("motion:kernel-clean-fail-rotate")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle()))
        .expect("motion posture plan")
        .certify()
        .expect("motion posture receipt")
}

fn dirty_recovery(source: &'static str) -> PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(PlanarRecoverySource::dirty_input(source))
        .prepare_next_step()
        .compile(&PlanarRecoveryPostureContracts::new(recovery_handle()))
        .expect("dirty recovery plan")
        .certify()
        .expect("dirty recovery receipt")
}

fn unbounded_recovery(source: &'static str) -> PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(PlanarRecoverySource::unbounded_or_open(
        source,
    ))
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle()))
    .expect("unbounded recovery plan")
    .certify()
    .expect("unbounded recovery receipt")
}

fn diagnostic(subject: PlanarDiagnosticSubject) -> PlanarDiagnosticBundleReceipt {
    PlanarDiagnosticBundle::explain_planar_failure(subject)
        .inspect_failure_locality()
        .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle()))
        .expect("diagnostic plan")
        .certify()
        .expect("diagnostic receipt")
}

fn clean_fail_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    worth_spatial::facade::planar_clean_fail_boundary::PlanarCleanFailBoundaryQueryDomain,
    worth_spatial::facade::planar_clean_fail_boundary::PlanarCleanFailBoundaryQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(worth_spatial::facade::planar_clean_fail_boundary::PlanarCleanFailBoundaryQueryDomain)
        .with_operating_context(
            worth_spatial::facade::planar_clean_fail_boundary::PlanarCleanFailBoundaryQueryWorld::new(
                "kernel-clean-fail-boundary",
            ),
        )
        .validate()
        .expect("validated clean-fail domain")
        .admit()
        .expect("admitted clean-fail domain")
}

fn recovery_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    worth_spatial::facade::planar_recovery::PlanarRecoveryPostureQueryDomain,
    worth_spatial::facade::planar_recovery::PlanarRecoveryPostureQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(worth_spatial::facade::planar_recovery::PlanarRecoveryPostureQueryDomain)
        .with_operating_context(
            worth_spatial::facade::planar_recovery::PlanarRecoveryPostureQueryWorld::new(
                "kernel-clean-fail-recovery",
            ),
        )
        .validate()
        .expect("validated recovery domain")
        .admit()
        .expect("admitted recovery domain")
}

fn diagnostic_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleQueryDomain,
    worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleQueryDomain)
        .with_operating_context(
            worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleQueryWorld::new(
                "kernel-clean-fail-diagnostic",
            ),
        )
        .validate()
        .expect("validated diagnostic domain")
        .admit()
        .expect("admitted diagnostic domain")
}
