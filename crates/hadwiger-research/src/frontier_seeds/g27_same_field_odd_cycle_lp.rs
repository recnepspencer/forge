use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_lp_relaxation::solve_lp;

const ROUND_LIMIT: usize = 8;
const BATCH_LIMIT: usize = 64;
const TOTAL_CUT_LIMIT: usize = 512;
const VIOLATION_EPSILON: f64 = 1.0e-6;

pub(super) struct OddCycleRelaxation {
    pub(super) objective_ceiling: i128,
    pub(super) values: Vec<f64>,
    pub(super) cut_count: usize,
    pub(super) round_count: usize,
    pub(super) best_violation_ppm: i128,
    pub(super) cuts: Vec<OddCycleAcceptedCut>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct OddCycleAcceptedCut {
    pub(super) support: Vec<usize>,
    pub(super) witness: Vec<usize>,
    pub(super) violation_ppm: i128,
}

pub(super) fn odd_cycle_cut_relaxation(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    clique_constraints: &[Vec<usize>],
) -> Result<OddCycleRelaxation, G27GeometricFractionalError> {
    let mut odd_cycle_constraints: Vec<OddCycleCut> = Vec::new();
    let mut seen = HashSet::new();
    let mut round_count = 0;
    let mut best_violation = 0.0f64;
    let mut solution = solve_lp(adjacency, weights, candidates, clique_constraints, &[])?;
    while round_count < ROUND_LIMIT && odd_cycle_constraints.len() < TOTAL_CUT_LIMIT {
        let mut cuts = violated_odd_cycles(adjacency, candidates, &solution.values, &seen);
        if cuts.is_empty() {
            break;
        }
        cuts.sort_by(|left, right| {
            right
                .violation
                .partial_cmp(&left.violation)
                .unwrap_or(Ordering::Equal)
                .then_with(|| left.key.cmp(&right.key))
        });
        let remaining = TOTAL_CUT_LIMIT - odd_cycle_constraints.len();
        for cut in cuts.into_iter().take(BATCH_LIMIT.min(remaining)) {
            best_violation = best_violation.max(cut.violation);
            seen.insert(cut.key.clone());
            odd_cycle_constraints.push(cut);
        }
        round_count += 1;
        let rows = odd_cycle_constraints
            .iter()
            .map(|cut| cut.support.clone())
            .collect::<Vec<_>>();
        solution = solve_lp(adjacency, weights, candidates, clique_constraints, &rows)?;
    }
    let cuts = odd_cycle_constraints
        .iter()
        .map(|cut| OddCycleAcceptedCut {
            support: cut.support.clone(),
            witness: cut.witness.clone(),
            violation_ppm: (cut.violation * 1_000_000.0).round() as i128,
        })
        .collect::<Vec<_>>();
    Ok(OddCycleRelaxation {
        objective_ceiling: solution.objective_ceiling,
        values: solution.values,
        cut_count: cuts.len(),
        round_count,
        best_violation_ppm: (best_violation * 1_000_000.0).round() as i128,
        cuts,
    })
}

struct OddCycleCut {
    support: Vec<usize>,
    witness: Vec<usize>,
    key: Vec<usize>,
    violation: f64,
}

fn violated_odd_cycles(
    adjacency: &[BitWords],
    candidates: &[usize],
    values: &[f64],
    seen: &HashSet<Vec<usize>>,
) -> Vec<OddCycleCut> {
    let edges = induced_edges(adjacency, candidates);
    let lengths = edges
        .iter()
        .map(|(left, right)| (1.0 - values[*left] - values[*right]).max(0.0))
        .collect::<Vec<_>>();
    let neighbors = weighted_local_adjacency(candidates.len(), &edges, &lengths);
    let mut cuts = Vec::new();
    let mut local_seen = HashSet::new();
    for (edge_index, (left, right)) in edges.iter().copied().enumerate() {
        let Some(path) = shortest_even_path(&neighbors, left, right, (left, right)) else {
            continue;
        };
        let total_length = path.cost + lengths[edge_index];
        if total_length >= 1.0 - VIOLATION_EPSILON {
            continue;
        }
        let witness = path.vertices;
        if witness.len() <= 3 || witness.len() % 2 == 0 || has_repeated_vertex(&witness) {
            continue;
        }
        let mut support = witness.clone();
        support.sort_unstable();
        let key = support.clone();
        if seen.contains(&key) || !local_seen.insert(key.clone()) {
            continue;
        }
        cuts.push(OddCycleCut {
            support,
            witness,
            key,
            violation: 1.0 - total_length,
        });
    }
    cuts
}

fn induced_edges(adjacency: &[BitWords], candidates: &[usize]) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for left in 0..candidates.len() {
        for right in (left + 1)..candidates.len() {
            if has_bit(&adjacency[candidates[left]], candidates[right]) {
                edges.push((left, right));
            }
        }
    }
    edges
}

fn weighted_local_adjacency(
    vertex_count: usize,
    edges: &[(usize, usize)],
    lengths: &[f64],
) -> Vec<Vec<(usize, f64)>> {
    let mut neighbors = vec![Vec::new(); vertex_count];
    for ((left, right), length) in edges.iter().zip(lengths.iter()) {
        neighbors[*left].push((*right, *length));
        neighbors[*right].push((*left, *length));
    }
    neighbors
}

struct OddPath {
    vertices: Vec<usize>,
    cost: f64,
}

#[derive(Clone, Copy, Debug)]
struct DijkstraState {
    cost: f64,
    vertex: usize,
    parity: usize,
}

impl Eq for DijkstraState {}

impl PartialEq for DijkstraState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.vertex == other.vertex && self.parity == other.parity
    }
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| other.vertex.cmp(&self.vertex))
            .then_with(|| other.parity.cmp(&self.parity))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_even_path(
    neighbors: &[Vec<(usize, f64)>],
    start: usize,
    target: usize,
    excluded_edge: (usize, usize),
) -> Option<OddPath> {
    let mut dist = vec![[f64::INFINITY; 2]; neighbors.len()];
    let mut previous = vec![[None; 2]; neighbors.len()];
    let mut heap = BinaryHeap::new();
    dist[start][0] = 0.0;
    heap.push(DijkstraState {
        cost: 0.0,
        vertex: start,
        parity: 0,
    });
    while let Some(state) = heap.pop() {
        if state.cost > dist[state.vertex][state.parity] {
            continue;
        }
        if state.vertex == target && state.parity == 0 {
            return Some(OddPath {
                vertices: reconstruct_path(&previous, start, target),
                cost: state.cost,
            });
        }
        for (next, length) in &neighbors[state.vertex] {
            if same_edge((state.vertex, *next), excluded_edge) {
                continue;
            }
            let next_parity = 1 - state.parity;
            let next_cost = state.cost + *length;
            if next_cost < dist[*next][next_parity] {
                dist[*next][next_parity] = next_cost;
                previous[*next][next_parity] = Some((state.vertex, state.parity));
                heap.push(DijkstraState {
                    cost: next_cost,
                    vertex: *next,
                    parity: next_parity,
                });
            }
        }
    }
    None
}

fn reconstruct_path(
    previous: &[[Option<(usize, usize)>; 2]],
    start: usize,
    target: usize,
) -> Vec<usize> {
    let mut path = Vec::new();
    let mut current = (target, 0);
    path.push(current.0);
    while current.0 != start || current.1 != 0 {
        let Some(prior) = previous[current.0][current.1] else {
            break;
        };
        current = prior;
        path.push(current.0);
    }
    path.reverse();
    path
}

fn same_edge(left: (usize, usize), right: (usize, usize)) -> bool {
    (left.0 == right.0 && left.1 == right.1) || (left.0 == right.1 && left.1 == right.0)
}

fn has_repeated_vertex(vertices: &[usize]) -> bool {
    let mut seen = HashSet::new();
    vertices.iter().any(|vertex| !seen.insert(*vertex))
}
