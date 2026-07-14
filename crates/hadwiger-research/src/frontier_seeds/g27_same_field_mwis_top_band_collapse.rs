use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

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
const EXPECTED_BEST_OPEN_TOTAL: i128 = 518_612;
const COLLAPSE_TOTAL: i128 = 517_612;
const AMBIGUOUS_TOTAL: i128 = 518_112;
const TOP_K: usize = 10;
const BAND_WIDTH: i128 = 1_000;
const EXPANSION_CAP: usize = 20;
const PER_ORIGIN_CAP: usize = 3;
const SECONDS_CAP: u64 = 300;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisTopBandCollapseStatus {
    TopBandCollapsed,
    AmbiguousBandThinned,
    SolverPrunesNeedExactDualReplay,
    BoundedContinuationNoUsefulProgress,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisTopBandCollapseReport {
    initial_best_total: i128,
    final_best_total: i128,
    final_tied_band_nodes: usize,
    selected_origin_count: usize,
    expanded_nodes: usize,
    solver_pruned_descendants: usize,
    open_frontier_nodes: usize,
    elapsed_millis: u128,
    status: G27MwisTopBandCollapseStatus,
}

impl G27MwisTopBandCollapseReport {
    pub fn summary(&self) -> (i128, i128, usize, usize, usize, usize) {
        (
            self.initial_best_total,
            self.final_best_total,
            self.final_tied_band_nodes,
            self.selected_origin_count,
            self.expanded_nodes,
            self.solver_pruned_descendants,
        )
    }

    pub fn search_summary(&self) -> (usize, u128) {
        (self.open_frontier_nodes, self.elapsed_millis)
    }

    pub fn status(&self) -> G27MwisTopBandCollapseStatus {
        self.status
    }
}

pub fn preflight_g27_same_field_mwis_top_band_collapse_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisTopBandCollapseReport, G27GeometricFractionalError> {
    let prefix = replay_g27_same_field_mwis_branch_prefix_checked(handle)?;
    let (expanded, pruned, open, best_open_total, _, _) = prefix.summary();
    if prefix.status() != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight
        || expanded != EXPECTED_PREFIX_EXPANDED
        || pruned != EXPECTED_PREFIX_PRUNED
        || open != EXPECTED_PREFIX_OPEN
        || best_open_total != EXPECTED_BEST_OPEN_TOTAL
    {
        return Ok(empty_report(
            G27MwisTopBandCollapseStatus::PrefixReplayMismatch,
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
            source: "top_band_collapse_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return Ok(empty_report(
            G27MwisTopBandCollapseStatus::FrozenInstanceMismatch,
        ));
    }
    collapse_top_band(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
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
    origin: Option<usize>,
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

fn collapse_top_band(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
) -> Result<G27MwisTopBandCollapseReport, G27GeometricFractionalError> {
    let started = Instant::now();
    let threshold = TARGET_WEIGHT - exact_side_weight;
    let frontier = initial_frontier(adjacency, weights, candidates, threshold)?;
    let selected_count = frontier
        .iter()
        .take(TOP_K)
        .filter(|entry| {
            exact_side_weight + entry.upper_bound >= EXPECTED_BEST_OPEN_TOTAL - BAND_WIDTH
        })
        .count();
    let mut queue = BinaryHeap::new();
    for (index, mut entry) in frontier.into_iter().enumerate() {
        if index < selected_count {
            entry.origin = Some(index);
        }
        queue.push(entry);
    }
    let mut origin_expansions = vec![0_usize; selected_count];
    let mut expanded_nodes = 0;
    let mut solver_prunes = 0;
    let mut sequence = EXPECTED_PREFIX_OPEN + 1;
    while expanded_nodes < EXPANSION_CAP
        && started.elapsed() < Duration::from_secs(SECONDS_CAP)
        && best_total(&queue, exact_side_weight) > COLLAPSE_TOTAL
    {
        let Some(entry) = queue.pop() else { break };
        let Some(origin) = entry.origin else {
            queue.push(entry);
            break;
        };
        if origin_expansions[origin] >= PER_ORIGIN_CAP || entry.upper_bound <= threshold {
            queue.push(entry);
            break;
        }
        origin_expansions[origin] += 1;
        expanded_nodes += 1;
        let branch = branch_vertex(adjacency, weights, &entry.node.candidates);
        for child in branch_children(adjacency, weights, &entry.node, branch) {
            let upper_bound = node_upper(adjacency, weights, &child)?;
            if upper_bound <= threshold {
                solver_prunes += 1;
            } else {
                queue.push(QueueEntry {
                    upper_bound,
                    sequence,
                    origin: Some(origin),
                    node: child,
                });
                sequence += 1;
            }
        }
    }
    let final_best = best_total(&queue, exact_side_weight);
    let final_tied = queue
        .iter()
        .filter(|entry| exact_side_weight + entry.upper_bound >= final_best - BAND_WIDTH)
        .count();
    let status = if final_best <= COLLAPSE_TOTAL {
        G27MwisTopBandCollapseStatus::TopBandCollapsed
    } else if solver_prunes > 0 {
        G27MwisTopBandCollapseStatus::SolverPrunesNeedExactDualReplay
    } else if final_best <= AMBIGUOUS_TOTAL && final_tied <= 4 {
        G27MwisTopBandCollapseStatus::AmbiguousBandThinned
    } else {
        G27MwisTopBandCollapseStatus::BoundedContinuationNoUsefulProgress
    };
    Ok(G27MwisTopBandCollapseReport {
        initial_best_total: EXPECTED_BEST_OPEN_TOTAL,
        final_best_total: final_best,
        final_tied_band_nodes: final_tied,
        selected_origin_count: selected_count,
        expanded_nodes,
        solver_pruned_descendants: solver_prunes,
        open_frontier_nodes: queue.len(),
        elapsed_millis: started.elapsed().as_millis(),
        status,
    })
}

fn initial_frontier(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    threshold: i128,
) -> Result<Vec<QueueEntry>, G27GeometricFractionalError> {
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
        origin: None,
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
                    origin: None,
                    node: child,
                });
                sequence += 1;
            }
        }
    }
    let mut frontier = queue.into_sorted_vec();
    frontier.reverse();
    Ok(frontier)
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

fn best_total(queue: &BinaryHeap<QueueEntry>, exact_side_weight: i128) -> i128 {
    exact_side_weight + queue.peek().map(|entry| entry.upper_bound).unwrap_or(0)
}

fn empty_report(status: G27MwisTopBandCollapseStatus) -> G27MwisTopBandCollapseReport {
    G27MwisTopBandCollapseReport {
        initial_best_total: 0,
        final_best_total: 0,
        final_tied_band_nodes: 0,
        selected_origin_count: 0,
        expanded_nodes: 0,
        solver_pruned_descendants: 0,
        open_frontier_nodes: 0,
        elapsed_millis: 0,
        status,
    }
}
