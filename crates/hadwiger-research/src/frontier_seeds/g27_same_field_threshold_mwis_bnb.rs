use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{
    clique_cover_weight_upper_bound, greedy_independent_witness, has_bit, BitWords,
};
use super::g27_same_field_mwis_exact::exact_mwis;
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_instance;

const TARGET_WEIGHT: i128 = 512_933;
const NODE_CAP: usize = 20;
const EXACT_RESIDUAL_LIMIT: usize = 36;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27ThresholdMwisBnbStatus {
    FoundWitness,
    ProvedBelowThreshold,
    UndecidedNodeCap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27ThresholdMwisBnbReport {
    status: G27ThresholdMwisBnbStatus,
    compatible_w_vertex_count: usize,
    component_count: usize,
    dominant_component_size: usize,
    exact_small_component_count: usize,
    exact_small_component_weight: i128,
    dominant_required_weight: i128,
    initial_total_lower_bound: i128,
    best_total_weight: i128,
    best_open_total_upper_bound: i128,
    node_count: usize,
    pruned_by_threshold_bound: usize,
    pruned_by_incumbent_bound: usize,
    solved_exact_residual_count: usize,
    max_depth: usize,
    witness_vertices: Vec<usize>,
}

impl G27ThresholdMwisBnbReport {
    pub fn status(&self) -> G27ThresholdMwisBnbStatus {
        self.status
    }

    pub fn component_summary(&self) -> (usize, usize, usize, i128, i128) {
        (
            self.compatible_w_vertex_count,
            self.component_count,
            self.dominant_component_size,
            self.exact_small_component_weight,
            self.dominant_required_weight,
        )
    }

    pub fn search_summary(&self) -> (i128, i128, i128, usize, usize, usize, usize, usize) {
        (
            self.initial_total_lower_bound,
            self.best_total_weight,
            self.best_open_total_upper_bound,
            self.node_count,
            self.pruned_by_threshold_bound,
            self.pruned_by_incumbent_bound,
            self.solved_exact_residual_count,
            self.max_depth,
        )
    }

    pub fn witness_vertices(&self) -> &[usize] {
        &self.witness_vertices
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

pub fn run_g27_same_field_threshold_mwis_bnb_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27ThresholdMwisBnbReport, G27GeometricFractionalError> {
    let instance = threshold_mwis_instance(handle)?;
    let adjacency = instance.adjacency;
    let weights = instance.weights;
    let candidates = instance.candidates;
    let components = connected_components(&adjacency, &candidates);
    let (small_weight, small_witness, dominant) =
        split_exact_small_components(&adjacency, &weights, &components);
    let required = TARGET_WEIGHT - small_weight;
    let (large_lower, large_witness) = greedy_independent_witness(&adjacency, &weights, &dominant);
    let initial_total = small_weight + large_lower;
    let search = threshold_bnb(
        &adjacency,
        &weights,
        &dominant,
        required,
        large_lower,
        large_witness,
    );
    let mut witness_vertices = small_witness;
    witness_vertices.extend(search.best_vertices);
    witness_vertices.sort_unstable();
    let best_total = small_weight + search.best_weight;
    let status = if best_total >= TARGET_WEIGHT {
        G27ThresholdMwisBnbStatus::FoundWitness
    } else if !search.cap_hit && small_weight + search.best_open_upper_bound < TARGET_WEIGHT {
        G27ThresholdMwisBnbStatus::ProvedBelowThreshold
    } else {
        G27ThresholdMwisBnbStatus::UndecidedNodeCap
    };
    Ok(G27ThresholdMwisBnbReport {
        status,
        compatible_w_vertex_count: candidates.len(),
        component_count: components.len(),
        dominant_component_size: dominant.len(),
        exact_small_component_count: components.len() - 1,
        exact_small_component_weight: small_weight,
        dominant_required_weight: required,
        initial_total_lower_bound: initial_total,
        best_total_weight: best_total,
        best_open_total_upper_bound: small_weight + search.best_open_upper_bound,
        node_count: search.node_count,
        pruned_by_threshold_bound: search.pruned_by_threshold_bound,
        pruned_by_incumbent_bound: search.pruned_by_incumbent_bound,
        solved_exact_residual_count: search.solved_exact_residual_count,
        max_depth: search.max_depth,
        witness_vertices: witness_vertices
            .into_iter()
            .map(|vertex| vertex + 1)
            .collect(),
    })
}

fn split_exact_small_components(
    adjacency: &[BitWords],
    weights: &[i128],
    components: &[Vec<usize>],
) -> (i128, Vec<usize>, Vec<usize>) {
    let mut small_weight = 0;
    let mut small_witness = Vec::new();
    for component in components.iter().skip(1) {
        let (weight, witness) = exact_mwis(adjacency, weights, component);
        small_weight += weight;
        small_witness.extend(witness);
    }
    (small_weight, small_witness, components[0].clone())
}

#[derive(Clone)]
struct SearchNode {
    candidates: Vec<usize>,
    chosen_weight: i128,
    chosen_vertices: Vec<usize>,
    depth: usize,
}

struct SearchResult {
    best_weight: i128,
    best_vertices: Vec<usize>,
    best_open_upper_bound: i128,
    node_count: usize,
    pruned_by_threshold_bound: usize,
    pruned_by_incumbent_bound: usize,
    solved_exact_residual_count: usize,
    max_depth: usize,
    cap_hit: bool,
}

fn threshold_bnb(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    threshold: i128,
    initial_weight: i128,
    initial_vertices: Vec<usize>,
) -> SearchResult {
    let mut result = SearchResult {
        best_weight: initial_weight,
        best_vertices: initial_vertices,
        best_open_upper_bound: initial_weight,
        node_count: 0,
        pruned_by_threshold_bound: 0,
        pruned_by_incumbent_bound: 0,
        solved_exact_residual_count: 0,
        max_depth: 0,
        cap_hit: false,
    };
    let mut stack = vec![SearchNode {
        candidates: candidates.to_vec(),
        chosen_weight: 0,
        chosen_vertices: Vec::new(),
        depth: 0,
    }];
    while let Some(mut node) = stack.pop() {
        if result.node_count >= NODE_CAP {
            result.cap_hit = true;
            stack.push(node);
            break;
        }
        result.node_count += 1;
        include_isolated_vertices(adjacency, weights, &mut node);
        result.max_depth = result.max_depth.max(node.depth);
        if node.candidates.len() <= EXACT_RESIDUAL_LIMIT {
            solve_small_residual(adjacency, weights, node, &mut result);
            continue;
        }
        let upper = node.chosen_weight
            + clique_cover_weight_upper_bound(adjacency, weights, &node.candidates);
        if upper < threshold {
            result.pruned_by_threshold_bound += 1;
            continue;
        }
        if upper <= result.best_weight {
            result.pruned_by_incumbent_bound += 1;
            continue;
        }
        let branch = choose_branch_vertex(adjacency, weights, &node.candidates);
        let excluded = remove_vertex(&node.candidates, branch);
        let mut included = excluded
            .iter()
            .copied()
            .filter(|vertex| !has_bit(&adjacency[branch], *vertex))
            .collect::<Vec<_>>();
        included.sort_unstable();
        let mut included_vertices = node.chosen_vertices.clone();
        included_vertices.push(branch);
        stack.push(SearchNode {
            candidates: excluded,
            chosen_weight: node.chosen_weight,
            chosen_vertices: node.chosen_vertices,
            depth: node.depth + 1,
        });
        stack.push(SearchNode {
            candidates: included,
            chosen_weight: node.chosen_weight + weights[branch],
            chosen_vertices: included_vertices,
            depth: node.depth + 1,
        });
        if result.best_weight >= threshold {
            break;
        }
    }
    for node in stack {
        let upper = node.chosen_weight
            + clique_cover_weight_upper_bound(adjacency, weights, &node.candidates);
        result.best_open_upper_bound = result.best_open_upper_bound.max(upper);
    }
    result
}

fn solve_small_residual(
    adjacency: &[BitWords],
    weights: &[i128],
    node: SearchNode,
    result: &mut SearchResult,
) {
    let (extra_weight, extra_vertices) = exact_mwis(adjacency, weights, &node.candidates);
    result.solved_exact_residual_count += 1;
    let total = node.chosen_weight + extra_weight;
    if total > result.best_weight {
        let mut vertices = node.chosen_vertices;
        vertices.extend(extra_vertices);
        vertices.sort_unstable();
        result.best_weight = total;
        result.best_vertices = vertices;
    }
}

fn include_isolated_vertices(adjacency: &[BitWords], weights: &[i128], node: &mut SearchNode) {
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
        node.chosen_vertices.push(vertex);
        node.candidates.retain(|candidate| *candidate != vertex);
    }
}

fn choose_branch_vertex(adjacency: &[BitWords], weights: &[i128], candidates: &[usize]) -> usize {
    candidates
        .iter()
        .copied()
        .max_by_key(|vertex| {
            (
                degree(adjacency, *vertex, candidates) as i128 * weights[*vertex],
                *vertex,
            )
        })
        .expect("nonempty candidates")
}

fn remove_vertex(candidates: &[usize], vertex: usize) -> Vec<usize> {
    candidates
        .iter()
        .copied()
        .filter(|candidate| *candidate != vertex)
        .collect()
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

fn degree(adjacency: &[BitWords], vertex: usize, candidates: &[usize]) -> usize {
    candidates
        .iter()
        .filter(|candidate| **candidate != vertex && has_bit(&adjacency[vertex], **candidate))
        .count()
}
