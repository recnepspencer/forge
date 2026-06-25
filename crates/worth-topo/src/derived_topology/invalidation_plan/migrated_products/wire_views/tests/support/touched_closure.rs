use schema::facade::platform::relations::TopologyRelationKind;

use super::identity::{entity_id, relation_id};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(in super::super) fn selected_wire_view_touched_closure(
    operator_family: &'static str,
) -> DerivedInvalidationTouchedClosure {
    wire_view_touched_closure(operator_family, 100, 300)
}

pub(in super::super) fn unbound_wire_view_touched_closure(
    operator_family: &'static str,
) -> DerivedInvalidationTouchedClosure {
    wire_view_touched_closure(operator_family, 999, 999)
}

fn wire_view_touched_closure(
    operator_family: &'static str,
    entity_slot: u64,
    relation_slot: u64,
) -> DerivedInvalidationTouchedClosure {
    let basis = test_basis_from_parts(
        vec![TopologyTouchedEntity::new(entity_id(entity_slot))],
        vec![TopologyTouchedRelation::new(relation_id(relation_slot))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
        vec![TopologyTouchedScope::Wire],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis(operator_family, basis)
        .expect("wire-view touch should lower to Query descriptor");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}
