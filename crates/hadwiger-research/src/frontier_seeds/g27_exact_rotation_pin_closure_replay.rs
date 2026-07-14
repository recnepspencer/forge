use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_exact_rotation_pin_equation::derive_g27_exact_rotation_pin_equation_checked;
use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_manufactured_rotation_field::{K4, Q8};

const HINGE: usize = 8;
const CLOSURE_PAIRS: [(usize, usize); 9] = [
    (1, 27),
    (1, 9),
    (10, 27),
    (3, 12),
    (6, 12),
    (6, 21),
    (6, 25),
    (6, 26),
    (6, 9),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27ExactRotationPinClosureReplayPosture {
    FloatClosureRetired,
}

impl G27ExactRotationPinClosureReplayPosture {
    pub fn as_str(self) -> &'static str {
        "float_closure_retired"
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27ExactRotationBranchReplay {
    branch_sign: i32,
    exact_unit_pairs: Vec<(String, String)>,
    rejected_pairs: Vec<G27ExactRotationClosurePairReplay>,
}

impl G27ExactRotationBranchReplay {
    pub fn branch_sign(&self) -> i32 {
        self.branch_sign
    }

    pub fn exact_unit_pairs(&self) -> &[(String, String)] {
        &self.exact_unit_pairs
    }

    pub fn rejected_pairs(&self) -> &[G27ExactRotationClosurePairReplay] {
        &self.rejected_pairs
    }

    fn stable_token(&self) -> String {
        let unit = self
            .exact_unit_pairs
            .iter()
            .map(|(left, right)| format!("{left}-{right}"))
            .collect::<Vec<_>>()
            .join(",");
        let rejected = self
            .rejected_pairs
            .iter()
            .map(G27ExactRotationClosurePairReplay::stable_token)
            .collect::<Vec<_>>()
            .join(",");
        format!("sign{}:unit[{unit}]:rejected[{rejected}]", self.branch_sign)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27ExactRotationClosurePairReplay {
    moving_vertex: String,
    static_vertex: String,
    exact_squared_distance: String,
}

impl G27ExactRotationClosurePairReplay {
    pub fn moving_vertex(&self) -> &str {
        &self.moving_vertex
    }

    pub fn static_vertex(&self) -> &str {
        &self.static_vertex
    }

    pub fn exact_squared_distance(&self) -> &str {
        &self.exact_squared_distance
    }

    fn stable_token(&self) -> String {
        format!(
            "{}-{}={}",
            self.moving_vertex, self.static_vertex, self.exact_squared_distance
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27ExactRotationPinClosureReplayReport {
    core: HadwigerArtifactCore,
    replayed_float_closure_pair_count: usize,
    exact_unit_pair_count: usize,
    rejected_float_closure_pair_count: usize,
    branches: Vec<G27ExactRotationBranchReplay>,
    conclusion: String,
    posture: G27ExactRotationPinClosureReplayPosture,
}

impl G27ExactRotationPinClosureReplayReport {
    pub fn replayed_float_closure_pair_count(&self) -> usize {
        self.replayed_float_closure_pair_count
    }

    pub fn exact_unit_pair_count(&self) -> usize {
        self.exact_unit_pair_count
    }

    pub fn rejected_float_closure_pair_count(&self) -> usize {
        self.rejected_float_closure_pair_count
    }

    pub fn branches(&self) -> &[G27ExactRotationBranchReplay] {
        &self.branches
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn posture(&self) -> G27ExactRotationPinClosureReplayPosture {
        self.posture
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27ExactRotationPinClosureReplayReport, core);

pub fn replay_g27_exact_rotation_pin_closures_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27ExactRotationPinClosureReplayReport, G27GeometricFractionalError> {
    let equation = derive_g27_exact_rotation_pin_equation_checked(handle)?;
    let coefficients = retained_g27_coefficients()?;
    let points = points(&coefficients);
    let w_squared = K4::rational(7, 8).add(K4::sqrt33(1, 8));
    let branches = [-1, 1]
        .into_iter()
        .map(|sign| replay_branch(sign, &points, w_squared))
        .collect::<Vec<_>>();
    let exact_unit_pair_count = branches
        .iter()
        .map(|branch| branch.exact_unit_pairs.len())
        .sum::<usize>();
    let rejected_float_closure_pair_count = branches
        .iter()
        .map(|branch| branch.rejected_pairs.len())
        .sum::<usize>();
    let conclusion = "exact manufactured-radical replay retires the 103.221 degree float closure: only the intended 10-27 pin is exact on either branch"
        .to_string();
    let posture = G27ExactRotationPinClosureReplayPosture::FloatClosureRetired;
    let core = artifact_core(
        HadwigerArtifactKind::G27ExactRotationPinClosureReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_exact_rotation_pin_closure_replay".to_string(),
        },
        vec![equation.reference()],
        payload(
            &branches,
            exact_unit_pair_count,
            rejected_float_closure_pair_count,
            &conclusion,
        ),
    )?;
    Ok(G27ExactRotationPinClosureReplayReport {
        core,
        replayed_float_closure_pair_count: CLOSURE_PAIRS.len(),
        exact_unit_pair_count,
        rejected_float_closure_pair_count,
        branches,
        conclusion,
        posture,
    })
}

fn replay_branch(sign: i32, points: &[(Q8, Q8)], w_squared: K4) -> G27ExactRotationBranchReplay {
    let (cosine, sine) = rotation_matrix_terms(sign);
    let hinge = points[HINGE - 1];
    let mut exact_unit_pairs = Vec::new();
    let mut rejected_pairs = Vec::new();
    for (moving, static_vertex) in CLOSURE_PAIRS {
        let moving_vector = vector(points[moving - 1], hinge);
        let static_vector = vector(points[static_vertex - 1], hinge);
        let rotated = rotate(moving_vector, cosine, sine, w_squared);
        let distance = squared_norm(vector(rotated, static_vector), w_squared);
        if distance.is_one() {
            exact_unit_pairs.push((moving.to_string(), static_vertex.to_string()));
        } else {
            rejected_pairs.push(G27ExactRotationClosurePairReplay {
                moving_vertex: moving.to_string(),
                static_vertex: static_vertex.to_string(),
                exact_squared_distance: distance.to_token(),
            });
        }
    }
    G27ExactRotationBranchReplay {
        branch_sign: sign,
        exact_unit_pairs,
        rejected_pairs,
    }
}

fn rotation_matrix_terms(sign: i32) -> (Q8, Q8) {
    let cosine = Q8::base(K4::rational(13, 24).add(K4::sqrt33(-1, 24))).add(Q8::w(
        K4::sqrt3(sign as i128, 12).add(K4::sqrt11(sign as i128, 12)),
    ));
    let sine = Q8::base(K4::sqrt3(1, 24).add(K4::sqrt11(5, 24)))
        .add(Q8::w(K4::rational(-(sign as i128), 6)));
    (cosine, sine)
}

fn points(coefficients: &[[i32; 4]]) -> Vec<(Q8, Q8)> {
    coefficients
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .fold((Q8::zero(), Q8::zero()), |point, (index, coefficient)| {
                    add_point(point, scale_point(basis_point(index), *coefficient as i128))
                })
        })
        .collect()
}

fn basis_point(index: usize) -> (Q8, Q8) {
    match index {
        0 => (Q8::one(), Q8::zero()),
        1 => (Q8::base(K4::rational(1, 2)), Q8::base(K4::sqrt3(1, 2))),
        2 => (Q8::base(K4::rational(5, 6)), Q8::base(K4::sqrt11(1, 6))),
        3 => (
            Q8::base(K4::rational(5, 12).add(K4::sqrt33(-1, 12))),
            Q8::base(K4::sqrt3(5, 12).add(K4::sqrt11(1, 12))),
        ),
        _ => (Q8::zero(), Q8::zero()),
    }
}

fn rotate(point: (Q8, Q8), cosine: Q8, sine: Q8, w_squared: K4) -> (Q8, Q8) {
    (
        cosine
            .mul(point.0, w_squared)
            .sub(sine.mul(point.1, w_squared)),
        sine.mul(point.0, w_squared)
            .add(cosine.mul(point.1, w_squared)),
    )
}

fn vector(left: (Q8, Q8), right: (Q8, Q8)) -> (Q8, Q8) {
    (left.0.sub(right.0), left.1.sub(right.1))
}

fn squared_norm(point: (Q8, Q8), w_squared: K4) -> Q8 {
    point
        .0
        .mul(point.0, w_squared)
        .add(point.1.mul(point.1, w_squared))
}

fn add_point(left: (Q8, Q8), right: (Q8, Q8)) -> (Q8, Q8) {
    (left.0.add(right.0), left.1.add(right.1))
}

fn scale_point(point: (Q8, Q8), factor: i128) -> (Q8, Q8) {
    (point.0.scale(factor), point.1.scale(factor))
}

fn payload(
    branches: &[G27ExactRotationBranchReplay],
    exact_unit_pair_count: usize,
    rejected_float_closure_pair_count: usize,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_rotation_closure.v1"),
        HadwigerArtifactPayloadEntry::unsigned("float_pair_count", CLOSURE_PAIRS.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned("branch_count", branches.len() as u128),
        HadwigerArtifactPayloadEntry::unsigned(
            "exact_unit_pair_count",
            exact_unit_pair_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "rejected_float_closure_pair_count",
            rejected_float_closure_pair_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for branch in branches {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "branch",
            branch.stable_token(),
        ));
    }
    payload
}
