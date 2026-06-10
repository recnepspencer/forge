use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};

pub(crate) fn admitted_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(world))
        .validate()
        .expect("validated planar predicate authority handle")
        .admit()
        .expect("admitted planar predicate authority handle")
}

pub(crate) fn receipt_for(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPredicateAuthorityQueryDomain,
        PlanarPredicateAuthorityQueryWorld,
    >,
    basis: PlanarPredicateInputBasis,
) -> PlanarPredicateFactReceipt {
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    planar_predicate_authority_facts(&entry, handle).expect("planar predicate facts")
}

pub(crate) fn orient_basis(
    movement_rotation: &'static str,
    projected_points: [[f64; 2]; 3],
) -> PlanarPredicateInputBasis {
    orient_basis_with_identities(
        "frame:xy",
        "topology:face-loop",
        movement_rotation,
        "tolerance:exact",
        projected_points,
    )
}

pub(crate) fn orient_basis_with_identities(
    local_frame: &'static str,
    topology_basis: &'static str,
    movement_rotation: &'static str,
    tolerance_policy: &'static str,
    projected_points: [[f64; 2]; 3],
) -> PlanarPredicateInputBasis {
    PlanarPredicateInputBasis::from_projected_orient2d_points(
        local_frame,
        topology_basis,
        movement_rotation,
        tolerance_policy,
        projected_points,
    )
}
