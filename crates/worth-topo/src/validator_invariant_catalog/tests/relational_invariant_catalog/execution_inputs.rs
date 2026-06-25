mod query_execution_input;
mod routing_closures;
mod validator_seed;

pub(in crate::validator_invariant_catalog::tests) use query_execution_input::{
    envelope_from_input_rows, relational_invariant_query_execution_input,
    relational_invariant_query_execution_input_for_loop_successor_program_slot,
    relational_invariant_query_execution_input_for_rewire_slot_with_rows,
};
use routing_closures::{
    routing_closure_for_loop_successor_program, routing_closure_for_rewire_operator,
};
use validator_seed::validator_phase_five_seed;

use crate::validator_invariant_catalog::selection_from_touched_closure::WorthTopologyValidatorRoutingClosure;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalitySelectionCloseout, WorthTopologyRelationalInvariantCatalogCloseout,
};

use super::super::production_phase_two_closeout;

pub(super) fn relational_invariant_closeout() -> WorthTopologyRelationalInvariantCatalogCloseout {
    relational_invariant_closeout_for_rewire_slot(30)
}

pub(super) fn relational_invariant_closeout_for_rewire_slot(
    relation_slot: u64,
) -> WorthTopologyRelationalInvariantCatalogCloseout {
    relational_invariant_closeout_from_routing_closure(routing_closure_for_rewire_operator(
        relation_slot,
    ))
}

pub(super) fn relational_invariant_closeout_for_loop_successor_program_slot(
    relation_slot: u64,
) -> WorthTopologyRelationalInvariantCatalogCloseout {
    relational_invariant_closeout_from_routing_closure(routing_closure_for_loop_successor_program(
        relation_slot,
    ))
}

fn relational_invariant_closeout_from_routing_closure(
    routing_closure: WorthTopologyValidatorRoutingClosure,
) -> WorthTopologyRelationalInvariantCatalogCloseout {
    let phase_two_closeout = production_phase_two_closeout();
    let selection_closeout =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &phase_two_closeout,
            &routing_closure,
        )
        .expect("rewire routing should select validator and invariant obligations");
    let validator_seed = validator_phase_five_seed(&selection_closeout);
    WorthTopologyRelationalInvariantCatalogCloseout::from_catalog_selected_plan_and_validator_seed(
        phase_two_closeout.catalog(),
        selection_closeout.selected_plan(),
        &validator_seed,
    )
    .expect("relational invariant catalog should close from selected plan")
}
