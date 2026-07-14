use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_w_circles_gamma0_leaf_dual_support::{
    one_based, LeafSuccess, BRANCH_VERTEX, DENOMINATOR,
};

#[derive(Deserialize)]
pub(crate) struct BranchSlackArtifact {
    pub(crate) schema: String,
    pub(crate) canonical_denominator: i128,
    pub(crate) gamma0_modified_num_d1024: i128,
    pub(crate) worst_gamma1_modified_num_d1024: i128,
    pub(crate) lift_coefficient_num_d1024: i128,
    pub(crate) positive_coefficients_num_d1024: BTreeMap<String, i128>,
    pub(crate) leaf_count: usize,
    pub(crate) successful_leaf_count: usize,
    pub(crate) leaf_reports: Vec<BranchSlackLeaf>,
    pub(crate) status: String,
}

#[derive(Deserialize)]
pub(crate) struct BranchSlackLeaf {
    pub(crate) included: Vec<usize>,
    pub(crate) excluded: Vec<usize>,
    pub(crate) active_vertices: usize,
    pub(crate) included_modified_weight: FractionJson,
    pub(crate) success: Option<LeafSuccess>,
}

#[derive(Deserialize)]
pub(crate) struct FractionJson {
    pub(crate) num: i128,
    pub(crate) den: i128,
}

pub(crate) fn coefficient_map(
    artifact: &BranchSlackArtifact,
) -> Result<BTreeMap<usize, i128>, G27GeometricFractionalError> {
    artifact
        .positive_coefficients_num_d1024
        .iter()
        .map(|(vertex, num)| {
            let vertex = vertex
                .parse::<usize>()
                .map_err(|_| malformed_err("w607_branch_slack_coeff"))?
                - 1;
            Ok((vertex, *num))
        })
        .collect()
}

pub(crate) fn fraction_to_d1024(value: &FractionJson) -> Result<i128, G27GeometricFractionalError> {
    let scaled = value.num * DENOMINATOR;
    if scaled % value.den != 0 {
        return malformed("w607_branch_slack_fraction");
    }
    Ok(scaled / value.den)
}

pub(crate) fn verify_branch_partition(
    leaves: &[BranchSlackLeaf],
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
                return malformed("w607_branch_slack_partition_branch");
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
            return malformed("w607_branch_slack_partition");
        }
    }
    Ok(())
}

fn leaf_matches_assignment(
    leaf: &BranchSlackLeaf,
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
