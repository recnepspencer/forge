use schema::facade::platform::relations::TopologyRelationKind;

use super::identity_slots::{entity_id, relation_id};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(crate) fn selected_radial_ring_touched_closure(
    semantic_family_key: &'static str,
) -> DerivedInvalidationTouchedClosure {
    selected_radial_ring_touched_closure_for_shell(semantic_family_key, 24)
}

pub(crate) fn selected_radial_ring_touched_closure_for_shell(
    semantic_family_key: &'static str,
    half_edge_slot: u64,
) -> DerivedInvalidationTouchedClosure {
    let basis = test_basis_from_parts(
        vec![TopologyTouchedEntity::new(entity_id(half_edge_slot))],
        vec![TopologyTouchedRelation::new(relation_id(160))],
        vec![TopologyRelationKind::HalfEdgeRadialNext],
        vec![TopologyTouchedAspect::TopologyRadial],
        vec![
            TopologyTouchedScope::RadialNeighborhood,
            TopologyTouchedScope::Relation,
        ],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis(semantic_family_key, basis)
        .expect("radial-ring touch should lower to Query descriptor");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}
