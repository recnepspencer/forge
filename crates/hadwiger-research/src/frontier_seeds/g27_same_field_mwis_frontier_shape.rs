use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_lp_relaxation::stable_set_lp_relaxation_rows;
use super::g27_same_field_mwis_branch_certificate_preflight::{
    branch_vertex, degree, dominant_and_exact_side_weight,
};
use super::g27_same_field_mwis_branch_prefix_replay::{
    replay_g27_same_field_mwis_branch_prefix_checked, G27MwisBranchPrefixReplayStatus,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_alignment_channel_instance_sets;

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const EXPECTED_EXACT_SIDE_WEIGHT: i128 = 61_655;
const EXPECTED_PREFIX_EXPANDED: usize = 29;
const EXPECTED_PREFIX_PRUNED: usize = 2;
const EXPECTED_PREFIX_OPEN: usize = 28;
const EXPECTED_PREFIX_BEST_OPEN_TOTAL: i128 = 518_612;
const TOP_COUNT: usize = 10;
const TIE_BAND_WIDTH: i128 = 1_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisFrontierShapeStatus {
    SingleWorstNodeContinuationPromising,
    TiedFrontierBandRequiresTopKContinuation,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisFrontierShapeReport {
    top_open_totals: Vec<i128>,
    top_open_depths: Vec<usize>,
    open_frontier_nodes: usize,
    tied_band_nodes: usize,
    gap_to_second: i128,
    best_open_total_bound: i128,
    status: G27MwisFrontierShapeStatus,
}

impl G27MwisFrontierShapeReport {
    pub fn summary(&self) -> (usize, usize, i128, i128) {
        (
            self.open_frontier_nodes,
            self.tied_band_nodes,
            self.gap_to_second,
            self.best_open_total_bound,
        )
    }

    pub fn top_open_totals(&self) -> &[i128] {
        &self.top_open_totals
    }

    pub fn top_open_depths(&self) -> &[usize] {
        &self.top_open_depths
    }

    pub fn status(&self) -> G27MwisFrontierShapeStatus {
        self.status
    }
}

pub fn diagnose_g27_same_field_mwis_frontier_shape_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisFrontierShapeReport, G27GeometricFractionalError> {
    diagnose_frontier_shape(handle, TOP_COUNT)
}

pub fn diagnose_g27_same_field_mwis_full_frontier_shape_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisFrontierShapeReport, G27GeometricFractionalError> {
    diagnose_frontier_shape(handle, EXPECTED_PREFIX_OPEN)
}

fn diagnose_frontier_shape(
    handle: &HadwigerResearchHandle,
    top_count: usize,
) -> Result<G27MwisFrontierShapeReport, G27GeometricFractionalError> {
    let prefix = replay_g27_same_field_mwis_branch_prefix_checked(handle)?;
    let (expanded, pruned, open, best_open_total, _, _) = prefix.summary();
    if prefix.status() != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight
        || expanded != EXPECTED_PREFIX_EXPANDED
        || pruned != EXPECTED_PREFIX_PRUNED
        || open != EXPECTED_PREFIX_OPEN
        || best_open_total != EXPECTED_PREFIX_BEST_OPEN_TOTAL
    {
        return Ok(empty_report(
            G27MwisFrontierShapeStatus::PrefixReplayMismatch,
        ));
    }
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
            source: "frontier_shape_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return Ok(empty_report(
            G27MwisFrontierShapeStatus::FrozenInstanceMismatch,
        ));
    }
    frontier_shape(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
        top_count,
    )
}

#[derive(Clone)]
struct NodeState {
    candidates: Vec<usize>,
    chosen_weight: i128,
    depth: usize,
}

#[derive(Clone)]
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

fn frontier_shape(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
    top_count: usize,
) -> Result<G27MwisFrontierShapeReport, G27GeometricFractionalError> {
    let threshold = TARGET_WEIGHT - exact_side_weight;
    let root = include_isolated(
        adjacency,
        weights,
        NodeState {
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
    while expanded < EXPECTED_PREFIX_EXPANDED || pruned < EXPECTED_PREFIX_PRUNED {
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
    let mut entries = queue.into_sorted_vec();
    entries.reverse();
    let top = entries.iter().take(top_count).collect::<Vec<_>>();
    let top_open_totals = top
        .iter()
        .map(|entry| exact_side_weight + entry.upper_bound)
        .collect::<Vec<_>>();
    let top_open_depths = top.iter().map(|entry| entry.node.depth).collect::<Vec<_>>();
    let best = top_open_totals.first().copied().unwrap_or(0);
    let second = top_open_totals.get(1).copied().unwrap_or(best);
    let tied_band_nodes = entries
        .iter()
        .filter(|entry| exact_side_weight + entry.upper_bound >= best - TIE_BAND_WIDTH)
        .count();
    let status = if tied_band_nodes > 1 {
        G27MwisFrontierShapeStatus::TiedFrontierBandRequiresTopKContinuation
    } else {
        G27MwisFrontierShapeStatus::SingleWorstNodeContinuationPromising
    };
    Ok(G27MwisFrontierShapeReport {
        top_open_totals,
        top_open_depths,
        open_frontier_nodes: entries.len(),
        tied_band_nodes,
        gap_to_second: best - second,
        best_open_total_bound: best,
        status,
    })
}

fn branch_children(
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
                candidates: included,
                chosen_weight: node.chosen_weight + weights[branch],
                depth: node.depth + 1,
            },
        ),
        include_isolated(
            adjacency,
            weights,
            NodeState {
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
        node.candidates.retain(|candidate| *candidate != vertex);
    }
    node
}

fn node_upper(
    adjacency: &[BitWords],
    weights: &[i128],
    node: &NodeState,
) -> Result<i128, G27GeometricFractionalError> {
    let rows = stable_set_lp_relaxation_rows(adjacency, weights, &node.candidates)?;
    Ok(node.chosen_weight + rows.odd_cycle_objective_ceiling)
}

fn empty_report(status: G27MwisFrontierShapeStatus) -> G27MwisFrontierShapeReport {
    G27MwisFrontierShapeReport {
        top_open_totals: Vec::new(),
        top_open_depths: Vec::new(),
        open_frontier_nodes: 0,
        tied_band_nodes: 0,
        gap_to_second: 0,
        best_open_total_bound: 0,
        status,
    }
}
