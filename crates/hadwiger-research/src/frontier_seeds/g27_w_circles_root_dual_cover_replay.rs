use std::collections::BTreeSet;

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

const CERTIFICATE: &str =
    include_str!("../../docs/w607-root-rank-triangle-dual-cover-den1024.json");
const W607_DIGEST: &str = "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad";
const DENOMINATOR: i128 = 1024;
const OBJECTIVE_NUMERATOR: i128 = 656_787_579;
const TARGET_WEIGHT: i128 = 512_933;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesRootDualCoverReplayStatus {
    ReplayedWeakRootCertificate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesRootDualCoverReplayReport {
    core: HadwigerArtifactCore,
    denominator: i128,
    objective_numerator: i128,
    triangle_row_count: usize,
    rank_row_count: usize,
    min_coverage_slack_numerator: i128,
    status: G27WCirclesRootDualCoverReplayStatus,
    conclusion: String,
}

impl G27WCirclesRootDualCoverReplayReport {
    pub fn summary(&self) -> (i128, i128, usize, usize, i128) {
        (
            self.denominator,
            self.objective_numerator,
            self.triangle_row_count,
            self.rank_row_count,
            self.min_coverage_slack_numerator,
        )
    }

    pub fn status(&self) -> G27WCirclesRootDualCoverReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_target_authority(&self) -> bool {
        self.objective_numerator <= TARGET_WEIGHT * self.denominator
    }
}

impl_hadwiger_artifact!(G27WCirclesRootDualCoverReplayReport, core);

pub fn replay_g27_w_circles_root_dual_cover_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesRootDualCoverReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return malformed("w607_root_dual_cover_shape");
    }
    require_contains(
        &format!("\"graph_digest\":\"{W607_DIGEST}\""),
        "w607_root_dual_digest",
    )?;
    require_contains(
        &format!("\"denominator\":{DENOMINATOR}"),
        "w607_root_dual_denominator",
    )?;
    require_contains(
        &format!("\"objective_numerator\":{OBJECTIVE_NUMERATOR}"),
        "w607_root_dual_objective",
    )?;
    let adjacency = adjacency_from_edges(&edges);
    let mut coverage = vec![0_i128; EXPECTED_VERTEX_COUNT];
    let mut objective = 0_i128;
    let mut triangle_rows = 0_usize;
    let mut rank_rows = 0_usize;
    for row in certificate_rows()? {
        match row {
            CertificateRow::Triangle {
                vertices,
                numerator,
            } => {
                if !is_triangle(&vertices, &adjacency) {
                    return malformed("w607_root_dual_triangle");
                }
                objective += numerator;
                for vertex in vertices {
                    coverage[vertex] += numerator;
                }
                triangle_rows += 1;
            }
            CertificateRow::WeightedRank {
                pocket_name,
                support_vertices,
                alpha_w,
                numerator,
            } => {
                let expected = pocket(pocket_name, &weights, &adjacency)?;
                if expected != support_vertices {
                    return malformed("w607_root_dual_rank_support");
                }
                let (replayed_alpha, witness) = exact_mwis(&adjacency, &weights, &expected);
                if replayed_alpha != alpha_w || !is_independent(&adjacency, &witness) {
                    return malformed("w607_root_dual_rank_alpha");
                }
                objective += numerator * alpha_w;
                for vertex in expected {
                    coverage[vertex] += numerator * weights[vertex];
                }
                rank_rows += 1;
            }
        }
    }
    let min_slack = coverage
        .iter()
        .zip(weights.iter())
        .map(|(covered, weight)| covered - weight * DENOMINATOR)
        .min()
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "w607_root_dual_empty_coverage",
        })?;
    if objective != OBJECTIVE_NUMERATOR || min_slack < 0 || triangle_rows != 595 || rank_rows != 2 {
        return malformed("w607_root_dual_replay");
    }
    let conclusion = format!(
        "replayed exact W607 triangle/rank dual cover at {OBJECTIVE_NUMERATOR}/{DENOMINATOR}; this certifies the retained root relaxation but remains above target {TARGET_WEIGHT}"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesRootDualCoverReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_root_dual_cover_replay".to_string(),
        },
        vec![source.reference()],
        payload(triangle_rows, rank_rows, min_slack, &conclusion),
    )?;
    Ok(G27WCirclesRootDualCoverReplayReport {
        core,
        denominator: DENOMINATOR,
        objective_numerator: OBJECTIVE_NUMERATOR,
        triangle_row_count: triangle_rows,
        rank_row_count: rank_rows,
        min_coverage_slack_numerator: min_slack,
        status: G27WCirclesRootDualCoverReplayStatus::ReplayedWeakRootCertificate,
        conclusion,
    })
}

enum CertificateRow<'a> {
    Triangle {
        vertices: Vec<usize>,
        numerator: i128,
    },
    WeightedRank {
        pocket_name: &'a str,
        support_vertices: Vec<usize>,
        alpha_w: i128,
        numerator: i128,
    },
}

fn certificate_rows() -> Result<Vec<CertificateRow<'static>>, G27GeometricFractionalError> {
    let rows_blob = between("\"rows\":[", "],\"generator\"")?;
    let mut rows = Vec::new();
    for entry in rows_blob.split("{\"kind\":\"").skip(1) {
        let (kind, rest) = entry
            .split_once('"')
            .ok_or(malformed_err("w607_root_dual_kind"))?;
        if kind == "triangle" {
            let vertices = parse_vertices_between(rest, "\"vertices\":[", "]")?;
            let numerator = parse_i128_after(rest, "\"numerator\":")?;
            rows.push(CertificateRow::Triangle {
                vertices,
                numerator,
            });
        } else if kind == "weighted_rank" {
            let pocket_name = between_in(rest, "\"pocket\":\"", "\"")?;
            let alpha_w = parse_i128_after(rest, "\"alpha_w\":")?;
            let support_vertices = parse_vertices_between(rest, "\"support_vertices\":[", "]")?;
            let numerator = parse_i128_after(rest, "\"numerator\":")?;
            rows.push(CertificateRow::WeightedRank {
                pocket_name,
                support_vertices,
                alpha_w,
                numerator,
            });
        } else {
            return malformed("w607_root_dual_unknown_row");
        }
    }
    Ok(rows)
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
    if name == "dense80_304" {
        return Ok(dense_expand(303, 80, weights, adjacency));
    }
    malformed("w607_root_dual_rank_name")
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
    let contact = set
        .iter()
        .filter(|other| has_bit(&adjacency[vertex], **other))
        .map(|other| weights[*other])
        .sum::<i128>();
    contact * 10_000 + weights[vertex]
}

fn adjacency_from_edges(edges: &BTreeSet<(usize, usize)>) -> Vec<BitWords> {
    let mut adjacency = vec![empty_words(); EXPECTED_VERTEX_COUNT];
    for (left, right) in edges {
        set_bit(&mut adjacency[left - 1], right - 1);
        set_bit(&mut adjacency[right - 1], left - 1);
    }
    adjacency
}

fn neighbors(vertex: usize, adjacency: &[BitWords]) -> Vec<usize> {
    (0..EXPECTED_VERTEX_COUNT)
        .filter(|candidate| has_bit(&adjacency[vertex], *candidate))
        .collect()
}

fn is_triangle(vertices: &[usize], adjacency: &[BitWords]) -> bool {
    vertices.len() == 3
        && has_bit(&adjacency[vertices[0]], vertices[1])
        && has_bit(&adjacency[vertices[0]], vertices[2])
        && has_bit(&adjacency[vertices[1]], vertices[2])
}

fn is_independent(adjacency: &[BitWords], vertices: &[usize]) -> bool {
    for left in 0..vertices.len() {
        for right in left + 1..vertices.len() {
            if has_bit(&adjacency[vertices[left]], vertices[right]) {
                return false;
            }
        }
    }
    true
}

fn parse_vertices_between(
    text: &str,
    start: &str,
    end: &str,
) -> Result<Vec<usize>, G27GeometricFractionalError> {
    between_in(text, start, end)?
        .split(',')
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .parse::<usize>()
                .ok()
                .and_then(|vertex| vertex.checked_sub(1))
                .filter(|vertex| *vertex < EXPECTED_VERTEX_COUNT)
                .ok_or(malformed_err("w607_root_dual_vertex"))
        })
        .collect()
}

fn parse_i128_after(text: &str, prefix: &str) -> Result<i128, G27GeometricFractionalError> {
    let rest = text
        .split_once(prefix)
        .map(|(_, rest)| rest)
        .ok_or(malformed_err("w607_root_dual_number_prefix"))?;
    let end = rest
        .find(|ch: char| !ch.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end]
        .parse()
        .map_err(|_| malformed_err("w607_root_dual_number"))
}

fn between(start: &str, end: &str) -> Result<&'static str, G27GeometricFractionalError> {
    between_in(CERTIFICATE, start, end)
}

fn between_in<'a>(
    text: &'a str,
    start: &str,
    end: &str,
) -> Result<&'a str, G27GeometricFractionalError> {
    text.split_once(start)
        .and_then(|(_, rest)| rest.split_once(end).map(|(body, _)| body))
        .ok_or(malformed_err("w607_root_dual_between"))
}

fn require_contains(
    pattern: &str,
    source: &'static str,
) -> Result<(), G27GeometricFractionalError> {
    if CERTIFICATE.contains(pattern) {
        Ok(())
    } else {
        malformed(source)
    }
}

fn payload(
    triangle_rows: usize,
    rank_rows: usize,
    min_slack: i128,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.w607_root_dual_cover_replay.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("triangle_rows", triangle_rows as u128),
        HadwigerArtifactPayloadEntry::unsigned("rank_rows", rank_rows as u128),
        HadwigerArtifactPayloadEntry::unsigned("denominator", DENOMINATOR as u128),
        HadwigerArtifactPayloadEntry::unsigned("objective_numerator", OBJECTIVE_NUMERATOR as u128),
        HadwigerArtifactPayloadEntry::unsigned("min_coverage_slack", min_slack as u128),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
