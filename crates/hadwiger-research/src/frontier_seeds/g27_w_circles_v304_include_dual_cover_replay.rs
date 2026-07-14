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
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, EXPECTED_EDGE_COUNT, EXPECTED_VERTEX_COUNT,
};

const CERT: &str = include_str!("../../docs/w607-v304-include-dual-cover-den1024.json");
const W607_DIGEST: &str = "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad";
const INCLUDED_VERTEX: usize = 304;
const DENOMINATOR: i128 = 1024;
const OBJECTIVE_NUMERATOR: i128 = 618_626_223;
const TARGET_WEIGHT: i128 = 512_933;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesV304IncludeDualCoverReplayStatus {
    ReplayedWeakBranchCertificate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesV304IncludeDualCoverReplayReport {
    core: HadwigerArtifactCore,
    objective_numerator: i128,
    denominator: i128,
    clique_rows: usize,
    min_active_slack: i128,
    conclusion: String,
}

impl G27WCirclesV304IncludeDualCoverReplayReport {
    pub fn summary(&self) -> (i128, i128, usize, i128) {
        (
            self.objective_numerator,
            self.denominator,
            self.clique_rows,
            self.min_active_slack,
        )
    }

    pub fn status(&self) -> G27WCirclesV304IncludeDualCoverReplayStatus {
        G27WCirclesV304IncludeDualCoverReplayStatus::ReplayedWeakBranchCertificate
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_target_authority(&self) -> bool {
        self.objective_numerator <= TARGET_WEIGHT * self.denominator
    }
}

impl_hadwiger_artifact!(G27WCirclesV304IncludeDualCoverReplayReport, core);

pub fn replay_g27_w_circles_v304_include_dual_cover_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesV304IncludeDualCoverReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return malformed("w607_v304_include_shape");
    }
    require(
        &format!("\"graph_digest\":\"{W607_DIGEST}\""),
        "w607_v304_include_digest",
    )?;
    require(
        &format!("\"included_vertex\":{INCLUDED_VERTEX}"),
        "w607_v304_include_vertex",
    )?;
    require(
        &format!("\"objective_numerator\":{OBJECTIVE_NUMERATOR}"),
        "w607_v304_include_objective",
    )?;
    let adjacency = adjacency_from_edges(&edges);
    let mut active = vec![true; EXPECTED_VERTEX_COUNT];
    active[INCLUDED_VERTEX - 1] = false;
    for neighbor in neighbors(INCLUDED_VERTEX - 1, &adjacency) {
        active[neighbor] = false;
    }
    let mut coverage = vec![0_i128; EXPECTED_VERTEX_COUNT];
    let mut objective = 0_i128;
    let mut clique_rows = 0_usize;
    let mut included_seen = false;
    for row in rows()? {
        match row {
            Row::Included {
                vertex,
                weight,
                numerator,
            } => {
                if vertex != INCLUDED_VERTEX - 1
                    || weight != weights[vertex]
                    || numerator != DENOMINATOR
                {
                    return malformed("w607_v304_include_row");
                }
                objective += weight * numerator;
                included_seen = true;
            }
            Row::Clique {
                vertices,
                rhs,
                numerator,
            } => {
                if rhs != 1 || !vertices.iter().all(|vertex| active[*vertex]) {
                    return malformed("w607_v304_include_active");
                }
                if !is_clique(&vertices, &adjacency) {
                    return malformed("w607_v304_include_clique");
                }
                objective += numerator;
                for vertex in vertices {
                    coverage[vertex] += numerator;
                }
                clique_rows += 1;
            }
        }
    }
    let min_slack = coverage
        .iter()
        .enumerate()
        .filter(|(vertex, _)| active[*vertex])
        .map(|(vertex, covered)| covered - weights[vertex] * DENOMINATOR)
        .min()
        .ok_or(malformed_err("w607_v304_include_empty"))?;
    if !included_seen || objective != OBJECTIVE_NUMERATOR || min_slack < 0 || clique_rows != 573 {
        return malformed("w607_v304_include_replay");
    }
    let conclusion = format!(
        "replayed exact W607 v304-include branch certificate at {OBJECTIVE_NUMERATOR}/{DENOMINATOR}; this is one side of a branch proof, not target authority"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesV304IncludeDualCoverReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_v304_include_dual_cover_replay".to_string(),
        },
        vec![source.reference()],
        payload(clique_rows, min_slack, &conclusion),
    )?;
    Ok(G27WCirclesV304IncludeDualCoverReplayReport {
        core,
        objective_numerator: OBJECTIVE_NUMERATOR,
        denominator: DENOMINATOR,
        clique_rows,
        min_active_slack: min_slack,
        conclusion,
    })
}

enum Row {
    Included {
        vertex: usize,
        weight: i128,
        numerator: i128,
    },
    Clique {
        vertices: Vec<usize>,
        rhs: i128,
        numerator: i128,
    },
}

fn rows() -> Result<Vec<Row>, G27GeometricFractionalError> {
    let blob = between("\"rows\":[", "],\"generator\"")?;
    let mut rows = Vec::new();
    for entry in blob.split("{\"kind\":\"").skip(1) {
        let (kind, rest) = entry
            .split_once('"')
            .ok_or(malformed_err("w607_v304_include_kind"))?;
        if kind == "included_vertex" {
            rows.push(Row::Included {
                vertex: number_after(rest, "\"vertex\":")? as usize - 1,
                weight: number_after(rest, "\"weight\":")?,
                numerator: number_after(rest, "\"numerator\":")?,
            });
        } else if kind == "edge" || kind == "triangle" {
            rows.push(Row::Clique {
                vertices: vertices_between(rest, "\"vertices\":[", "]")?,
                rhs: number_after(rest, "\"rhs\":")?,
                numerator: number_after(rest, "\"numerator\":")?,
            });
        } else {
            return malformed("w607_v304_include_row_kind");
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

fn neighbors(vertex: usize, adjacency: &[BitWords]) -> Vec<usize> {
    (0..EXPECTED_VERTEX_COUNT)
        .filter(|candidate| has_bit(&adjacency[vertex], *candidate))
        .collect()
}

fn is_clique(vertices: &[usize], adjacency: &[BitWords]) -> bool {
    if vertices.len() < 2 || vertices.len() > 3 {
        return false;
    }
    for left in 0..vertices.len() {
        for right in left + 1..vertices.len() {
            if !has_bit(&adjacency[vertices[left]], vertices[right]) {
                return false;
            }
        }
    }
    true
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
                .ok_or(malformed_err("w607_v304_include_vertex_parse"))
        })
        .collect()
}

fn number_after(text: &str, prefix: &str) -> Result<i128, G27GeometricFractionalError> {
    let rest = text
        .split_once(prefix)
        .map(|(_, rest)| rest)
        .ok_or(malformed_err("w607_v304_include_number_prefix"))?;
    let end = rest
        .find(|ch: char| !ch.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end]
        .parse()
        .map_err(|_| malformed_err("w607_v304_include_number"))
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
        .ok_or(malformed_err("w607_v304_include_between"))
}

fn require(pattern: &str, source: &'static str) -> Result<(), G27GeometricFractionalError> {
    if CERT.contains(pattern) {
        Ok(())
    } else {
        malformed(source)
    }
}

fn payload(
    clique_rows: usize,
    min_slack: i128,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.w607_v304_include_replay.v1"),
        HadwigerArtifactPayloadEntry::unsigned("clique_rows", clique_rows as u128),
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
