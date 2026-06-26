use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    LoopEndpointKind, LoopSuccessorKind, TopologyDeclaredMutationSequence,
    TopologyDeclaredTouchedGraphBasis, TopologyLoopSuccessorRewireMember,
    TopologyRewireLoopEndpointDeclaration, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologyTouchedOperatingWorld,
};
use crate::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use crate::validator_invariant_catalog::selection_from_touched_closure::WorthTopologyValidatorRoutingClosure;

pub(super) fn routing_closure_for_rewire_operator(
    relation_slot: u64,
) -> WorthTopologyValidatorRoutingClosure {
    let declaration = TopologyRewireLoopEndpointDeclaration::new(
        relation_id(relation_slot),
        LoopEndpointKind::End,
        entity_id(relation_slot + 1),
        entity_id(relation_slot + 2),
    );
    routing_closure_from_declared_rewire_touch(
        TopologyRewireLoopEndpointDeclaration::SEMANTIC_FAMILY_KEY,
        declaration.clone(),
        &declaration.into_mutation_sequence(),
    )
}

pub(super) fn routing_closure_for_loop_successor_program(
    relation_slot: u64,
) -> WorthTopologyValidatorRoutingClosure {
    let declaration = TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        TopologyLoopSuccessorRewireMember::new(
            relation_id(relation_slot),
            LoopSuccessorKind::Next,
            entity_id(relation_slot + 1),
            entity_id(relation_slot + 2),
        ),
    ]);
    routing_closure_from_declared_rewire_touch(
        TopologyRewireLoopSuccessorProgramDeclaration::SEMANTIC_FAMILY_KEY,
        declaration.clone(),
        &declaration.into_mutation_sequence(),
    )
}

fn routing_closure_from_declared_rewire_touch<Declaration>(
    semantic_family_key: &'static str,
    declaration: Declaration,
    mutation_sequence: &TopologyDeclaredMutationSequence,
) -> WorthTopologyValidatorRoutingClosure
where
    Declaration: Clone + Send + Sync + 'static,
{
    let declared = TopologyDeclaredTouchedGraphBasis::from_sequence(
        semantic_family_key,
        declaration,
        mutation_sequence,
        TopologyTouchedOperatingWorld::mainline(),
    )
    .expect("operator declaration should produce declared touched graph proof");
    WorthTopologyValidatorRoutingClosure::from_declared_touch(
        declared.proof(),
        &WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout(),
    )
    .expect("operator-derived routing closure should be admitted")
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
