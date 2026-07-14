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

const CERT: &str = include_str!("../../docs/w607-v304-exclude-dual-cover-den1024.json");
const W607_DIGEST: &str = "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad";
const EXCLUDED_VERTEX: usize = 304;
const DENOMINATOR: i128 = 1024;
const OBJECTIVE_NUMERATOR: i128 = 647_496_725;
const TARGET_WEIGHT: i128 = 512_933;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesV304ExcludeDualCoverReplayStatus {
    ReplayedWeakBranchCertificate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesV304ExcludeDualCoverReplayReport {
    core: HadwigerArtifactCore,
    objective_numerator: i128,
    denominator: i128,
    triangle_rows: usize,
    rank_rows: usize,
    min_active_slack: i128,
    conclusion: String,
}

impl G27WCirclesV304ExcludeDualCoverReplayReport {
    pub fn summary(&self) -> (i128, i128, usize, usize, i128) {
        (
            self.objective_numerator,
            self.denominator,
            self.triangle_rows,
            self.rank_rows,
            self.min_active_slack,
        )
    }

    pub fn status(&self) -> G27WCirclesV304ExcludeDualCoverReplayStatus {
        G27WCirclesV304ExcludeDualCoverReplayStatus::ReplayedWeakBranchCertificate
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_target_authority(&self) -> bool {
        self.objective_numerator <= TARGET_WEIGHT * self.denominator
    }
}

impl_hadwiger_artifact!(G27WCirclesV304ExcludeDualCoverReplayReport, core);

pub fn replay_g27_w_circles_v304_exclude_dual_cover_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesV304ExcludeDualCoverReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return malformed("w607_v304_dual_shape");
    }
    require(
        &format!("\"graph_digest\":\"{W607_DIGEST}\""),
        "w607_v304_dual_digest",
    )?;
    require(
        &format!("\"excluded_vertex\":{EXCLUDED_VERTEX}"),
        "w607_v304_dual_excluded",
    )?;
    require(
        &format!("\"objective_numerator\":{OBJECTIVE_NUMERATOR}"),
        "w607_v304_dual_objective",
    )?;
    let adjacency = adjacency_from_edges(&edges);
    let mut coverage = vec![0_i128; EXPECTED_VERTEX_COUNT];
    let mut objective = 0_i128;
    let mut triangle_rows = 0_usize;
    let mut rank_rows = 0_usize;
    for row in rows()? {
        match row {
            Row::Triangle {
                vertices,
                numerator,
            } => {
                if !is_triangle(&vertices, &adjacency) {
                    return malformed("w607_v304_dual_triangle");
                }
                objective += numerator;
                for vertex in vertices {
                    if vertex + 1 != EXCLUDED_VERTEX {
                        coverage[vertex] += numerator;
                    }
                }
                triangle_rows += 1;
            }
            Row::ChildRank {
                support,
                alpha_w,
                numerator,
            } => {
                if !is_sorted_unique_active_or_excluded(&support) {
                    return malformed("w607_v304_dual_rank_support");
                }
                let (replayed_alpha, witness) = exact_mwis(&adjacency, &weights, &support);
                if replayed_alpha != alpha_w || !is_independent(&adjacency, &witness) {
                    return malformed("w607_v304_dual_rank_alpha");
                }
                objective += numerator * alpha_w;
                for vertex in support {
                    if vertex + 1 != EXCLUDED_VERTEX {
                        coverage[vertex] += numerator * weights[vertex];
                    }
                }
                rank_rows += 1;
            }
        }
    }
    let min_slack = coverage
        .iter()
        .enumerate()
        .filter(|(vertex, _)| *vertex + 1 != EXCLUDED_VERTEX)
        .map(|(vertex, covered)| covered - weights[vertex] * DENOMINATOR)
        .min()
        .ok_or(malformed_err("w607_v304_dual_empty"))?;
    if objective != OBJECTIVE_NUMERATOR || min_slack < 0 || triangle_rows != 599 || rank_rows != 1 {
        return malformed("w607_v304_dual_replay");
    }
    let conclusion = format!(
        "replayed exact W607 v304-exclude branch certificate at {OBJECTIVE_NUMERATOR}/{DENOMINATOR}; this is branch-local proof plumbing, not target authority"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesV304ExcludeDualCoverReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_v304_exclude_dual_cover_replay".to_string(),
        },
        vec![source.reference()],
        payload(triangle_rows, rank_rows, min_slack, &conclusion),
    )?;
    Ok(G27WCirclesV304ExcludeDualCoverReplayReport {
        core,
        objective_numerator: OBJECTIVE_NUMERATOR,
        denominator: DENOMINATOR,
        triangle_rows,
        rank_rows,
        min_active_slack: min_slack,
        conclusion,
    })
}

enum Row {
    Triangle {
        vertices: Vec<usize>,
        numerator: i128,
    },
    ChildRank {
        support: Vec<usize>,
        alpha_w: i128,
        numerator: i128,
    },
}

fn rows() -> Result<Vec<Row>, G27GeometricFractionalError> {
    let blob = between("\"rows\":[", "],\"generator\"")?;
    let mut rows = Vec::new();
    for entry in blob.split("{\"kind\":\"").skip(1) {
        let (kind, rest) = entry
            .split_once('"')
            .ok_or(malformed_err("w607_v304_dual_kind"))?;
        if kind == "parent_triangle" {
            rows.push(Row::Triangle {
                vertices: vertices_between(rest, "\"vertices\":[", "]")?,
                numerator: number_after(rest, "\"numerator\":")?,
            });
        } else if kind == "child_weighted_rank" {
            require_in(rest, "\"pocket\":\"dense120_303\"", "w607_v304_dual_pocket")?;
            rows.push(Row::ChildRank {
                support: vertices_between(rest, "\"support_vertices\":[", "]")?,
                alpha_w: number_after(rest, "\"alpha_w\":")?,
                numerator: number_after(rest, "\"numerator\":")?,
            });
        } else {
            return malformed("w607_v304_dual_row_kind");
        }
    }
    Ok(rows)
}

fn adjacency_from_edges(edges: &BTreeSet<(usize, usize)>) -> Vec<BitWords> {
    let mut adjacency = vec![empty_words(); EXPECTED_VERTEX_COUNT];
    for (left, right) in edges {
        set_bit(&mut adjacency[left - 1], right - 1);
        set_bit(&mut adjacency[right - 1], left - 1);
    }
    adjacency
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

fn is_sorted_unique_active_or_excluded(vertices: &[usize]) -> bool {
    vertices
        .windows(2)
        .all(|pair| pair[0] < pair[1] && pair[1] < EXPECTED_VERTEX_COUNT)
        && vertices
            .first()
            .is_some_and(|first| *first < EXPECTED_VERTEX_COUNT)
}

fn vertices_between(
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
                .ok_or(malformed_err("w607_v304_dual_vertex"))
        })
        .collect()
}

fn number_after(text: &str, prefix: &str) -> Result<i128, G27GeometricFractionalError> {
    let rest = text
        .split_once(prefix)
        .map(|(_, rest)| rest)
        .ok_or(malformed_err("w607_v304_dual_number_prefix"))?;
    let end = rest
        .find(|ch: char| !ch.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end]
        .parse()
        .map_err(|_| malformed_err("w607_v304_dual_number"))
}

fn between(start: &str, end: &str) -> Result<&'static str, G27GeometricFractionalError> {
    between_in(CERT, start, end)
}

fn between_in<'a>(
    text: &'a str,
    start: &str,
    end: &str,
) -> Result<&'a str, G27GeometricFractionalError> {
    text.split_once(start)
        .and_then(|(_, rest)| rest.split_once(end).map(|(body, _)| body))
        .ok_or(malformed_err("w607_v304_dual_between"))
}

fn require(pattern: &str, source: &'static str) -> Result<(), G27GeometricFractionalError> {
    if CERT.contains(pattern) {
        Ok(())
    } else {
        malformed(source)
    }
}

fn require_in(
    text: &str,
    pattern: &str,
    source: &'static str,
) -> Result<(), G27GeometricFractionalError> {
    if text.contains(pattern) {
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
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.w607_v304_dual_replay.v1"),
        HadwigerArtifactPayloadEntry::unsigned("triangle_rows", triangle_rows as u128),
        HadwigerArtifactPayloadEntry::unsigned("rank_rows", rank_rows as u128),
        HadwigerArtifactPayloadEntry::unsigned("objective_numerator", OBJECTIVE_NUMERATOR as u128),
        HadwigerArtifactPayloadEntry::unsigned("denominator", DENOMINATOR as u128),
        HadwigerArtifactPayloadEntry::unsigned("min_active_slack", min_slack as u128),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
