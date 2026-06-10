use forge_query::facade::ForgeQueryApplicationFacade;
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
        .expect("validated planar precision handle")
        .admit()
        .expect("admitted planar precision handle")
}

pub(crate) fn predicate_receipt(
    movement_rotation: &'static str,
    projected_points: [[f64; 2]; 3],
) -> PlanarPredicateFactReceipt {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new("precision-source"))
        .validate()
        .expect("validated predicate handle")
        .admit()
        .expect("admitted predicate handle");
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:micro-feature-local-xy",
        "topology:thin-slot-loop",
        movement_rotation,
        "tolerance:micro-feature-exact",
        projected_points,
    );
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    planar_predicate_authority_facts(&entry, &handle).expect("predicate receipt")
}

pub(crate) fn precision_basis(receipt: &PlanarPredicateFactReceipt) -> PlanarPrecisionBasis {
    PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:micro-feature-local-xy")
        .topology_basis_identity("topology:thin-slot-loop")
        .movement_rotation_posture_identity(
            receipt.input_basis().movement_rotation_posture_identity(),
        )
        .tolerance_policy_identity("tolerance:micro-feature-exact")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(receipt)
        .build()
        .expect("valid precision basis")
}

pub(crate) fn precision_receipt_for(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPrecisionCertificationQueryDomain,
        PlanarPrecisionCertificationQueryWorld,
    >,
    predicate: PlanarPredicateFactReceipt,
) -> PlanarPrecisionCertificateReceipt {
    let basis = precision_basis(&predicate);
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate, basis),
    );
    planar_precision_certification_facts(&entry, handle).expect("precision receipt")
}
