use forge_relational::facade::identity::{EntityId, PartitionId};

use crate::validator_invariant_catalog::{
    WorthTopologyLoopWiringAdmittedLocalFacts, WorthTopologyLoopWiringHalfEdgeWitnessRow,
    WorthTopologyLoopWiringLoopWitnessRow, WorthTopologyLoopWiringWitnessInput,
    WorthTopologySelectedLegalityObligationRow,
};

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn passing_admitted_facts(
    selected_obligation: &WorthTopologySelectedLegalityObligationRow,
) -> WorthTopologyLoopWiringAdmittedLocalFacts {
    WorthTopologyLoopWiringAdmittedLocalFacts::from_selected_obligation_and_rows(
        selected_obligation,
        "loop-wiring-fixture-admitted-facts:passing",
        passing_loop_rows(),
        passing_half_edge_rows(),
    )
}

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn passing_admitted_facts_with_outside_rejections(
    selected_obligation: &WorthTopologySelectedLegalityObligationRow,
) -> WorthTopologyLoopWiringAdmittedLocalFacts {
    WorthTopologyLoopWiringAdmittedLocalFacts::from_selected_obligation_rows_and_rejected_counts(
        selected_obligation,
        "loop-wiring-fixture-admitted-facts:passing-with-outside-rejections",
        passing_loop_rows(),
        passing_half_edge_rows(),
        1,
        2,
    )
}

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn duplicate_half_edge_admitted_facts(
    selected_obligation: &WorthTopologySelectedLegalityObligationRow,
) -> WorthTopologyLoopWiringAdmittedLocalFacts {
    WorthTopologyLoopWiringAdmittedLocalFacts::from_selected_obligation_and_rows(
        selected_obligation,
        "loop-wiring-fixture-admitted-facts:duplicate-half-edge",
        [WorthTopologyLoopWiringLoopWitnessRow::new(
            entity_id(10),
            vec![entity_id(20), entity_id(20)],
        )],
        [WorthTopologyLoopWiringHalfEdgeWitnessRow::new(
            entity_id(20),
            Some(entity_id(10)),
            Some(entity_id(20)),
            Some(entity_id(20)),
        )],
    )
}

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn unreciprocated_next_admitted_facts(
    selected_obligation: &WorthTopologySelectedLegalityObligationRow,
) -> WorthTopologyLoopWiringAdmittedLocalFacts {
    WorthTopologyLoopWiringAdmittedLocalFacts::from_selected_obligation_and_rows(
        selected_obligation,
        "loop-wiring-fixture-admitted-facts:unreciprocated-next",
        passing_loop_rows(),
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
                None,
            ),
        ],
    )
}

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn wrong_selected_obligation_admitted_facts(
) -> WorthTopologyLoopWiringAdmittedLocalFacts {
    WorthTopologyLoopWiringAdmittedLocalFacts::from_unbound_selected_obligation_digest_for_tests(
        "not-the-selected-loop-wiring-row",
        "loop-wiring-fixture-admitted-facts:wrong-selected-obligation",
        [WorthTopologyLoopWiringLoopWitnessRow::new(
            entity_id(10),
            vec![entity_id(20)],
        )],
        [WorthTopologyLoopWiringHalfEdgeWitnessRow::new(
            entity_id(20),
            Some(entity_id(10)),
            Some(entity_id(20)),
            Some(entity_id(20)),
        )],
    )
}

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn witness_input_from_admitted_facts(
    admitted_facts: &WorthTopologyLoopWiringAdmittedLocalFacts,
) -> WorthTopologyLoopWiringWitnessInput {
    WorthTopologyLoopWiringWitnessInput::from_selected_obligation_and_rows(
        admitted_facts.selected_obligation_digest(),
        admitted_facts.loop_rows().iter().cloned(),
        admitted_facts.half_edge_rows().iter().cloned(),
    )
}

fn passing_loop_rows() -> [WorthTopologyLoopWiringLoopWitnessRow; 1] {
    [WorthTopologyLoopWiringLoopWitnessRow::new(
        entity_id(10),
        vec![entity_id(20), entity_id(21)],
    )]
}

fn passing_half_edge_rows() -> [WorthTopologyLoopWiringHalfEdgeWitnessRow; 2] {
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
    ]
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}
