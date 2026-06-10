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
    ProjectPointToCertifiedPlane2DQueryDomain, ProjectPointToCertifiedPlane2DQueryWorld,
    ProjectPointToCertifiedPlane2DReceipt,
};
use worth_spatial::facade::planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld,
};
use worth_spatial::facade::planar_winding::{
    CertifiedPolygonWinding2DContracts, CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld,
};

pub(crate) fn winding_contracts(
    world: &'static str,
) -> CertifiedPolygonWinding2DContracts<
    CertifiedPolygonWinding2DQueryWorld,
    CertifiedSegmentSegment2DQueryWorld,
    PlanarPredicateAuthorityQueryWorld,
> {
    CertifiedPolygonWinding2DContracts::new(
        winding_handle(world),
        CertifiedSegmentSegment2DContracts::new(segment_handle(world), predicate_handle()),
        predicate_handle(),
    )
}

pub(crate) fn loop_points(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    prefix: &'static str,
    points: &[[f64; 2]],
) -> Vec<ProjectPointToCertifiedPlane2DReceipt> {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            projected_point(
                world,
                frame,
                format!("{prefix}:corner:{index}"),
                [point[0], point[1], 0.0],
            )
        })
        .collect()
}

pub(crate) fn projected_point(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    identity: String,
    local_delta: [f64; 3],
) -> ProjectPointToCertifiedPlane2DReceipt {
    let basis = ProjectPointToCertifiedPlane2DBasis::builder()
        .source_point_identity(identity)
        .source_point([1.0e12, 0.0, 0.0])
        .source_point_basis_digest("point-basis:winding-local-normalized")
        .local_delta_from_frame_origin(local_delta)
        .local_frame_receipt(frame)
        .build()
        .expect("valid projected point basis");
    let entry = project_point_to_certified_plane_2d_entry(
        ProjectPointToCertifiedPlane2DCase::from_local_frame(basis),
    );
    project_point_to_certified_plane_2d_facts(&entry, &projection_handle(world))
        .expect("projection receipt")
}

pub(crate) fn certified_frame(
    world: &'static str,
    movement_rotation: &'static str,
) -> PlanarLocalFrameCertificateReceipt {
    let predicate = predicate_receipt(movement_rotation);
    let precision = precision_receipt(world, &predicate);
    let basis = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:winding-local-xy")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:winding")
        .movement_rotation_posture_identity(movement_rotation)
        .tolerance_policy_identity("tolerance:winding-exact")
        .precision_receipt(&precision)
        .build()
        .expect("valid local-frame basis");
    let entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(basis),
    );
    planar_local_frame_certificate_facts(&entry, &frame_handle(world)).expect("frame receipt")
}

fn predicate_receipt(movement_rotation: &'static str) -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:winding-local-xy",
        "topology:winding-fixture",
        movement_rotation,
        "tolerance:winding-exact",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    planar_predicate_authority_facts(&entry, &predicate_handle()).expect("predicate receipt")
}

fn precision_receipt(
    world: &'static str,
    predicate: &PlanarPredicateFactReceipt,
) -> PlanarPrecisionCertificateReceipt {
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:winding-local-xy")
        .topology_basis_identity("topology:winding-fixture")
        .movement_rotation_posture_identity(
            predicate.input_basis().movement_rotation_posture_identity(),
        )
        .tolerance_policy_identity("tolerance:winding-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(predicate)
        .build()
        .expect("valid precision basis");
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
    );
    planar_precision_certification_facts(&entry, &precision_handle(world))
        .expect("precision receipt")
}

fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new("winding-predicate"))
        .validate()
        .expect("validated predicate handle")
        .admit()
        .expect("admitted predicate handle")
}

fn winding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    CertifiedPolygonWinding2DQueryDomain,
    CertifiedPolygonWinding2DQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(CertifiedPolygonWinding2DQueryDomain)
        .with_operating_context(CertifiedPolygonWinding2DQueryWorld::new(world))
        .validate()
        .expect("validated winding handle")
        .admit()
        .expect("admitted winding handle")
}

fn segment_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    CertifiedSegmentSegment2DQueryDomain,
    CertifiedSegmentSegment2DQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(CertifiedSegmentSegment2DQueryDomain)
        .with_operating_context(CertifiedSegmentSegment2DQueryWorld::new(world))
        .validate()
        .expect("validated segment handle")
        .admit()
        .expect("admitted segment handle")
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
