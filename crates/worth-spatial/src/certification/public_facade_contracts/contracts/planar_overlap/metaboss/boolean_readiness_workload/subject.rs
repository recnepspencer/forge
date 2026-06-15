use topology::facade::TopologySeed;
use worth_spatial::facade::boolean_readiness_workload::{
    PlanarBooleanReadinessEvidenceBasis, PlanarBooleanReadinessWorkload,
    PlanarBooleanReadinessWorkloadDenial, PlanarBooleanReadinessWorkloadReceipt,
};
use worth_spatial::facade::planar_contract_bundle::{
    PlanarContractBundleValidationContracts, PlanarM7ReadinessBundle,
    PlanarM7ReadinessSupportPosture,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleReceipt,
    PlanarDiagnosticSubject,
};
use worth_spatial::facade::planar_motion_posture::PlanarMotionPostureReceipt;
use worth_spatial::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt;
use worth_spatial::facade::planar_recovery::PlanarRecoveryPostureReceipt;
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use worth_spatial::facade::projection_fact_parity::{
    ProjectionFactParityLane, ProjectionFactParityLaneStatus, ProjectionFactParityWorkload,
};
use worth_spatial::facade::surface_support::{
    SurfaceFamily, SurfaceSupportWorkload, UnsupportedSurfaceSupportReceipt,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};
use worth_spatial::facade::workload_binding::{
    GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet, PlanarLoopCarrierSet,
};

use super::runtime_handles::bundle_handle;
use crate::public_api_planar_diagnostics::contract_subject::causal_reference;
use crate::public_api_planar_overlap::metaboss::dirty_planar_clean_fail::subject::dirty_clean_fail_with_topology_seed;
use crate::public_api_planar_overlap::metaboss::projection_fact_parity::catalog::ProjectionParityCatalog;
use crate::public_api_planar_overlap::metaboss::projection_fact_parity::runtime_handles::diagnostic_handle;
use crate::public_api_planar_overlap::metaboss::projection_fact_parity::subject::{
    admitted_basis, local_neighborhood_receipt, local_rebuild_receipt, real_parity_parts,
    RealParityParts,
};
use worth_spatial::facade::dirty_planar_clean_fail::{
    DirtyPlanarCleanFailCase, DirtyPlanarCleanFailReceipt,
};

#[derive(Debug)]
pub(crate) struct BooleanReadinessFinalBossSubject {
    pub(crate) receipt: PlanarBooleanReadinessWorkloadReceipt,
    pub(crate) user_outcome: WorthUserOutcome,
}

pub(crate) fn certify_final_boss(world: &'static str) -> BooleanReadinessFinalBossSubject {
    run_final_boss(world, |basis| basis).expect("final boss readiness receipt")
}

pub(crate) fn policy_required_final_boss(
    world: &'static str,
) -> PlanarBooleanReadinessWorkloadDenial {
    let parts = final_boss_parts(world);
    let parity_denial =
        ProjectionFactParityWorkload::from_evidence_basis(admitted_basis(&parts).with_lane_status(
            ProjectionFactParityLane::ProjectionConsumed,
            ProjectionFactParityLaneStatus::PolicyRequired,
        ))
        .declared(format!("MB-M6-8 policy parity blocker {world}"))
        .compare_lanes()
        .certify()
        .expect_err("policy-required parity blocker");
    denied_final_boss(world, |basis| {
        basis.with_policy_required_projection_parity_denial(&parity_denial)
    })
}

pub(crate) fn clean_failure_final_boss(
    world: &'static str,
) -> (
    PlanarBooleanReadinessWorkloadDenial,
    DirtyPlanarCleanFailReceipt,
) {
    let dirty_receipt =
        dirty_clean_fail_with_topology_seed(world, DirtyPlanarCleanFailCase::SelfIntersectingLoop)
            .receipt;
    let denial = denied_final_boss(world, |basis| basis.with_clean_failure(&dirty_receipt));
    (denial, dirty_receipt)
}

pub(crate) fn unsupported_final_boss(
    world: &'static str,
) -> (
    PlanarBooleanReadinessWorkloadDenial,
    UnsupportedSurfaceSupportReceipt,
) {
    let unsupported_receipt = unsupported_surface_support_receipt(world);
    let denial = denied_final_boss(world, |basis| {
        basis.with_unsupported_surface_support(&unsupported_receipt)
    });
    (denial, unsupported_receipt)
}

pub(crate) fn predicate_uncertain_final_boss(
    world: &'static str,
) -> (
    PlanarBooleanReadinessWorkloadDenial,
    PlanarDiagnosticBundleReceipt,
) {
    let diagnostics = predicate_uncertainty_diagnostics(world);
    let denial = denied_final_boss(world, |basis| {
        basis.with_predicate_uncertainty_diagnostics(&diagnostics)
    });
    (denial, diagnostics)
}

pub(crate) fn orientation_flip_final_boss(
    world: &'static str,
) -> (
    PlanarBooleanReadinessWorkloadDenial,
    PlanarDiagnosticBundleReceipt,
) {
    let diagnostics = orientation_flip_diagnostics(world);
    let denial = denied_final_boss(world, |basis| {
        basis.with_orientation_flip_diagnostics(&diagnostics)
    });
    (denial, diagnostics)
}

pub(crate) fn kernel_summary_substitution_final_boss(
    world: &'static str,
) -> PlanarBooleanReadinessWorkloadDenial {
    denied_final_boss(world, |basis| {
        basis.with_rejected_kernel_summary_substitution(
            "Kernel summaries cannot replace spatial and topology readiness receipts.",
        )
    })
}

pub(crate) fn mismatched_parity_final_boss(
    world: &'static str,
) -> PlanarBooleanReadinessWorkloadDenial {
    let parts = final_boss_parts(world);
    let foreign = final_boss_parts_for("mb-m6-8-foreign-parity", ProjectionParityCatalog::Cube);
    let parity = ProjectionFactParityWorkload::from_evidence_basis(admitted_basis(&foreign))
        .declared("MB-M6-8 foreign parity")
        .compare_lanes()
        .certify()
        .expect("foreign parity receipt");
    let basis = PlanarBooleanReadinessEvidenceBasis::from_real_workload_evidence(
        parts.ledger.clone(),
        readiness_bundle(&parts),
        parity,
    );
    certify_workload(world, basis).expect_err("foreign parity must not certify")
}

pub(crate) fn recovery_replay_mismatch_final_boss(
    world: &'static str,
) -> PlanarBooleanReadinessWorkloadDenial {
    let parts = final_boss_parts(world);
    let foreign = final_boss_parts_for(
        "mb-m6-8-foreign-readiness",
        ProjectionParityCatalog::CoplanarOverlapStorm,
    );
    let parity = parity_receipt(&parts);
    let basis = PlanarBooleanReadinessEvidenceBasis::from_real_workload_evidence(
        parts.ledger.clone(),
        readiness_bundle_with_foreign_recovery(&parts, &foreign),
        parity,
    );
    certify_workload(world, basis).expect_err("foreign readiness bundle must not certify")
}

fn denied_final_boss(
    world: &'static str,
    mutate: impl FnOnce(PlanarBooleanReadinessEvidenceBasis) -> PlanarBooleanReadinessEvidenceBasis,
) -> PlanarBooleanReadinessWorkloadDenial {
    run_final_boss(world, mutate).expect_err("final boss blocker must deny")
}

fn run_final_boss(
    world: &'static str,
    mutate: impl FnOnce(PlanarBooleanReadinessEvidenceBasis) -> PlanarBooleanReadinessEvidenceBasis,
) -> Result<BooleanReadinessFinalBossSubject, PlanarBooleanReadinessWorkloadDenial> {
    let parts = final_boss_parts(world);
    let parity = parity_receipt(&parts);
    let basis = mutate(
        PlanarBooleanReadinessEvidenceBasis::from_real_workload_evidence(
            parts.ledger.clone(),
            readiness_bundle(&parts),
            parity,
        ),
    );
    let receipt = certify_workload(world, basis)?;
    let user_outcome = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_boolean_readiness_workload(&receipt),
    )
    .declared("explain final boolean-readiness")
    .respond()
    .expect("final readiness response")
    .outcome()
    .clone();
    Ok(BooleanReadinessFinalBossSubject {
        receipt,
        user_outcome,
    })
}

fn certify_workload(
    world: &'static str,
    basis: PlanarBooleanReadinessEvidenceBasis,
) -> Result<PlanarBooleanReadinessWorkloadReceipt, PlanarBooleanReadinessWorkloadDenial> {
    PlanarBooleanReadinessWorkload::from_real_workload_evidence(basis)
        .declared(format!("MB-M6-8 boolean-readiness final boss {world}"))
        .certify_pre_boolean_readiness(&PlanarContractBundleValidationContracts::new(
            bundle_handle(world),
        ))
}

fn readiness_bundle(parts: &RealParityParts) -> PlanarM7ReadinessBundle {
    PlanarM7ReadinessBundle::from_certified_planar_bundle(
        parts.retained.basis().boolean_readiness_receipt().clone(),
    )
    .with_structural_identity(parts.retained.basis().structural_identity_receipt().clone())
    .with_motion_posture(parts.retained.basis().motion_posture_receipt().clone())
    .with_retained_planar_facts(parts.retained.clone())
    .with_projection_consumed_facts(parts.projected.clone())
    .with_recovery_posture(parts.recovery.clone())
    .with_diagnostics(parts.diagnostics.clone())
    .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
        "M7 boolean split/classify/assemble is support-gated until Milestone 7",
    ))
}

fn readiness_bundle_with_foreign_recovery(
    parts: &RealParityParts,
    foreign: &RealParityParts,
) -> PlanarM7ReadinessBundle {
    PlanarM7ReadinessBundle::from_certified_planar_bundle(
        parts.retained.basis().boolean_readiness_receipt().clone(),
    )
    .with_structural_identity(parts.retained.basis().structural_identity_receipt().clone())
    .with_motion_posture(parts.retained.basis().motion_posture_receipt().clone())
    .with_retained_planar_facts(parts.retained.clone())
    .with_projection_consumed_facts(parts.projected.clone())
    .with_recovery_posture(foreign.recovery.clone())
    .with_diagnostics(parts.diagnostics.clone())
    .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
        "M7 boolean split/classify/assemble is support-gated until Milestone 7",
    ))
}

fn parity_receipt(
    parts: &RealParityParts,
) -> worth_spatial::facade::projection_fact_parity::ProjectionFactParityReceipt {
    ProjectionFactParityWorkload::from_evidence_basis(admitted_basis(parts))
        .declared("MB-M6-8 projection parity input")
        .compare_lanes()
        .certify()
        .expect("projection parity input")
}

fn final_boss_parts(world: &'static str) -> RealParityParts {
    final_boss_parts_for(world, ProjectionParityCatalog::CoplanarOverlapStorm)
}

fn final_boss_parts_for(world: &'static str, catalog: ProjectionParityCatalog) -> RealParityParts {
    let mut parts = real_parity_parts(world, catalog);
    parts.diagnostics = readiness_diagnostics(
        world,
        parts.recovery.clone(),
        parts.retained.clone(),
        parts.projected.clone(),
        parts.retained.basis().motion_posture_receipt().clone(),
    );
    parts.local_rebuild = local_rebuild_receipt(
        world,
        parts.retained.clone(),
        parts.projected.clone(),
        parts.recovery.clone(),
        parts.diagnostics.clone(),
        local_neighborhood_receipt(world),
    );
    parts
}

fn readiness_diagnostics(
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
    .expect("final boss diagnostics plan")
    .certify()
    .expect("final boss diagnostics receipt")
}

fn orientation_flip_diagnostics(world: &'static str) -> PlanarDiagnosticBundleReceipt {
    let parts = final_boss_parts(world);
    let motion = parts.retained.basis().motion_posture_receipt().clone();
    PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::retained_transform_failure(format!(
            "mb-m6-8-orientation-flip-step:{world}"
        )),
    )
    .with_retained_planar_facts(parts.retained)
    .with_projection_consumed_planar_facts(parts.projected)
    .with_motion_posture(motion)
    .with_query_causal_inspection(causal_reference(world))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        world,
    )))
    .expect("orientation flip diagnostics plan")
    .certify()
    .expect("orientation flip diagnostics receipt")
}

fn predicate_uncertainty_diagnostics(world: &'static str) -> PlanarDiagnosticBundleReceipt {
    let parts = final_boss_parts(world);
    let motion = parts.retained.basis().motion_posture_receipt().clone();
    PlanarDiagnosticBundle::explain_planar_failure(PlanarDiagnosticSubject::predicate_failure(
        format!("mb-m6-8-predicate-uncertainty:{world}"),
    ))
    .with_retained_planar_facts(parts.retained)
    .with_projection_consumed_planar_facts(parts.projected)
    .with_motion_posture(motion)
    .with_query_causal_inspection(causal_reference(world))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        world,
    )))
    .expect("predicate uncertainty diagnostics plan")
    .certify()
    .expect("predicate uncertainty diagnostics receipt")
}

fn unsupported_surface_support_receipt(world: &'static str) -> UnsupportedSurfaceSupportReceipt {
    let topology = TopologySeed::cube()
        .with_declaration(format!("MB-M6-8 unsupported topology {world}"))
        .build()
        .expect("unsupported branch uses admitted topology before surface denial");
    let bound_geometry = GeometryBindingWorkload::for_topology_seed(&topology)
        .declared(format!("MB-M6-8 unsupported binding {world}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("unsupported branch uses admitted binding before surface denial");
    SurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared(format!("MB-M6-8 unsupported surface support {world}"))
        .with_surface_family(SurfaceFamily::GeneratedFeature)
        .certify()
        .expect_err("generated feature surface must deny before boolean readiness")
        .receipt()
        .expect("unsupported surface support must produce a receipt")
        .clone()
}
