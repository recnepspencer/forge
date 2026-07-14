use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_finite_fractional_core_audit::audit_g27_w_circles_607_finite_fractional_core_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{empty_words, has_bit, set_bit, BitWords};
use super::g27_same_field_mwis_exact::exact_mwis;
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, EXPECTED_EDGE_COUNT, EXPECTED_VERTEX_COUNT,
};

const VIOLATION_NUMERATOR_THRESHOLD: i128 = 3_000;
const ROOT_CLIQUE_LP_OBJECTIVE: i128 = 666_661;
const WEIGHTED_RANK_LP_FLOOR: i128 = 641_090;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AcceptedWeightedRankCut {
    name: &'static str,
    expected_size: usize,
    expected_alpha_w: i128,
    expected_violation_numerator: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesWeightedRankCutReplayRow {
    name: String,
    vertex_count: usize,
    edge_count: usize,
    weight_sum: i128,
    alpha_w: i128,
    violation_numerator: i128,
    witness_size: usize,
}

impl G27WCirclesWeightedRankCutReplayRow {
    pub fn summary(&self) -> (&str, usize, usize, i128, i128, i128, usize) {
        (
            &self.name,
            self.vertex_count,
            self.edge_count,
            self.weight_sum,
            self.alpha_w,
            self.violation_numerator,
            self.witness_size,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesWeightedRankCutReplayStatus {
    RetainedWeightedRankCuts,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesWeightedRankCutReplayReport {
    core: HadwigerArtifactCore,
    rows: Vec<G27WCirclesWeightedRankCutReplayRow>,
    root_clique_lp_objective: i128,
    weighted_rank_lp_floor: i128,
    status: G27WCirclesWeightedRankCutReplayStatus,
    conclusion: String,
}

impl G27WCirclesWeightedRankCutReplayReport {
    pub fn rows(&self) -> &[G27WCirclesWeightedRankCutReplayRow] {
        &self.rows
    }

    pub fn lp_summary(&self) -> (i128, i128) {
        (self.root_clique_lp_objective, self.weighted_rank_lp_floor)
    }

    pub fn status(&self) -> G27WCirclesWeightedRankCutReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27WCirclesWeightedRankCutReplayReport, core);

pub fn replay_g27_w_circles_weighted_rank_cuts_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesWeightedRankCutReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return malformed("w607_weighted_rank_shape");
    }
    let adjacency = adjacency_from_edges(&edges);
    let rows = ACCEPTED_CUTS
        .iter()
        .map(|cut| replay_cut(*cut, &weights, &adjacency))
        .collect::<Result<Vec<_>, _>>()?;
    let conclusion = format!(
        "replayed {} weighted local-rank cuts in crate; these retain the mechanism that drops diagnostic root LP from {ROOT_CLIQUE_LP_OBJECTIVE} to about {WEIGHTED_RANK_LP_FLOOR}, but do not prove the W607 target",
        rows.len()
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesWeightedRankCutReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_weighted_rank_cut_replay".to_string(),
        },
        vec![source.reference()],
        payload(&rows, &conclusion),
    )?;
    Ok(G27WCirclesWeightedRankCutReplayReport {
        core,
        rows,
        root_clique_lp_objective: ROOT_CLIQUE_LP_OBJECTIVE,
        weighted_rank_lp_floor: WEIGHTED_RANK_LP_FLOOR,
        status: G27WCirclesWeightedRankCutReplayStatus::RetainedWeightedRankCuts,
        conclusion,
    })
}

fn replay_cut(
    cut: AcceptedWeightedRankCut,
    weights: &[i128],
    adjacency: &[BitWords],
) -> Result<G27WCirclesWeightedRankCutReplayRow, G27GeometricFractionalError> {
    let vertices = pocket(cut.name, weights, adjacency)?;
    let weight_sum = vertices.iter().map(|vertex| weights[*vertex]).sum::<i128>();
    let (alpha_w, witness) = exact_mwis(adjacency, weights, &vertices);
    let violation_numerator = weight_sum - 3 * alpha_w;
    if vertices.len() != cut.expected_size
        || alpha_w != cut.expected_alpha_w
        || violation_numerator != cut.expected_violation_numerator
        || violation_numerator < VIOLATION_NUMERATOR_THRESHOLD
        || !is_independent(adjacency, &witness)
        || witness.iter().map(|vertex| weights[*vertex]).sum::<i128>() != alpha_w
    {
        return malformed("w607_weighted_rank_cut_replay");
    }
    Ok(G27WCirclesWeightedRankCutReplayRow {
        name: cut.name.to_string(),
        edge_count: induced_edge_count(adjacency, &vertices),
        vertex_count: vertices.len(),
        weight_sum,
        alpha_w,
        violation_numerator,
        witness_size: witness.len(),
    })
}

fn pocket(
    name: &str,
    weights: &[i128],
    adjacency: &[BitWords],
) -> Result<Vec<usize>, G27GeometricFractionalError> {
    if name == "top_weight_120" {
        return Ok(top_weight(weights, 120));
    }
    if let Some(rest) = name.strip_prefix("twohop80_") {
        return clipped_two_hop(parse_vertex(rest)?, 80, weights, adjacency);
    }
    if let Some(rest) = name.strip_prefix("twohop120_") {
        return clipped_two_hop(parse_vertex(rest)?, 120, weights, adjacency);
    }
    if let Some(rest) = name.strip_prefix("dense80_") {
        return Ok(dense_expand(parse_vertex(rest)?, 80, weights, adjacency));
    }
    if let Some(rest) = name.strip_prefix("dense120_") {
        return Ok(dense_expand(parse_vertex(rest)?, 120, weights, adjacency));
    }
    malformed("w607_weighted_rank_cut_name")
}

fn clipped_two_hop(
    center: usize,
    limit: usize,
    weights: &[i128],
    adjacency: &[BitWords],
) -> Result<Vec<usize>, G27GeometricFractionalError> {
    let mut set = vec![center];
    push_neighbors(&mut set, center, adjacency);
    let first_hop = set.clone();
    for vertex in first_hop {
        push_neighbors(&mut set, vertex, adjacency);
    }
    set.sort_unstable();
    set.dedup();
    if set.len() > limit {
        set.sort_by(|left, right| {
            weights[*right]
                .cmp(&weights[*left])
                .then_with(|| left.cmp(right))
        });
        set.truncate(limit);
        set.sort_unstable();
    }
    Ok(set)
}

fn dense_expand(seed: usize, limit: usize, weights: &[i128], adjacency: &[BitWords]) -> Vec<usize> {
    let mut set = vec![seed];
    let mut frontier = neighbors(seed, adjacency);
    while set.len() < limit && !frontier.is_empty() {
        frontier.sort_by(|left, right| {
            dense_score(*right, &set, weights, adjacency)
                .cmp(&dense_score(*left, &set, weights, adjacency))
                .then_with(|| left.cmp(right))
        });
        let vertex = frontier.remove(0);
        set.push(vertex);
        for neighbor in neighbors(vertex, adjacency) {
            if !set.contains(&neighbor) && !frontier.contains(&neighbor) {
                frontier.push(neighbor);
            }
        }
    }
    set.sort_unstable();
    set
}

fn dense_score(vertex: usize, set: &[usize], weights: &[i128], adjacency: &[BitWords]) -> i128 {
    let contact_weight = set
        .iter()
        .filter(|other| has_bit(&adjacency[vertex], **other))
        .map(|other| weights[*other])
        .sum::<i128>();
    contact_weight * 10_000 + weights[vertex]
}

fn top_weight(weights: &[i128], limit: usize) -> Vec<usize> {
    let mut vertices = (0..weights.len()).collect::<Vec<_>>();
    vertices.sort_by(|left, right| {
        weights[*right]
            .cmp(&weights[*left])
            .then_with(|| left.cmp(right))
    });
    vertices.truncate(limit);
    vertices.sort_unstable();
    vertices
}

fn adjacency_from_edges(edges: &std::collections::BTreeSet<(usize, usize)>) -> Vec<BitWords> {
    let mut adjacency = vec![empty_words(); EXPECTED_VERTEX_COUNT];
    for (left, right) in edges {
        set_bit(&mut adjacency[left - 1], right - 1);
        set_bit(&mut adjacency[right - 1], left - 1);
    }
    adjacency
}

fn push_neighbors(set: &mut Vec<usize>, vertex: usize, adjacency: &[BitWords]) {
    for neighbor in neighbors(vertex, adjacency) {
        if !set.contains(&neighbor) {
            set.push(neighbor);
        }
    }
}

fn neighbors(vertex: usize, adjacency: &[BitWords]) -> Vec<usize> {
    (0..EXPECTED_VERTEX_COUNT)
        .filter(|candidate| has_bit(&adjacency[vertex], *candidate))
        .collect()
}

fn induced_edge_count(adjacency: &[BitWords], vertices: &[usize]) -> usize {
    let mut count = 0;
    for left in 0..vertices.len() {
        for right in left + 1..vertices.len() {
            count += usize::from(has_bit(&adjacency[vertices[left]], vertices[right]));
        }
    }
    count
}

fn is_independent(adjacency: &[BitWords], vertices: &[usize]) -> bool {
    induced_edge_count(adjacency, vertices) == 0
}

fn parse_vertex(value: &str) -> Result<usize, G27GeometricFractionalError> {
    value
        .parse::<usize>()
        .ok()
        .and_then(|vertex| vertex.checked_sub(1))
        .filter(|vertex| *vertex < EXPECTED_VERTEX_COUNT)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "w607_weighted_rank_vertex",
        })
}

fn payload(
    rows: &[G27WCirclesWeightedRankCutReplayRow],
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let best = rows
        .iter()
        .map(|row| row.violation_numerator)
        .max()
        .unwrap_or(0);
    vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.w607_weighted_rank_cut_replay.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("cut_count", rows.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned("best_violation_numerator", best as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "root_clique_lp_objective",
            ROOT_CLIQUE_LP_OBJECTIVE as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "weighted_rank_lp_floor",
            WEIGHTED_RANK_LP_FLOOR as u128,
        ),
        HadwigerArtifactPayloadEntry::text("status", "retained_weighted_rank_cuts"),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(G27GeometricFractionalError::MalformedData { source })
}

const ACCEPTED_CUTS: &[AcceptedWeightedRankCut] = &[
    AcceptedWeightedRankCut {
        name: "top_weight_120",
        expected_size: 120,
        expected_alpha_w: 316_539,
        expected_violation_numerator: 98_138,
    },
    AcceptedWeightedRankCut {
        name: "twohop80_304",
        expected_size: 80,
        expected_alpha_w: 255_387,
        expected_violation_numerator: 5_906,
    },
    AcceptedWeightedRankCut {
        name: "twohop120_304",
        expected_size: 120,
        expected_alpha_w: 306_879,
        expected_violation_numerator: 17_488,
    },
    AcceptedWeightedRankCut {
        name: "twohop120_152",
        expected_size: 120,
        expected_alpha_w: 262_126,
        expected_violation_numerator: 10_743,
    },
    AcceptedWeightedRankCut {
        name: "twohop120_222",
        expected_size: 120,
        expected_alpha_w: 262_126,
        expected_violation_numerator: 10_743,
    },
    AcceptedWeightedRankCut {
        name: "twohop120_225",
        expected_size: 120,
        expected_alpha_w: 262_126,
        expected_violation_numerator: 10_743,
    },
    AcceptedWeightedRankCut {
        name: "twohop120_383",
        expected_size: 120,
        expected_alpha_w: 262_126,
        expected_violation_numerator: 10_743,
    },
    AcceptedWeightedRankCut {
        name: "twohop120_386",
        expected_size: 120,
        expected_alpha_w: 262_126,
        expected_violation_numerator: 10_743,
    },
    AcceptedWeightedRankCut {
        name: "twohop120_456",
        expected_size: 120,
        expected_alpha_w: 262_126,
        expected_violation_numerator: 10_743,
    },
    AcceptedWeightedRankCut {
        name: "twohop80_223",
        expected_size: 80,
        expected_alpha_w: 216_958,
        expected_violation_numerator: 3_919,
    },
    AcceptedWeightedRankCut {
        name: "twohop80_224",
        expected_size: 80,
        expected_alpha_w: 216_958,
        expected_violation_numerator: 3_919,
    },
    AcceptedWeightedRankCut {
        name: "dense80_304",
        expected_size: 80,
        expected_alpha_w: 202_259,
        expected_violation_numerator: 12_578,
    },
    AcceptedWeightedRankCut {
        name: "dense80_223",
        expected_size: 80,
        expected_alpha_w: 235_789,
        expected_violation_numerator: 8_778,
    },
    AcceptedWeightedRankCut {
        name: "dense120_223",
        expected_size: 120,
        expected_alpha_w: 315_855,
        expected_violation_numerator: 48_638,
    },
    AcceptedWeightedRankCut {
        name: "dense80_224",
        expected_size: 80,
        expected_alpha_w: 235_789,
        expected_violation_numerator: 8_778,
    },
    AcceptedWeightedRankCut {
        name: "dense120_224",
        expected_size: 120,
        expected_alpha_w: 315_855,
        expected_violation_numerator: 48_638,
    },
];
