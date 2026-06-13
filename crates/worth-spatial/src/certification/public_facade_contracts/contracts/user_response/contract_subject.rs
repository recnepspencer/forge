use worth_spatial::facade::planar_clean_fail_boundary::{
    PlanarDirtyInputKind, PlanarOpenInputKind,
};
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticSubject;
use worth_spatial::facade::planar_overlap::{
    CoplanarOverlapContractExtractor, CoplanarOverlapContractReceipt, CoplanarOverlapDenialKind,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateCoincidencePolicy,
};
use worth_spatial::facade::retained_replay_workload::{ReplayWorkload, RetainedArtifactSet};
use worth_spatial::facade::surface_support::{SurfaceFamily, SurfaceSupportWorkload};
use worth_spatial::facade::user_response::{
    WorthUserResponseReceipt, WorthUserResponseSource, WorthUserResponseWorkload,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};

use crate::public_api_planar_clean_fail_boundary::clean_fail_fixture::{
    certify_clean_fail_boundary, diagnostic, dirty_input_with_kind, dirty_recovery,
    open_input_with_kind, unbounded_recovery,
};
use crate::public_api_planar_overlap::metaboss::diagnostics::certify_tiny_rotation_diagnostic;
use crate::public_api_planar_overlap::metaboss::storm_extraction_subject::certify_projected_storm_extraction_bundle;
use crate::public_api_planar_overlap::proof_fixture::{
    overlap_contracts, overlap_face, NEIGHBORHOOD,
};
use crate::public_api_planar_predicate::proof_fixture::{admitted_handle, orient_basis};
use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
use crate::public_api_retained_replay_workload::contract_subject::{
    projection_consumed_receipt, retained_replay_parts,
};
use worth_kernel::workload_composition::{WorkloadCatalog, WorkloadTopologyBreadth};

pub(crate) fn admitted_response(world: &'static str) -> WorthUserResponseReceipt {
    let receipt = admitted_overlap_receipt(world);
    user_response(WorthUserResponseSource::from_overlap_receipt(&receipt))
}

pub(crate) fn policy_required_response(world: &'static str) -> WorthUserResponseReceipt {
    let receipt = policy_required_overlap_receipt(world);
    user_response(WorthUserResponseSource::from_overlap_receipt(&receipt))
}

pub(crate) fn admitted_overlap_receipt(world: &'static str) -> CoplanarOverlapContractReceipt {
    projected_storm_overlap_receipts(world)
        .into_iter()
        .find(|receipt| receipt.policy_required_exits().is_empty())
        .expect("projected storm should include an admitted overlap receipt")
}

pub(crate) fn policy_required_overlap_receipt(
    world: &'static str,
) -> CoplanarOverlapContractReceipt {
    projected_storm_overlap_receipts(world)
        .into_iter()
        .find(|receipt| !receipt.policy_required_exits().is_empty())
        .expect("projected storm should include a policy-required overlap receipt")
}

fn projected_storm_overlap_receipts(world: &'static str) -> Vec<CoplanarOverlapContractReceipt> {
    let built = WorkloadCatalog::coplanar_overlap_storm()
        .with_topology_breadth(WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 })
        .declared(format!("user response projected overlap storm {world}"))
        .build()
        .expect("user response projected overlap workload should build");
    certify_projected_storm_extraction_bundle(
        world,
        built.projected_workload(),
        built.transform_receipts(),
    )
    .receipts()
    .to_vec()
}

pub(crate) fn dirty_input_response(world: &'static str) -> WorthUserResponseReceipt {
    let source = "user-response:dirty:self-intersecting-loop";
    let receipt = certify_clean_fail_boundary(
        world,
        dirty_input_with_kind(world, source, PlanarDirtyInputKind::SelfIntersectingLoop),
        dirty_recovery(world, source),
        diagnostic(world, PlanarDiagnosticSubject::topology_failure(source)),
    );
    user_response(WorthUserResponseSource::from_clean_fail_boundary(&receipt))
}

pub(crate) fn unsupported_input_response(world: &'static str) -> WorthUserResponseReceipt {
    let source = "user-response:unsupported:open-planar-domain";
    let receipt = certify_clean_fail_boundary(
        world,
        open_input_with_kind(world, source, PlanarOpenInputKind::OpenPlanarDomain),
        unbounded_recovery(world, source),
        diagnostic(
            world,
            PlanarDiagnosticSubject::unsupported_planar_class(source),
        ),
    );
    user_response(WorthUserResponseSource::from_clean_fail_boundary(&receipt))
}

pub(crate) fn unsupported_surface_support_response(
    world: &'static str,
) -> WorthUserResponseReceipt {
    let unsupported = SurfaceSupportWorkload::for_bound_geometry(bound_cube_geometry(world))
        .declared("classify unsupported freeform surface for user response")
        .with_surface_family(SurfaceFamily::Freeform)
        .certify()
        .expect_err("freeform surface must remain unsupported in M6.5");
    let response = user_response(WorthUserResponseSource::from_unsupported_surface_support(
        &unsupported,
    ));
    let receipt = unsupported
        .receipt()
        .expect("unsupported surface support should expose posture receipt");
    assert_eq!(
        response.evidence().digest(),
        receipt.stage_identity().receipt_identity()
    );
    response
}

pub(crate) fn denied_movement_response(world: &'static str) -> WorthUserResponseReceipt {
    let first = overlap_face(
        world,
        "face:denied-left",
        "movement:user-response-stable",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
    );
    let moved = overlap_face(
        world,
        "face:denied-right",
        "movement:tiny-rotation-exits-coplanar-class",
        &[
            [2.0e-9, 0.0],
            [4.0e-9, 0.0],
            [4.0e-9, 2.0e-9],
            [2.0e-9, 2.0e-9],
        ],
    );
    let denial = match CoplanarOverlapContractExtractor::between(first, moved)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&overlap_contracts(world))
    {
        Ok(_) => panic!("movement mismatch should deny before extraction"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial.kind(),
        CoplanarOverlapDenialKind::MismatchedMovementRotationPosture
    );
    let diagnostic = certify_tiny_rotation_diagnostic(denial.reason());
    user_response(WorthUserResponseSource::from_overlap_denial(
        &denial,
        &diagnostic,
    ))
}

pub(crate) fn predicate_uncertain_response(world: &'static str) -> WorthUserResponseReceipt {
    let handle = admitted_handle(world);
    let basis = orient_basis(
        "movement:user-response-predicate-uncertain",
        [[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]],
    )
    .with_coincidence_policy(PlanarPredicateCoincidencePolicy::DenyCertifiedZeroBeforeRepair);
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    let error = planar_predicate_authority_facts(&entry, &handle)
        .expect_err("certified zero must require policy or repair before boolean work");
    user_response(WorthUserResponseSource::from_predicate_authority_error(
        &error,
    ))
}

pub(crate) fn integrity_mismatch_response(world: &'static str) -> WorthUserResponseReceipt {
    let retained_world = retained_replay_parts(world);
    let other_retained = projection_consumed_planar_parts("user-response-integrity-other");
    let drifted_projection = projection_consumed_receipt(world, &other_retained);
    let artifacts =
        RetainedArtifactSet::from_retained_planar_facts(retained_world.retained_parts.retained)
            .with_projection_consumed_facts(drifted_projection);
    let denial = ReplayWorkload::for_transformed_workload(retained_world.transformed)
        .declared("reject projection-consumed facts from another retained basis")
        .with_retained_artifacts(artifacts)
        .replay()
        .expect_err("retained/projection drift must deny replay");
    user_response(WorthUserResponseSource::from_unsupported_replay(&denial))
}

pub(crate) fn missing_evidence_response(world: &'static str) -> WorthUserResponseReceipt {
    let retained_world = retained_replay_parts(world);
    let denial = ReplayWorkload::for_transformed_workload(retained_world.transformed)
        .declared("reject retained replay without captured artifacts")
        .replay()
        .expect_err("missing retained artifacts must deny replay");
    user_response(WorthUserResponseSource::from_unsupported_replay(&denial))
}

pub(crate) fn user_response(source: WorthUserResponseSource) -> WorthUserResponseReceipt {
    WorthUserResponseWorkload::from_source(source)
        .declared("certify product-facing user response")
        .respond()
        .expect("user response should certify")
}

fn bound_cube_geometry(declaration: &str) -> BoundGeometryWorkload {
    let topology = topology::facade::TopologySeed::cube()
        .with_declaration(declaration)
        .build()
        .expect("cube topology seed should be admitted");

    GeometryBindingWorkload::for_topology_seed(&topology)
        .declared(format!("bind {declaration}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("complete planar geometry binding should admit")
}
