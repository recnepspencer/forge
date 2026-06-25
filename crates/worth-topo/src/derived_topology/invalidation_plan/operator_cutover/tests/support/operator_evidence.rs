use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::relations::TopologyRelationKind;

use crate::topology_operators::application::TopologyMutationApplicationEvidence;
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(in super::super) fn matching_operator_touch_proof() -> TopologyDeclaredTouchedGraphBasisProof {
    loop_cycle_touch_proof("phase-seven-operator-cutover")
}

pub(in super::super) fn mismatched_operator_touch_proof() -> TopologyDeclaredTouchedGraphBasisProof
{
    let basis = test_basis_from_parts(
        vec![TopologyTouchedEntity::new(entity_id(41))],
        Vec::new(),
        Vec::new(),
        vec![TopologyTouchedAspect::GeometryBinding],
        vec![TopologyTouchedScope::Entity],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    TopologyDeclaredTouchedGraphBasisProof::from_basis_for_tests("geometry-touch", basis)
        .expect("mismatched touch proof")
}

pub(in super::super) fn admitted_operator_evidence() -> TopologyMutationApplicationEvidence {
    TopologyMutationApplicationEvidence::from_cutover_test_parts(
        Some("graph-obligation-envelope.phase-seven".to_string()),
        Some("graph-obligation-dispatch.phase-seven".to_string()),
        1,
    )
}

pub(in super::super) fn missing_graph_obligation_evidence() -> TopologyMutationApplicationEvidence {
    TopologyMutationApplicationEvidence::from_cutover_test_parts(None, None, 0)
}

fn loop_cycle_touch_proof(
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
    TopologyDeclaredTouchedGraphBasisProof::from_basis_for_tests(semantic_family_key, basis)
        .expect("loop-cycle touch proof")
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
