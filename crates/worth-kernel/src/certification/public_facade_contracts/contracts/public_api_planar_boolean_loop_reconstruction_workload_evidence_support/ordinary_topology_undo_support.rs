use topology::derived_invalidation_authority_inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};
use topology::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use topology::derived_invalidation_family_catalog::{
    current_derived_invalidation_family_catalog, DerivedInvalidationFamilyCatalogCloseout,
};
use topology::facade::{
    lower_topology_undo_scope_product_from_traversal_views_request,
    DerivedInvalidationDensityPolicy, DerivedInvalidationLegalitySupportEvidence,
    DerivedInvalidationQuerySupportEvidence, DerivedInvalidationSelectedPlan,
    DerivedInvalidationTouchedClosure, EntityId, LoopSuccessorKind, PartitionId, RelationId,
    TopologyDeclaredTouchedGraphBasisProof, TopologyLoopSuccessorRewireMember,
    TopologyRewireLoopSuccessorProgramDeclaration, TopologyTouchedOperatingWorld,
    TraversalViewsRollbackRequest,
};
use topology::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use topology::validator_invariant_catalog::{
    current_worth_topology_legality_catalog_closeout, WorthTopologyLegalitySelectionCloseout,
    WorthTopologyValidatorRoutingClosure,
};

pub(crate) struct OrdinaryTraversalViewsUndoScopeSupport {
    touched_closure: DerivedInvalidationTouchedClosure,
    invalidation_receipt: DerivedInvalidationExecutionReceipt,
}

impl OrdinaryTraversalViewsUndoScopeSupport {
    pub(crate) fn lower_undo_scope_product(
        &self,
    ) -> Result<
        topology::facade::TopologyUndoScopeProduct<'_>,
        topology::facade::TopologyUndoFamilyExecutionError,
    > {
        lower_topology_undo_scope_product_from_traversal_views_request(
            TraversalViewsRollbackRequest::new(&self.touched_closure, &self.invalidation_receipt),
        )
    }
}

pub(crate) fn ordinary_traversal_views_undo_scope_support() -> OrdinaryTraversalViewsUndoScopeSupport
{
    undo_scope_support_for_loop_successor(20, 10, 11)
}

pub(crate) fn undo_scope_support_for_loop_successor(
    relation_slot: u64,
    source_slot: u64,
    target_slot: u64,
) -> OrdinaryTraversalViewsUndoScopeSupport {
    let proof = ordinary_loop_successor_touch_proof(relation_slot, source_slot, target_slot);
    let touched_closure = DerivedInvalidationTouchedClosure::from_declared_touch(&proof);
    let legality_support = ordinary_legality_support_for_touch(&proof);
    let inventory = current_derived_invalidation_authority_inventory();
    let inventory_closeout = DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .expect("inventory closeout");
    let catalog =
        current_derived_invalidation_family_catalog(inventory_closeout.phase_two_seed().clone())
            .expect("derived invalidation family catalog");
    let catalog_closeout =
        DerivedInvalidationFamilyCatalogCloseout::close(catalog).expect("catalog closeout");
    let selected_plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout,
        &touched_closure,
        &DerivedInvalidationQuerySupportEvidence::missing(),
        &legality_support,
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("selected invalidation plan");
    let invalidation_receipt =
        DerivedInvalidationExecutionReceipt::execute_selected_plan(&selected_plan)
            .expect("selected invalidation receipt");

    OrdinaryTraversalViewsUndoScopeSupport {
        touched_closure,
        invalidation_receipt,
    }
}

fn ordinary_loop_successor_touch_proof(
    relation_slot: u64,
    source_slot: u64,
    target_slot: u64,
) -> TopologyDeclaredTouchedGraphBasisProof {
    let declaration = TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        TopologyLoopSuccessorRewireMember::new(
            relation_id(relation_slot),
            LoopSuccessorKind::Next,
            entity_id(source_slot),
            entity_id(target_slot),
        ),
    ]);
    declaration
        .declared_touched_basis_proof(
            "topology.rewire_loop_successor_program",
            TopologyTouchedOperatingWorld::mainline(),
        )
        .expect("ordinary topology declaration should lower touched proof")
}

fn ordinary_legality_support_for_touch(
    proof: &TopologyDeclaredTouchedGraphBasisProof,
) -> DerivedInvalidationLegalitySupportEvidence {
    let milestone_eight_summary =
        WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout();
    let routing_closure =
        WorthTopologyValidatorRoutingClosure::from_declared_touch(proof, &milestone_eight_summary)
            .expect("ordinary topology touch should lower legality routing closure");
    let catalog_closeout = current_worth_topology_legality_catalog_closeout()
        .expect("current topology legality catalog closeout");
    let selection_closeout =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &catalog_closeout,
            &routing_closure,
        )
        .expect("ordinary topology touch should select legality obligations");
    DerivedInvalidationLegalitySupportEvidence::from_selected_legality_plan(
        selection_closeout.selected_plan(),
    )
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
