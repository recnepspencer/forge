use forge_query::facade::ForgeQueryApplicationFacade;
use worth_math::sign::TriSign;
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityFactError,
    PlanarPredicateAuthorityQueryDomain, PlanarPredicateAuthorityQueryWorld,
    PlanarPredicateCoincidencePolicy, PlanarPredicateInputBasis, PlanarPredicateKind,
};

#[test]
fn kernel_consumes_spatial_planar_predicate_receipts_without_local_math() {
    let handle = admitted_handle("kernel-consumer");
    let entry =
        planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(orient_basis([
            [0.0, 0.0],
            [2.0, 0.0],
            [0.0, 3.0],
        ])));

    let receipt =
        planar_predicate_authority_facts(&entry, &handle).expect("spatial predicate receipt");

    assert_eq!(receipt.predicate_kind(), PlanarPredicateKind::Orient2d);
    assert_eq!(receipt.certified_sign().sign(), TriSign::Pos);
    assert_eq!(receipt.counters().predicate_evaluations(), 1);
    assert_eq!(receipt.counters().input_point_count(), 3);
    assert_eq!(receipt.counters().canonical_basis_part_count(), 12);
    assert!(!receipt.declaration_digest().is_empty());
    assert!(!receipt.envelope_digest().is_empty());
    assert!(!receipt.fact_digest().is_empty());
    assert!(!receipt
        .precision_escalation()
        .get_target_triple()
        .is_empty());
}

#[test]
fn kernel_cannot_upgrade_predicate_uncertainty_into_receipt() {
    let handle = admitted_handle("kernel-zero-denial");
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(
        orient_basis([[0.0, 0.0], [4.0, 4.0], [8.0, 8.0]]).with_coincidence_policy(
            PlanarPredicateCoincidencePolicy::DenyCertifiedZeroBeforeRepair,
        ),
    ));

    let error = planar_predicate_authority_facts(&entry, &handle)
        .expect_err("certified zero remains a typed uncertainty");

    assert!(matches!(
        error,
        PlanarPredicateAuthorityFactError::PredicateUncertain {
            certified_sign,
            counters,
            ..
        } if certified_sign.sign() == TriSign::Zero
            && counters.predicate_evaluations() == 1
            && counters.canonical_basis_part_count() == 12
    ));
}

fn admitted_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(world))
        .validate()
        .expect("validated planar predicate handle")
        .admit()
        .expect("admitted planar predicate handle")
}

fn orient_basis(projected_points: [[f64; 2]; 3]) -> PlanarPredicateInputBasis {
    PlanarPredicateInputBasis::from_projected_orient2d_points(
        "frame:kernel-xy",
        "topology:kernel-loop",
        "movement:kernel-rotation",
        "tolerance:kernel-exact",
        projected_points,
    )
}
