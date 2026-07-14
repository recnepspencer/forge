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
use super::g27_w_circles_exact_geometry_support::{
    parse_w_integer_weights, parse_w_retained_edges, EXPECTED_EDGE_COUNT, EXPECTED_VERTEX_COUNT,
};

const W607_DIGEST: &str = "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad";
const W607_TARGET: i128 = 512_933;
const TOY_DIGEST: &str = "toy:path3_weighted_v1";
const W607_ILP_COVER_JSON: &str = include_str!("../../docs/w607-weighted-clique-cover-ilp.json");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeightedCliqueCoverLeafV1 {
    graph_digest: String,
    candidate_vertices: Vec<usize>,
    objective_bound: i128,
    cliques: Vec<WeightedCliqueCoverRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeightedCliqueCoverRow {
    vertices: Vec<usize>,
    capacity: i128,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MwisUpperBoundCertificateReplayStatus {
    ReplayedBelowTarget,
    ReplayedWeakRootCover,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MwisUpperBoundCertificateReplayCase {
    name: String,
    graph_digest: String,
    vertex_count: usize,
    edge_count: usize,
    clique_count: usize,
    objective_bound: i128,
    target_weight: i128,
    excess_over_target: i128,
    status: MwisUpperBoundCertificateReplayStatus,
}

impl MwisUpperBoundCertificateReplayCase {
    pub fn summary(&self) -> (&str, usize, usize, usize, i128, i128, i128) {
        (
            &self.name,
            self.vertex_count,
            self.edge_count,
            self.clique_count,
            self.objective_bound,
            self.target_weight,
            self.excess_over_target,
        )
    }

    pub fn graph_digest(&self) -> &str {
        &self.graph_digest
    }

    pub fn status(&self) -> MwisUpperBoundCertificateReplayStatus {
        self.status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MwisUpperBoundCertificateReplayReport {
    core: HadwigerArtifactCore,
    schema_name: String,
    cases: Vec<MwisUpperBoundCertificateReplayCase>,
    conclusion: String,
}

impl MwisUpperBoundCertificateReplayReport {
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    pub fn cases(&self) -> &[MwisUpperBoundCertificateReplayCase] {
        &self.cases
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        self.cases.iter().any(|case| case.name == "w607_root_cover")
            && self.cases.iter().all(|case| {
                case.status == MwisUpperBoundCertificateReplayStatus::ReplayedBelowTarget
            })
    }
}

impl_hadwiger_artifact!(MwisUpperBoundCertificateReplayReport, core);

pub fn replay_mwis_upper_bound_certificate_fixtures_checked(
    handle: &HadwigerResearchHandle,
) -> Result<MwisUpperBoundCertificateReplayReport, G27GeometricFractionalError> {
    let source = audit_g27_w_circles_607_finite_fractional_core_checked(handle)?;
    let toy_case = replay_toy_case()?;
    let root_case = replay_w607_root_cover_case()?;
    let ilp_case = replay_w607_ilp_cover_case()?;
    let conclusion = "weighted clique-cover leaf replay now accepts an external W607 ILP cover artifact at 666661, materially improving the greedy 959119 cover but still above the 512933 target; future progress needs branch or rational-dual artifacts".to_string();
    let core = artifact_core(
        HadwigerArtifactKind::MwisUpperBoundCertificateReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "mwis_upper_bound_certificate_replay_fixtures".to_string(),
        },
        vec![source.reference()],
        payload(&toy_case, &root_case, &ilp_case, &conclusion),
    )?;
    Ok(MwisUpperBoundCertificateReplayReport {
        core,
        schema_name: "forge.hadwiger.mwis_upper_bound_certificate.v1".to_string(),
        cases: vec![toy_case, root_case, ilp_case],
        conclusion,
    })
}

fn replay_toy_case() -> Result<MwisUpperBoundCertificateReplayCase, G27GeometricFractionalError> {
    let weights = vec![2, 3, 2];
    let edges = BTreeSet::from([(1, 2), (2, 3)]);
    let certificate = WeightedCliqueCoverLeafV1 {
        graph_digest: TOY_DIGEST.to_string(),
        candidate_vertices: vec![1, 2, 3],
        objective_bound: 4,
        cliques: vec![
            WeightedCliqueCoverRow {
                vertices: vec![1, 2],
                capacity: 2,
            },
            WeightedCliqueCoverRow {
                vertices: vec![2, 3],
                capacity: 2,
            },
        ],
    };
    replay_leaf(&certificate, TOY_DIGEST, &weights, &edges)?;
    Ok(case("toy_path3_cover", TOY_DIGEST, 3, 2, 2, 4, 4))
}

fn replay_w607_root_cover_case(
) -> Result<MwisUpperBoundCertificateReplayCase, G27GeometricFractionalError> {
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    if weights.len() != EXPECTED_VERTEX_COUNT || edges.len() != EXPECTED_EDGE_COUNT {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "w607_certificate_fixture_shape",
        });
    }
    let candidates = (1..=EXPECTED_VERTEX_COUNT).collect::<Vec<_>>();
    let certificate = greedy_weighted_clique_cover_leaf(W607_DIGEST, &weights, &edges, &candidates);
    replay_leaf(&certificate, W607_DIGEST, &weights, &edges)?;
    Ok(case(
        "w607_root_cover",
        W607_DIGEST,
        EXPECTED_VERTEX_COUNT,
        EXPECTED_EDGE_COUNT,
        certificate.cliques.len(),
        certificate.objective_bound,
        W607_TARGET,
    ))
}

fn replay_w607_ilp_cover_case(
) -> Result<MwisUpperBoundCertificateReplayCase, G27GeometricFractionalError> {
    let weights = parse_w_integer_weights()?;
    let edges = parse_w_retained_edges(EXPECTED_VERTEX_COUNT)?;
    let certificate = parse_w607_ilp_cover_json()?;
    replay_leaf(&certificate, W607_DIGEST, &weights, &edges)?;
    Ok(case(
        "w607_ilp_cover",
        W607_DIGEST,
        EXPECTED_VERTEX_COUNT,
        EXPECTED_EDGE_COUNT,
        certificate.cliques.len(),
        certificate.objective_bound,
        W607_TARGET,
    ))
}

fn replay_leaf(
    certificate: &WeightedCliqueCoverLeafV1,
    expected_digest: &str,
    weights: &[i128],
    edges: &BTreeSet<(usize, usize)>,
) -> Result<(), G27GeometricFractionalError> {
    if certificate.graph_digest != expected_digest {
        return malformed("mwis_certificate_digest");
    }
    let mut coverage = vec![0; weights.len()];
    let mut objective = 0;
    for row in &certificate.cliques {
        if row.capacity < 0 || !is_clique(&row.vertices, weights.len(), edges) {
            return malformed("mwis_certificate_clique_row");
        }
        objective += row.capacity;
        for vertex in &row.vertices {
            coverage[*vertex - 1] += row.capacity;
        }
    }
    let mut candidates = certificate.candidate_vertices.clone();
    candidates.sort_unstable();
    candidates.dedup();
    if candidates != certificate.candidate_vertices {
        return malformed("mwis_certificate_candidates");
    }
    for vertex in &certificate.candidate_vertices {
        if *vertex == 0 || *vertex > weights.len() || coverage[*vertex - 1] < weights[*vertex - 1] {
            return malformed("mwis_certificate_coverage");
        }
    }
    if objective != certificate.objective_bound {
        return malformed("mwis_certificate_objective");
    }
    Ok(())
}

fn parse_w607_ilp_cover_json() -> Result<WeightedCliqueCoverLeafV1, G27GeometricFractionalError> {
    if !W607_ILP_COVER_JSON.contains(&format!("\"graph_digest\":\"{W607_DIGEST}\"")) {
        return malformed("w607_ilp_cover_digest");
    }
    let objective_bound = parse_json_i128_after("\"objective_bound\":")?;
    let rows_blob = between_json("\"rows\":[", "],\"solver\"")?;
    let mut cliques = Vec::new();
    for entry in rows_blob.split("{\"vertices\":[").skip(1) {
        let (vertices_blob, rest) = entry.split_once("],\"capacity\":").ok_or(
            G27GeometricFractionalError::MalformedData {
                source: "w607_ilp_cover_row_vertices",
            },
        )?;
        let capacity_end = rest
            .find('}')
            .ok_or(G27GeometricFractionalError::MalformedData {
                source: "w607_ilp_cover_row_capacity",
            })?;
        let vertices = vertices_blob
            .split(',')
            .filter(|value| !value.is_empty())
            .map(|value| parse_usize(value, "w607_ilp_cover_vertex"))
            .collect::<Result<Vec<_>, _>>()?;
        let capacity = rest[..capacity_end].parse::<i128>().map_err(|_| {
            G27GeometricFractionalError::MalformedData {
                source: "w607_ilp_cover_capacity",
            }
        })?;
        cliques.push(WeightedCliqueCoverRow { vertices, capacity });
    }
    Ok(WeightedCliqueCoverLeafV1 {
        graph_digest: W607_DIGEST.to_string(),
        candidate_vertices: (1..=EXPECTED_VERTEX_COUNT).collect(),
        objective_bound,
        cliques,
    })
}

fn greedy_weighted_clique_cover_leaf(
    digest: &str,
    weights: &[i128],
    edges: &BTreeSet<(usize, usize)>,
    candidates: &[usize],
) -> WeightedCliqueCoverLeafV1 {
    let mut ordered = candidates.to_vec();
    ordered.sort_by(|left, right| {
        weights[*right - 1]
            .cmp(&weights[*left - 1])
            .then_with(|| left.cmp(right))
    });
    let mut cliques: Vec<WeightedCliqueCoverRow> = Vec::new();
    for vertex in ordered {
        let mut assigned = false;
        for row in &mut cliques {
            if row
                .vertices
                .iter()
                .all(|other| has_edge(vertex, *other, edges))
            {
                row.vertices.push(vertex);
                row.capacity = row.capacity.max(weights[vertex - 1]);
                assigned = true;
                break;
            }
        }
        if !assigned {
            cliques.push(WeightedCliqueCoverRow {
                vertices: vec![vertex],
                capacity: weights[vertex - 1],
            });
        }
    }
    let objective_bound = cliques.iter().map(|row| row.capacity).sum();
    for row in &mut cliques {
        row.vertices.sort_unstable();
    }
    WeightedCliqueCoverLeafV1 {
        graph_digest: digest.to_string(),
        candidate_vertices: candidates.to_vec(),
        objective_bound,
        cliques,
    }
}

fn is_clique(vertices: &[usize], vertex_count: usize, edges: &BTreeSet<(usize, usize)>) -> bool {
    let mut sorted = vertices.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    sorted == vertices
        && vertices
            .iter()
            .all(|vertex| *vertex > 0 && *vertex <= vertex_count)
        && vertices.iter().enumerate().all(|(index, left)| {
            vertices[(index + 1)..]
                .iter()
                .all(|right| has_edge(*left, *right, edges))
        })
}

fn has_edge(left: usize, right: usize, edges: &BTreeSet<(usize, usize)>) -> bool {
    edges.contains(&(left.min(right), left.max(right)))
}

fn case(
    name: &str,
    digest: &str,
    vertex_count: usize,
    edge_count: usize,
    clique_count: usize,
    objective_bound: i128,
    target_weight: i128,
) -> MwisUpperBoundCertificateReplayCase {
    MwisUpperBoundCertificateReplayCase {
        name: name.to_string(),
        graph_digest: digest.to_string(),
        vertex_count,
        edge_count,
        clique_count,
        objective_bound,
        target_weight,
        excess_over_target: (objective_bound - target_weight).max(0),
        status: if objective_bound <= target_weight {
            MwisUpperBoundCertificateReplayStatus::ReplayedBelowTarget
        } else {
            MwisUpperBoundCertificateReplayStatus::ReplayedWeakRootCover
        },
    }
}

#[rustfmt::skip]
fn payload(toy_case: &MwisUpperBoundCertificateReplayCase, root_case: &MwisUpperBoundCertificateReplayCase, ilp_case: &MwisUpperBoundCertificateReplayCase, conclusion: &str) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.mwis_upper_bound_certificate_replay.v1"),
        HadwigerArtifactPayloadEntry::unsigned("toy_objective_bound", toy_case.objective_bound as u128),
        HadwigerArtifactPayloadEntry::unsigned("w607_root_clique_count", root_case.clique_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("w607_root_objective_bound", root_case.objective_bound as u128),
        HadwigerArtifactPayloadEntry::unsigned("w607_ilp_clique_count", ilp_case.clique_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("w607_ilp_objective_bound", ilp_case.objective_bound as u128),
        HadwigerArtifactPayloadEntry::unsigned("w607_target_weight", W607_TARGET as u128),
        HadwigerArtifactPayloadEntry::unsigned("w607_ilp_excess_over_target", ilp_case.excess_over_target as u128),
        HadwigerArtifactPayloadEntry::unsigned("w607_ilp_improvement_over_root", (root_case.objective_bound - ilp_case.objective_bound) as u128),
        HadwigerArtifactPayloadEntry::text("w607_ilp_status", status_token(ilp_case.status)),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

fn status_token(status: MwisUpperBoundCertificateReplayStatus) -> &'static str {
    match status {
        MwisUpperBoundCertificateReplayStatus::ReplayedBelowTarget => "replayed_below_target",
        MwisUpperBoundCertificateReplayStatus::ReplayedWeakRootCover => "replayed_weak_root_cover",
    }
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(G27GeometricFractionalError::MalformedData { source })
}

#[rustfmt::skip]
fn parse_json_i128_after(prefix: &str) -> Result<i128, G27GeometricFractionalError> {
    let after = W607_ILP_COVER_JSON
        .split_once(prefix)
        .map(|(_, rest)| rest)
        .ok_or(G27GeometricFractionalError::MalformedData { source: "w607_ilp_cover_objective_prefix" })?;
    let end = after
        .find(',')
        .ok_or(G27GeometricFractionalError::MalformedData { source: "w607_ilp_cover_objective_end" })?;
    after[..end]
        .parse::<i128>()
        .map_err(|_| G27GeometricFractionalError::MalformedData { source: "w607_ilp_cover_objective" })
}

#[rustfmt::skip]
fn between_json(start: &str, end: &str) -> Result<&'static str, G27GeometricFractionalError> {
    let after = W607_ILP_COVER_JSON
        .split_once(start)
        .map(|(_, rest)| rest)
        .ok_or(G27GeometricFractionalError::MalformedData { source: "w607_ilp_cover_rows_start" })?;
    after
        .split_once(end)
        .map(|(body, _)| body)
        .ok_or(G27GeometricFractionalError::MalformedData { source: "w607_ilp_cover_rows_end" })
}

fn parse_usize(value: &str, source: &'static str) -> Result<usize, G27GeometricFractionalError> {
    value
        .parse::<usize>()
        .map_err(|_| G27GeometricFractionalError::MalformedData { source })
}
