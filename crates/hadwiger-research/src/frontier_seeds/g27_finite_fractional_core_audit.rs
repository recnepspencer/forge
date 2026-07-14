use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::candidate_screening::ScreeningRational;
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_cross_ring_column_generation_replay::replay_g27_cross_ring_column_generation_state_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;

const W_CIRCLES_607_DATA: &str = include_str!("g27_finite_fractional/W_circles_607_integers.dat");
const SOURCE_URL: &str =
    "https://www.labri.fr/perso/pecher/pmwiki/pmwiki.php/Research/AvoidingDistance1";
const ARCHIVE_SHA256: &str =
    "sha256:c9a563a82f9e1a097329f72ab8b4baaa9104f5530990802ab2295f7afce09a09";
const DATA_SHA256: &str = "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad";
const EXPECTED_VERTEX_COUNT: usize = 607;
const EXPECTED_EDGE_COUNT: usize = 3_390;
const EXPECTED_INTEGER_WEIGHT_SUM: i128 = 1_999_983;
const WEIGHTED_INDEPENDENCE_UPPER_BOUND: i128 = 512_933;
const G27_LOWER_BOUND: i128 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27FiniteFractionalCoreAuditPosture {
    RetainedFiniteCoreNeedsWeightedIndependenceReplay,
}

impl G27FiniteFractionalCoreAuditPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RetainedFiniteCoreNeedsWeightedIndependenceReplay => {
                "retained_finite_core_needs_weighted_independence_replay"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WCircle607Replay {
    vertex_count: usize,
    edge_count: usize,
    weight_count: usize,
    integer_weight_sum: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27FiniteFractionalCoreAuditReport {
    core: HadwigerArtifactCore,
    source_url: String,
    archive_sha256: String,
    data_sha256: String,
    computed_data_sha256: String,
    vertex_count: usize,
    edge_count: usize,
    weight_count: usize,
    integer_weight_sum: i128,
    weighted_independence_upper_bound: i128,
    retained_lower_bound: ScreeningRational,
    lift_needed_to_beat_g27: ScreeningRational,
    posture: G27FiniteFractionalCoreAuditPosture,
    next_obligation: String,
}

impl G27FiniteFractionalCoreAuditReport {
    pub fn source_url(&self) -> &str {
        &self.source_url
    }

    pub fn archive_sha256(&self) -> &str {
        &self.archive_sha256
    }

    pub fn data_sha256(&self) -> &str {
        &self.data_sha256
    }

    pub fn computed_data_sha256(&self) -> &str {
        &self.computed_data_sha256
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn weight_count(&self) -> usize {
        self.weight_count
    }

    pub fn integer_weight_sum(&self) -> i128 {
        self.integer_weight_sum
    }

    pub fn weighted_independence_upper_bound(&self) -> i128 {
        self.weighted_independence_upper_bound
    }

    pub fn retained_lower_bound(&self) -> &ScreeningRational {
        &self.retained_lower_bound
    }

    pub fn lift_needed_to_beat_g27(&self) -> &ScreeningRational {
        &self.lift_needed_to_beat_g27
    }

    pub fn posture(&self) -> G27FiniteFractionalCoreAuditPosture {
        self.posture
    }

    pub fn next_obligation(&self) -> &str {
        &self.next_obligation
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27FiniteFractionalCoreAuditReport, core);

pub fn audit_g27_w_circles_607_finite_fractional_core_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27FiniteFractionalCoreAuditReport, G27GeometricFractionalError> {
    let retired_asymptotic_lead = replay_g27_cross_ring_column_generation_state_checked(handle)?;
    let replay = replay_w_circles_607_data()?;
    let computed_data_sha256 = sha256_token(W_CIRCLES_607_DATA.as_bytes());
    let retained_lower_bound =
        ScreeningRational::fraction(replay.integer_weight_sum, WEIGHTED_INDEPENDENCE_UPPER_BOUND)?;
    let g27_lower_bound = ScreeningRational::integer(G27_LOWER_BOUND);
    let lift_needed_to_beat_g27 = g27_lower_bound.sub(&retained_lower_bound);
    let posture =
        G27FiniteFractionalCoreAuditPosture::RetainedFiniteCoreNeedsWeightedIndependenceReplay;
    let next_obligation = format!(
        "retain weighted-independent-set certificate at <= {WEIGHTED_INDEPENDENCE_UPPER_BOUND} and price fused G27/W_circles columns above {}",
        lift_needed_to_beat_g27.stable_token()
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27FiniteFractionalCoreAuditReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_607_finite_fractional_core_audit".to_string(),
        },
        vec![retired_asymptotic_lead.reference()],
        payload(
            &replay,
            &computed_data_sha256,
            &retained_lower_bound,
            &lift_needed_to_beat_g27,
            posture,
            &next_obligation,
        ),
    )?;
    Ok(G27FiniteFractionalCoreAuditReport {
        core,
        source_url: SOURCE_URL.to_string(),
        archive_sha256: ARCHIVE_SHA256.to_string(),
        data_sha256: DATA_SHA256.to_string(),
        computed_data_sha256,
        vertex_count: replay.vertex_count,
        edge_count: replay.edge_count,
        weight_count: replay.weight_count,
        integer_weight_sum: replay.integer_weight_sum,
        weighted_independence_upper_bound: WEIGHTED_INDEPENDENCE_UPPER_BOUND,
        retained_lower_bound,
        lift_needed_to_beat_g27,
        posture,
        next_obligation,
    })
}

fn replay_w_circles_607_data() -> Result<WCircle607Replay, G27GeometricFractionalError> {
    let vertex_count = parse_assignment_usize("n")?;
    let edge_count = parse_assignment_usize("m")?;
    let edges = parse_edges(vertex_count)?;
    let weights = parse_weights()?;
    if vertex_count != EXPECTED_VERTEX_COUNT
        || edge_count != EXPECTED_EDGE_COUNT
        || edges.len() != EXPECTED_EDGE_COUNT
        || weights.len() != EXPECTED_VERTEX_COUNT
        || weights.iter().sum::<i128>() != EXPECTED_INTEGER_WEIGHT_SUM
        || sha256_token(W_CIRCLES_607_DATA.as_bytes()) != DATA_SHA256
    {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "w_circles_607_shape",
        });
    }
    Ok(WCircle607Replay {
        vertex_count,
        edge_count: edges.len(),
        weight_count: weights.len(),
        integer_weight_sum: weights.into_iter().sum(),
    })
}

fn parse_assignment_usize(name: &'static str) -> Result<usize, G27GeometricFractionalError> {
    let prefix = format!("{name} = ");
    W_CIRCLES_607_DATA
        .lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .and_then(|value| value.trim_end_matches(';').parse::<usize>().ok())
        .ok_or(G27GeometricFractionalError::MalformedData { source: name })
}

fn parse_edges(
    vertex_count: usize,
) -> Result<BTreeSet<(usize, usize)>, G27GeometricFractionalError> {
    let edge_blob = between("Edges = {", "};")?;
    let mut edges = BTreeSet::new();
    for entry in edge_blob.split('<').skip(1) {
        let pair = entry.split_once('>').map(|(pair, _)| pair).ok_or(
            G27GeometricFractionalError::MalformedData {
                source: "w_circles_607_edge",
            },
        )?;
        let (left, right) =
            pair.split_once(',')
                .ok_or(G27GeometricFractionalError::MalformedData {
                    source: "w_circles_607_edge_pair",
                })?;
        let left = left.trim().parse::<usize>().map_err(|_| {
            G27GeometricFractionalError::MalformedData {
                source: "w_circles_607_edge_left",
            }
        })?;
        let right = right.trim().parse::<usize>().map_err(|_| {
            G27GeometricFractionalError::MalformedData {
                source: "w_circles_607_edge_right",
            }
        })?;
        if left == right || left == 0 || right == 0 || left > vertex_count || right > vertex_count {
            return Err(G27GeometricFractionalError::MalformedData {
                source: "w_circles_607_edge_endpoint",
            });
        }
        let edge = if left < right {
            (left, right)
        } else {
            (right, left)
        };
        if !edges.insert(edge) {
            return Err(G27GeometricFractionalError::MalformedData {
                source: "w_circles_607_duplicate_edge",
            });
        }
    }
    Ok(edges)
}

fn parse_weights() -> Result<Vec<i128>, G27GeometricFractionalError> {
    between("w = [", "];")?
        .split(',')
        .filter(|value| !value.trim().is_empty())
        .map(parse_integer_weight)
        .collect()
}

fn parse_integer_weight(value: &str) -> Result<i128, G27GeometricFractionalError> {
    let (integer, fraction) =
        value
            .trim()
            .split_once('.')
            .ok_or(G27GeometricFractionalError::MalformedData {
                source: "w_circles_607_weight",
            })?;
    if !fraction.chars().all(|ch| ch == '0') {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "w_circles_607_integer_weight",
        });
    }
    integer
        .parse::<i128>()
        .map_err(|_| G27GeometricFractionalError::MalformedData {
            source: "w_circles_607_weight_parse",
        })
}

fn between(start: &str, end: &str) -> Result<&'static str, G27GeometricFractionalError> {
    let after_start = W_CIRCLES_607_DATA
        .split_once(start)
        .map(|(_, rest)| rest)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "w_circles_607_section_start",
        })?;
    after_start.split_once(end).map(|(body, _)| body).ok_or(
        G27GeometricFractionalError::MalformedData {
            source: "w_circles_607_section_end",
        },
    )
}

fn payload(
    replay: &WCircle607Replay,
    computed_data_sha256: &str,
    lower_bound: &ScreeningRational,
    lift: &ScreeningRational,
    posture: G27FiniteFractionalCoreAuditPosture,
    next_obligation: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_finite_fractional_core_audit.v1",
        ),
        HadwigerArtifactPayloadEntry::text("source_url", SOURCE_URL),
        HadwigerArtifactPayloadEntry::text("source_archive_sha256", ARCHIVE_SHA256),
        HadwigerArtifactPayloadEntry::text("source_data_sha256", DATA_SHA256),
        HadwigerArtifactPayloadEntry::text("computed_data_sha256", computed_data_sha256),
        HadwigerArtifactPayloadEntry::unsigned("vertex_count", replay.vertex_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("edge_count", replay.edge_count as u128),
        HadwigerArtifactPayloadEntry::unsigned("weight_count", replay.weight_count as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "integer_weight_sum",
            replay.integer_weight_sum as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "weighted_independence_upper_bound",
            WEIGHTED_INDEPENDENCE_UPPER_BOUND as u128,
        ),
        HadwigerArtifactPayloadEntry::text("retained_lower_bound", lower_bound.stable_token()),
        HadwigerArtifactPayloadEntry::text("lift_needed_to_beat_g27", lift.stable_token()),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("next_obligation", next_obligation),
    ]
}

fn sha256_token(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("sha256:{:x}", hasher.finalize())
}
