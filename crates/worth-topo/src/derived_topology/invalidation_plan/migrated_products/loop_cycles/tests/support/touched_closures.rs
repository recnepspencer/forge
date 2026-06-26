use schema::facade::platform::relations::TopologyRelationKind;

use super::identity_slots::{entity_id, relation_id};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(crate) fn selected_loop_cycle_touched_closure(
    semantic_family_key: &'static str,
) -> DerivedInvalidationTouchedClosure {
    selected_loop_cycle_touched_closure_for_shell(semantic_family_key, 24)
}

pub(crate) fn selected_loop_cycle_touched_closure_for_shell(
    semantic_family_key: &'static str,
    shell_slot: u64,
) -> DerivedInvalidationTouchedClosure {
    let basis = test_basis_from_parts(
        vec![TopologyTouchedEntity::new(entity_id(shell_slot))],
        vec![TopologyTouchedRelation::new(relation_id(160))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
        vec![TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis(semantic_family_key, basis)
        .expect("loop-cycle touch should lower to Query descriptor");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}
