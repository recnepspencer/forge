use forge_query::facade::ForgeQueryApplicationFacade;
use worth_kernel::workload_composition::{TransformRecipe, WorkloadCatalog};
use worth_spatial::facade::boolean_readiness_workload::{
    PlanarBooleanReadinessEvidenceBasis, PlanarBooleanReadinessWorkload,
    PlanarBooleanReadinessWorkloadReceipt,
};
use worth_spatial::facade::planar_contract_bundle::{
    PlanarContractBundleValidationContracts, PlanarM7ReadinessBundle,
    PlanarM7ReadinessSupportPosture,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld, PlanarDiagnosticBundleReceipt, PlanarDiagnosticSubject,
};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarReorientation,
};
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsQueryDomain, ProjectionConsumedPlanarFactsQueryWorld,
    ProjectionConsumedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld, PlanarRecoveryPostureReceipt, PlanarRecoverySource,
};
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFacts, RetainedPlanarFactsContracts, RetainedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts,
};
use worth_spatial::facade::projection_fact_parity::{
    ProjectionFactParityEvidenceBasis, ProjectionFactParityWorkload,
};

use super::contract_bundle_support::{
    bundle_handle, motion_posture_handle, readiness_receipt, retained_planar_handle,
    structural_identity_handle,
};
use super::local_rebuild_fixture::local_rebuild_receipt;

pub(crate) fn certified_boolean_readiness_workload_receipt(
    world: &'static str,
) -> PlanarBooleanReadinessWorkloadReceipt {
    let bundle_receipt = readiness_receipt();
    let retained = retained_planar_facts(bundle_receipt.clone());
    let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
        .consume_bundle_projection_receipts(bundle_receipt.basis().projection_receipts().to_vec())
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(),
        ))
        .expect("projection-consumed planar facts plan")
        .consume()
        .expect("projection-consumed planar facts receipt");
    let recovery = recovery_receipt(world, retained.clone(), projected.clone());
    let diagnostics = diagnostics_receipt(world, retained.clone(), projected.clone());
    let local_rebuild = local_rebuild_receipt(
        world,
        retained.clone(),
        projected.clone(),
        recovery.clone(),
        diagnostics.clone(),
    );
    let ledger = workload_ledger(world);
    let parity = ProjectionFactParityWorkload::from_evidence_basis(
        ProjectionFactParityEvidenceBasis::from_evidence_ledger(ledger.clone())
            .with_live_lane_from_ledger()
            .with_projected_lane_from_ledger()
            .with_projection_consumed_facts(&projected)
            .with_retained_workload(&retained)
            .with_replay(
                &retained
                    .historical_replay(&retained.replay_subject())
                    .expect("historical replay from retained facts"),
            )
            .with_transformed_lane_from_ledger()
            .with_recovery(&recovery)
            .with_local_rebuild(&local_rebuild)
            .with_diagnostics(&diagnostics),
    )
    .declared(format!("phase-1 boolean parity {world}"))
    .compare_lanes()
    .certify()
    .expect("projection fact parity receipt");
    let readiness_bundle = PlanarM7ReadinessBundle::from_certified_planar_bundle(bundle_receipt)
        .with_structural_identity(retained.basis().structural_identity_receipt().clone())
        .with_motion_posture(retained.basis().motion_posture_receipt().clone())
        .with_retained_planar_facts(retained)
        .with_projection_consumed_facts(projected)
        .with_recovery_posture(recovery)
        .with_diagnostics(diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "Milestone 7.0 keeps boolean execution support-gated while declaration lanes harden",
        ));
    let basis = PlanarBooleanReadinessEvidenceBasis::from_real_workload_evidence(ledger, readiness_bundle, parity);

    PlanarBooleanReadinessWorkload::from_real_workload_evidence(basis)
        .declared(format!("phase-1 planar boolean readiness workload {world}"))
        .certify_pre_boolean_readiness(&PlanarContractBundleValidationContracts::new(
            bundle_handle(),
        ))
        .expect("boolean readiness workload receipt")
}

fn workload_ledger(
    world: &'static str,
) -> worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger {
    WorkloadCatalog::coplanar_overlap_storm()
        .with_transform(TransformRecipe::HostileCancellation)
        .with_retained_replay_artifacts()
        .declared(format!("phase-1 boolean workload ledger {world}"))
        .build()
        .expect("catalog-backed workload ledger")
        .workload()
        .evidence_ledger()
        .clone()
}

fn retained_planar_facts(
    bundle_receipt: worth_spatial::facade::planar_contract_bundle::PlanarContractBundleValidationReceipt,
) -> RetainedPlanarFactsReceipt {
    let topology_contract = bundle_receipt.basis().topology_contract_receipt().clone();
    let motion = PlanarMotionPosture::from_boolean_readiness(bundle_receipt.clone())
        .after_exact_translation("motion:phase-1-translate")
        .after_exact_rotation("motion:phase-1-rotation")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle()))
        .expect("motion posture plan")
        .certify()
        .expect("motion posture receipt");
    let structural = PlanarStructuralIdentity::from_boolean_readiness(bundle_receipt.clone())
        .with_motion_posture(motion.clone())
        .with_topology_identity("topology:phase-1-projection-consumed")
        .with_persistent_name("name:phase-1-projection-consumed")
        .with_binding_identity("binding:phase-1-projection-consumed")
        .with_lineage_identity("lineage:phase-1-projection-consumed")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(),
        ))
        .expect("structural identity plan")
        .certify()
        .expect("structural identity receipt");
    RetainedPlanarFacts::from_boolean_readiness(bundle_receipt)
        .retain_planar_classification()
        .retain_structural_identity(structural)
        .retain_motion_posture(motion)
        .retain_topology_contract(topology_contract)
        .compile(&RetainedPlanarFactsContracts::new(retained_planar_handle()))
        .expect("retained planar facts plan")
        .retain()
        .expect("retained planar facts receipt")
}

fn recovery_receipt(
    world: &'static str,
    retained: RetainedPlanarFactsReceipt,
    projected: ProjectionConsumedPlanarFactsReceipt,
) -> PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_retained_or_projection_basis_denial(format!(
            "phase-1 boolean recovery:{world}"
        )),
    )
    .with_retained_planar_facts(retained)
    .with_projection_consumed_facts(projected)
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(world)))
    .expect("recovery posture plan")
    .certify()
    .expect("recovery posture receipt")
}

fn diagnostics_receipt(
    world: &'static str,
    retained: RetainedPlanarFactsReceipt,
    projected: ProjectionConsumedPlanarFactsReceipt,
) -> PlanarDiagnosticBundleReceipt {
    PlanarDiagnosticBundle::explain_planar_failure(PlanarDiagnosticSubject::binding_failure(
        format!("phase-1 boolean diagnostic:{world}"),
    ))
    .with_retained_planar_facts(retained)
    .with_projection_consumed_planar_facts(projected)
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        world,
    )))
    .expect("diagnostic bundle plan")
    .certify()
    .expect("diagnostic bundle receipt")
}

fn projection_consumption_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ProjectionConsumedPlanarFactsQueryDomain)
        .with_operating_context(ProjectionConsumedPlanarFactsQueryWorld::new(
            "phase-1-projection-consumed",
        ))
        .validate()
        .expect("validated projection consumption domain")
        .admit()
        .expect("admitted projection consumption domain")
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
        .expect("validated diagnostic domain")
        .admit()
        .expect("admitted diagnostic domain")
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
        .expect("validated recovery domain")
        .admit()
        .expect("admitted recovery domain")
}
