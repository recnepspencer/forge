use topology::facade::{TopologySeed, TopologySeedCleanFailReceipt};
use worth_spatial::facade::dirty_planar_clean_fail::{
    DirtyPlanarCleanFailCase, DirtyPlanarCleanFailReceipt, DirtyPlanarCleanFailWorkload,
};
use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarCleanFailBoundary, PlanarCleanFailBoundaryContracts, PlanarCleanFailInput,
    PlanarDirtyInputKind,
};
use worth_spatial::facade::planar_contracts::{
    planar_admission_matrix, PlanarAdmissionFamily, PlanarRuntimeConcern,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleReceipt,
    PlanarDiagnosticSubject, PlanarDiagnosticTriggerLocality,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoverySource,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

use crate::public_api_planar_clean_fail_boundary::runtime_handles::{
    clean_fail_handle, diagnostic_handle, recovery_handle,
};
use crate::public_api_planar_diagnostics::contract_subject::{causal_reference, topology_surface};
use crate::public_api_planar_motion_posture::contract_subject::cancellation_motion_receipt;

pub(crate) struct DirtyPlanarCleanFailSubject {
    pub(crate) receipt: DirtyPlanarCleanFailReceipt,
    pub(crate) user_outcome: WorthUserOutcome,
}

pub(crate) fn dirty_clean_fail_with_topology_seed(
    world: &'static str,
    dirty_case: DirtyPlanarCleanFailCase,
) -> DirtyPlanarCleanFailSubject {
    let topology_clean_fail = topology_clean_fail_for_case(world, dirty_case);
    let topology_identity = topology_clean_fail.clean_fail_identity();
    let dirty_kind = dirty_kind_for_case(dirty_case);
    let boundary = clean_fail_boundary(world, topology_identity.clone(), dirty_kind);
    let boundary_response = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_clean_fail_boundary(&boundary),
    )
    .declared(format!("boundary response for dirty clean-fail {world}"))
    .respond()
    .expect("dirty boundary response receipt");
    let receipt = DirtyPlanarCleanFailWorkload::from_topology_clean_fail(topology_clean_fail)
        .declared(format!("MB-M6-5 dirty clean-fail {world}"))
        .with_clean_fail_boundary(boundary)
        .with_user_response(boundary_response)
        .certify()
        .expect("dirty clean-fail workload receipt");
    let user_outcome = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_dirty_planar_clean_fail(&receipt),
    )
    .declared(format!("dirty clean-fail response {world}"))
    .respond()
    .expect("dirty response receipt")
    .outcome()
    .clone();

    DirtyPlanarCleanFailSubject {
        receipt,
        user_outcome,
    }
}

pub(crate) fn dirty_clean_fail_outcome_matrix(world: &'static str) -> Vec<WorthUserOutcome> {
    dirty_clean_fail_subject_matrix(world)
        .into_iter()
        .map(|subject| subject.user_outcome)
        .collect()
}

pub(crate) fn dirty_clean_fail_subject_matrix(
    world: &'static str,
) -> Vec<DirtyPlanarCleanFailSubject> {
    [
        DirtyPlanarCleanFailCase::SelfIntersectingLoop,
        DirtyPlanarCleanFailCase::NonManifoldWire,
        DirtyPlanarCleanFailCase::ThinWallOrInvalidLocalBasis,
        DirtyPlanarCleanFailCase::OrientationInconsistency,
    ]
    .into_iter()
    .map(|dirty_case| dirty_clean_fail_with_topology_seed(world, dirty_case))
    .collect()
}

pub(crate) fn dirty_transform_pressure_subject(world: &'static str) -> DirtyPlanarCleanFailSubject {
    dirty_clean_fail_with_topology_seed(world, DirtyPlanarCleanFailCase::NonManifoldWire)
}

pub(crate) fn dirty_clean_fail_rejects_wrong_user_response(
    world: &'static str,
) -> worth_spatial::facade::dirty_planar_clean_fail::DirtyPlanarCleanFailError {
    let topology_clean_fail =
        topology_clean_fail_for_case(world, DirtyPlanarCleanFailCase::SelfIntersectingLoop);
    let topology_identity = topology_clean_fail.clean_fail_identity();
    let boundary = clean_fail_boundary(
        world,
        topology_identity,
        PlanarDirtyInputKind::SelfIntersectingLoop,
    );
    let wrong_response = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_dirty_planar_clean_fail_error(
            worth_spatial::facade::dirty_planar_clean_fail::DirtyPlanarCleanFailError::MissingTopologyCleanFail,
        ),
    )
    .declared(format!("wrong dirty clean-fail response {world}"))
    .respond()
    .expect("wrong dirty response still has a receipt");

    DirtyPlanarCleanFailWorkload::from_topology_clean_fail(topology_clean_fail)
        .declared(format!("MB-M6-5 wrong user response {world}"))
        .with_clean_fail_boundary(boundary)
        .with_user_response(wrong_response)
        .certify()
        .expect_err("dirty workload must reject response evidence from another source")
}

pub(crate) fn dirty_clean_fail_rejects_foreign_boundary_response(
    world: &'static str,
) -> worth_spatial::facade::dirty_planar_clean_fail::DirtyPlanarCleanFailError {
    let topology_clean_fail =
        topology_clean_fail_for_case(world, DirtyPlanarCleanFailCase::SelfIntersectingLoop);
    let topology_identity = topology_clean_fail.clean_fail_identity();
    let boundary = clean_fail_boundary(
        world,
        topology_identity,
        PlanarDirtyInputKind::SelfIntersectingLoop,
    );
    let foreign_boundary = clean_fail_boundary(
        "mb-m6-5-foreign-response",
        "foreign-dirty-topology-clean-fail".to_string(),
        PlanarDirtyInputKind::SelfIntersectingLoop,
    );
    let foreign_response = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_clean_fail_boundary(&foreign_boundary),
    )
    .declared(format!("foreign boundary response {world}"))
    .respond()
    .expect("foreign boundary response receipt");

    DirtyPlanarCleanFailWorkload::from_topology_clean_fail(topology_clean_fail)
        .declared(format!("MB-M6-5 foreign user response {world}"))
        .with_clean_fail_boundary(boundary)
        .with_user_response(foreign_response)
        .certify()
        .expect_err("dirty workload must reject dirty response evidence from another boundary")
}

pub(crate) fn stable_identity_mismatch_outcome(world: &'static str) -> WorthUserOutcome {
    let topology_clean_fail =
        topology_clean_fail_for_case(world, DirtyPlanarCleanFailCase::SelfIntersectingLoop);
    let topology_identity = topology_clean_fail.clean_fail_identity();
    let mut boundary = clean_fail_boundary(
        world,
        topology_identity,
        PlanarDirtyInputKind::SelfIntersectingLoop,
    );
    boundary = clean_fail_boundary_with_stable_identity_equal_to_topology(
        world,
        boundary,
        topology_clean_fail.clean_fail_identity(),
    );
    let boundary_response = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_clean_fail_boundary(&boundary),
    )
    .declared(format!("stable identity mismatch response {world}"))
    .respond()
    .expect("stable identity mismatch boundary response");
    let error = DirtyPlanarCleanFailWorkload::from_topology_clean_fail(topology_clean_fail)
        .declared(format!("MB-M6-5 stable identity mismatch {world}"))
        .with_clean_fail_boundary(boundary)
        .with_user_response(boundary_response)
        .certify()
        .expect_err("stable topology identity cannot hide dirty geometry");
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_dirty_planar_clean_fail_error(error),
    )
    .declared(format!("stable identity mismatch final response {world}"))
    .respond()
    .expect("stable identity mismatch response")
    .outcome()
    .clone()
}

fn topology_clean_fail_for_case(
    world: &'static str,
    dirty_case: DirtyPlanarCleanFailCase,
) -> TopologySeedCleanFailReceipt {
    let seed = match dirty_case {
        DirtyPlanarCleanFailCase::SelfIntersectingLoop => TopologySeed::self_intersecting_loop(),
        DirtyPlanarCleanFailCase::NonManifoldWire => TopologySeed::non_manifold_wire(),
        DirtyPlanarCleanFailCase::ThinWallOrInvalidLocalBasis => {
            TopologySeed::thin_wall_local_basis()
        }
        DirtyPlanarCleanFailCase::OrientationInconsistency => {
            TopologySeed::orientation_inconsistency()
        }
    };
    seed.with_declaration(format!("MB-M6-5 topology seed {world}"))
        .build()
        .expect_err("dirty topology seed must clean-fail")
}

fn clean_fail_boundary(
    world: &'static str,
    source_digest: String,
    dirty_kind: PlanarDirtyInputKind,
) -> worth_spatial::facade::planar_clean_fail_boundary::PlanarCleanFailBoundaryReceipt {
    let input = dirty_input_for_kind(source_digest.clone(), dirty_kind)
        .with_topology_identity(format!("stable-dirty-topology-id:{world}"))
        .with_transform_posture(cancellation_motion_receipt(world))
        .with_admission_row(
            planar_admission_matrix()
                .row(
                    PlanarAdmissionFamily::DirtyPlanarInput,
                    PlanarRuntimeConcern::DiagnosticsLocalization,
                )
                .expect("dirty admission row")
                .clone(),
        );
    PlanarCleanFailBoundary::from_planar_input(input)
        .recovery_posture(dirty_recovery(world, source_digest.clone()))
        .diagnostics(diagnostic(
            world,
            PlanarDiagnosticSubject::topology_failure(source_digest),
        ))
        .certify_clean_fail_boundary()
        .compile(&PlanarCleanFailBoundaryContracts::new(clean_fail_handle(
            world,
        )))
        .expect("dirty clean-fail boundary plan")
        .certify()
        .expect("dirty clean-fail boundary receipt")
}

fn clean_fail_boundary_with_stable_identity_equal_to_topology(
    world: &'static str,
    boundary: worth_spatial::facade::planar_clean_fail_boundary::PlanarCleanFailBoundaryReceipt,
    topology_identity: String,
) -> worth_spatial::facade::planar_clean_fail_boundary::PlanarCleanFailBoundaryReceipt {
    let source_digest = boundary.basis().input().source_digest().to_string();
    let input = dirty_input_for_kind(
        source_digest.clone(),
        boundary
            .basis()
            .input()
            .dirty_input_kind()
            .expect("dirty kind"),
    )
    .with_topology_identity(topology_identity)
    .with_transform_posture(cancellation_motion_receipt(world))
    .with_admission_row(
        planar_admission_matrix()
            .row(
                PlanarAdmissionFamily::DirtyPlanarInput,
                PlanarRuntimeConcern::DiagnosticsLocalization,
            )
            .expect("dirty admission row")
            .clone(),
    );
    PlanarCleanFailBoundary::from_planar_input(input)
        .recovery_posture(dirty_recovery(world, source_digest.clone()))
        .diagnostics(diagnostic(
            world,
            PlanarDiagnosticSubject::topology_failure(source_digest),
        ))
        .certify_clean_fail_boundary()
        .compile(&PlanarCleanFailBoundaryContracts::new(clean_fail_handle(
            world,
        )))
        .expect("stable identity clean-fail plan")
        .certify()
        .expect("stable identity clean-fail receipt")
}

fn dirty_recovery(
    world: &'static str,
    source_digest: String,
) -> worth_spatial::facade::planar_recovery::PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(PlanarRecoverySource::dirty_input(
        source_digest,
    ))
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(world)))
    .expect("dirty recovery plan")
    .certify()
    .expect("dirty recovery receipt")
}

fn diagnostic(
    world: &'static str,
    subject: PlanarDiagnosticSubject,
) -> PlanarDiagnosticBundleReceipt {
    let locality = subject.trigger_locality();
    let mut bundle = PlanarDiagnosticBundle::explain_planar_failure(subject);
    if locality == PlanarDiagnosticTriggerLocality::TopologyContract {
        bundle = bundle
            .with_topology_declared_surface(topology_surface(world))
            .with_query_causal_inspection(causal_reference(world));
    }
    bundle
        .inspect_failure_locality()
        .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
            world,
        )))
        .expect("dirty diagnostic plan")
        .certify()
        .expect("dirty diagnostic receipt")
}

fn dirty_input_for_kind(
    source_digest: String,
    dirty_kind: PlanarDirtyInputKind,
) -> PlanarCleanFailInput {
    match dirty_kind {
        PlanarDirtyInputKind::SelfIntersectingLoop => {
            PlanarCleanFailInput::dirty_planar_loop(source_digest)
        }
        PlanarDirtyInputKind::NonManifoldWire => {
            PlanarCleanFailInput::non_manifold_wire(source_digest)
        }
        PlanarDirtyInputKind::ThinWall => PlanarCleanFailInput::thin_wall(source_digest),
        PlanarDirtyInputKind::OrientationInconsistency => {
            PlanarCleanFailInput::orientation_inconsistency(source_digest)
        }
    }
}

fn dirty_kind_for_case(dirty_case: DirtyPlanarCleanFailCase) -> PlanarDirtyInputKind {
    match dirty_case {
        DirtyPlanarCleanFailCase::SelfIntersectingLoop => {
            PlanarDirtyInputKind::SelfIntersectingLoop
        }
        DirtyPlanarCleanFailCase::NonManifoldWire => PlanarDirtyInputKind::NonManifoldWire,
        DirtyPlanarCleanFailCase::ThinWallOrInvalidLocalBasis => PlanarDirtyInputKind::ThinWall,
        DirtyPlanarCleanFailCase::OrientationInconsistency => {
            PlanarDirtyInputKind::OrientationInconsistency
        }
    }
}
