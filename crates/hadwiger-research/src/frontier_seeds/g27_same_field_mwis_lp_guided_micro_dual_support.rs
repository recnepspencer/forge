use std::collections::HashSet;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_lp_relaxation::StableSetLpRelaxationRows;
use super::g27_same_field_mwis_lp_guided_branch_support::QueueEntry;
use super::g27_same_field_mwis_odd_cycle_dual_replay_support::ExplicitRow;

pub(super) fn explicit_rows(
    adjacency: &[BitWords],
    candidates: &[usize],
    rows: &StableSetLpRelaxationRows,
) -> Vec<ExplicitRow> {
    let mut explicit = Vec::new();
    for left in 0..candidates.len() {
        explicit.push(ExplicitRow {
            support: vec![left],
            rhs: 1,
        });
        for right in (left + 1)..candidates.len() {
            if has_bit(&adjacency[candidates[left]], candidates[right]) {
                explicit.push(ExplicitRow {
                    support: vec![left, right],
                    rhs: 1,
                });
            }
        }
    }
    explicit.extend(rows.clique_constraints.iter().map(|support| ExplicitRow {
        support: support.clone(),
        rhs: 1,
    }));
    explicit.extend(rows.odd_cycle_cuts.iter().map(|cut| ExplicitRow {
        support: cut.support.clone(),
        rhs: (cut.support.len() / 2) as i128,
    }));
    explicit
}

pub(super) fn validate_rows(
    adjacency: &[BitWords],
    candidates: &[usize],
    rows: &StableSetLpRelaxationRows,
) -> Result<(), G27GeometricFractionalError> {
    for clique in &rows.clique_constraints {
        for left in 0..clique.len() {
            for right in (left + 1)..clique.len() {
                if !has_bit(
                    &adjacency[candidates[clique[left]]],
                    candidates[clique[right]],
                ) {
                    return malformed("lp_guided_micro_dual_clique");
                }
            }
        }
    }
    for cut in &rows.odd_cycle_cuts {
        if cut.witness.len() < 5 || cut.witness.len() % 2 == 0 {
            return malformed("lp_guided_micro_dual_odd_length");
        }
        let mut sorted = cut.witness.clone();
        sorted.sort_unstable();
        if sorted != cut.support || has_duplicates(&cut.witness) {
            return malformed("lp_guided_micro_dual_odd_support");
        }
        for index in 0..cut.witness.len() {
            let left = cut.witness[index];
            let right = cut.witness[(index + 1) % cut.witness.len()];
            if !has_bit(&adjacency[candidates[left]], candidates[right]) {
                return malformed("lp_guided_micro_dual_odd_edge");
            }
        }
    }
    Ok(())
}

pub(super) fn write_record(
    index: usize,
    child: &QueueEntry,
    rows: &StableSetLpRelaxationRows,
    payload: &mut String,
) {
    payload.push_str(&format!(
        "child|{}|{}|{}|{}|{}\n",
        index,
        child.node.chosen_weight,
        child.upper_bound,
        rows.clique_constraints.len(),
        rows.odd_cycle_cuts.len()
    ));
    write_numbers(&child.node.candidates, payload);
    payload.push('\n');
    for clique in &rows.clique_constraints {
        payload.push_str("C:");
        write_numbers(clique, payload);
        payload.push('\n');
    }
    for cut in &rows.odd_cycle_cuts {
        payload.push_str("O:");
        write_numbers(&cut.support, payload);
        payload.push(':');
        write_numbers(&cut.witness, payload);
        payload.push('\n');
    }
}

fn write_numbers(numbers: &[usize], payload: &mut String) {
    for (index, number) in numbers.iter().enumerate() {
        if index > 0 {
            payload.push(',');
        }
        payload.push_str(&number.to_string());
    }
}

fn has_duplicates(values: &[usize]) -> bool {
    let mut seen = HashSet::new();
    values.iter().any(|value| !seen.insert(*value))
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(G27GeometricFractionalError::MalformedData { source })
}
