use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_contract_bundle::{
    PlanarM7ReadinessBundle, PlanarM7ReadinessFamily, PlanarM7ReadinessSupportPosture,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld, PlanarDiagnosticSubject,
};
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld, PlanarRecoverySource,
};

use super::super::retained_views::projection_consumption::retained_planar_facts;
use super::runtime_handles::bundle_handle;

#[test]
fn kernel_consumes_m7_readiness_bundle_without_boolean_execution_synthesis() {
    let readiness = super::contract_bundle::readiness_receipt();
    let readiness_declaration_digest = readiness.declaration_digest().to_string();
    let readiness_envelope_digest = readiness.envelope_digest().to_string();
    let retained = retained_planar_facts(readiness.clone());
    let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
        .consume_bundle_projection_receipts(readiness.basis().projection_receipts().to_vec())
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            super::super::retained_views::projection_consumption::projection_consumption_handle(),
        ))
        .expect("kernel M7 projection-consumption plan")
        .consume()
        .expect("kernel M7 projection-consumption receipt");
    let motion = retained.basis().motion_posture_receipt().clone();
    let structural = retained.basis().structural_identity_receipt().clone();
    let recovery = PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_projection_denial("kernel-m7:projection-basis"),
    )
    .with_retained_planar_facts(retained.clone())
    .with_projection_consumed_facts(projected.clone())
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle()))
    .expect("kernel M7 recovery plan")
    .certify()
    .expect("kernel M7 recovery receipt");
    let diagnostics = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::binding_failure("kernel-m7:closeout-diagnostic"),
    )
    .with_retained_planar_facts(retained.clone())
    .with_projection_consumed_planar_facts(projected.clone())
    .with_motion_posture(motion.clone())
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle()))
    .expect("kernel M7 diagnostics plan")
    .certify()
    .expect("kernel M7 diagnostics receipt");

    let receipt = PlanarM7ReadinessBundle::from_certified_planar_bundle(readiness)
        .with_structural_identity(structural)
        .with_motion_posture(motion)
        .with_retained_planar_facts(retained)
        .with_projection_consumed_facts(projected)
        .with_recovery_posture(recovery)
        .with_diagnostics(diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "kernel may consume M6 readiness, but M7 boolean execution is support-gated",
        ))
        .compile(&worth_spatial::facade::planar_contract_bundle::PlanarContractBundleValidationContracts::new(
            bundle_handle(),
        ))
        .expect("kernel M7 readiness plan")
        .certify()
        .expect("kernel M7 readiness receipt");

    assert!(receipt.is_acceptable_m7_input());
    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert_eq!(receipt.declaration_digest(), readiness_declaration_digest);
    assert_eq!(receipt.envelope_digest(), readiness_envelope_digest);
    assert!(receipt
        .family_rows()
        .iter()
        .any(|row| { row.family() == PlanarM7ReadinessFamily::PredicateAuthority }));
    assert!(receipt
        .family_rows()
        .iter()
        .any(|row| { row.family() == PlanarM7ReadinessFamily::SupportPosture }));
    assert_eq!(receipt.counters().support_posture_rows(), 1);
}

fn recovery_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarRecoveryPostureQueryDomain)
        .with_operating_context(PlanarRecoveryPostureQueryWorld::new("kernel-m7-readiness"))
        .validate()
        .expect("validated kernel M7 recovery domain")
        .admit()
        .expect("admitted kernel M7 recovery domain")
}

fn diagnostic_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarDiagnosticBundleQueryDomain)
        .with_operating_context(PlanarDiagnosticBundleQueryWorld::new("kernel-m7-readiness"))
        .validate()
        .expect("validated kernel M7 diagnostic domain")
        .admit()
        .expect("admitted kernel M7 diagnostic domain")
}
