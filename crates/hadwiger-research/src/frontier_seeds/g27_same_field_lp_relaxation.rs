use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_odd_cycle_lp::{odd_cycle_cut_relaxation, OddCycleAcceptedCut};

const MAXIMAL_CLIQUE_CAP: usize = 50_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct StableSetLpRelaxationBound {
    pub(super) objective_ceiling: i128,
    pub(super) clique_objective_ceiling: i128,
    pub(super) variable_count: usize,
    pub(super) edge_constraint_count: usize,
    pub(super) clique_constraint_count: usize,
    pub(super) odd_cycle_objective_ceiling: i128,
    pub(super) odd_cycle_cut_count: usize,
    pub(super) odd_cycle_round_count: usize,
    pub(super) best_odd_cycle_violation_ppm: i128,
    pub(super) maximal_clique_count: usize,
    pub(super) maximal_clique_cap_hit: bool,
    pub(super) largest_clique_size: usize,
}

pub(super) struct StableSetLpRelaxationRows {
    pub(super) objective_ceiling: i128,
    pub(super) clique_objective_ceiling: i128,
    pub(super) odd_cycle_objective_ceiling: i128,
    pub(super) clique_constraints: Vec<Vec<usize>>,
    pub(super) odd_cycle_cuts: Vec<OddCycleAcceptedCut>,
    pub(super) odd_cycle_round_count: usize,
    pub(super) maximal_clique_cap_hit: bool,
}

pub(super) fn stable_set_lp_relaxation_bound(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
) -> Result<StableSetLpRelaxationBound, G27GeometricFractionalError> {
    let edge_constraint_count = induced_edge_count(adjacency, candidates);
    let clique_enumeration = enumerate_maximal_cliques(adjacency, candidates);
    let clique_constraints = clique_enumeration
        .cliques
        .iter()
        .filter(|clique| clique.len() > 2)
        .cloned()
        .collect::<Vec<_>>();
    let objective_ceiling = solve_lp(adjacency, weights, candidates, &[], &[])?.objective_ceiling;
    let clique_solution = solve_lp(adjacency, weights, candidates, &clique_constraints, &[])?;
    let odd_cycle_relaxation =
        odd_cycle_cut_relaxation(adjacency, weights, candidates, &clique_constraints)?;
    Ok(StableSetLpRelaxationBound {
        objective_ceiling,
        clique_objective_ceiling: clique_solution.objective_ceiling,
        variable_count: candidates.len(),
        edge_constraint_count,
        clique_constraint_count: clique_constraints.len(),
        odd_cycle_objective_ceiling: odd_cycle_relaxation.objective_ceiling,
        odd_cycle_cut_count: odd_cycle_relaxation.cut_count,
        odd_cycle_round_count: odd_cycle_relaxation.round_count,
        best_odd_cycle_violation_ppm: odd_cycle_relaxation.best_violation_ppm,
        maximal_clique_count: clique_enumeration.cliques.len(),
        maximal_clique_cap_hit: clique_enumeration.cap_hit,
        largest_clique_size: clique_enumeration.largest_clique_size,
    })
}

pub(super) fn stable_set_lp_relaxation_rows(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
) -> Result<StableSetLpRelaxationRows, G27GeometricFractionalError> {
    let clique_enumeration = enumerate_maximal_cliques(adjacency, candidates);
    let clique_constraints = clique_enumeration
        .cliques
        .iter()
        .filter(|clique| clique.len() > 2)
        .cloned()
        .collect::<Vec<_>>();
    let objective_ceiling = solve_lp(adjacency, weights, candidates, &[], &[])?.objective_ceiling;
    let clique_solution = solve_lp(adjacency, weights, candidates, &clique_constraints, &[])?;
    let odd_cycle_relaxation =
        odd_cycle_cut_relaxation(adjacency, weights, candidates, &clique_constraints)?;
    Ok(StableSetLpRelaxationRows {
        objective_ceiling,
        clique_objective_ceiling: clique_solution.objective_ceiling,
        odd_cycle_objective_ceiling: odd_cycle_relaxation.objective_ceiling,
        clique_constraints,
        odd_cycle_cuts: odd_cycle_relaxation.cuts,
        odd_cycle_round_count: odd_cycle_relaxation.round_count,
        maximal_clique_cap_hit: clique_enumeration.cap_hit,
    })
}

pub(super) fn stable_set_lp_guidance_values(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
) -> Result<Vec<f64>, G27GeometricFractionalError> {
    let clique_constraints = maximal_clique_constraints(adjacency, candidates);
    Ok(odd_cycle_cut_relaxation(adjacency, weights, candidates, &clique_constraints)?.values)
}

fn maximal_clique_constraints(adjacency: &[BitWords], candidates: &[usize]) -> Vec<Vec<usize>> {
    enumerate_maximal_cliques(adjacency, candidates)
        .cliques
        .into_iter()
        .filter(|clique| clique.len() > 2)
        .collect()
}

pub(super) struct LpSolution {
    pub(super) objective_ceiling: i128,
    pub(super) values: Vec<f64>,
}

pub(super) fn solve_lp(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    clique_constraints: &[Vec<usize>],
    odd_cycle_constraints: &[Vec<usize>],
) -> Result<LpSolution, G27GeometricFractionalError> {
    let mut variables = variables!();
    let xs = candidates
        .iter()
        .map(|_| variables.add(variable().min(0.0).max(1.0)))
        .collect::<Vec<_>>();
    let objective = candidates
        .iter()
        .zip(xs.iter())
        .fold(Expression::from(0.0), |sum, (vertex, x)| {
            sum + weights[*vertex] as f64 * *x
        });
    let mut problem = variables.maximise(objective.clone()).using(default_solver);
    for left in 0..candidates.len() {
        for right in (left + 1)..candidates.len() {
            if has_bit(&adjacency[candidates[left]], candidates[right]) {
                problem = problem.with(constraint!(xs[left] + xs[right] <= 1.0));
            }
        }
    }
    for clique in clique_constraints {
        let expression = clique
            .iter()
            .fold(Expression::from(0.0), |sum, local_index| {
                sum + xs[*local_index]
            });
        problem = problem.with(constraint!(expression <= 1.0));
    }
    for cycle in odd_cycle_constraints {
        let expression = cycle
            .iter()
            .fold(Expression::from(0.0), |sum, local_index| {
                sum + xs[*local_index]
            });
        problem = problem.with(constraint!(expression <= (cycle.len() / 2) as f64));
    }
    let solution = problem
        .solve()
        .map_err(|error| G27GeometricFractionalError::MatrixZip(error.to_string()))?;
    let objective_value = solution.eval(&objective);
    if !objective_value.is_finite() {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "stable_set_lp_objective",
        });
    }
    let values = xs
        .iter()
        .map(|variable| solution.value(*variable))
        .collect::<Vec<_>>();
    Ok(LpSolution {
        objective_ceiling: objective_value.ceil() as i128,
        values,
    })
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

struct CliqueEnumeration {
    cliques: Vec<Vec<usize>>,
    cap_hit: bool,
    largest_clique_size: usize,
}

fn enumerate_maximal_cliques(adjacency: &[BitWords], candidates: &[usize]) -> CliqueEnumeration {
    let mut state = CliqueEnumeration {
        cliques: Vec::new(),
        cap_hit: false,
        largest_clique_size: 0,
    };
    let p = (0..candidates.len()).collect::<Vec<_>>();
    bron_kerbosch(adjacency, candidates, Vec::new(), p, Vec::new(), &mut state);
    state.cliques.sort();
    state
}

fn bron_kerbosch(
    adjacency: &[BitWords],
    candidates: &[usize],
    r: Vec<usize>,
    mut p: Vec<usize>,
    mut x: Vec<usize>,
    state: &mut CliqueEnumeration,
) {
    if state.cap_hit {
        return;
    }
    if p.is_empty() && x.is_empty() {
        state.largest_clique_size = state.largest_clique_size.max(r.len());
        state.cliques.push(r);
        if state.cliques.len() >= MAXIMAL_CLIQUE_CAP {
            state.cap_hit = true;
        }
        return;
    }
    let pivot = choose_pivot(adjacency, candidates, &p, &x);
    let mut branch = p
        .iter()
        .copied()
        .filter(|vertex| {
            pivot.is_none_or(|pivot| !has_bit(&adjacency[candidates[pivot]], candidates[*vertex]))
        })
        .collect::<Vec<_>>();
    branch.sort_unstable();
    for vertex in branch {
        if state.cap_hit {
            return;
        }
        let mut next_r = r.clone();
        next_r.push(vertex);
        next_r.sort_unstable();
        bron_kerbosch(
            adjacency,
            candidates,
            next_r,
            intersect_neighbors(adjacency, candidates, &p, vertex),
            intersect_neighbors(adjacency, candidates, &x, vertex),
            state,
        );
        p.retain(|candidate| *candidate != vertex);
        x.push(vertex);
        x.sort_unstable();
    }
}

fn choose_pivot(
    adjacency: &[BitWords],
    candidates: &[usize],
    p: &[usize],
    x: &[usize],
) -> Option<usize> {
    p.iter().chain(x.iter()).copied().max_by_key(|pivot| {
        p.iter()
            .filter(|candidate| has_bit(&adjacency[candidates[*pivot]], candidates[**candidate]))
            .count()
    })
}

fn intersect_neighbors(
    adjacency: &[BitWords],
    candidates: &[usize],
    vertices: &[usize],
    vertex: usize,
) -> Vec<usize> {
    vertices
        .iter()
        .copied()
        .filter(|candidate| has_bit(&adjacency[candidates[vertex]], candidates[*candidate]))
        .collect()
}
