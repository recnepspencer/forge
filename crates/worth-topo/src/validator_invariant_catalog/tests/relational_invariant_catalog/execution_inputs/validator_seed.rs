use forge_relational::facade::identity::{EntityId, PartitionId};

use crate::validation::loop_wiring_rule;
use crate::validator_invariant_catalog::{
    WorthTopologyLegalitySelectionCloseout, WorthTopologyLoopWiringAdmittedLocalFacts,
    WorthTopologyLoopWiringHalfEdgeWitnessRow, WorthTopologyLoopWiringLoopWitnessRow,
    WorthTopologySelectedLegalityObligationRow, WorthTopologySelectedValidatorEnforcementCloseout,
    WorthTopologySelectedValidatorEnforcementPhaseFiveSeed, WorthTopologyValidatorFamilyIdentity,
};

pub(super) fn validator_phase_five_seed(
    selection_closeout: &WorthTopologyLegalitySelectionCloseout,
) -> WorthTopologySelectedValidatorEnforcementPhaseFiveSeed {
    let selected_obligation = selected_loop_wiring_validator_row(selection_closeout);
    let admitted_facts =
        WorthTopologyLoopWiringAdmittedLocalFacts::from_selected_obligation_and_rows(
            selected_obligation,
            "relational-invariant-fixture:loop-wiring-validator-facts",
            [WorthTopologyLoopWiringLoopWitnessRow::new(
                entity_id(10),
                vec![entity_id(20), entity_id(21)],
            )],
            [
                WorthTopologyLoopWiringHalfEdgeWitnessRow::new(
                    entity_id(20),
                    Some(entity_id(10)),
                    Some(entity_id(21)),
                    Some(entity_id(21)),
                ),
                WorthTopologyLoopWiringHalfEdgeWitnessRow::new(
                    entity_id(21),
                    Some(entity_id(10)),
                    Some(entity_id(20)),
                    Some(entity_id(20)),
                ),
            ],
        );
    WorthTopologySelectedValidatorEnforcementCloseout::execute_loop_wiring_family_from_admitted_facts(
        selection_closeout,
        &admitted_facts,
    )
    .expect("validator Phase 4 should execute from admitted facts")
    .phase_five_seed()
    .clone()
}

fn selected_loop_wiring_validator_row(
    selection_closeout: &WorthTopologyLegalitySelectionCloseout,
) -> &WorthTopologySelectedLegalityObligationRow {
    let loop_wiring_identity =
        WorthTopologyValidatorFamilyIdentity::from_registered_rule(loop_wiring_rule());
    selection_closeout
        .selected_plan()
        .selected_obligation_rows()
        .iter()
        .find(|row| {
            row.query_obligation_kind()
                == forge_query::facade::ForgeQueryGraphObligationKind::SchemaContractValidator
                && row.worth_family_identity_digest() == loop_wiring_identity.identity_digest()
        })
        .expect("rewire routing should select loop wiring validator")
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}
