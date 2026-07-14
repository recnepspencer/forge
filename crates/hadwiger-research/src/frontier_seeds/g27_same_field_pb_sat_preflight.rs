use std::collections::BTreeSet;

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_mwis_exact::exact_mwis;
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_instance;

const TARGET_WEIGHT: i128 = 512_933;
const MAX_NODE_SUM_COUNT: usize = 50_000;
const MAX_AUXILIARY_VARIABLES: usize = 1_000_000;
const MAX_CLAUSES: usize = 5_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27PbSatPreflightStatus {
    EncodingWithinBudget,
    EncodingTooLarge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27PbSatPreflightReport {
    status: G27PbSatPreflightStatus,
    compatible_w_vertex_count: usize,
    component_count: usize,
    dominant_component_size: usize,
    exact_small_component_weight: i128,
    dominant_required_weight: i128,
    edge_clause_count: usize,
    totalizer_auxiliary_variable_estimate: usize,
    totalizer_clause_estimate: usize,
    max_totalizer_node_sum_count: usize,
    capped_node_count: usize,
}

impl G27PbSatPreflightReport {
    pub fn status(&self) -> G27PbSatPreflightStatus {
        self.status
    }

    pub fn instance_summary(&self) -> (usize, usize, usize, i128, i128, usize) {
        (
            self.compatible_w_vertex_count,
            self.component_count,
            self.dominant_component_size,
            self.exact_small_component_weight,
            self.dominant_required_weight,
            self.edge_clause_count,
        )
    }

    pub fn encoding_summary(&self) -> (usize, usize, usize, usize) {
        (
            self.totalizer_auxiliary_variable_estimate,
            self.totalizer_clause_estimate,
            self.max_totalizer_node_sum_count,
            self.capped_node_count,
        )
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

pub fn preflight_g27_same_field_pb_sat_threshold_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27PbSatPreflightReport, G27GeometricFractionalError> {
    let instance = threshold_mwis_instance(handle)?;
    let components = connected_components(&instance.adjacency, &instance.candidates);
    let small_weight =
        exact_small_component_weight(&instance.adjacency, &instance.weights, &components);
    let dominant = components[0].clone();
    let dominant_target = TARGET_WEIGHT - small_weight;
    let dominant_weights = dominant
        .iter()
        .map(|vertex| instance.weights[*vertex])
        .collect::<Vec<_>>();
    let totalizer = totalizer_preflight(&dominant_weights, dominant_target);
    let edge_clause_count = induced_edge_count(&instance.adjacency, &dominant);
    let status = if totalizer.capped_node_count == 0
        && totalizer.auxiliary_variables <= MAX_AUXILIARY_VARIABLES
        && totalizer.clauses + edge_clause_count <= MAX_CLAUSES
    {
        G27PbSatPreflightStatus::EncodingWithinBudget
    } else {
        G27PbSatPreflightStatus::EncodingTooLarge
    };
    Ok(G27PbSatPreflightReport {
        status,
        compatible_w_vertex_count: instance.candidates.len(),
        component_count: components.len(),
        dominant_component_size: dominant.len(),
        exact_small_component_weight: small_weight,
        dominant_required_weight: dominant_target,
        edge_clause_count,
        totalizer_auxiliary_variable_estimate: totalizer.auxiliary_variables,
        totalizer_clause_estimate: totalizer.clauses,
        max_totalizer_node_sum_count: totalizer.max_node_sum_count,
        capped_node_count: totalizer.capped_node_count,
    })
}

struct TotalizerPreflight {
    auxiliary_variables: usize,
    clauses: usize,
    max_node_sum_count: usize,
    capped_node_count: usize,
}

fn totalizer_preflight(weights: &[i128], target: i128) -> TotalizerPreflight {
    let nodes = weights
        .iter()
        .map(|weight| vec![0, (*weight).min(target)])
        .collect::<Vec<_>>();
    let mut state = TotalizerPreflight {
        auxiliary_variables: 0,
        clauses: 0,
        max_node_sum_count: 0,
        capped_node_count: 0,
    };
    let _ = merge_level(nodes, target, &mut state);
    state
}

fn merge_level(
    mut nodes: Vec<Vec<i128>>,
    target: i128,
    state: &mut TotalizerPreflight,
) -> Vec<i128> {
    while nodes.len() > 1 {
        let mut next = Vec::new();
        let mut pairs = nodes.chunks_exact(2);
        for pair in &mut pairs {
            next.push(merge_sums(&pair[0], &pair[1], target, state));
        }
        if let Some(remainder) = pairs.remainder().first() {
            next.push(remainder.clone());
        }
        nodes = next;
    }
    nodes.pop().unwrap_or_else(|| vec![0])
}

fn merge_sums(
    left: &[i128],
    right: &[i128],
    target: i128,
    state: &mut TotalizerPreflight,
) -> Vec<i128> {
    let pair_count = left.len().saturating_mul(right.len());
    state.clauses = state
        .clauses
        .saturating_add(pair_count)
        .saturating_add(left.len())
        .saturating_add(right.len());
    if pair_count > MAX_NODE_SUM_COUNT {
        state.capped_node_count += 1;
        state.max_node_sum_count = state.max_node_sum_count.max(MAX_NODE_SUM_COUNT);
        state.auxiliary_variables += MAX_NODE_SUM_COUNT - 1;
        state.clauses = state.clauses.saturating_add(MAX_NODE_SUM_COUNT);
        return (0..MAX_NODE_SUM_COUNT as i128).collect();
    }
    let mut sums = BTreeSet::new();
    for left_sum in left {
        for right_sum in right {
            sums.insert((left_sum + right_sum).min(target));
            if sums.len() > MAX_NODE_SUM_COUNT {
                state.capped_node_count += 1;
                let capped = sums
                    .into_iter()
                    .take(MAX_NODE_SUM_COUNT)
                    .collect::<Vec<_>>();
                state.max_node_sum_count = state.max_node_sum_count.max(capped.len());
                state.auxiliary_variables += capped.len().saturating_sub(1);
                state.clauses = state.clauses.saturating_add(capped.len());
                return capped;
            }
        }
    }
    let sums = sums.into_iter().collect::<Vec<_>>();
    state.max_node_sum_count = state.max_node_sum_count.max(sums.len());
    state.auxiliary_variables += sums.len().saturating_sub(1);
    state.clauses = state.clauses.saturating_add(sums.len());
    sums
}

fn exact_small_component_weight(
    adjacency: &[BitWords],
    weights: &[i128],
    components: &[Vec<usize>],
) -> i128 {
    components
        .iter()
        .skip(1)
        .map(|component| exact_mwis(adjacency, weights, component).0)
        .sum()
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

fn induced_edge_count(adjacency: &[BitWords], candidates: &[usize]) -> usize {
    let mut count = 0;
    for left in 0..candidates.len() {
        for right in (left + 1)..candidates.len() {
            if has_bit(&adjacency[candidates[left]], candidates[right]) {
                count += 1;
            }
        }
    }
    count
}
