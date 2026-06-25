use schema::facade::platform::relations::TopologyRelationKind;

use super::identity_slots::{entity_id, relation_id};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(crate) fn selected_vertex_disk_touched_closure(
    semantic_family_key: &'static str,
) -> DerivedInvalidationTouchedClosure {
    let basis = test_basis_from_parts(
        vec![TopologyTouchedEntity::new(entity_id(24))],
        vec![TopologyTouchedRelation::new(relation_id(160))],
        vec![TopologyRelationKind::HalfEdgeStartsAtVertex],
        vec![TopologyTouchedAspect::TopologyStructure],
        vec![TopologyTouchedScope::LocalNeighborhood],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis(semantic_family_key, basis)
        .expect("vertex-disk touch should lower to Query descriptor");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}
