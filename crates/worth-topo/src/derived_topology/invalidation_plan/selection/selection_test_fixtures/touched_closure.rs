use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::relations::TopologyRelationKind;

use super::super::DerivedInvalidationTouchedClosure;
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(crate) fn loop_cycles_touched_closure(
    semantic_family_key: &'static str,
) -> DerivedInvalidationTouchedClosure {
    DerivedInvalidationTouchedClosure::from_declared_touch(&loop_cycles_touch_proof(
        semantic_family_key,
    ))
}

pub(crate) fn unrelated_geometry_touched_closure() -> DerivedInvalidationTouchedClosure {
    let basis = test_basis_from_parts(
        vec![TopologyTouchedEntity::new(entity_id(1))],
        Vec::new(),
        Vec::new(),
        vec![TopologyTouchedAspect::GeometryBinding],
        vec![TopologyTouchedScope::Entity],
    );
    let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis("geometry-touch", basis)
        .expect("geometry touch should lower to Query descriptor");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}

pub(crate) fn empty_touched_closure() -> DerivedInvalidationTouchedClosure {
    let basis = test_basis_from_parts(Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis("empty-touch", basis)
        .expect("empty touch descriptor remains a valid empty proof");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}

fn loop_cycles_touch_proof(
    semantic_family_key: &'static str,
) -> TopologyDeclaredTouchedGraphBasisProof {
    let basis = test_basis_from_parts(
        vec![
            TopologyTouchedEntity::new(entity_id(1)),
            TopologyTouchedEntity::new(entity_id(2)),
        ],
        vec![TopologyTouchedRelation::new(relation_id(3))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
        vec![TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    TopologyDeclaredTouchedGraphBasisProof::from_basis(semantic_family_key, basis)
        .expect("loop-cycle touch should lower to Query descriptor")
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
