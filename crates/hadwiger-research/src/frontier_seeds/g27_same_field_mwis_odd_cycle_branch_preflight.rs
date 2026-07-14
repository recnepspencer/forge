use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::BitWords;
use super::g27_same_field_lp_relaxation::stable_set_lp_relaxation_bound;
use super::g27_same_field_mwis_branch_certificate_preflight::{
    branch_children, branch_vertex, dominant_and_exact_side_weight, include_isolated, SearchNode,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_alignment_channel_instance_sets;

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const EXPECTED_EXACT_SIDE_WEIGHT: i128 = 61_655;
const EXPECTED_DOMINANT_THRESHOLD: i128 = 451_278;
const ROOT_REPRODUCTION_TOTAL_CEILING: i128 = 543_428;
const CONTINUATION_MOVEMENT: i128 = 10_000;
const NODE_CAP: usize = 64;
const SECONDS_CAP: u64 = 300;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisOddCycleBranchPreflightStatus {
    ProvedBelowThreshold,
    PromisingOddCycleBranchBound,
    RootReproductionFailed,
    FrozenInstanceMismatch,
    RetiredSlowNodeBound,
    RetiredWeakOddCycleNodeBound,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisOddCycleBranchPreflightReport {
    atom_mask: u32,
    dominant_vertex_count: usize,
    exact_side_component_weight: i128,
    dominant_threshold: i128,
    root_total_odd_cycle_bound: i128,
    best_open_total_odd_cycle_bound: i128,
    node_count: usize,
    open_node_count: usize,
    pruned_below_threshold_count: usize,
    max_depth: usize,
    elapsed_millis: u128,
    total_odd_cycle_cuts: usize,
    max_node_millis: u128,
    status: G27MwisOddCycleBranchPreflightStatus,
}

impl G27MwisOddCycleBranchPreflightReport {
    pub fn summary(&self) -> (u32, usize, i128, i128, i128, i128) {
        (
            self.atom_mask,
            self.dominant_vertex_count,
            self.exact_side_component_weight,
            self.dominant_threshold,
            self.root_total_odd_cycle_bound,
            self.best_open_total_odd_cycle_bound,
        )
    }

    pub fn search_summary(&self) -> (usize, usize, usize, usize, u128) {
        (
            self.node_count,
            self.open_node_count,
            self.pruned_below_threshold_count,
            self.max_depth,
            self.elapsed_millis,
        )
    }

    pub fn lp_summary(&self) -> (usize, u128) {
        (self.total_odd_cycle_cuts, self.max_node_millis)
    }

    pub fn status(&self) -> G27MwisOddCycleBranchPreflightStatus {
        self.status
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

pub fn preflight_g27_same_field_mwis_odd_cycle_branch_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisOddCycleBranchPreflightReport, G27GeometricFractionalError> {
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
            source: "odd_cycle_branch_preflight_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    let dominant_threshold = TARGET_WEIGHT - small_weight;
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT
        || dominant_threshold != EXPECTED_DOMINANT_THRESHOLD
    {
        return Ok(G27MwisOddCycleBranchPreflightReport::empty_with_status(
            dominant.len(),
            small_weight,
            dominant_threshold,
            G27MwisOddCycleBranchPreflightStatus::FrozenInstanceMismatch,
        ));
    }
    let search = branch_preflight(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        dominant_threshold,
    )?;
    let root_total = small_weight + search.root_upper_bound;
    let best_open_total = small_weight + search.best_open_upper_bound;
    let status = if root_total > ROOT_REPRODUCTION_TOTAL_CEILING {
        G27MwisOddCycleBranchPreflightStatus::RootReproductionFailed
    } else if search.best_open_upper_bound <= dominant_threshold || search.open_node_count == 0 {
        G27MwisOddCycleBranchPreflightStatus::ProvedBelowThreshold
    } else if best_open_total <= root_total - CONTINUATION_MOVEMENT {
        G27MwisOddCycleBranchPreflightStatus::PromisingOddCycleBranchBound
    } else if search.node_count < NODE_CAP
        && search.elapsed_millis >= u128::from(SECONDS_CAP) * 1000
    {
        G27MwisOddCycleBranchPreflightStatus::RetiredSlowNodeBound
    } else {
        G27MwisOddCycleBranchPreflightStatus::RetiredWeakOddCycleNodeBound
    };
    Ok(G27MwisOddCycleBranchPreflightReport {
        atom_mask: ATOM_MASK,
        dominant_vertex_count: dominant.len(),
        exact_side_component_weight: small_weight,
        dominant_threshold,
        root_total_odd_cycle_bound: root_total,
        best_open_total_odd_cycle_bound: best_open_total,
        node_count: search.node_count,
        open_node_count: search.open_node_count,
        pruned_below_threshold_count: search.pruned_below_threshold_count,
        max_depth: search.max_depth,
        elapsed_millis: search.elapsed_millis,
        total_odd_cycle_cuts: search.total_odd_cycle_cuts,
        max_node_millis: search.max_node_millis,
        status,
    })
}

impl G27MwisOddCycleBranchPreflightReport {
    fn empty_with_status(
        dominant_vertex_count: usize,
        exact_side_component_weight: i128,
        dominant_threshold: i128,
        status: G27MwisOddCycleBranchPreflightStatus,
    ) -> Self {
        Self {
            atom_mask: ATOM_MASK,
            dominant_vertex_count,
            exact_side_component_weight,
            dominant_threshold,
            root_total_odd_cycle_bound: 0,
            best_open_total_odd_cycle_bound: 0,
            node_count: 0,
            open_node_count: 0,
            pruned_below_threshold_count: 0,
            max_depth: 0,
            elapsed_millis: 0,
            total_odd_cycle_cuts: 0,
            max_node_millis: 0,
            status,
        }
    }
}

struct QueueEntry {
    upper_bound: i128,
    sequence: usize,
    node: SearchNode,
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

struct SearchResult {
    root_upper_bound: i128,
    best_open_upper_bound: i128,
    node_count: usize,
    open_node_count: usize,
    pruned_below_threshold_count: usize,
    max_depth: usize,
    elapsed_millis: u128,
    total_odd_cycle_cuts: usize,
    max_node_millis: u128,
}

struct NodeBound {
    upper_bound: i128,
    odd_cycle_cuts: usize,
    elapsed_millis: u128,
}

fn branch_preflight(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    threshold: i128,
) -> Result<SearchResult, G27GeometricFractionalError> {
    let started = Instant::now();
    let root = include_isolated(
        adjacency,
        weights,
        SearchNode {
            candidates: candidates.to_vec(),
            chosen_weight: 0,
            depth: 0,
        },
    );
    let root_bound = node_upper_bound(adjacency, weights, &root)?;
    let mut total_cuts = root_bound.odd_cycle_cuts;
    let mut max_node_millis = root_bound.elapsed_millis;
    let root_upper = root_bound.upper_bound;
    let mut queue = BinaryHeap::from([QueueEntry {
        upper_bound: root_upper,
        sequence: 0,
        node: root,
    }]);
    let mut sequence = 1;
    let mut node_count = 0;
    let mut pruned = 0;
    let mut max_depth = 0;
    while node_count < NODE_CAP && started.elapsed() < Duration::from_secs(SECONDS_CAP) {
        let Some(entry) = queue.pop() else {
            break;
        };
        if entry.upper_bound <= threshold {
            pruned += 1;
            continue;
        }
        node_count += 1;
        max_depth = max_depth.max(entry.node.depth);
        if entry.node.candidates.is_empty() {
            continue;
        }
        let branch = branch_vertex(adjacency, weights, &entry.node.candidates);
        for child in branch_children(adjacency, weights, entry.node, branch) {
            let bound = node_upper_bound(adjacency, weights, &child)?;
            total_cuts += bound.odd_cycle_cuts;
            max_node_millis = max_node_millis.max(bound.elapsed_millis);
            if bound.upper_bound <= threshold {
                pruned += 1;
            } else {
                queue.push(QueueEntry {
                    upper_bound: bound.upper_bound,
                    sequence,
                    node: child,
                });
                sequence += 1;
            }
        }
    }
    Ok(SearchResult {
        root_upper_bound: root_upper,
        best_open_upper_bound: queue.peek().map(|entry| entry.upper_bound).unwrap_or(0),
        node_count,
        open_node_count: queue.len(),
        pruned_below_threshold_count: pruned,
        max_depth,
        elapsed_millis: started.elapsed().as_millis(),
        total_odd_cycle_cuts: total_cuts,
        max_node_millis,
    })
}

fn node_upper_bound(
    adjacency: &[BitWords],
    weights: &[i128],
    node: &SearchNode,
) -> Result<NodeBound, G27GeometricFractionalError> {
    let started = Instant::now();
    let lp = stable_set_lp_relaxation_bound(adjacency, weights, &node.candidates)?;
    Ok(NodeBound {
        upper_bound: node.chosen_weight + lp.odd_cycle_objective_ceiling,
        odd_cycle_cuts: lp.odd_cycle_cut_count,
        elapsed_millis: started.elapsed().as_millis(),
    })
}
