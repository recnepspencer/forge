use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{
    clique_cover_weight_upper_bound, has_bit, BitWords,
};
use super::g27_same_field_mwis_exact::exact_mwis;
use super::g27_same_field_threshold_mwis_bnb_setup::{
    threshold_mwis_alignment_channel_instance_sets, ThresholdMwisInstance,
};

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const EXACT_SIDE_COMPONENT_LIMIT: usize = 32;
const NODE_CAP: usize = 10_000;
const SECONDS_CAP: u64 = 300;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisBranchCertificatePreflightStatus {
    ProvedBelowThreshold,
    RetiredWeakNodeBound,
    UndecidedCap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisBranchCertificatePreflightReport {
    atom_mask: u32,
    dominant_vertex_count: usize,
    exact_side_component_weight: i128,
    dominant_threshold: i128,
    root_total_upper_bound: i128,
    best_open_total_upper_bound: i128,
    node_count: usize,
    open_node_count: usize,
    pruned_below_threshold_count: usize,
    max_depth: usize,
    elapsed_millis: u128,
    status: G27MwisBranchCertificatePreflightStatus,
}

impl G27MwisBranchCertificatePreflightReport {
    pub fn summary(&self) -> (u32, usize, i128, i128, i128, i128) {
        (
            self.atom_mask,
            self.dominant_vertex_count,
            self.exact_side_component_weight,
            self.dominant_threshold,
            self.root_total_upper_bound,
            self.best_open_total_upper_bound,
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

    pub fn status(&self) -> G27MwisBranchCertificatePreflightStatus {
        self.status
    }

    pub fn admits_theorem_authority(&self) -> bool {
        self.status == G27MwisBranchCertificatePreflightStatus::ProvedBelowThreshold
    }
}

pub fn preflight_g27_same_field_mwis_branch_certificate_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisBranchCertificatePreflightReport, G27GeometricFractionalError> {
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
            source: "branch_preflight_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    let dominant_threshold = TARGET_WEIGHT - small_weight;
    let search = branch_preflight(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        dominant_threshold,
    );
    let status = if search.best_open_upper_bound < dominant_threshold {
        G27MwisBranchCertificatePreflightStatus::ProvedBelowThreshold
    } else if search.node_count >= NODE_CAP
        && small_weight + search.best_open_upper_bound > TARGET_WEIGHT + 50_000
    {
        G27MwisBranchCertificatePreflightStatus::RetiredWeakNodeBound
    } else {
        G27MwisBranchCertificatePreflightStatus::UndecidedCap
    };
    Ok(G27MwisBranchCertificatePreflightReport {
        atom_mask: ATOM_MASK,
        dominant_vertex_count: dominant.len(),
        exact_side_component_weight: small_weight,
        dominant_threshold,
        root_total_upper_bound: small_weight + search.root_upper_bound,
        best_open_total_upper_bound: small_weight + search.best_open_upper_bound,
        node_count: search.node_count,
        open_node_count: search.open_node_count,
        pruned_below_threshold_count: search.pruned_below_threshold_count,
        max_depth: search.max_depth,
        elapsed_millis: search.elapsed_millis,
        status,
    })
}

#[derive(Clone)]
pub(super) struct SearchNode {
    pub(super) candidates: Vec<usize>,
    pub(super) chosen_weight: i128,
    pub(super) depth: usize,
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
}

fn branch_preflight(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    threshold: i128,
) -> SearchResult {
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
    let root_upper = node_upper_bound(adjacency, weights, &root);
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
        if entry.upper_bound < threshold {
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
            let upper_bound = node_upper_bound(adjacency, weights, &child);
            if upper_bound < threshold {
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
    SearchResult {
        root_upper_bound: root_upper,
        best_open_upper_bound: queue.peek().map(|entry| entry.upper_bound).unwrap_or(0),
        node_count,
        open_node_count: queue.len(),
        pruned_below_threshold_count: pruned,
        max_depth,
        elapsed_millis: started.elapsed().as_millis(),
    }
}

pub(super) fn branch_children(
    adjacency: &[BitWords],
    weights: &[i128],
    node: SearchNode,
    branch: usize,
) -> [SearchNode; 2] {
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
            SearchNode {
                candidates: included,
                chosen_weight: node.chosen_weight + weights[branch],
                depth: node.depth + 1,
            },
        ),
        include_isolated(
            adjacency,
            weights,
            SearchNode {
                candidates: remaining,
                chosen_weight: node.chosen_weight,
                depth: node.depth + 1,
            },
        ),
    ]
}

pub(super) fn include_isolated(
    adjacency: &[BitWords],
    weights: &[i128],
    mut node: SearchNode,
) -> SearchNode {
    loop {
        let isolated = node
            .candidates
            .iter()
            .copied()
            .find(|vertex| degree(adjacency, *vertex, &node.candidates) == 0);
        let Some(vertex) = isolated else {
            break;
        };
        node.chosen_weight += weights[vertex];
        node.candidates.retain(|candidate| *candidate != vertex);
    }
    node
}

fn node_upper_bound(adjacency: &[BitWords], weights: &[i128], node: &SearchNode) -> i128 {
    node.chosen_weight + clique_cover_weight_upper_bound(adjacency, weights, &node.candidates)
}

pub(super) fn branch_vertex(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
) -> usize {
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

pub(super) fn dominant_and_exact_side_weight(
    instance: &ThresholdMwisInstance,
) -> (Vec<usize>, i128) {
    let components = connected_components(&instance.adjacency, &instance.candidates);
    let mut dominant = Vec::new();
    let mut small_weight = 0;
    for (index, component) in components.iter().enumerate() {
        if index == 0 || component.len() > EXACT_SIDE_COMPONENT_LIMIT {
            dominant.extend(component);
        } else {
            small_weight += exact_mwis(&instance.adjacency, &instance.weights, component).0;
        }
    }
    dominant.sort_unstable();
    (dominant, small_weight)
}

fn connected_components(adjacency: &[BitWords], candidates: &[usize]) -> Vec<Vec<usize>> {
    let mut remaining = candidates.to_vec();
    remaining.sort_unstable();
    let mut components = Vec::new();
    while let Some(start) = remaining.pop() {
        let mut stack = vec![start];
        let mut component = Vec::new();
        while let Some(vertex) = stack.pop() {
            component.push(vertex);
            let mut index = 0;
            while index < remaining.len() {
                if has_bit(&adjacency[vertex], remaining[index]) {
                    stack.push(remaining.swap_remove(index));
                } else {
                    index += 1;
                }
            }
        }
        component.sort_unstable();
        components.push(component);
    }
    components.sort_by(|left, right| right.len().cmp(&left.len()).then_with(|| left.cmp(right)));
    components
}

pub(super) fn degree(adjacency: &[BitWords], vertex: usize, candidates: &[usize]) -> usize {
    candidates
        .iter()
        .filter(|candidate| **candidate != vertex && has_bit(&adjacency[vertex], **candidate))
        .count()
}
