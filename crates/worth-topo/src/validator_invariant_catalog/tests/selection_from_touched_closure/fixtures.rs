use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::relations::TopologyRelationKind;

use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    test_basis_from_parts, LoopEndpointKind, TopologyDeclaredTouchedGraphBasis,
    TopologyDeclaredTouchedGraphBasisProof, TopologyGraphLifecyclePosture,
    TopologyRewireLoopEndpointDeclaration, TopologyTouchedAspect, TopologyTouchedEntity,
    TopologyTouchedOperatingWorld, TopologyTouchedRelation, TopologyTouchedScope,
};
use crate::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalogCloseout, WorthTopologyLegalitySelectionCloseout,
    WorthTopologyTouchedApplicability, WorthTopologyValidatorRoutingClosure,
};

pub(super) fn routing_closure_for_loop_touch(
    operating_world: TopologyTouchedOperatingWorld,
) -> WorthTopologyValidatorRoutingClosure {
    let proof = touched_basis_proof(operating_world);
    WorthTopologyValidatorRoutingClosure::from_declared_touch(
        &proof,
        &WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout(),
    )
    .expect("valid Milestone 8 summary should admit routing closure")
}

pub(super) fn routing_closure_for_rewire_operator(
    operating_world: TopologyTouchedOperatingWorld,
) -> WorthTopologyValidatorRoutingClosure {
    let declaration = TopologyRewireLoopEndpointDeclaration::new(
        relation_id(30),
        LoopEndpointKind::End,
        entity_id(31),
        entity_id(32),
    );
    let sequence = declaration.clone().into_mutation_sequence();
    let declared = TopologyDeclaredTouchedGraphBasis::from_sequence(
        TopologyRewireLoopEndpointDeclaration::SEMANTIC_FAMILY_KEY,
        declaration,
        &sequence,
        operating_world,
    )
    .expect("operator declaration should produce declared touched graph proof");
    WorthTopologyValidatorRoutingClosure::from_declared_touch(
        declared.proof(),
        &WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout(),
    )
    .expect("operator-derived routing closure should be admitted")
}

pub(super) fn selected_family_names(
    selection: &WorthTopologyLegalitySelectionCloseout,
    closeout: &WorthTopologyLegalityCatalogCloseout,
) -> Vec<String> {
    selection
        .selected_plan()
        .selected_obligation_rows()
        .iter()
        .filter_map(|selected| {
            closeout
                .catalog()
                .records()
                .iter()
                .find(|record| {
                    record.identity().identity_digest() == selected.worth_family_identity_digest()
                })
                .map(|record| record.identity().name().to_string())
        })
        .collect()
}

pub(super) fn loop_touch_applicability() -> WorthTopologyTouchedApplicability {
    WorthTopologyTouchedApplicability::from_parts(
        [
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedAspect::TopologyStructure,
        ],
        [TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
        TopologyGraphLifecyclePosture::ExistingRelationRetarget,
    )
}

pub(super) fn unrelated_geometry_applicability() -> WorthTopologyTouchedApplicability {
    WorthTopologyTouchedApplicability::from_parts(
        [
            TopologyTouchedAspect::GeometryBinding,
            TopologyTouchedAspect::GeometryCarrier,
        ],
        [TopologyTouchedScope::Entity],
        TopologyGraphLifecyclePosture::ExistingRelationRetarget,
    )
}

pub(super) fn touched_basis_proof(
    operating_world: TopologyTouchedOperatingWorld,
) -> TopologyDeclaredTouchedGraphBasisProof {
    let basis = test_basis_from_parts(
        vec![
            TopologyTouchedEntity::new(entity_id(1)),
            TopologyTouchedEntity::new(entity_id(2)),
        ],
        vec![TopologyTouchedRelation::new(relation_id(3))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![
            TopologyTouchedAspect::TopologyBoundary,
            TopologyTouchedAspect::TopologyStructure,
        ],
        vec![TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
    )
    .with_operating_world_for_tests(operating_world);
    TopologyDeclaredTouchedGraphBasisProof::from_basis("selection-test", basis)
        .expect("test basis should lower to Query descriptor")
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
