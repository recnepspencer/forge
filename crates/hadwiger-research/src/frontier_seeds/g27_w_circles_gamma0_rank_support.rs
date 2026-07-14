use std::collections::{BTreeMap, BTreeSet};

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_w_circles_exact_geometry_support::EXPECTED_VERTEX_COUNT;

const ACCEPTED_RANKS: [(&str, i128); 16] = [
    ("top_weight_120", 316_539),
    ("twohop80_304", 255_387),
    ("twohop120_304", 306_879),
    ("twohop120_152", 262_126),
    ("twohop120_222", 262_126),
    ("twohop120_225", 262_126),
    ("twohop120_383", 262_126),
    ("twohop120_386", 262_126),
    ("twohop120_456", 262_126),
    ("twohop80_223", 216_958),
    ("twohop80_224", 216_958),
    ("dense80_304", 202_259),
    ("dense80_223", 235_789),
    ("dense120_223", 315_855),
    ("dense80_224", 235_789),
    ("dense120_224", 315_855),
];

pub(crate) struct RankCut {
    pub(crate) name: &'static str,
    pub(crate) support: Vec<usize>,
    pub(crate) alpha_w: i128,
}

pub(crate) fn rank_registry(
    weights: &[i128],
    adjacency: &[BitWords],
) -> Result<Vec<RankCut>, G27GeometricFractionalError> {
    ACCEPTED_RANKS
        .iter()
        .map(|(name, alpha_w)| {
            Ok(RankCut {
                name,
                support: pocket(name, weights, adjacency)?,
                alpha_w: *alpha_w,
            })
        })
        .collect()
}

fn pocket(
    name: &str,
    weights: &[i128],
    adjacency: &[BitWords],
) -> Result<Vec<usize>, G27GeometricFractionalError> {
    if name == "top_weight_120" {
        let mut vertices = (0..weights.len()).collect::<Vec<_>>();
        vertices.sort_by(|left, right| weights[*right].cmp(&weights[*left]).then(left.cmp(right)));
        vertices.truncate(120);
        vertices.sort_unstable();
        return Ok(vertices);
    }
    let (kind, raw) = name
        .rsplit_once('_')
        .ok_or(malformed_err("w607_gamma0_rank_kind"))?;
    let center = raw
        .parse::<usize>()
        .map_err(|_| malformed_err("w607_gamma0_rank_center"))?
        - 1;
    let limit = kind
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .map_err(|_| malformed_err("w607_gamma0_rank_limit"))?;
    match kind {
        k if k.starts_with("twohop") => Ok(twohop(center, limit, weights, adjacency)),
        k if k.starts_with("dense") => Ok(dense(center, limit, weights, adjacency)),
        _ => malformed("w607_gamma0_rank_kind"),
    }
}

fn twohop(center: usize, limit: usize, weights: &[i128], adjacency: &[BitWords]) -> Vec<usize> {
    let mut seen = BTreeSet::from([center]);
    seen.extend(neighbors(center, adjacency));
    for vertex in seen.clone() {
        seen.extend(neighbors(vertex, adjacency));
    }
    let mut vertices = seen.into_iter().collect::<Vec<_>>();
    vertices.sort_by(|left, right| weights[*right].cmp(&weights[*left]).then(left.cmp(right)));
    vertices.truncate(limit);
    vertices.sort_unstable();
    vertices
}

fn dense(center: usize, limit: usize, weights: &[i128], adjacency: &[BitWords]) -> Vec<usize> {
    let mut chosen = vec![center];
    let mut frontier = neighbors(center, adjacency);
    while chosen.len() < limit && !frontier.is_empty() {
        let scores = frontier
            .iter()
            .map(|v| (*v, dense_score(*v, &chosen, weights, adjacency)))
            .collect::<BTreeMap<_, _>>();
        frontier.sort_by(|left, right| scores[right].cmp(&scores[left]).then(left.cmp(right)));
        let vertex = frontier.remove(0);
        chosen.push(vertex);
        for neighbor in neighbors(vertex, adjacency) {
            if !chosen.contains(&neighbor) && !frontier.contains(&neighbor) {
                frontier.push(neighbor);
            }
        }
    }
    chosen.sort_unstable();
    chosen
}

fn dense_score(vertex: usize, chosen: &[usize], weights: &[i128], adjacency: &[BitWords]) -> i128 {
    let contact = chosen
        .iter()
        .filter(|other| has_bit(&adjacency[vertex], **other))
        .map(|other| weights[*other])
        .sum::<i128>();
    contact * 1_000_010 + weights[vertex]
}

fn neighbors(vertex: usize, adjacency: &[BitWords]) -> Vec<usize> {
    (0..EXPECTED_VERTEX_COUNT)
        .filter(|candidate| has_bit(&adjacency[vertex], *candidate))
        .collect()
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
