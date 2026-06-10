use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld, PlanarLocalFrameCertificateReceipt,
};
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionBasis, PlanarPrecisionCertificateReceipt, PlanarPrecisionCertificationCase,
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};
use worth_spatial::facade::planar_projection::{
    project_point_to_certified_plane_2d_entry, project_point_to_certified_plane_2d_facts,
    ProjectPointToCertifiedPlane2DBasis, ProjectPointToCertifiedPlane2DCase,
    ProjectPointToCertifiedPlane2DDenialKind, ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld, ProjectPointToCertifiedPlane2DReceipt,
};

#[test]
fn kernel_consumes_spatial_project_point_to_certified_plane_2d_receipts_without_local_projection_synthesis(
) {
    let frame = local_frame_receipt(
        "kernel-projection-frame",
        "movement:kernel-rotation-cancelled",
        "transform:kernel-planar-chain",
    );
    let receipt = projection_receipt(
        "kernel-projection",
        projection_basis(&frame, "point:kernel-a", [1.0e-9, 0.0, 0.0]),
    );

    assert_eq!(receipt.point_2d(), [0.0, -1.0e-9]);
    assert_eq!(receipt.local_frame_fact_digest(), frame.fact_digest());
    assert_eq!(
        receipt.mutation_evidence().local_frame_fact_digest(),
        frame.fact_digest()
    );
    assert_eq!(
        receipt.mutation_evidence().fact_digest(),
        receipt.fact_digest()
    );
    assert_eq!(receipt.signed_distance_to_plane_bits(), 0.0f64.to_bits());
    assert_eq!(receipt.counters().projection_derivations(), 1);
    assert!(!receipt.declaration_digest().is_empty());
    assert!(!receipt.envelope_digest().is_empty());
    assert!(!receipt.fact_digest().is_empty());
}

#[test]
fn kernel_cannot_upgrade_missing_projection_basis_into_receipt() {
    let denial = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity("point:kernel-missing-frame")
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:kernel-local-normalized")
        .local_delta_from_frame_origin([1.0e-9, 0.0, 0.0])
        .movement_rotation_posture_identity("movement:kernel-rotation-cancelled")
        .tolerance_policy_identity("tolerance:kernel-micro")
        .build()
        .expect_err("kernel cannot synthesize missing projection basis");

    assert_eq!(
        denial.kind(),
        ProjectPointToCertifiedPlane2DDenialKind::MissingLocalFrameReceipt
    );
}

#[test]
fn kernel_projection_preserves_local_delta_and_frame_identity_under_scale_separation() {
    let base_frame = local_frame_receipt(
        "kernel-base-frame",
        "movement:kernel-rotation-cancelled",
        "transform:kernel-planar-chain",
    );
    let alternate_frame = local_frame_receipt(
        "kernel-alternate-frame",
        "movement:kernel-rotation-cancelled",
        "transform:kernel-alternate-planar-chain",
    );
    let base = projection_receipt(
        "kernel-base-projection",
        projection_basis(&base_frame, "point:kernel-scale", [1.0e-9, 0.0, 0.0]),
    );
    let alternate = projection_receipt(
        "kernel-alternate-projection",
        projection_basis(&alternate_frame, "point:kernel-scale", [1.0e-9, 0.0, 0.0]),
    );

    assert_eq!(base.basis().source_point(), [1.0e12, 0.0, 0.0]);
    assert_eq!(
        base.basis().local_delta_from_frame_origin(),
        [1.0e-9, 0.0, 0.0]
    );
    assert_eq!(base.point_2d(), [0.0, -1.0e-9]);
    assert_ne!(
        base.local_frame_fact_digest(),
        alternate.local_frame_fact_digest()
    );
    assert_ne!(base.fact_digest(), alternate.fact_digest());
}

fn projection_receipt(
    world: &'static str,
    basis: ProjectPointToCertifiedPlane2DBasis,
) -> ProjectPointToCertifiedPlane2DReceipt {
    let entry = project_point_to_certified_plane_2d_entry(
        ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
    );
    project_point_to_certified_plane_2d_facts(&entry, &projection_handle(world))
        .expect("projection receipt")
}

fn projection_basis(
    frame: &PlanarLocalFrameCertificateReceipt,
    source_point_identity: &'static str,
    local_delta: [f64; 3],
) -> ProjectPointToCertifiedPlane2DBasis {
    ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(source_point_identity)
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:kernel-local-normalized")
        .local_delta_from_frame_origin(local_delta)
        .local_frame_receipt(frame)
        .build()
        .expect("projection basis")
}

fn local_frame_receipt(
    world: &'static str,
    movement_rotation: &'static str,
    transform_chain: &'static str,
) -> PlanarLocalFrameCertificateReceipt {
    let precision = precision_receipt(world, movement_rotation);
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:kernel-projection")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest(transform_chain)
        .movement_rotation_posture_identity(movement_rotation)
        .tolerance_policy_identity("tolerance:kernel-micro")
        .precision_receipt(&precision)
        .build()
        .expect("local-frame basis");
    let entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(basis),
    );
    planar_local_frame_certificate_facts(&entry, &frame_handle(world)).expect("frame receipt")
}

fn precision_receipt(
    world: &'static str,
    movement_rotation: &'static str,
) -> PlanarPrecisionCertificateReceipt {
    let predicate = predicate_receipt(movement_rotation);
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:kernel-projection")
        .topology_basis_identity("topology:kernel-slot")
        .movement_rotation_posture_identity(movement_rotation)
        .tolerance_policy_identity("tolerance:kernel-micro")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(&predicate)
        .build()
        .expect("precision basis");
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate, basis),
    );
    planar_precision_certification_facts(&entry, &precision_handle(world))
        .expect("precision receipt")
}

fn predicate_receipt(movement_rotation: &'static str) -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:kernel-projection",
        "topology:kernel-slot",
        movement_rotation,
        "tolerance:kernel-micro",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    planar_predicate_authority_facts(&entry, &predicate_handle()).expect("predicate receipt")
}

fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "kernel-projection-source",
        ))
        .validate()
        .expect("validated predicate handle")
        .admit()
        .expect("admitted predicate handle")
}

fn precision_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPrecisionCertificationQueryDomain)
        .with_operating_context(PlanarPrecisionCertificationQueryWorld::new(world))
        .validate()
        .expect("validated precision handle")
        .admit()
        .expect("admitted precision handle")
}

fn frame_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarLocalFrameCertificateQueryDomain)
        .with_operating_context(PlanarLocalFrameCertificateQueryWorld::new(world))
        .validate()
        .expect("validated frame handle")
        .admit()
        .expect("admitted frame handle")
}

fn projection_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    ProjectPointToCertifiedPlane2DQueryDomain,
    ProjectPointToCertifiedPlane2DQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ProjectPointToCertifiedPlane2DQueryDomain)
        .with_operating_context(ProjectPointToCertifiedPlane2DQueryWorld::new(world))
        .validate()
        .expect("validated projection handle")
        .admit()
        .expect("admitted projection handle")
}
