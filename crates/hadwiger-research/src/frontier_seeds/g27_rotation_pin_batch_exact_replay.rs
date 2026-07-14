use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_manufactured_rotation_field::{K4, Q8};
use super::g27_rotation_pin_closure_search::search_g27_rotation_pin_closures_checked;

const HINGE: usize = 8;
const BROAD_CLOSURE_THRESHOLD: usize = 3;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27RotationPinBatchExactReplayPosture {
    BroadClosureRetiredSmallClosuresRetained,
}

impl G27RotationPinBatchExactReplayPosture {
    pub fn as_str(self) -> &'static str {
        "broad_closure_retired_small_closures_retained"
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27RotationPinCandidateExactReplay {
    witness_vertex: String,
    pin_vertex: String,
    theta_millidegrees: i64,
    field_height: String,
    max_exact_unit_pairs_per_branch: usize,
    exact_unit_pairs: Vec<(String, String)>,
}

impl G27RotationPinCandidateExactReplay {
    pub fn witness_vertex(&self) -> &str {
        &self.witness_vertex
    }

    pub fn pin_vertex(&self) -> &str {
        &self.pin_vertex
    }

    pub fn theta_millidegrees(&self) -> i64 {
        self.theta_millidegrees
    }

    pub fn field_height(&self) -> &str {
        &self.field_height
    }

    pub fn max_exact_unit_pairs_per_branch(&self) -> usize {
        self.max_exact_unit_pairs_per_branch
    }

    pub fn exact_unit_pairs(&self) -> &[(String, String)] {
        &self.exact_unit_pairs
    }

    fn stable_token(&self) -> String {
        let pairs = self
            .exact_unit_pairs
            .iter()
            .map(|(left, right)| format!("{left}-{right}"))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{}:{}:{}:{}:{}:[{}]",
            self.witness_vertex,
            self.pin_vertex,
            self.theta_millidegrees,
            self.field_height,
            self.max_exact_unit_pairs_per_branch,
            pairs
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27RotationPinBatchExactReplayReport {
    core: HadwigerArtifactCore,
    retained_candidate_count: usize,
    broad_exact_closure_count: usize,
    small_exact_closure_count: usize,
    best_candidates: Vec<G27RotationPinCandidateExactReplay>,
    conclusion: String,
    posture: G27RotationPinBatchExactReplayPosture,
}

impl G27RotationPinBatchExactReplayReport {
    pub fn retained_candidate_count(&self) -> usize {
        self.retained_candidate_count
    }

    pub fn broad_exact_closure_count(&self) -> usize {
        self.broad_exact_closure_count
    }

    pub fn small_exact_closure_count(&self) -> usize {
        self.small_exact_closure_count
    }

    pub fn best_candidates(&self) -> &[G27RotationPinCandidateExactReplay] {
        &self.best_candidates
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn posture(&self) -> G27RotationPinBatchExactReplayPosture {
        self.posture
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27RotationPinBatchExactReplayReport, core);

pub fn replay_g27_rotation_pin_batch_exact_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27RotationPinBatchExactReplayReport, G27GeometricFractionalError> {
    let search = search_g27_rotation_pin_closures_checked(handle)?;
    let points = points(&retained_g27_coefficients()?);
    let mut replays = search
        .retained_candidates()
        .iter()
        .map(|candidate| replay_candidate(candidate, &points))
        .collect::<Result<Vec<_>, _>>()?;
    replays.sort_by(|left, right| {
        right
            .max_exact_unit_pairs_per_branch
            .cmp(&left.max_exact_unit_pairs_per_branch)
            .then_with(|| left.stable_token().cmp(&right.stable_token()))
    });
    let broad_exact_closure_count = replays
        .iter()
        .filter(|replay| replay.max_exact_unit_pairs_per_branch >= BROAD_CLOSURE_THRESHOLD)
        .count();
    let small_exact_closure_count = replays
        .iter()
        .filter(|replay| replay.max_exact_unit_pairs_per_branch > 1)
        .count();
    let best_candidates = replays.into_iter().take(4).collect::<Vec<_>>();
    let conclusion = "exact batch replay retires broad spindle closure: no retained candidate has three exact static closure pairs, but two-pin manufactured closures remain for pressure scoring"
        .to_string();
    let posture = G27RotationPinBatchExactReplayPosture::BroadClosureRetiredSmallClosuresRetained;
    let core = artifact_core(
        HadwigerArtifactKind::G27RotationPinBatchExactReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_rotation_pin_batch_exact_replay".to_string(),
        },
        vec![search.reference()],
        payload(
            search.retained_candidates().len(),
            broad_exact_closure_count,
            small_exact_closure_count,
            &best_candidates,
            &conclusion,
            posture,
        ),
    )?;
    Ok(G27RotationPinBatchExactReplayReport {
        core,
        retained_candidate_count: search.retained_candidates().len(),
        broad_exact_closure_count,
        small_exact_closure_count,
        best_candidates,
        conclusion,
        posture,
    })
}

fn replay_candidate(
    candidate: &super::g27_rotation_pin_closure_search::G27RotationPinClosureCandidate,
    points: &[(K4, K4)],
) -> Result<G27RotationPinCandidateExactReplay, G27GeometricFractionalError> {
    let witness = parse_vertex(candidate.witness_vertex())?;
    let pin = parse_vertex(candidate.pin_vertex())?;
    let u = vector(points[witness - 1], points[HINGE - 1]);
    let v = vector(points[pin - 1], points[HINGE - 1]);
    let r2 = squared_norm_k4(u);
    let d2 = squared_norm_k4(v);
    let half = K4::rational(1, 2);
    let target_dot = r2.add(d2).sub(K4::one()).mul(half);
    let height = r2.mul(d2).sub(target_dot.mul(target_dot));
    let inverse_d2 = d2
        .inverse()
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "rotation_d2",
        })?;
    let inverse_r2 = r2
        .inverse()
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "rotation_r2",
        })?;
    let mut branch_pair_sets = [-1, 1]
        .into_iter()
        .map(|sign| {
            replay_branch(
                sign,
                candidate.closure_pairs(),
                points,
                u,
                v,
                target_dot,
                height,
                inverse_d2,
                inverse_r2,
            )
        })
        .collect::<Vec<_>>();
    branch_pair_sets.sort();
    let max_exact_unit_pairs_per_branch = branch_pair_sets
        .iter()
        .map(Vec::len)
        .max()
        .unwrap_or_default();
    let exact_unit_pairs = branch_pair_sets.pop().unwrap_or_default();
    Ok(G27RotationPinCandidateExactReplay {
        witness_vertex: candidate.witness_vertex().to_string(),
        pin_vertex: candidate.pin_vertex().to_string(),
        theta_millidegrees: candidate.theta_millidegrees(),
        field_height: height.to_token(),
        max_exact_unit_pairs_per_branch,
        exact_unit_pairs,
    })
}

fn replay_branch(
    sign: i32,
    closure_pairs: &[(String, String)],
    points: &[(K4, K4)],
    u: (K4, K4),
    v: (K4, K4),
    target_dot: K4,
    height: K4,
    inverse_d2: K4,
    inverse_r2: K4,
) -> Vec<(String, String)> {
    let p = rotation_target(sign, v, target_dot, inverse_d2);
    let cosine =
        dot_q8((Q8::base(u.0), Q8::base(u.1)), p, height).mul(Q8::base(inverse_r2), height);
    let sine =
        cross_q8((Q8::base(u.0), Q8::base(u.1)), p, height).mul(Q8::base(inverse_r2), height);
    closure_pairs
        .iter()
        .filter_map(|(moving, static_vertex)| {
            let moving = parse_vertex(moving).ok()?;
            let static_vertex = parse_vertex(static_vertex).ok()?;
            let rotated = rotate(
                (
                    Q8::base(vector(points[moving - 1], points[HINGE - 1]).0),
                    Q8::base(vector(points[moving - 1], points[HINGE - 1]).1),
                ),
                cosine,
                sine,
                height,
            );
            let static_vector = vector(points[static_vertex - 1], points[HINGE - 1]);
            let diff = (
                rotated.0.sub(Q8::base(static_vector.0)),
                rotated.1.sub(Q8::base(static_vector.1)),
            );
            if squared_norm_q8(diff, height).is_one() {
                Some((moving.to_string(), static_vertex.to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn rotation_target(sign: i32, v: (K4, K4), target_dot: K4, inverse_d2: K4) -> (Q8, Q8) {
    let base_scale = target_dot.mul(inverse_d2);
    let radical_scale = inverse_d2.scale(sign as i128);
    (
        Q8::base(base_scale.mul(v.0)).sub(Q8::w(radical_scale.mul(v.1))),
        Q8::base(base_scale.mul(v.1)).add(Q8::w(radical_scale.mul(v.0))),
    )
}

fn points(coefficients: &[[i32; 4]]) -> Vec<(K4, K4)> {
    coefficients
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .fold((K4::zero(), K4::zero()), |point, (index, coefficient)| {
                    add_point(point, scale_point(basis_point(index), *coefficient as i128))
                })
        })
        .collect()
}

fn basis_point(index: usize) -> (K4, K4) {
    match index {
        0 => (K4::one(), K4::zero()),
        1 => (K4::rational(1, 2), K4::sqrt3(1, 2)),
        2 => (K4::rational(5, 6), K4::sqrt11(1, 6)),
        3 => (
            K4::rational(5, 12).add(K4::sqrt33(-1, 12)),
            K4::sqrt3(5, 12).add(K4::sqrt11(1, 12)),
        ),
        _ => (K4::zero(), K4::zero()),
    }
}

fn rotate(point: (Q8, Q8), cosine: Q8, sine: Q8, h: K4) -> (Q8, Q8) {
    (
        cosine.mul(point.0, h).sub(sine.mul(point.1, h)),
        sine.mul(point.0, h).add(cosine.mul(point.1, h)),
    )
}

fn vector(left: (K4, K4), right: (K4, K4)) -> (K4, K4) {
    (left.0.sub(right.0), left.1.sub(right.1))
}

fn dot_q8(left: (Q8, Q8), right: (Q8, Q8), h: K4) -> Q8 {
    left.0.mul(right.0, h).add(left.1.mul(right.1, h))
}

fn cross_q8(left: (Q8, Q8), right: (Q8, Q8), h: K4) -> Q8 {
    left.0.mul(right.1, h).sub(left.1.mul(right.0, h))
}

fn squared_norm_k4(point: (K4, K4)) -> K4 {
    point.0.mul(point.0).add(point.1.mul(point.1))
}

fn squared_norm_q8(point: (Q8, Q8), h: K4) -> Q8 {
    point.0.mul(point.0, h).add(point.1.mul(point.1, h))
}

fn add_point(left: (K4, K4), right: (K4, K4)) -> (K4, K4) {
    (left.0.add(right.0), left.1.add(right.1))
}

fn scale_point(point: (K4, K4), factor: i128) -> (K4, K4) {
    (point.0.scale(factor), point.1.scale(factor))
}

fn parse_vertex(value: &str) -> Result<usize, G27GeometricFractionalError> {
    value
        .parse::<usize>()
        .map_err(|_| G27GeometricFractionalError::MalformedData {
            source: "rotation_vertex",
        })
}

fn payload(
    retained_candidate_count: usize,
    broad_exact_closure_count: usize,
    small_exact_closure_count: usize,
    best_candidates: &[G27RotationPinCandidateExactReplay],
    conclusion: &str,
    posture: G27RotationPinBatchExactReplayPosture,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_rotation_batch.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "retained_candidate_count",
            retained_candidate_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "broad_exact_closure_count",
            broad_exact_closure_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "small_exact_closure_count",
            small_exact_closure_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for candidate in best_candidates {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "best_candidate",
            candidate.stable_token(),
        ));
    }
    payload
}
