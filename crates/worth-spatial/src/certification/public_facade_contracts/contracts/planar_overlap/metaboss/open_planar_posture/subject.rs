use topology::facade::{TopologySeed, TopologySeedReceipt};
use worth_spatial::facade::open_planar_posture::{
    OpenPlanarPostureCase, OpenPlanarPostureError, OpenPlanarPostureReceipt,
    OpenPlanarPostureWorkload,
};
use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarCleanFailBoundary, PlanarCleanFailBoundaryContracts, PlanarCleanFailInput,
};
use worth_spatial::facade::planar_contracts::{
    planar_admission_matrix, PlanarAdmissionFamily, PlanarRuntimeConcern,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleReceipt,
    PlanarDiagnosticSubject,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoverySource,
};
use worth_spatial::facade::surface_support::{
    SurfaceFamily, SurfaceSupportWorkload, UnsupportedSurfaceSupport,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};
use worth_spatial::facade::workload_binding::{
    GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet, PlanarLoopCarrierSet,
};

use crate::public_api_planar_clean_fail_boundary::runtime_handles::{
    clean_fail_handle, diagnostic_handle, recovery_handle,
};
use crate::public_api_planar_motion_posture::contract_subject::cancellation_motion_receipt;

pub(crate) struct OpenPlanarPostureSubject {
    pub(crate) receipt: OpenPlanarPostureReceipt,
    pub(crate) user_outcome: WorthUserOutcome,
}

pub(crate) fn half_space_subject(world: &'static str) -> OpenPlanarPostureSubject {
    posture_subject(world, OpenPlanarPostureCase::PolicyRequiredHalfSpace)
}

pub(crate) fn posture_matrix(world: &'static str) -> Vec<OpenPlanarPostureSubject> {
    [
        OpenPlanarPostureCase::UnsupportedOpenSheet,
        OpenPlanarPostureCase::UnsupportedOpenWire,
        OpenPlanarPostureCase::PolicyRequiredHalfSpace,
        OpenPlanarPostureCase::PredicateUncertain,
        OpenPlanarPostureCase::BoundedOperatorIncompatibility,
        OpenPlanarPostureCase::IntegrityMismatch,
        OpenPlanarPostureCase::TransformDivergence,
    ]
    .into_iter()
    .map(|posture_case| posture_subject(world, posture_case))
    .collect()
}

pub(crate) fn bounded_surrogate_denials(world: &'static str) -> Vec<OpenPlanarPostureError> {
    [bounded_cube_surrogate, bounded_multi_face_surrogate]
        .into_iter()
        .map(|surrogate| bounded_surrogate_denial(world, surrogate))
        .collect()
}

fn bounded_surrogate_denial(
    world: &'static str,
    surrogate: fn(&'static str) -> TopologySeedReceipt,
) -> OpenPlanarPostureError {
    let topology = open_topology(world);
    let unsupported_surface = unsupported_surface(world, &topology);
    let boundary = clean_fail_boundary(world, &topology, OpenPlanarPostureCase::IntegrityMismatch);
    let preview = OpenPlanarPostureWorkload::from_open_topology(topology.clone())
        .declared(format!("MB-M6-6 bounded surrogate {world}"))
        .with_unsupported_surface_support(unsupported_surface.clone())
        .with_clean_fail_boundary(boundary.clone())
        .classify_as(OpenPlanarPostureCase::IntegrityMismatch)
        .posture_identity_preview()
        .expect("posture identity preview");
    let response = response_for_case(world, OpenPlanarPostureCase::IntegrityMismatch, &preview);

    OpenPlanarPostureWorkload::from_open_topology(topology)
        .declared(format!("MB-M6-6 bounded surrogate {world}"))
        .with_unsupported_surface_support(unsupported_surface)
        .with_clean_fail_boundary(boundary)
        .classify_as(OpenPlanarPostureCase::IntegrityMismatch)
        .with_attempted_bounded_surrogate(surrogate(world))
        .with_user_response(response)
        .certify()
        .expect_err("bounded surrogate must be rejected")
}

fn posture_subject(
    world: &'static str,
    posture_case: OpenPlanarPostureCase,
) -> OpenPlanarPostureSubject {
    let topology = open_topology_for_case(world, posture_case);
    let unsupported_surface = unsupported_surface(world, &topology);
    let boundary = clean_fail_boundary(world, &topology, posture_case);
    let preview = OpenPlanarPostureWorkload::from_open_topology(topology.clone())
        .declared(format!("MB-M6-6 open posture {world}"))
        .with_unsupported_surface_support(unsupported_surface.clone())
        .with_clean_fail_boundary(boundary.clone())
        .classify_as(posture_case)
        .posture_identity_preview()
        .expect("posture identity preview");
    let response = response_for_case(world, posture_case, &preview);
    let receipt = OpenPlanarPostureWorkload::from_open_topology(topology)
        .declared(format!("MB-M6-6 open posture {world}"))
        .with_unsupported_surface_support(unsupported_surface)
        .with_clean_fail_boundary(boundary)
        .classify_as(posture_case)
        .with_user_response(response)
        .certify()
        .expect("open posture receipt");
    let user_outcome = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_open_planar_posture(&receipt),
    )
    .declared(format!("open posture final response {world}"))
    .respond()
    .expect("open posture final response")
    .outcome()
    .clone();

    OpenPlanarPostureSubject {
        receipt,
        user_outcome,
    }
}

fn open_topology(world: &'static str) -> TopologySeedReceipt {
    TopologySeed::open_sheet()
        .with_declaration(format!("MB-M6-6 open sheet topology {world}"))
        .build()
        .expect("open sheet topology receipt")
}

fn open_wire_topology(world: &'static str) -> TopologySeedReceipt {
    TopologySeed::open_wire()
        .with_declaration(format!("MB-M6-6 open wire topology {world}"))
        .build()
        .expect("open wire topology receipt")
}

fn open_topology_for_case(
    world: &'static str,
    posture_case: OpenPlanarPostureCase,
) -> TopologySeedReceipt {
    match posture_case {
        OpenPlanarPostureCase::UnsupportedOpenWire => open_wire_topology(world),
        OpenPlanarPostureCase::UnsupportedOpenSheet
        | OpenPlanarPostureCase::PolicyRequiredHalfSpace
        | OpenPlanarPostureCase::PredicateUncertain
        | OpenPlanarPostureCase::BoundedOperatorIncompatibility
        | OpenPlanarPostureCase::IntegrityMismatch
        | OpenPlanarPostureCase::TransformDivergence => open_topology(world),
    }
}

fn bounded_cube_surrogate(world: &'static str) -> TopologySeedReceipt {
    TopologySeed::cube()
        .with_declaration(format!("bounded surrogate cube {world}"))
        .build()
        .expect("bounded surrogate cube topology")
}

fn bounded_multi_face_surrogate(world: &'static str) -> TopologySeedReceipt {
    TopologySeed::multi_face_shell(4)
        .with_declaration(format!("bounded surrogate multi face shell {world}"))
        .build()
        .expect("bounded surrogate multi face topology")
}

fn unsupported_surface(
    world: &'static str,
    topology: &TopologySeedReceipt,
) -> UnsupportedSurfaceSupport {
    let bound = GeometryBindingWorkload::for_topology_seed(topology)
        .declared(format!("MB-M6-6 open topology binding {world}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(topology))
        .admit()
        .expect("open topology binding");
    SurfaceSupportWorkload::for_bound_geometry(bound)
        .declared(format!("MB-M6-6 unsupported open support {world}"))
        .with_surface_family(SurfaceFamily::Freeform)
        .certify()
        .expect_err("freeform support should be unsupported")
}

fn clean_fail_boundary(
    world: &'static str,
    topology: &TopologySeedReceipt,
    posture_case: OpenPlanarPostureCase,
) -> worth_spatial::facade::planar_clean_fail_boundary::PlanarCleanFailBoundaryReceipt {
    let source_digest = topology
        .query_receipts()
        .declaration_receipt()
        .identity()
        .name()
        .to_string();
    let input = open_input_for_case(posture_case, source_digest.clone())
        .with_topology_identity(format!("open-topology-stays-open:{world}"))
        .with_transform_posture(cancellation_motion_receipt(world))
        .with_admission_row(
            planar_admission_matrix()
                .row(
                    PlanarAdmissionFamily::UnboundedPlanarDomain,
                    PlanarRuntimeConcern::DiagnosticsLocalization,
                )
                .expect("unbounded admission row")
                .clone(),
        );

    PlanarCleanFailBoundary::from_planar_input(input)
        .recovery_posture(open_recovery(world, source_digest.clone()))
        .diagnostics(open_diagnostic(world, source_digest, posture_case))
        .certify_clean_fail_boundary()
        .compile(&PlanarCleanFailBoundaryContracts::new(clean_fail_handle(
            world,
        )))
        .expect("open clean-fail plan")
        .certify()
        .expect("open clean-fail receipt")
}

fn open_input_for_case(
    posture_case: OpenPlanarPostureCase,
    source_digest: String,
) -> PlanarCleanFailInput {
    match posture_case {
        OpenPlanarPostureCase::PolicyRequiredHalfSpace => {
            PlanarCleanFailInput::unbounded_half_space(source_digest)
        }
        OpenPlanarPostureCase::UnsupportedOpenSheet
        | OpenPlanarPostureCase::UnsupportedOpenWire
        | OpenPlanarPostureCase::PredicateUncertain
        | OpenPlanarPostureCase::BoundedOperatorIncompatibility
        | OpenPlanarPostureCase::IntegrityMismatch
        | OpenPlanarPostureCase::TransformDivergence => {
            PlanarCleanFailInput::open_planar_domain(source_digest)
        }
    }
}

fn open_recovery(
    world: &'static str,
    source_digest: String,
) -> worth_spatial::facade::planar_recovery::PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(PlanarRecoverySource::unbounded_or_open(
        source_digest,
    ))
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(world)))
    .expect("open recovery plan")
    .certify()
    .expect("open recovery receipt")
}

fn open_diagnostic(
    world: &'static str,
    source_digest: String,
    posture_case: OpenPlanarPostureCase,
) -> PlanarDiagnosticBundleReceipt {
    PlanarDiagnosticBundle::explain_planar_failure(diagnostic_subject(source_digest, posture_case))
        .inspect_failure_locality()
        .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
            world,
        )))
        .expect("open diagnostic plan")
        .certify()
        .expect("open diagnostic receipt")
}

fn diagnostic_subject(
    source_digest: String,
    posture_case: OpenPlanarPostureCase,
) -> PlanarDiagnosticSubject {
    match posture_case {
        OpenPlanarPostureCase::UnsupportedOpenSheet
        | OpenPlanarPostureCase::UnsupportedOpenWire
        | OpenPlanarPostureCase::BoundedOperatorIncompatibility => {
            PlanarDiagnosticSubject::unsupported_planar_class(source_digest)
        }
        OpenPlanarPostureCase::PolicyRequiredHalfSpace => {
            PlanarDiagnosticSubject::policy_required(source_digest)
        }
        OpenPlanarPostureCase::PredicateUncertain => {
            PlanarDiagnosticSubject::predicate_failure(source_digest)
        }
        OpenPlanarPostureCase::IntegrityMismatch | OpenPlanarPostureCase::TransformDivergence => {
            PlanarDiagnosticSubject::unsupported_planar_class(source_digest)
        }
    }
}

fn response_for_case(
    world: &'static str,
    posture_case: OpenPlanarPostureCase,
    posture_identity: &str,
) -> worth_spatial::facade::user_response::WorthUserResponseReceipt {
    WorthUserResponseWorkload::from_source(WorthUserResponseSource::from_open_planar_posture_case(
        posture_case,
        posture_identity.to_string(),
    ))
    .declared(format!("open posture branch response {world}"))
    .respond()
    .expect("open posture branch response")
}
