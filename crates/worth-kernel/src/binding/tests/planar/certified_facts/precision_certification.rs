use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionBasis, PlanarPrecisionBasisDenialKind, PlanarPrecisionCertificationCase,
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};

#[test]
fn kernel_consumes_spatial_planar_precision_receipts_without_local_synthesis() {
    let predicate = predicate_receipt("movement:kernel-rotation-cancelled");
    let basis = precision_basis(&predicate).expect("basis");
    let entry = planar_precision_certification_entry(
        PlanarPrecisionCertificationCase::from_predicate_receipt(predicate.clone(), basis),
    );
    let receipt = planar_precision_certification_facts(&entry, &precision_handle("kernel"))
        .expect("spatial precision receipt");

    assert_eq!(receipt.predicate_fact_digest(), predicate.fact_digest());
    assert_eq!(receipt.scale_separation_orders(), 21);
    assert_eq!(receipt.counters().predicate_precision_rows_consumed(), 1);
    assert_eq!(
        receipt.counters().precision_escalation_breadth(),
        predicate
            .precision_escalation()
            .get_expansion_length()
            .unwrap_or(0)
    );
    assert_eq!(receipt.counters().local_coordinate_normalizations(), 1);
    assert!(!receipt.declaration_digest().is_empty());
    assert!(!receipt.envelope_digest().is_empty());
    assert!(!receipt.fact_digest().is_empty());
    assert_eq!(
        receipt.precision_escalation().get_resolved_at(),
        predicate.precision_escalation().get_resolved_at()
    );
}

#[test]
fn kernel_cannot_upgrade_missing_precision_basis_into_receipt() {
    let predicate = predicate_receipt("movement:kernel-rotation-cancelled");
    let denial = PlanarPrecisionBasis::builder()
        .local_frame_identity("frame:kernel-micro")
        .topology_basis_identity("topology:kernel-slot")
        .movement_rotation_posture_identity("movement:kernel-rotation-cancelled")
        .tolerance_policy_identity("tolerance:kernel-micro")
        .local_feature_scale_order(-9)
        .world_magnitude_order(12)
        .normalization_scale(1.0e-9)
        .build()
        .expect_err("kernel cannot synthesize missing predicate receipt");

    assert_eq!(
        denial.kind(),
        PlanarPrecisionBasisDenialKind::MissingPredicateReceipt
    );
    assert!(!predicate.fact_digest().is_empty());
}

#[test]
fn kernel_precision_basis_preserves_movement_rotation_as_semantic_posture() {
    let cancelled = predicate_receipt("movement:kernel-rotation-cancelled");
    let invalidated = predicate_receipt("movement:kernel-tiny-rotation-invalidated");
    let cancelled_receipt = precision_receipt("cancelled", cancelled);
    let invalidated_receipt = precision_receipt("invalidated", invalidated);

    assert_eq!(cancelled_receipt.scale_separation_orders(), 21);
    assert_ne!(
        cancelled_receipt.fact_digest(),
        invalidated_receipt.fact_digest()
    );
    assert_ne!(
        cancelled_receipt
            .basis()
            .movement_rotation_posture_identity(),
        invalidated_receipt
            .basis()
            .movement_rotation_posture_identity()
    );
}

fn precision_receipt(
    world: &'static str,
    predicate: PlanarPredicateFactReceipt,
) -> worth_spatial::facade::planar_precision::PlanarPrecisionCertificateReceipt {
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
            "kernel-precision-predicate",
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
