use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_lp_relaxation::{
    stable_set_lp_relaxation_rows, StableSetLpRelaxationRows,
};

pub(super) const TARGET_PRUNED_RECORDS: usize = 2;
const NODE_CAP: usize = 64;

#[derive(Clone)]
struct Node {
    candidates: Vec<usize>,
    included: Vec<usize>,
    excluded: Vec<usize>,
    chosen_weight: i128,
    depth: usize,
}

impl Node {
    fn root(candidates: &[usize]) -> Self {
        Self {
            candidates: candidates.to_vec(),
            included: Vec::new(),
            excluded: Vec::new(),
            chosen_weight: 0,
            depth: 0,
        }
    }
}

struct QueueEntry {
    upper_bound: i128,
    sequence: usize,
    node: Node,
}

impl QueueEntry {
    fn new(upper_bound: i128, sequence: usize, node: Node) -> Self {
        Self {
            upper_bound,
            sequence,
            node,
        }
    }
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

pub(super) struct NodeRecord {
    pub(super) label: &'static str,
    node: Node,
    pub(super) upper_bound: i128,
    pub(super) rows: StableSetLpRelaxationRows,
}

impl NodeRecord {
    pub(super) fn candidates(&self) -> &[usize] {
        &self.node.candidates
    }

    pub(super) fn chosen_weight(&self) -> i128 {
        self.node.chosen_weight
    }
}

pub(super) fn collect_row_records(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    threshold: i128,
) -> Result<Vec<NodeRecord>, G27GeometricFractionalError> {
    let root = include_isolated(adjacency, weights, Node::root(candidates));
    let root_record = node_record(adjacency, weights, &root, "root")?;
    let mut queue = BinaryHeap::from([QueueEntry::new(root_record.upper_bound, 0, root)]);
    let mut records = vec![root_record];
    let mut sequence = 1;
    let mut nodes = 0;
    while nodes < NODE_CAP && records.len() < TARGET_PRUNED_RECORDS + 1 {
        let Some(entry) = queue.pop() else { break };
        if entry.upper_bound <= threshold {
            continue;
        }
        nodes += 1;
        let branch = branch_vertex(adjacency, weights, &entry.node.candidates);
        for child in branch_children(adjacency, weights, entry.node.clone(), branch) {
            let label = if child.included.contains(&branch) {
                "include"
            } else {
                "exclude"
            };
            let record = node_record(adjacency, weights, &child, label)?;
            if record.upper_bound <= threshold {
                records.push(record);
                if records.len() >= TARGET_PRUNED_RECORDS + 1 {
                    break;
                }
            } else {
                queue.push(QueueEntry::new(record.upper_bound, sequence, child));
                sequence += 1;
            }
        }
    }
    Ok(records)
}

pub(super) fn write_record(record: &NodeRecord, payload: &mut String) {
    payload.push_str(record.label);
    payload.push('|');
    write_numbers(&record.node.included, payload);
    payload.push('|');
    write_numbers(&record.node.excluded, payload);
    payload.push('|');
    write_numbers(&record.node.candidates, payload);
    payload.push('|');
    payload.push_str(&record.node.chosen_weight.to_string());
    payload.push('|');
    payload.push_str(&record.upper_bound.to_string());
    payload.push('|');
    payload.push_str(&record.rows.objective_ceiling.to_string());
    payload.push('|');
    payload.push_str(&record.rows.clique_objective_ceiling.to_string());
    payload.push('|');
    payload.push_str(&record.rows.odd_cycle_objective_ceiling.to_string());
    payload.push('|');
    payload.push_str(&record.rows.odd_cycle_round_count.to_string());
    payload.push('\n');
    for clique in &record.rows.clique_constraints {
        payload.push_str("C:");
        write_local_as_global(clique, &record.node.candidates, payload);
        payload.push('\n');
    }
    for cut in &record.rows.odd_cycle_cuts {
        payload.push_str("O:");
        write_local_as_global(&cut.support, &record.node.candidates, payload);
        payload.push(':');
        write_local_as_global(&cut.witness, &record.node.candidates, payload);
        payload.push(':');
        payload.push_str(&cut.violation_ppm.to_string());
        payload.push('\n');
    }
}

fn node_record(
    adjacency: &[BitWords],
    weights: &[i128],
    node: &Node,
    label: &'static str,
) -> Result<NodeRecord, G27GeometricFractionalError> {
    let rows = stable_set_lp_relaxation_rows(adjacency, weights, &node.candidates)?;
    validate_rows(adjacency, &node.candidates, &rows)?;
    Ok(NodeRecord {
        label,
        node: node.clone(),
        upper_bound: node.chosen_weight + rows.odd_cycle_objective_ceiling,
        rows,
    })
}

fn validate_rows(
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
                    return malformed("odd_cycle_replay_clique_row");
                }
            }
        }
    }
    for cut in &rows.odd_cycle_cuts {
        if cut.witness.len() < 5 || cut.witness.len() % 2 == 0 {
            return malformed("odd_cycle_replay_length");
        }
        let mut sorted = cut.witness.clone();
        sorted.sort_unstable();
        if sorted != cut.support || has_duplicates(&cut.witness) {
            return malformed("odd_cycle_replay_support");
        }
        for index in 0..cut.witness.len() {
            let left = cut.witness[index];
            let right = cut.witness[(index + 1) % cut.witness.len()];
            if !has_bit(&adjacency[candidates[left]], candidates[right]) {
                return malformed("odd_cycle_replay_witness_edge");
            }
        }
    }
    Ok(())
}

fn branch_children(
    adjacency: &[BitWords],
    weights: &[i128],
    node: Node,
    branch: usize,
) -> [Node; 2] {
    let remaining = node
        .candidates
        .iter()
        .copied()
        .filter(|vertex| *vertex != branch)
        .collect::<Vec<_>>();
    let included_candidates = remaining
        .iter()
        .copied()
        .filter(|vertex| !has_bit(&adjacency[branch], *vertex))
        .collect::<Vec<_>>();
    let include_node = include_isolated(
        adjacency,
        weights,
        Node {
            candidates: included_candidates,
            included: push_sorted(&node.included, branch),
            excluded: node.excluded.clone(),
            chosen_weight: node.chosen_weight + weights[branch],
            depth: node.depth + 1,
        },
    );
    let exclude_node = include_isolated(
        adjacency,
        weights,
        Node {
            candidates: remaining,
            included: node.included,
            excluded: push_sorted(&node.excluded, branch),
            chosen_weight: node.chosen_weight,
            depth: node.depth + 1,
        },
    );
    [include_node, exclude_node]
}

fn include_isolated(adjacency: &[BitWords], weights: &[i128], mut node: Node) -> Node {
    loop {
        let isolated = node
            .candidates
            .iter()
            .copied()
            .find(|vertex| degree(adjacency, *vertex, &node.candidates) == 0);
        let Some(vertex) = isolated else { break };
        node.chosen_weight += weights[vertex];
        node.included = push_sorted(&node.included, vertex);
        node.candidates.retain(|candidate| *candidate != vertex);
    }
    node
}

fn branch_vertex(adjacency: &[BitWords], weights: &[i128], candidates: &[usize]) -> usize {
    candidates
        .iter()
        .copied()
        .max_by_key(|vertex| {
            (
                degree(adjacency, *vertex, candidates) as i128 * weights[*vertex],
                *vertex,
            )
        })
        .expect("nonempty branch candidate")
}

fn write_numbers(numbers: &[usize], payload: &mut String) {
    for (index, number) in numbers.iter().enumerate() {
        if index > 0 {
            payload.push(',');
        }
        payload.push_str(&number.to_string());
    }
}

fn write_local_as_global(local: &[usize], candidates: &[usize], payload: &mut String) {
    for (index, vertex) in local.iter().enumerate() {
        if index > 0 {
            payload.push(',');
        }
        payload.push_str(&candidates[*vertex].to_string());
    }
}

fn push_sorted(values: &[usize], value: usize) -> Vec<usize> {
    let mut next = values.to_vec();
    next.push(value);
    next.sort_unstable();
    next
}

fn has_duplicates(values: &[usize]) -> bool {
    let mut seen = HashSet::new();
    values.iter().any(|value| !seen.insert(*value))
}

fn degree(adjacency: &[BitWords], vertex: usize, candidates: &[usize]) -> usize {
    candidates
        .iter()
        .filter(|candidate| **candidate != vertex && has_bit(&adjacency[vertex], **candidate))
        .count()
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(G27GeometricFractionalError::MalformedData { source })
}
