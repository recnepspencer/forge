use std::collections::{BTreeMap, BTreeSet};

use crate::validator_invariant_catalog::selected_validator_enforcement::loop_wiring::{
    WorthTopologyLoopWiringHalfEdgeWitnessRow, WorthTopologyLoopWiringViolationKind,
    WorthTopologyLoopWiringWitnessInput, WorthTopologyLoopWiringWitnessRow,
};
use crate::validator_invariant_catalog::selected_validator_enforcement::WorthTopologySelectedValidatorEnforcementOutcome;

pub(in crate::validator_invariant_catalog) fn execute_loop_wiring_obligation(
    witness_input: &WorthTopologyLoopWiringWitnessInput,
) -> WorthTopologySelectedValidatorEnforcementOutcome {
    if let Some(violation) = first_loop_membership_violation(witness_input) {
        return WorthTopologySelectedValidatorEnforcementOutcome::Violation(violation);
    }
    if let Some(violation) = first_prev_next_symmetry_violation(witness_input) {
        return WorthTopologySelectedValidatorEnforcementOutcome::Violation(violation);
    }
    WorthTopologySelectedValidatorEnforcementOutcome::Passed
}

fn first_loop_membership_violation(
    witness_input: &WorthTopologyLoopWiringWitnessInput,
) -> Option<WorthTopologyLoopWiringWitnessRow> {
    let half_edges = half_edges_by_id(witness_input.half_edge_rows());
    for loop_row in witness_input.loop_rows() {
        if loop_row.half_edge_ids().is_empty() {
            return Some(WorthTopologyLoopWiringWitnessRow::violation(
                WorthTopologyLoopWiringViolationKind::EmptyLoop,
                Some(loop_row.loop_id()),
                None,
                None,
                format!("loop {:?} contains no half-edges", loop_row.loop_id()),
            ));
        }
        let mut seen = BTreeSet::new();
        for half_edge_id in loop_row.half_edge_ids() {
            if !seen.insert(*half_edge_id) {
                return Some(WorthTopologyLoopWiringWitnessRow::violation(
                    WorthTopologyLoopWiringViolationKind::DuplicateHalfEdgeInLoop,
                    Some(loop_row.loop_id()),
                    Some(*half_edge_id),
                    None,
                    format!(
                        "loop {:?} references half-edge {:?} more than once",
                        loop_row.loop_id(),
                        half_edge_id
                    ),
                ));
            }
            let Some(half_edge) = half_edges.get(half_edge_id) else {
                return Some(WorthTopologyLoopWiringWitnessRow::violation(
                    WorthTopologyLoopWiringViolationKind::MissingLoopHalfEdge,
                    Some(loop_row.loop_id()),
                    Some(*half_edge_id),
                    None,
                    format!(
                        "loop {:?} references missing half-edge {:?}",
                        loop_row.loop_id(),
                        half_edge_id
                    ),
                ));
            };
            if half_edge.loop_id() != Some(loop_row.loop_id()) {
                return Some(WorthTopologyLoopWiringWitnessRow::violation(
                    WorthTopologyLoopWiringViolationKind::MismatchedHalfEdgeLoopMembership,
                    Some(loop_row.loop_id()),
                    Some(*half_edge_id),
                    None,
                    format!(
                        "half-edge {:?} is listed in loop {:?} but records loop {:?}",
                        half_edge_id,
                        loop_row.loop_id(),
                        half_edge.loop_id()
                    ),
                ));
            }
        }
    }
    None
}

fn first_prev_next_symmetry_violation(
    witness_input: &WorthTopologyLoopWiringWitnessInput,
) -> Option<WorthTopologyLoopWiringWitnessRow> {
    let half_edges = half_edges_by_id(witness_input.half_edge_rows());
    for half_edge in witness_input.half_edge_rows() {
        let Some(next_id) = half_edge.next_half_edge_id() else {
            return Some(WorthTopologyLoopWiringWitnessRow::violation(
                WorthTopologyLoopWiringViolationKind::MissingNextLink,
                half_edge.loop_id(),
                Some(half_edge.half_edge_id()),
                None,
                format!("half-edge {:?} has no next", half_edge.half_edge_id()),
            ));
        };
        let Some(prev_id) = half_edge.prev_half_edge_id() else {
            return Some(WorthTopologyLoopWiringWitnessRow::violation(
                WorthTopologyLoopWiringViolationKind::MissingPrevLink,
                half_edge.loop_id(),
                Some(half_edge.half_edge_id()),
                None,
                format!("half-edge {:?} has no prev", half_edge.half_edge_id()),
            ));
        };
        let Some(next) = half_edges.get(&next_id) else {
            return Some(WorthTopologyLoopWiringWitnessRow::violation(
                WorthTopologyLoopWiringViolationKind::MissingNextHalfEdge,
                half_edge.loop_id(),
                Some(half_edge.half_edge_id()),
                Some(next_id),
                format!("missing next half-edge {:?}", next_id),
            ));
        };
        let Some(prev) = half_edges.get(&prev_id) else {
            return Some(WorthTopologyLoopWiringWitnessRow::violation(
                WorthTopologyLoopWiringViolationKind::MissingPrevHalfEdge,
                half_edge.loop_id(),
                Some(half_edge.half_edge_id()),
                Some(prev_id),
                format!("missing prev half-edge {:?}", prev_id),
            ));
        };
        if next.prev_half_edge_id() != Some(half_edge.half_edge_id()) {
            return Some(WorthTopologyLoopWiringWitnessRow::violation(
                WorthTopologyLoopWiringViolationKind::UnreciprocatedNextLink,
                half_edge.loop_id(),
                Some(half_edge.half_edge_id()),
                Some(next_id),
                format!(
                    "next link from {:?} to {:?} is not reciprocated",
                    half_edge.half_edge_id(),
                    next_id
                ),
            ));
        }
        if prev.next_half_edge_id() != Some(half_edge.half_edge_id()) {
            return Some(WorthTopologyLoopWiringWitnessRow::violation(
                WorthTopologyLoopWiringViolationKind::UnreciprocatedPrevLink,
                half_edge.loop_id(),
                Some(half_edge.half_edge_id()),
                Some(prev_id),
                format!(
                    "prev link from {:?} to {:?} is not reciprocated",
                    half_edge.half_edge_id(),
                    prev_id
                ),
            ));
        }
    }
    None
}

fn half_edges_by_id(
    rows: &[WorthTopologyLoopWiringHalfEdgeWitnessRow],
) -> BTreeMap<
    forge_relational::facade::identity::EntityId,
    &WorthTopologyLoopWiringHalfEdgeWitnessRow,
> {
    rows.iter().map(|row| (row.half_edge_id(), row)).collect()
}
