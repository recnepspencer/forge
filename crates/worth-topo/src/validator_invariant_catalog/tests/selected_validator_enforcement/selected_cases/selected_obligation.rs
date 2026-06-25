use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    LoopEndpointKind, TopologyDeclaredTouchedGraphBasis, TopologyRewireLoopEndpointDeclaration,
    TopologyTouchedOperatingWorld,
};
use crate::validation::loop_wiring_rule;
use crate::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use crate::validator_invariant_catalog::selection_from_touched_closure::WorthTopologyValidatorRoutingClosure;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalitySelectionCloseout, WorthTopologySelectedLegalityObligationRow,
    WorthTopologyValidatorFamilyIdentity,
};

use super::super::super::production_phase_two_closeout;

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn selected_loop_wiring_closeout(
) -> WorthTopologyLegalitySelectionCloseout {
    let closeout = production_phase_two_closeout();
    let routing_closure = routing_closure_for_rewire_operator();
    WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
        &closeout,
        &routing_closure,
    )
    .expect("rewire routing should select loop wiring")
}

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn selected_loop_wiring_row(
    selection: &WorthTopologyLegalitySelectionCloseout,
) -> &WorthTopologySelectedLegalityObligationRow {
    let loop_wiring_identity =
        WorthTopologyValidatorFamilyIdentity::from_registered_rule(loop_wiring_rule());
    selection
        .selected_plan()
        .selected_obligation_rows()
        .iter()
        .find(|row| row.worth_family_identity_digest() == loop_wiring_identity.identity_digest())
        .expect("selection should contain loop wiring")
}

fn routing_closure_for_rewire_operator() -> WorthTopologyValidatorRoutingClosure {
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
