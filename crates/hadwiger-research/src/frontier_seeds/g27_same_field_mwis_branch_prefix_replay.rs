use std::cmp::Ordering;
use std::collections::BinaryHeap;

use sha2::{Digest, Sha256};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_lp_relaxation::{
    stable_set_lp_relaxation_rows, StableSetLpRelaxationRows,
};
use super::g27_same_field_mwis_branch_certificate_preflight::{
    branch_vertex, degree, dominant_and_exact_side_weight,
};
use super::g27_same_field_mwis_odd_cycle_dual_replay::{
    replay_g27_same_field_mwis_odd_cycle_one_sided_duals_checked, G27MwisOddCycleDualReplayStatus,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_alignment_channel_instance_sets;

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const EXPECTED_EXACT_SIDE_WEIGHT: i128 = 61_655;
const EXPECTED_DOMINANT_THRESHOLD: i128 = 451_278;
const EXPECTED_ROOT_TOTAL: i128 = 543_428;
const TARGET_PRUNES: usize = 2;
const EXPANSION_CAP: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisBranchPrefixReplayStatus {
    BranchPrefixSemanticsPreflight,
    FrozenInstanceMismatch,
    RootObjectiveMismatch,
    MissingCertifiedPrunes,
    UnstablePrefixReplay,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisBranchPrefixReplayReport {
    expanded_nodes: usize,
    pruned_nodes: usize,
    open_frontier_nodes: usize,
    best_open_total_bound: i128,
    root_total_bound: i128,
    h32c_certified_prunes: usize,
    prefix_digest: String,
    status: G27MwisBranchPrefixReplayStatus,
}

impl G27MwisBranchPrefixReplayReport {
    pub fn summary(&self) -> (usize, usize, usize, i128, i128, usize) {
        (
            self.expanded_nodes,
            self.pruned_nodes,
            self.open_frontier_nodes,
            self.best_open_total_bound,
            self.root_total_bound,
            self.h32c_certified_prunes,
        )
    }

    pub fn status(&self) -> G27MwisBranchPrefixReplayStatus {
        self.status
    }
}

pub fn replay_g27_same_field_mwis_branch_prefix_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisBranchPrefixReplayReport, G27GeometricFractionalError> {
    let Some(certified_prunes) = certified_h32c_prunes(handle)? else {
        return Ok(empty_report(
            G27MwisBranchPrefixReplayStatus::MissingCertifiedPrunes,
        ));
    };
    let first = build_prefix(handle, certified_prunes)?;
    if first.status != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight {
        return Ok(first);
    }
    let second = build_prefix(handle, certified_prunes)?;
    if first.summary() != second.summary() || first.prefix_digest != second.prefix_digest {
        return Ok(G27MwisBranchPrefixReplayReport {
            status: G27MwisBranchPrefixReplayStatus::UnstablePrefixReplay,
            ..first
        });
    }
    Ok(first)
}

fn certified_h32c_prunes(
    handle: &HadwigerResearchHandle,
) -> Result<Option<usize>, G27GeometricFractionalError> {
    let h32c = replay_g27_same_field_mwis_odd_cycle_one_sided_duals_checked(handle)?;
    let (_, certified_prunes, _, _, h32c_root_total) = h32c.summary();
    let (_, min_slack, max_excess) = h32c.exact_summary();
    if h32c.status() != G27MwisOddCycleDualReplayStatus::OneSidedOddCycleNodeDualReplayPreflight
        || certified_prunes != TARGET_PRUNES
        || h32c_root_total != EXPECTED_ROOT_TOTAL
        || min_slack < 0
        || max_excess != 0
    {
        return Ok(None);
    }
    Ok(Some(certified_prunes))
}

fn build_prefix(
    handle: &HadwigerResearchHandle,
    certified_prunes: usize,
) -> Result<G27MwisBranchPrefixReplayReport, G27GeometricFractionalError> {
    let mut channel_sets = threshold_mwis_alignment_channel_instance_sets(
        handle,
        &[(G27_ANCHOR_INDEX, W_ANCHOR_INDEX)],
        ATOM_LIMIT,
    )?;
    let channel = channel_sets
        .pop()
        .and_then(|channels| {
            channels
                .into_iter()
                .find(|channel| channel.atom_mask == ATOM_MASK)
        })
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "branch_prefix_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    let threshold = TARGET_WEIGHT - small_weight;
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT || threshold != EXPECTED_DOMINANT_THRESHOLD {
        return Ok(empty_report(
            G27MwisBranchPrefixReplayStatus::FrozenInstanceMismatch,
        ));
    }
    replay_prefix(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
        threshold,
        certified_prunes,
    )
}

#[derive(Clone)]
struct NodeState {
    branch_included: Vec<usize>,
    forced_included: Vec<usize>,
    excluded: Vec<usize>,
    candidates: Vec<usize>,
    chosen_weight: i128,
    depth: usize,
}

struct QueueEntry {
    upper_bound: i128,
    sequence: usize,
    node: NodeState,
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

fn replay_prefix(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
    threshold: i128,
    certified_prunes: usize,
) -> Result<G27MwisBranchPrefixReplayReport, G27GeometricFractionalError> {
    let root = include_isolated(adjacency, weights, NodeState::root(candidates));
    let root_rows = stable_set_lp_relaxation_rows(adjacency, weights, &root.candidates)?;
    let root_upper = root.chosen_weight + root_rows.odd_cycle_objective_ceiling;
    if exact_side_weight + root_upper != EXPECTED_ROOT_TOTAL {
        return Ok(empty_report(
            G27MwisBranchPrefixReplayStatus::RootObjectiveMismatch,
        ));
    }
    let mut payload = String::new();
    write_node("root", &root, root_upper, &root_rows, &mut payload);
    let mut queue = BinaryHeap::from([QueueEntry {
        upper_bound: root_upper,
        sequence: 0,
        node: root,
    }]);
    let mut sequence = 1;
    let mut expanded = 0;
    let mut pruned = 0;
    while expanded < EXPANSION_CAP && pruned < TARGET_PRUNES {
        let Some(entry) = queue.pop() else { break };
        if entry.upper_bound <= threshold {
            let rows = stable_set_lp_relaxation_rows(adjacency, weights, &entry.node.candidates)?;
            pruned += 1;
            write_node(
                "pruned-frontier",
                &entry.node,
                entry.upper_bound,
                &rows,
                &mut payload,
            );
            continue;
        }
        expanded += 1;
        let branch = branch_vertex(adjacency, weights, &entry.node.candidates);
        for (label, child) in branch_children(adjacency, weights, &entry.node, branch) {
            let rows = stable_set_lp_relaxation_rows(adjacency, weights, &child.candidates)?;
            let upper = child.chosen_weight + rows.odd_cycle_objective_ceiling;
            write_branch(&entry.node, &child, branch, label, &mut payload)?;
            write_node(label, &child, upper, &rows, &mut payload);
            if upper <= threshold {
                pruned += 1;
            } else {
                queue.push(QueueEntry {
                    upper_bound: upper,
                    sequence,
                    node: child,
                });
                sequence += 1;
            }
        }
    }
    Ok(G27MwisBranchPrefixReplayReport {
        expanded_nodes: expanded,
        pruned_nodes: pruned,
        open_frontier_nodes: queue.len(),
        best_open_total_bound: exact_side_weight
            + queue.peek().map(|entry| entry.upper_bound).unwrap_or(0),
        root_total_bound: EXPECTED_ROOT_TOTAL,
        h32c_certified_prunes: certified_prunes,
        prefix_digest: format!("{:x}", Sha256::digest(payload.as_bytes())),
        status: G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight,
    })
}

impl NodeState {
    fn root(candidates: &[usize]) -> Self {
        Self {
            branch_included: Vec::new(),
            forced_included: Vec::new(),
            excluded: Vec::new(),
            candidates: candidates.to_vec(),
            chosen_weight: 0,
            depth: 0,
        }
    }
}

fn branch_children(
    adjacency: &[BitWords],
    weights: &[i128],
    node: &NodeState,
    branch: usize,
) -> [(&'static str, NodeState); 2] {
    let remaining = node
        .candidates
        .iter()
        .copied()
        .filter(|vertex| *vertex != branch)
        .collect::<Vec<_>>();
    let include_candidates = remaining
        .iter()
        .copied()
        .filter(|vertex| !has_bit(&adjacency[branch], *vertex))
        .collect::<Vec<_>>();
    let include = include_isolated(
        adjacency,
        weights,
        NodeState {
            branch_included: push_sorted(&node.branch_included, branch),
            forced_included: node.forced_included.clone(),
            excluded: node.excluded.clone(),
            candidates: include_candidates,
            chosen_weight: node.chosen_weight + weights[branch],
            depth: node.depth + 1,
        },
    );
    let exclude = include_isolated(
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
    );
    [("include", include), ("exclude", exclude)]
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

fn write_branch(
    parent: &NodeState,
    child: &NodeState,
    branch: usize,
    label: &str,
    payload: &mut String,
) -> Result<(), G27GeometricFractionalError> {
    let include_has_branch = child.branch_included.binary_search(&branch).is_ok();
    let exclude_has_branch = child.excluded.binary_search(&branch).is_ok();
    if (label == "include" && !include_has_branch)
        || (label == "exclude" && !exclude_has_branch)
        || include_has_branch == exclude_has_branch
        || child.depth != parent.depth + 1
    {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "branch_prefix_child_semantics",
        });
    }
    payload.push_str("B|");
    payload.push_str(label);
    payload.push('|');
    payload.push_str(&branch.to_string());
    payload.push('|');
    write_numbers(&parent.candidates, payload);
    payload.push('|');
    write_numbers(&child.candidates, payload);
    payload.push('\n');
    Ok(())
}

fn write_node(
    label: &str,
    node: &NodeState,
    upper_bound: i128,
    rows: &StableSetLpRelaxationRows,
    payload: &mut String,
) {
    payload.push_str("N|");
    payload.push_str(label);
    payload.push('|');
    payload.push_str(&node.depth.to_string());
    payload.push('|');
    payload.push_str(&node.chosen_weight.to_string());
    payload.push('|');
    payload.push_str(&upper_bound.to_string());
    payload.push('|');
    write_numbers(&node.branch_included, payload);
    payload.push('|');
    write_numbers(&node.forced_included, payload);
    payload.push('|');
    write_numbers(&node.excluded, payload);
    payload.push('|');
    write_numbers(&node.candidates, payload);
    payload.push('|');
    payload.push_str(&rows.clique_constraints.len().to_string());
    payload.push('|');
    payload.push_str(&rows.odd_cycle_cuts.len().to_string());
    payload.push('|');
    payload.push_str(&rows.odd_cycle_objective_ceiling.to_string());
    payload.push('\n');
}

fn push_sorted(values: &[usize], value: usize) -> Vec<usize> {
    let mut next = values.to_vec();
    next.push(value);
    next.sort_unstable();
    next
}

fn write_numbers(numbers: &[usize], payload: &mut String) {
    for (index, number) in numbers.iter().enumerate() {
        if index > 0 {
            payload.push(',');
        }
        payload.push_str(&number.to_string());
    }
}

fn empty_report(status: G27MwisBranchPrefixReplayStatus) -> G27MwisBranchPrefixReplayReport {
    G27MwisBranchPrefixReplayReport {
        expanded_nodes: 0,
        pruned_nodes: 0,
        open_frontier_nodes: 0,
        best_open_total_bound: 0,
        root_total_bound: 0,
        h32c_certified_prunes: 0,
        prefix_digest: String::new(),
        status,
    }
}
