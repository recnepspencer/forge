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

pub(crate) fn frame_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarLocalFrameCertificateQueryDomain)
        .with_operating_context(PlanarLocalFrameCertificateQueryWorld::new(world))
        .validate()
        .expect("validated local-frame handle")
        .admit()
        .expect("admitted local-frame handle")
}

pub(crate) fn precision_handle(
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

pub(crate) fn predicate_receipt(movement_rotation: &'static str) -> PlanarPredicateFactReceipt {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "local-frame-source",
        ))
        .validate()
        .expect("validated predicate handle")
        .admit()
        .expect("admitted predicate handle");
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:micro-feature-local-xy",
        "topology:thin-slot-loop",
        movement_rotation,
        "tolerance:micro-feature-exact",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    planar_predicate_authority_facts(&entry, &handle).expect("predicate receipt")
}

pub(crate) fn precision_receipt(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPrecisionCertificationQueryDomain,
        PlanarPrecisionCertificationQueryWorld,
    >,
    movement_rotation: &'static str,
) -> PlanarPrecisionCertificateReceipt {
    let predicate = predicate_receipt(movement_rotation);
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:micro-feature-local-xy")
        .topology_basis_identity("topology:thin-slot-loop")
        .movement_rotation_posture_identity(movement_rotation)
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(&predicate)
        .build()
        .expect("valid precision basis");
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate, basis),
    );
    planar_precision_certification_facts(&entry, handle).expect("precision receipt")
}

pub(crate) fn local_frame_basis(
    precision: &PlanarPrecisionCertificateReceipt,
    movement_rotation: &'static str,
    transform_chain: &'static str,
) -> PlanarLocalFrameBasis {
    PlanarLocalFrameBasis::builder()
        .frame_identity("frame:micro-feature-local-xy")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest(transform_chain)
        .movement_rotation_posture_identity(movement_rotation)
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .precision_receipt(precision)
        .build()
        .expect("valid local-frame basis")
}

pub(crate) fn local_frame_receipt(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarLocalFrameCertificateQueryDomain,
        PlanarLocalFrameCertificateQueryWorld,
    >,
    basis: PlanarLocalFrameBasis,
) -> PlanarLocalFrameCertificateReceipt {
    let entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(basis),
    );
    planar_local_frame_certificate_facts(&entry, handle).expect("local-frame receipt")
}
