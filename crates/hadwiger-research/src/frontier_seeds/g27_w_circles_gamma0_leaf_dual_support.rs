use std::collections::BTreeSet;

use serde::Deserialize;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{empty_words, has_bit, set_bit, BitWords};
use super::g27_w_circles_exact_geometry_support::EXPECTED_VERTEX_COUNT;
use super::g27_w_circles_gamma0_rank_support::RankCut;

pub(crate) const BRANCH_VERTEX: usize = 303;
pub(crate) const DENOMINATOR: i128 = 1024;
pub(crate) const TARGET_GAMMA0: i128 = 613_372_392;
pub(crate) const EXPECTED_LEAVES: usize = 18;
pub(crate) const EXPECTED_POSITIVE_ROWS: usize = 10_135;

#[derive(Deserialize)]
pub(crate) struct Gamma0Artifact {
    pub(crate) schema: String,
    pub(crate) branch_domain: String,
    pub(crate) target_gamma0: i128,
    pub(crate) leaf_count: usize,
    pub(crate) successful_leaf_count: usize,
    pub(crate) max_success_denominator: Option<i128>,
    pub(crate) status: String,
    pub(crate) leaves: Vec<Leaf>,
}

#[derive(Deserialize)]
pub(crate) struct Gamma1Artifact {
    pub(crate) schema: String,
    pub(crate) branch_domain: String,
    pub(crate) base_included_vertex: usize,
    pub(crate) target_gamma1: i128,
    pub(crate) leaf_count: usize,
    pub(crate) successful_leaf_count: usize,
    pub(crate) max_success_denominator: Option<i128>,
    pub(crate) status: String,
    pub(crate) leaves: Vec<Leaf>,
}

#[derive(Deserialize)]
pub(crate) struct Leaf {
    pub(crate) included: Vec<usize>,
    pub(crate) excluded: Vec<usize>,
    pub(crate) active_vertices: usize,
    pub(crate) included_c0_weight: i128,
    pub(crate) success: Option<LeafSuccess>,
}

#[derive(Deserialize)]
pub(crate) struct LeafSuccess {
    pub(crate) denominator: i128,
    pub(crate) objective_num: i128,
    #[serde(default)]
    pub(crate) target_num: i128,
    pub(crate) min_slack: i128,
    pub(crate) positive_row_count: usize,
    pub(crate) rows: Vec<LeafRow>,
}

#[derive(Deserialize)]
pub(crate) struct LeafRow {
    pub(crate) kind: String,
    pub(crate) vertices: Vec<usize>,
    pub(crate) rhs: i128,
    pub(crate) numerator: i128,
    pub(crate) name: Option<String>,
    pub(crate) full_support_size: Option<usize>,
}

#[derive(Deserialize)]
struct ExcludeCertificate {
    rows: Vec<ExcludeRow>,
}

#[derive(Deserialize)]
struct ExcludeRow {
    kind: String,
    numerator: i128,
    #[serde(default)]
    vertices: Vec<usize>,
    #[serde(default)]
    support_vertices: Vec<usize>,
}

pub(crate) fn adjacency_from_edges(edges: &BTreeSet<(usize, usize)>) -> Vec<BitWords> {
    let mut adjacency = vec![empty_words(); EXPECTED_VERTEX_COUNT];
    for (left, right) in edges {
        set_bit(&mut adjacency[left - 1], right - 1);
        set_bit(&mut adjacency[right - 1], left - 1);
    }
    adjacency
}

pub(crate) fn recompute_c0(
    cert: &str,
    weights: &[i128],
) -> Result<Vec<i128>, G27GeometricFractionalError> {
    let cert: ExcludeCertificate =
        serde_json::from_str(cert).map_err(|_| malformed_err("w607_gamma0_c0_json"))?;
    let mut coverage = vec![0_i128; EXPECTED_VERTEX_COUNT];
    for row in cert.rows {
        match row.kind.as_str() {
            "parent_triangle" => {
                for vertex in one_based(&row.vertices)? {
                    if vertex != BRANCH_VERTEX {
                        coverage[vertex] += row.numerator;
                    }
                }
            }
            "child_weighted_rank" => {
                for vertex in one_based(&row.support_vertices)? {
                    if vertex != BRANCH_VERTEX {
                        coverage[vertex] += row.numerator * weights[vertex];
                    }
                }
            }
            _ => return malformed("w607_gamma0_c0_row"),
        }
    }
    Ok(coverage)
}

pub(crate) fn verify_partition(
    leaves: &[Leaf],
    adjacency: &[BitWords],
) -> Result<(), G27GeometricFractionalError> {
    verify_partition_with_base(leaves, &[], adjacency)
}

pub(crate) fn verify_partition_with_base(
    leaves: &[Leaf],
    base_included: &[usize],
    adjacency: &[BitWords],
) -> Result<(), G27GeometricFractionalError> {
    let mut branch_vertices = BTreeSet::new();
    for leaf in leaves {
        for vertex in one_based(&leaf.included)?
            .into_iter()
            .chain(one_based(&leaf.excluded)?)
        {
            if vertex == BRANCH_VERTEX {
                return malformed("w607_gamma0_partition_branch");
            }
            branch_vertices.insert(vertex);
        }
    }
    let branch_vertices = branch_vertices.into_iter().collect::<Vec<_>>();
    for mask in 0..(1_usize << branch_vertices.len()) {
        let chosen = branch_vertices
            .iter()
            .enumerate()
            .filter_map(|(index, vertex)| ((mask & (1 << index)) != 0).then_some(*vertex))
            .collect::<Vec<_>>();
        let mut selected = base_included.to_vec();
        selected.extend(chosen.iter().copied());
        if !is_independent(&selected, adjacency) {
            continue;
        }
        let matches = leaves
            .iter()
            .filter(|leaf| leaf_matches_assignment(leaf, &chosen).unwrap_or(false))
            .count();
        if matches != 1 {
            return malformed("w607_gamma0_partition");
        }
    }
    Ok(())
}

pub(crate) fn verify_leaf_sets(
    included: &[usize],
    excluded: &[usize],
    adjacency: &[BitWords],
) -> Result<(), G27GeometricFractionalError> {
    verify_leaf_sets_with_base(included, excluded, adjacency, false)
}

pub(crate) fn verify_leaf_sets_with_base(
    included: &[usize],
    excluded: &[usize],
    adjacency: &[BitWords],
    allow_branch_vertex: bool,
) -> Result<(), G27GeometricFractionalError> {
    if included.contains(&BRANCH_VERTEX) && !allow_branch_vertex
        || excluded.contains(&BRANCH_VERTEX)
        || !is_independent(included, adjacency)
    {
        return malformed("w607_gamma0_leaf_sets");
    }
    let included = included.iter().copied().collect::<BTreeSet<_>>();
    if excluded.iter().any(|vertex| included.contains(vertex)) {
        return malformed("w607_gamma0_leaf_conflict");
    }
    Ok(())
}

pub(crate) fn active_vertices(
    included: &[usize],
    excluded: &[usize],
    adjacency: &[BitWords],
) -> Vec<usize> {
    let mut blocked = BTreeSet::from([BRANCH_VERTEX]);
    blocked.extend(excluded.iter().copied());
    for vertex in included {
        blocked.insert(*vertex);
        blocked.extend(neighbors(*vertex, adjacency));
    }
    (0..EXPECTED_VERTEX_COUNT)
        .filter(|vertex| !blocked.contains(vertex))
        .collect()
}

pub(crate) fn replay_leaf(
    success: &LeafSuccess,
    included: &[usize],
    active: &[usize],
    weights: &[i128],
    c0: &[i128],
    adjacency: &[BitWords],
    ranks: &[RankCut],
) -> Result<(i128, i128), G27GeometricFractionalError> {
    let active_set = active.iter().copied().collect::<BTreeSet<_>>();
    let included_set = included.iter().copied().collect::<BTreeSet<_>>();
    let mut coverage = vec![0_i128; EXPECTED_VERTEX_COUNT];
    let mut objective = included.iter().map(|vertex| c0[*vertex]).sum::<i128>() * DENOMINATOR;
    for row in &success.rows {
        if row.numerator <= 0 {
            return malformed("w607_gamma0_row_numerator");
        }
        objective += row.numerator * row.rhs;
        let vertices = one_based(&row.vertices)?;
        match row.kind.as_str() {
            "edge" => replay_edge(row, &vertices, adjacency, &active_set, &mut coverage)?,
            "triangle" => replay_triangle(row, &vertices, adjacency, &active_set, &mut coverage)?,
            "rank" => replay_rank(
                row,
                &vertices,
                weights,
                &included_set,
                &active_set,
                ranks,
                &mut coverage,
            )?,
            _ => return malformed("w607_gamma0_row_kind"),
        }
    }
    let min_slack = active
        .iter()
        .map(|vertex| coverage[*vertex] - c0[*vertex] * DENOMINATOR)
        .min()
        .unwrap_or(0);
    Ok((objective, min_slack))
}

pub(crate) fn one_based(vertices: &[usize]) -> Result<Vec<usize>, G27GeometricFractionalError> {
    vertices
        .iter()
        .map(|vertex| {
            vertex
                .checked_sub(1)
                .filter(|v| *v < EXPECTED_VERTEX_COUNT)
                .ok_or(malformed_err("w607_gamma0_vertex"))
        })
        .collect()
}

fn replay_edge(
    row: &LeafRow,
    vertices: &[usize],
    adjacency: &[BitWords],
    active: &BTreeSet<usize>,
    coverage: &mut [i128],
) -> Result<(), G27GeometricFractionalError> {
    if row.rhs != 1 || vertices.len() != 2 || !has_bit(&adjacency[vertices[0]], vertices[1]) {
        return malformed("w607_gamma0_edge");
    }
    add_unit_coverage(vertices, row.numerator, active, coverage)
}

fn replay_triangle(
    row: &LeafRow,
    vertices: &[usize],
    adjacency: &[BitWords],
    active: &BTreeSet<usize>,
    coverage: &mut [i128],
) -> Result<(), G27GeometricFractionalError> {
    if row.rhs != 1 || vertices.len() != 3 || !is_triangle(vertices, adjacency) {
        return malformed("w607_gamma0_triangle");
    }
    add_unit_coverage(vertices, row.numerator, active, coverage)
}

fn replay_rank(
    row: &LeafRow,
    vertices: &[usize],
    weights: &[i128],
    included: &BTreeSet<usize>,
    active: &BTreeSet<usize>,
    ranks: &[RankCut],
    coverage: &mut [i128],
) -> Result<(), G27GeometricFractionalError> {
    let name = row
        .name
        .as_deref()
        .ok_or(malformed_err("w607_gamma0_rank_name"))?;
    let cut = ranks
        .iter()
        .find(|cut| cut.name == name)
        .ok_or(malformed_err("w607_gamma0_rank_unknown"))?;
    let local = cut
        .support
        .iter()
        .copied()
        .filter(|vertex| active.contains(vertex))
        .collect::<Vec<_>>();
    let used = cut
        .support
        .iter()
        .filter(|vertex| included.contains(vertex))
        .map(|vertex| weights[*vertex])
        .sum::<i128>();
    if vertices != local
        || row.rhs != cut.alpha_w - used
        || row.full_support_size != Some(cut.support.len())
    {
        return malformed("w607_gamma0_rank_row");
    }
    for vertex in vertices {
        coverage[*vertex] += row.numerator * weights[*vertex];
    }
    Ok(())
}

fn add_unit_coverage(
    vertices: &[usize],
    numerator: i128,
    active: &BTreeSet<usize>,
    coverage: &mut [i128],
) -> Result<(), G27GeometricFractionalError> {
    if vertices.iter().any(|vertex| !active.contains(vertex)) {
        return malformed("w607_gamma0_row_inactive");
    }
    for vertex in vertices {
        coverage[*vertex] += numerator;
    }
    Ok(())
}

fn leaf_matches_assignment(
    leaf: &Leaf,
    chosen: &[usize],
) -> Result<bool, G27GeometricFractionalError> {
    let chosen = chosen.iter().copied().collect::<BTreeSet<_>>();
    Ok(one_based(&leaf.included)?
        .iter()
        .all(|vertex| chosen.contains(vertex))
        && one_based(&leaf.excluded)?
            .iter()
            .all(|vertex| !chosen.contains(vertex)))
}

fn neighbors(vertex: usize, adjacency: &[BitWords]) -> Vec<usize> {
    (0..EXPECTED_VERTEX_COUNT)
        .filter(|candidate| has_bit(&adjacency[vertex], *candidate))
        .collect()
}

fn is_triangle(vertices: &[usize], adjacency: &[BitWords]) -> bool {
    has_bit(&adjacency[vertices[0]], vertices[1])
        && has_bit(&adjacency[vertices[0]], vertices[2])
        && has_bit(&adjacency[vertices[1]], vertices[2])
}

fn is_independent(vertices: &[usize], adjacency: &[BitWords]) -> bool {
    vertices.iter().enumerate().all(|(index, left)| {
        vertices[index + 1..]
            .iter()
            .all(|right| !has_bit(&adjacency[*left], *right))
    })
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
