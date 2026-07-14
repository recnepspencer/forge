use std::cmp::Ordering;
use std::collections::BinaryHeap;

use sha2::{Digest, Sha256};

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_lp_relaxation::stable_set_lp_relaxation_rows;
use super::g27_same_field_mwis_branch_certificate_preflight::{branch_vertex, degree};

#[derive(Clone)]
pub(super) struct NodeState {
    pub(super) branch_included: Vec<usize>,
    pub(super) forced_included: Vec<usize>,
    pub(super) excluded: Vec<usize>,
    pub(super) candidates: Vec<usize>,
    pub(super) chosen_weight: i128,
    pub(super) depth: usize,
}

#[derive(Clone)]
pub(super) struct QueueEntry {
    pub(super) upper_bound: i128,
    pub(super) sequence: usize,
    pub(super) node: NodeState,
}

impl Eq for QueueEntry {}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.upper_bound == other.upper_bound && self.sequence == other.sequence
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.upper_bound
            .cmp(&other.upper_bound)
            .then_with(|| other.sequence.cmp(&self.sequence))
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(super) fn initial_frontier(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    threshold: i128,
    expected_expanded: usize,
    expected_pruned: usize,
    expected_open: usize,
) -> Result<Vec<QueueEntry>, G27GeometricFractionalError> {
    let root = include_isolated(
        adjacency,
        weights,
        NodeState {
            branch_included: Vec::new(),
            forced_included: Vec::new(),
            excluded: Vec::new(),
            candidates: candidates.to_vec(),
            chosen_weight: 0,
            depth: 0,
        },
    );
    let root_upper = node_upper(adjacency, weights, &root)?;
    let mut queue = BinaryHeap::from([QueueEntry {
        upper_bound: root_upper,
        sequence: 0,
        node: root,
    }]);
    let mut sequence = 1;
    let mut expanded = 0;
    let mut pruned = 0;
    while expanded < expected_expanded || pruned < expected_pruned {
        let Some(entry) = queue.pop() else { break };
        if entry.upper_bound <= threshold {
            pruned += 1;
            continue;
        }
        expanded += 1;
        let branch = branch_vertex(adjacency, weights, &entry.node.candidates);
        for child in branch_children(adjacency, weights, &entry.node, branch) {
            let upper_bound = node_upper(adjacency, weights, &child)?;
            if upper_bound <= threshold {
                pruned += 1;
            } else {
                queue.push(QueueEntry {
                    upper_bound,
                    sequence,
                    node: child,
                });
                sequence += 1;
            }
        }
    }
    let mut frontier = queue.into_sorted_vec();
    frontier.reverse();
    frontier.truncate(expected_open);
    Ok(frontier)
}

pub(super) fn lp_guided_branch(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    values: &[f64],
) -> usize {
    candidates
        .iter()
        .zip(values.iter())
        .max_by_key(|(vertex, value)| {
            (
                lp_score(weights[**vertex], **value),
                degree(adjacency, **vertex, candidates) as i128 * weights[**vertex],
                **vertex,
            )
        })
        .map(|(vertex, _)| *vertex)
        .expect("nonempty LP-guided branch candidates")
}

pub(super) fn lp_score(weight: i128, value: f64) -> i128 {
    (weight as f64 * value * (1.0 - value) * 1_000_000.0).round() as i128
}

pub(super) fn worst_child_upper(
    adjacency: &[BitWords],
    weights: &[i128],
    node: &NodeState,
    branch: usize,
) -> Result<i128, G27GeometricFractionalError> {
    let children = branch_children(adjacency, weights, node, branch);
    Ok(
        node_upper(adjacency, weights, &children[0])?.max(node_upper(
            adjacency,
            weights,
            &children[1],
        )?),
    )
}

pub(super) fn branch_children(
    adjacency: &[BitWords],
    weights: &[i128],
    node: &NodeState,
    branch: usize,
) -> [NodeState; 2] {
    let remaining = node
        .candidates
        .iter()
        .copied()
        .filter(|vertex| *vertex != branch)
        .collect::<Vec<_>>();
    let included = remaining
        .iter()
        .copied()
        .filter(|vertex| !has_bit(&adjacency[branch], *vertex))
        .collect::<Vec<_>>();
    [
        include_isolated(
            adjacency,
            weights,
            NodeState {
                branch_included: push_sorted(&node.branch_included, branch),
                forced_included: node.forced_included.clone(),
                excluded: node.excluded.clone(),
                candidates: included,
                chosen_weight: node.chosen_weight + weights[branch],
                depth: node.depth + 1,
            },
        ),
        include_isolated(
            adjacency,
            weights,
            NodeState {
                branch_included: node.branch_included.clone(),
                forced_included: node.forced_included.clone(),
                excluded: push_sorted(&node.excluded, branch),
                candidates: remaining,
                chosen_weight: node.chosen_weight,
                depth: node.depth + 1,
            },
        ),
    ]
}

fn include_isolated(adjacency: &[BitWords], weights: &[i128], mut node: NodeState) -> NodeState {
    loop {
        let isolated = node
            .candidates
            .iter()
            .copied()
            .find(|vertex| degree(adjacency, *vertex, &node.candidates) == 0);
        let Some(vertex) = isolated else { break };
        node.chosen_weight += weights[vertex];
        node.forced_included = push_sorted(&node.forced_included, vertex);
        node.candidates.retain(|candidate| *candidate != vertex);
    }
    node
}

fn push_sorted(values: &[usize], value: usize) -> Vec<usize> {
    let mut next = values.to_vec();
    next.push(value);
    next.sort_unstable();
    next
}

pub(super) fn node_upper(
    adjacency: &[BitWords],
    weights: &[i128],
    node: &NodeState,
) -> Result<i128, G27GeometricFractionalError> {
    let rows = stable_set_lp_relaxation_rows(adjacency, weights, &node.candidates)?;
    Ok(node.chosen_weight + rows.odd_cycle_objective_ceiling)
}

pub(super) fn child_entries(
    adjacency: &[BitWords],
    weights: &[i128],
    parent: &QueueEntry,
    branch: usize,
) -> Result<[QueueEntry; 2], G27GeometricFractionalError> {
    let children = branch_children(adjacency, weights, &parent.node, branch);
    Ok([
        QueueEntry {
            upper_bound: node_upper(adjacency, weights, &children[0])?,
            sequence: 0,
            node: children[0].clone(),
        },
        QueueEntry {
            upper_bound: node_upper(adjacency, weights, &children[1])?,
            sequence: 1,
            node: children[1].clone(),
        },
    ])
}

pub(super) fn node_digest(entry: &QueueEntry) -> String {
    let mut payload = String::new();
    write_numbers(&entry.node.branch_included, &mut payload);
    payload.push('|');
    write_numbers(&entry.node.forced_included, &mut payload);
    payload.push('|');
    write_numbers(&entry.node.excluded, &mut payload);
    payload.push('|');
    write_numbers(&entry.node.candidates, &mut payload);
    payload.push('|');
    payload.push_str(&entry.node.chosen_weight.to_string());
    payload.push('|');
    payload.push_str(&entry.node.depth.to_string());
    format!("{:x}", Sha256::digest(payload.as_bytes()))
}

fn write_numbers(numbers: &[usize], payload: &mut String) {
    for (index, number) in numbers.iter().enumerate() {
        if index > 0 {
            payload.push(',');
        }
        payload.push_str(&number.to_string());
    }
}
