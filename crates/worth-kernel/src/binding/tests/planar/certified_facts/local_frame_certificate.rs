use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_local_frame::{
    planar_local_frame_certificate_entry, planar_local_frame_certificate_facts,
    PlanarLocalFrameBasis, PlanarLocalFrameCertificateCase, PlanarLocalFrameCertificateQueryDomain,
    PlanarLocalFrameCertificateQueryWorld, PlanarLocalFrameDenialKind,
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

#[test]
fn kernel_consumes_spatial_planar_local_frame_receipts_without_local_frame_synthesis() {
    let precision = precision_receipt("kernel-frame", "movement:kernel-rotation-cancelled");
    let basis = local_frame_basis(
        &precision,
        "movement:kernel-rotation-cancelled",
        "transform:kernel-planar-chain",
    )
    .expect("basis");
    let receipt = local_frame_receipt("kernel-frame", basis);

    assert_eq!(receipt.frame_identity(), "frame:kernel-micro");
    assert_eq!(receipt.precision_fact_digest(), precision.fact_digest());
    assert_eq!(receipt.scale_separation_orders(), 21);
    assert_eq!(receipt.basis().w_axis(), [0.0, 0.0, 1.0]);
    assert_eq!(receipt.counters().local_frame_derivations(), 1);
    assert_eq!(receipt.counters().retained_precision_receipts_consumed(), 1);
    assert_eq!(receipt.counters().normalization_basis_count(), 1);
    assert!(!receipt.declaration_digest().is_empty());
    assert!(!receipt.envelope_digest().is_empty());
    assert!(!receipt.fact_digest().is_empty());
}

#[test]
fn kernel_cannot_upgrade_missing_local_frame_basis_into_receipt() {
    let denial = PlanarLocalFrameBasis::builder()
        .frame_identity("frame:kernel-micro")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest("transform:kernel-planar-chain")
        .movement_rotation_posture_identity("movement:kernel-rotation-cancelled")
        .tolerance_policy_identity("tolerance:kernel-micro")
        .build()
        .expect_err("kernel cannot synthesize missing precision receipt");

    assert_eq!(
        denial.kind(),
        PlanarLocalFrameDenialKind::MissingPrecisionReceipt
    );
}

#[test]
fn kernel_local_frame_preserves_movement_rotation_and_transform_chain_identity() {
    let cancelled = precision_receipt("cancelled", "movement:kernel-rotation-cancelled");
    let translated = precision_receipt("translated", "movement:kernel-translated-world");
    let cancelled_receipt = local_frame_receipt(
        "cancelled",
        local_frame_basis(
            &cancelled,
            "movement:kernel-rotation-cancelled",
            "transform:kernel-planar-chain",
        )
        .expect("cancelled basis"),
    );
    let translated_receipt = local_frame_receipt(
        "translated",
        local_frame_basis(
            &translated,
            "movement:kernel-translated-world",
            "transform:kernel-planar-chain",
        )
        .expect("translated basis"),
    );
    let alternate_transform_receipt = local_frame_receipt(
        "alternate-transform",
        local_frame_basis(
            &cancelled,
            "movement:kernel-rotation-cancelled",
            "transform:kernel-alternate-planar-chain",
        )
        .expect("alternate transform basis"),
    );

    assert_ne!(
        cancelled_receipt.fact_digest(),
        translated_receipt.fact_digest()
    );
    assert_ne!(
        cancelled_receipt.fact_digest(),
        alternate_transform_receipt.fact_digest()
    );
}

fn local_frame_receipt(
    world: &'static str,
    basis: PlanarLocalFrameBasis,
) -> worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt {
    let entry = planar_local_frame_certificate_entry(
        PlanarLocalFrameCertificateCase::from_precision_basis(basis),
    );
    planar_local_frame_certificate_facts(&entry, &frame_handle(world)).expect("local-frame receipt")
}

fn local_frame_basis(
    precision: &PlanarPrecisionCertificateReceipt,
    movement_rotation: &'static str,
    transform_chain: &'static str,
) -> Result<PlanarLocalFrameBasis, worth_spatial::facade::planar_local_frame::PlanarLocalFrameDenial>
{
    PlanarLocalFrameBasis::builder()
        .frame_identity("frame:kernel-micro")
        .origin([1.0e12, 0.0, 0.0])
        .normal([0.0, 0.0, 1.0])
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .transform_chain_digest(transform_chain)
        .movement_rotation_posture_identity(movement_rotation)
        .tolerance_policy_identity("tolerance:kernel-micro")
        .precision_receipt(precision)
        .build()
}

fn precision_receipt(
    world: &'static str,
    movement_rotation: &'static str,
) -> PlanarPrecisionCertificateReceipt {
    let predicate = predicate_receipt(movement_rotation);
    let basis = precision_basis(&predicate).expect("basis");
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate, basis),
    );
    planar_precision_certification_facts(&entry, &precision_handle(world))
        .expect("precision receipt")
}

fn predicate_receipt(movement_rotation: &'static str) -> PlanarPredicateFactReceipt {
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:kernel-micro",
        "topology:kernel-slot",
        movement_rotation,
        "tolerance:kernel-micro",
        [[0.0, 0.0], [1.0e-9, 0.0], [0.0, 1.0e-9]],
    );
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    planar_predicate_authority_facts(&entry, &predicate_handle()).expect("predicate receipt")
}

fn precision_basis(
    predicate: &PlanarPredicateFactReceipt,
) -> Result<PlanarPrecisionBasis, worth_spatial::facade::planar_precision::PlanarPrecisionBasisDenial>
{
    PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:kernel-micro")
        .topology_basis_identity("topology:kernel-slot")
        .movement_rotation_posture_identity(
            predicate.input_basis().movement_rotation_posture_identity(),
        )
        .tolerance_policy_identity("tolerance:kernel-micro")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .predicate_receipt(predicate)
        .build()
}

fn predicate_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "kernel-local-frame-predicate",
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
