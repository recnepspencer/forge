use std::cmp::Ordering;
use std::collections::HashSet;

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{
    greedy_independent_witness, has_bit, BitWords,
};
use super::g27_same_field_lp_relaxation::stable_set_lp_guidance_values;
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_instance;

const TARGET_WEIGHT: i128 = 512_933;
const DESTROY_SIZES: [usize; 6] = [4, 8, 12, 16, 24, 32];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WitnessRepairStatus {
    FoundThresholdWitness,
    NotFoundWithinBudget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WitnessRepairReport {
    status: G27WitnessRepairStatus,
    compatible_w_vertex_count: usize,
    initial_weight: i128,
    best_weight: i128,
    target_weight: i128,
    attempt_count: usize,
    improvement_count: usize,
    witness_vertices: Vec<usize>,
}

impl G27WitnessRepairReport {
    pub fn status(&self) -> G27WitnessRepairStatus {
        self.status
    }

    pub fn search_summary(&self) -> (usize, i128, i128, i128, usize, usize, usize) {
        (
            self.compatible_w_vertex_count,
            self.initial_weight,
            self.best_weight,
            self.target_weight,
            self.attempt_count,
            self.improvement_count,
            self.witness_vertices.len(),
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

pub fn search_g27_same_field_witness_repair_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WitnessRepairReport, G27GeometricFractionalError> {
    let instance = threshold_mwis_instance(handle)?;
    let lp_values = stable_set_lp_guidance_values(
        &instance.adjacency,
        &instance.weights,
        &instance.candidates,
    )?;
    let (initial_weight, initial_witness) =
        greedy_independent_witness(&instance.adjacency, &instance.weights, &instance.candidates);
    let mut best = CandidateWitness {
        weight: initial_weight,
        vertices: initial_witness,
    };
    let mut attempt_count = 0;
    let mut improvement_count = 0;
    for destroy in DestroyPolicy::all() {
        for destroy_size in DESTROY_SIZES {
            for refill in RefillPolicy::all() {
                attempt_count += 1;
                let candidate = repair_candidate(
                    &instance.adjacency,
                    &instance.weights,
                    &instance.candidates,
                    &lp_values,
                    &best.vertices,
                    destroy,
                    destroy_size,
                    refill,
                );
                if candidate.weight > best.weight {
                    best = candidate;
                    improvement_count += 1;
                    if best.weight >= TARGET_WEIGHT {
                        break;
                    }
                }
            }
        }
    }
    let status = if best.weight >= TARGET_WEIGHT {
        G27WitnessRepairStatus::FoundThresholdWitness
    } else {
        G27WitnessRepairStatus::NotFoundWithinBudget
    };
    let mut witness_vertices = best.vertices;
    witness_vertices.sort_unstable();
    Ok(G27WitnessRepairReport {
        status,
        compatible_w_vertex_count: instance.candidates.len(),
        initial_weight,
        best_weight: best.weight,
        target_weight: TARGET_WEIGHT,
        attempt_count,
        improvement_count,
        witness_vertices: witness_vertices
            .into_iter()
            .map(|vertex| vertex + 1)
            .collect(),
    })
}

#[derive(Clone)]
struct CandidateWitness {
    weight: i128,
    vertices: Vec<usize>,
}

#[derive(Clone, Copy)]
enum DestroyPolicy {
    LowWeight,
    LowLp,
    LowWeightLp,
    HighConflictWeight,
    HighDegree,
}

impl DestroyPolicy {
    fn all() -> [Self; 5] {
        [
            Self::LowWeight,
            Self::LowLp,
            Self::LowWeightLp,
            Self::HighConflictWeight,
            Self::HighDegree,
        ]
    }
}

#[derive(Clone, Copy)]
enum RefillPolicy {
    Weight,
    WeightLp,
    WeightPerConflict,
    WeightLpPerConflict,
}

impl RefillPolicy {
    fn all() -> [Self; 4] {
        [
            Self::Weight,
            Self::WeightLp,
            Self::WeightPerConflict,
            Self::WeightLpPerConflict,
        ]
    }
}

fn repair_candidate(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    lp_values: &[f64],
    incumbent: &[usize],
    destroy: DestroyPolicy,
    destroy_size: usize,
    refill: RefillPolicy,
) -> CandidateWitness {
    let local_index = local_index(candidates);
    let removed = destroy_vertices(
        adjacency,
        weights,
        candidates,
        lp_values,
        incumbent,
        &local_index,
        destroy,
        destroy_size,
    );
    let removed_set = removed.into_iter().collect::<HashSet<_>>();
    let mut vertices = incumbent
        .iter()
        .copied()
        .filter(|vertex| !removed_set.contains(vertex))
        .collect::<Vec<_>>();
    let mut refill_order = candidates
        .iter()
        .copied()
        .filter(|vertex| !vertices.contains(vertex))
        .collect::<Vec<_>>();
    sort_refill(
        adjacency,
        weights,
        lp_values,
        &local_index,
        &vertices,
        &mut refill_order,
        refill,
    );
    for vertex in refill_order {
        if vertices
            .iter()
            .all(|chosen| !has_bit(&adjacency[vertex], *chosen))
        {
            vertices.push(vertex);
        }
    }
    vertices.sort_unstable();
    CandidateWitness {
        weight: vertices.iter().map(|vertex| weights[*vertex]).sum(),
        vertices,
    }
}

fn destroy_vertices(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    lp_values: &[f64],
    incumbent: &[usize],
    local_index: &[Option<usize>],
    policy: DestroyPolicy,
    destroy_size: usize,
) -> Vec<usize> {
    let mut vertices = incumbent.to_vec();
    vertices.sort_by(|left, right| {
        compare_destroy(
            adjacency,
            weights,
            candidates,
            lp_values,
            local_index,
            policy,
            *left,
            *right,
        )
    });
    vertices.truncate(destroy_size.min(vertices.len()));
    vertices
}

fn compare_destroy(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    lp_values: &[f64],
    local_index: &[Option<usize>],
    policy: DestroyPolicy,
    left: usize,
    right: usize,
) -> Ordering {
    match policy {
        DestroyPolicy::LowWeight => weights[left].cmp(&weights[right]),
        DestroyPolicy::LowLp => float_cmp(
            lp_value(local_index, lp_values, left),
            lp_value(local_index, lp_values, right),
        ),
        DestroyPolicy::LowWeightLp => float_cmp(
            weights[left] as f64 * lp_value(local_index, lp_values, left),
            weights[right] as f64 * lp_value(local_index, lp_values, right),
        ),
        DestroyPolicy::HighConflictWeight => conflict_weight(adjacency, weights, candidates, right)
            .cmp(&conflict_weight(adjacency, weights, candidates, left)),
        DestroyPolicy::HighDegree => {
            degree(adjacency, right, candidates).cmp(&degree(adjacency, left, candidates))
        }
    }
    .then_with(|| left.cmp(&right))
}

fn sort_refill(
    adjacency: &[BitWords],
    weights: &[i128],
    lp_values: &[f64],
    local_index: &[Option<usize>],
    remaining: &[usize],
    refill_order: &mut [usize],
    policy: RefillPolicy,
) {
    refill_order.sort_by(|left, right| {
        refill_score(
            adjacency,
            weights,
            lp_values,
            local_index,
            remaining,
            *right,
            policy,
        )
        .partial_cmp(&refill_score(
            adjacency,
            weights,
            lp_values,
            local_index,
            remaining,
            *left,
            policy,
        ))
        .unwrap_or(Ordering::Equal)
        .then_with(|| left.cmp(right))
    });
}

fn refill_score(
    adjacency: &[BitWords],
    weights: &[i128],
    lp_values: &[f64],
    local_index: &[Option<usize>],
    remaining: &[usize],
    vertex: usize,
    policy: RefillPolicy,
) -> f64 {
    let lp = lp_value(local_index, lp_values, vertex).max(0.0);
    let conflicts = remaining
        .iter()
        .filter(|chosen| has_bit(&adjacency[vertex], **chosen))
        .count() as f64;
    match policy {
        RefillPolicy::Weight => weights[vertex] as f64,
        RefillPolicy::WeightLp => weights[vertex] as f64 * lp,
        RefillPolicy::WeightPerConflict => weights[vertex] as f64 / (1.0 + conflicts),
        RefillPolicy::WeightLpPerConflict => weights[vertex] as f64 * lp / (1.0 + conflicts),
    }
}

fn local_index(candidates: &[usize]) -> Vec<Option<usize>> {
    let mut index = vec![None; 607];
    for (local, vertex) in candidates.iter().enumerate() {
        index[*vertex] = Some(local);
    }
    index
}

fn lp_value(local_index: &[Option<usize>], lp_values: &[f64], vertex: usize) -> f64 {
    local_index[vertex]
        .map(|index| lp_values[index])
        .unwrap_or(0.0)
}

fn conflict_weight(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    vertex: usize,
) -> i128 {
    candidates
        .iter()
        .filter(|candidate| **candidate != vertex && has_bit(&adjacency[vertex], **candidate))
        .map(|candidate| weights[*candidate])
        .sum()
}

fn degree(adjacency: &[BitWords], vertex: usize, candidates: &[usize]) -> usize {
    candidates
        .iter()
        .filter(|candidate| **candidate != vertex && has_bit(&adjacency[vertex], **candidate))
        .count()
}

fn float_cmp(left: f64, right: f64) -> Ordering {
    left.partial_cmp(&right).unwrap_or(Ordering::Equal)
}
