use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_contract_bundle::{
    PlanarContractBundleValidationContracts, PlanarContractBundleValidationReceipt,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld, PlanarDiagnosticBundleReceipt, PlanarDiagnosticSubject,
};
use worth_spatial::facade::planar_motion_posture::PlanarMotionPostureReceipt;
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsQueryDomain, ProjectionConsumedPlanarFactsQueryWorld,
    ProjectionConsumedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld, PlanarRecoveryPostureReceipt, PlanarRecoverySource,
};
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use worth_spatial::facade::planar_structural_identity::PlanarStructuralIdentityReceipt;

use crate::public_api_planar_contract_bundle::runtime_handles::bundle_handle;
use crate::public_api_planar_diagnostics::contract_subject::causal_reference;
use crate::public_api_planar_projection_consumption::contract_subject::{
    projection_consumed_planar_parts, ProjectionConsumedPlanarParts,
};

pub(crate) struct M7ReadinessParts {
    pub(crate) readiness: PlanarContractBundleValidationReceipt,
    pub(crate) structural: PlanarStructuralIdentityReceipt,
    pub(crate) motion: PlanarMotionPostureReceipt,
    pub(crate) retained: RetainedPlanarFactsReceipt,
    pub(crate) projected: ProjectionConsumedPlanarFactsReceipt,
    pub(crate) recovery: PlanarRecoveryPostureReceipt,
    pub(crate) diagnostics: PlanarDiagnosticBundleReceipt,
}

pub(crate) fn m7_readiness_parts(world: &'static str) -> M7ReadinessParts {
    let projected_parts = projection_consumed_planar_parts(world);
    let projected = projected_receipt(world, &projected_parts);
    let retained = projected_parts.retained;
    let structural = retained.basis().structural_identity_receipt().clone();
    let motion = retained.basis().motion_posture_receipt().clone();
    let readiness = projected_parts.readiness;
    let recovery = recovery_receipt(world, retained.clone(), projected.clone());
    let diagnostics = diagnostics_receipt(
        world,
        recovery.clone(),
        retained.clone(),
        projected.clone(),
        motion.clone(),
    );
    M7ReadinessParts {
        readiness,
        structural,
        motion,
        retained,
        projected,
        recovery,
        diagnostics,
    }
}

pub(crate) fn bundle_contracts(
    world: &'static str,
) -> PlanarContractBundleValidationContracts<
    worth_spatial::facade::planar_contract_bundle::PlanarContractBundleValidationQueryWorld,
> {
    PlanarContractBundleValidationContracts::new(bundle_handle(world))
}

fn projected_receipt(
    world: &'static str,
    parts: &ProjectionConsumedPlanarParts,
) -> ProjectionConsumedPlanarFactsReceipt {
    ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
        .consume_bundle_projection_receipts(parts.projections.clone())
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(world),
        ))
        .expect("M7 projection-consumed plan")
        .consume()
        .expect("M7 projection-consumed receipt")
}

fn recovery_receipt(
    world: &'static str,
    retained: RetainedPlanarFactsReceipt,
    projected: ProjectionConsumedPlanarFactsReceipt,
) -> PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(PlanarRecoverySource::from_projection_denial(
        "denial:m7-projection-basis",
    ))
    .with_retained_planar_facts(retained)
    .with_projection_consumed_facts(projected)
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(world)))
    .expect("M7 recovery plan")
    .certify()
    .expect("M7 recovery receipt")
}

fn diagnostics_receipt(
    world: &'static str,
    recovery: PlanarRecoveryPostureReceipt,
    retained: RetainedPlanarFactsReceipt,
    projected: ProjectionConsumedPlanarFactsReceipt,
    motion: PlanarMotionPostureReceipt,
) -> PlanarDiagnosticBundleReceipt {
    PlanarDiagnosticBundle::explain_planar_failure(PlanarDiagnosticSubject::from_recovery_posture(
        recovery,
    ))
    .with_retained_planar_facts(retained)
    .with_projection_consumed_planar_facts(projected)
    .with_motion_posture(motion)
    .with_query_causal_inspection(causal_reference(world))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        world,
    )))
    .expect("M7 diagnostics plan")
    .certify()
    .expect("M7 diagnostics receipt")
}

fn projection_consumption_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ProjectionConsumedPlanarFactsQueryDomain)
        .with_operating_context(ProjectionConsumedPlanarFactsQueryWorld::new(world))
        .validate()
        .expect("validated M7 projection-consumption test domain")
        .admit()
        .expect("admitted M7 projection-consumption test domain")
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
        .expect("validated M7 recovery test domain")
        .admit()
        .expect("admitted M7 recovery test domain")
}

fn diagnostic_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarDiagnosticBundleQueryDomain)
        .with_operating_context(PlanarDiagnosticBundleQueryWorld::new(world))
        .validate()
        .expect("validated M7 diagnostic test domain")
        .admit()
        .expect("admitted M7 diagnostic test domain")
}
