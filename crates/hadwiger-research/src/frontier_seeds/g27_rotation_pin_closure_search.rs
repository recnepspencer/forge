use std::collections::{BTreeMap, BTreeSet};

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_exact_moser_basis::audit_g27_exact_moser_basis_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;

const G27_FLOAT_COORDS: &str = include_str!("g27_geometric_fractional/G_27.txt");
const PRESSURE_FRAGMENT: [usize; 11] = [1, 3, 6, 8, 10, 13, 15, 18, 19, 20, 23];
const HINGE_VERTEX: usize = 8;
const RETAINED_CANDIDATE_LIMIT: usize = 12;
const EPSILON: f64 = 1.0e-7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27RotationPinClosurePosture {
    FloatScreenedNeedsExactReplay,
}

impl G27RotationPinClosurePosture {
    pub fn as_str(self) -> &'static str {
        "float_screened_needs_exact_replay"
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct G27RotationPinClosureCandidate {
    witness_vertex: String,
    pin_vertex: String,
    theta_milliradians: i64,
    theta_millidegrees: i64,
    closure_pairs: Vec<(String, String)>,
    special_angle_suppressed: bool,
}

impl G27RotationPinClosureCandidate {
    pub fn witness_vertex(&self) -> &str {
        &self.witness_vertex
    }

    pub fn pin_vertex(&self) -> &str {
        &self.pin_vertex
    }

    pub fn theta_millidegrees(&self) -> i64 {
        self.theta_millidegrees
    }

    pub fn closure_pairs(&self) -> &[(String, String)] {
        &self.closure_pairs
    }

    pub fn special_angle_suppressed(&self) -> bool {
        self.special_angle_suppressed
    }

    fn stable_token(&self) -> String {
        let pairs = self
            .closure_pairs
            .iter()
            .map(|(left, right)| format!("{left}-{right}"))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{}:{}:{}:{}:{}:[{}]",
            self.witness_vertex,
            self.pin_vertex,
            self.theta_milliradians,
            self.theta_millidegrees,
            self.special_angle_suppressed,
            pairs
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct G27RotationPinClosureSearchReport {
    core: HadwigerArtifactCore,
    total_candidate_count: usize,
    retained_candidates: Vec<G27RotationPinClosureCandidate>,
    posture: G27RotationPinClosurePosture,
    next_replay: String,
}

impl G27RotationPinClosureSearchReport {
    pub fn total_candidate_count(&self) -> usize {
        self.total_candidate_count
    }

    pub fn retained_candidates(&self) -> &[G27RotationPinClosureCandidate] {
        &self.retained_candidates
    }

    pub fn posture(&self) -> G27RotationPinClosurePosture {
        self.posture
    }

    pub fn next_replay(&self) -> &str {
        &self.next_replay
    }

    pub fn best_unsuppressed_candidate(&self) -> Option<&G27RotationPinClosureCandidate> {
        self.retained_candidates
            .iter()
            .find(|candidate| !candidate.special_angle_suppressed())
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27RotationPinClosureSearchReport, core);

pub fn search_g27_rotation_pin_closures_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27RotationPinClosureSearchReport, G27GeometricFractionalError> {
    let basis_audit = audit_g27_exact_moser_basis_checked(handle)?;
    let points = parse_points()?;
    let mut candidates = enumerate_candidates(&points);
    let total_candidate_count = candidates.len();
    candidates.sort_by(|left, right| {
        left.special_angle_suppressed
            .cmp(&right.special_angle_suppressed)
            .then_with(|| right.closure_pairs.len().cmp(&left.closure_pairs.len()))
            .then_with(|| {
                left.theta_millidegrees
                    .abs()
                    .cmp(&right.theta_millidegrees.abs())
            })
            .then_with(|| left.stable_token().cmp(&right.stable_token()))
    });
    candidates.truncate(RETAINED_CANDIDATE_LIMIT);
    let posture = G27RotationPinClosurePosture::FloatScreenedNeedsExactReplay;
    let next_replay = "derive exact algebraic angle for best unsuppressed closure candidate and replay rotated unit distances"
        .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27RotationPinClosureSearchReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_rotation_pin_closure_search".to_string(),
        },
        vec![basis_audit.reference()],
        payload(total_candidate_count, &candidates, posture, &next_replay),
    )?;
    Ok(G27RotationPinClosureSearchReport {
        core,
        total_candidate_count,
        retained_candidates: candidates,
        posture,
        next_replay,
    })
}

fn enumerate_candidates(points: &[(f64, f64)]) -> Vec<G27RotationPinClosureCandidate> {
    let hinge = points[HINGE_VERTEX - 1];
    let fragment = PRESSURE_FRAGMENT.into_iter().collect::<BTreeSet<_>>();
    let mut unique = BTreeMap::new();
    for witness in PRESSURE_FRAGMENT {
        if witness == HINGE_VERTEX {
            continue;
        }
        let u = subtract(points[witness - 1], hinge);
        let u_len = norm(u);
        for pin in 1..=points.len() {
            if fragment.contains(&pin) {
                continue;
            }
            let v = subtract(points[pin - 1], hinge);
            let v_len = norm(v);
            if u_len == 0.0 || v_len == 0.0 {
                continue;
            }
            let target = (u_len * u_len + v_len * v_len - 1.0) / (2.0 * u_len * v_len);
            if !(-1.0 - EPSILON..=1.0 + EPSILON).contains(&target) {
                continue;
            }
            let target = target.clamp(-1.0, 1.0);
            let base = v.1.atan2(v.0) - u.1.atan2(u.0);
            for theta in [base + target.acos(), base - target.acos()] {
                let theta = normalize_angle(theta);
                if theta.abs() < EPSILON {
                    continue;
                }
                let closure_pairs = closure_pairs(points, hinge, theta, &fragment);
                if closure_pairs.is_empty() {
                    continue;
                }
                let candidate = candidate(witness, pin, theta, closure_pairs);
                unique.insert(candidate.stable_token(), candidate);
            }
        }
    }
    unique.into_values().collect()
}

fn closure_pairs(
    points: &[(f64, f64)],
    hinge: (f64, f64),
    theta: f64,
    fragment: &BTreeSet<usize>,
) -> Vec<(String, String)> {
    let mut pairs = BTreeSet::new();
    for moving in PRESSURE_FRAGMENT {
        if moving == HINGE_VERTEX {
            continue;
        }
        let rotated = rotate(points[moving - 1], hinge, theta);
        for static_vertex in 1..=points.len() {
            if fragment.contains(&static_vertex) {
                continue;
            }
            if (squared_distance(rotated, points[static_vertex - 1]) - 1.0).abs() < EPSILON {
                pairs.insert((moving.to_string(), static_vertex.to_string()));
            }
        }
    }
    pairs.into_iter().collect()
}

fn candidate(
    witness: usize,
    pin: usize,
    theta: f64,
    closure_pairs: Vec<(String, String)>,
) -> G27RotationPinClosureCandidate {
    G27RotationPinClosureCandidate {
        witness_vertex: witness.to_string(),
        pin_vertex: pin.to_string(),
        theta_milliradians: (theta * 1000.0).round() as i64,
        theta_millidegrees: (theta.to_degrees() * 1000.0).round() as i64,
        closure_pairs,
        special_angle_suppressed: is_special_angle(theta),
    }
}

fn is_special_angle(theta: f64) -> bool {
    let degrees = theta.to_degrees().abs();
    [45.0, 60.0, 90.0, 120.0, 180.0]
        .iter()
        .any(|special| (degrees - special).abs() < 1.0e-4)
}

fn parse_points() -> Result<Vec<(f64, f64)>, G27GeometricFractionalError> {
    let points = G27_FLOAT_COORDS
        .lines()
        .map(|line| {
            let values =
                line.split_whitespace()
                    .map(|value| {
                        value.parse::<f64>().map_err(|_| {
                            G27GeometricFractionalError::MalformedData { source: "G_27.txt" }
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
            match values.as_slice() {
                [x, y] => Ok((*x, *y)),
                _ => Err(G27GeometricFractionalError::MalformedData { source: "G_27.txt" }),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    if points.len() == 27 {
        Ok(points)
    } else {
        Err(G27GeometricFractionalError::MalformedData { source: "G_27.txt" })
    }
}

fn rotate(point: (f64, f64), hinge: (f64, f64), theta: f64) -> (f64, f64) {
    let (x, y) = subtract(point, hinge);
    (
        theta.cos() * x - theta.sin() * y + hinge.0,
        theta.sin() * x + theta.cos() * y + hinge.1,
    )
}

fn subtract(left: (f64, f64), right: (f64, f64)) -> (f64, f64) {
    (left.0 - right.0, left.1 - right.1)
}

fn norm(point: (f64, f64)) -> f64 {
    (point.0 * point.0 + point.1 * point.1).sqrt()
}

fn squared_distance(left: (f64, f64), right: (f64, f64)) -> f64 {
    let diff = subtract(left, right);
    diff.0 * diff.0 + diff.1 * diff.1
}

fn normalize_angle(theta: f64) -> f64 {
    let two_pi = 2.0 * std::f64::consts::PI;
    (theta + std::f64::consts::PI).rem_euclid(two_pi) - std::f64::consts::PI
}

fn payload(
    total_candidate_count: usize,
    candidates: &[G27RotationPinClosureCandidate],
    posture: G27RotationPinClosurePosture,
    next_replay: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_rotation_pin.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "total_candidate_count",
            total_candidate_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("next_replay", next_replay),
    ];
    for candidate in candidates {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "candidate",
            candidate.stable_token(),
        ));
    }
    payload
}
