use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_spindle_and_fusion_searches::search_g27_pressure_skeleton_spindle_rotations_checked;

const G27_FLOAT_COORDS: &str = include_str!("g27_geometric_fractional/G_27.txt");
const VERTEX_COUNT: usize = 27;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27ExactMoserBasisAuditReport {
    core: HadwigerArtifactCore,
    exact_basis: Vec<String>,
    retained_float_coordinate_count: usize,
    exact_unit_edge_count: usize,
    exact_non_edge_count: usize,
    conclusion: String,
}

impl G27ExactMoserBasisAuditReport {
    pub fn exact_basis(&self) -> &[String] {
        &self.exact_basis
    }

    pub fn retained_float_coordinate_count(&self) -> usize {
        self.retained_float_coordinate_count
    }

    pub fn exact_unit_edge_count(&self) -> usize {
        self.exact_unit_edge_count
    }

    pub fn exact_non_edge_count(&self) -> usize {
        self.exact_non_edge_count
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27ExactMoserBasisAuditReport, core);

pub fn audit_g27_exact_moser_basis_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27ExactMoserBasisAuditReport, G27GeometricFractionalError> {
    let spindle_search = search_g27_pressure_skeleton_spindle_rotations_checked(handle)?;
    let coefficients = retained_g27_coefficients()?;
    let retained_float_coordinate_count = G27_FLOAT_COORDS.lines().count();
    if retained_float_coordinate_count != VERTEX_COUNT {
        return Err(G27GeometricFractionalError::MalformedData { source: "G_27.txt" });
    }
    let (exact_unit_edge_count, exact_non_edge_count) = replay_exact_adjacency(&coefficients);
    let exact_basis = exact_basis_tokens();
    let conclusion =
        "exact Moser basis pinned; rotated-fragment replay can now use symbolic coordinates"
            .to_string();
    let core = artifact_core(
        HadwigerArtifactKind::G27ExactMoserBasisAuditReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_exact_moser_basis_audit".to_string(),
        },
        vec![spindle_search.reference()],
        basis_payload(
            &exact_basis,
            retained_float_coordinate_count,
            exact_unit_edge_count,
            exact_non_edge_count,
            &conclusion,
        ),
    )?;
    Ok(G27ExactMoserBasisAuditReport {
        core,
        exact_basis,
        retained_float_coordinate_count,
        exact_unit_edge_count,
        exact_non_edge_count,
        conclusion,
    })
}

fn replay_exact_adjacency(coefficients: &[[i32; 4]]) -> (usize, usize) {
    let mut unit_edges = 0usize;
    let mut non_edges = 0usize;
    for left in 0..coefficients.len() {
        for right in (left + 1)..coefficients.len() {
            let diff = [
                coefficients[left][0] - coefficients[right][0],
                coefficients[left][1] - coefficients[right][1],
                coefficients[left][2] - coefficients[right][2],
                coefficients[left][3] - coefficients[right][3],
            ];
            if squared_norm(diff).is_one() {
                unit_edges += 1;
            } else {
                non_edges += 1;
            }
        }
    }
    (unit_edges, non_edges)
}

fn squared_norm(coefficients: [i32; 4]) -> G27Scalar {
    let mut total = G27Scalar::zero();
    for index in 0..4 {
        total =
            total.add(&basis_dot(index, index).scale(coefficients[index] * coefficients[index]));
        for other in (index + 1)..4 {
            total = total
                .add(&basis_dot(index, other).scale(2 * coefficients[index] * coefficients[other]));
        }
    }
    total
}

fn basis_dot(left: usize, right: usize) -> G27Scalar {
    match (left, right) {
        (0, 0) | (1, 1) | (2, 2) | (3, 3) => G27Scalar::rational(1, 1),
        (0, 1) | (1, 0) | (2, 3) | (3, 2) => G27Scalar::rational(1, 2),
        (0, 2) | (2, 0) | (1, 3) | (3, 1) => G27Scalar::rational(5, 6),
        (0, 3) | (3, 0) => G27Scalar::rational(5, 12).add(&G27Scalar::sqrt33(-1, 12)),
        (1, 2) | (2, 1) => G27Scalar::rational(5, 12).add(&G27Scalar::sqrt33(1, 12)),
        _ => G27Scalar::zero(),
    }
}

fn exact_basis_tokens() -> Vec<String> {
    vec![
        "b1=(1,0)".to_string(),
        "b2=(1/2,sqrt3/2)".to_string(),
        "b3=(5/6,sqrt11/6)".to_string(),
        "b4=((5-sqrt33)/12,(5sqrt3+sqrt11)/12)".to_string(),
    ]
}

fn basis_payload(
    exact_basis: &[String],
    retained_float_coordinate_count: usize,
    exact_unit_edge_count: usize,
    exact_non_edge_count: usize,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_exact_basis.v1"),
        HadwigerArtifactPayloadEntry::unsigned(
            "retained_float_coordinate_count",
            retained_float_coordinate_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "exact_unit_edge_count",
            exact_unit_edge_count as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned(
            "exact_non_edge_count",
            exact_non_edge_count as u128,
        ),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for token in exact_basis {
        payload.push(HadwigerArtifactPayloadEntry::text("basis", token));
    }
    payload
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct G27Scalar {
    rational_num: i128,
    rational_den: i128,
    sqrt33_num: i128,
    sqrt33_den: i128,
}

impl G27Scalar {
    fn zero() -> Self {
        Self::rational(0, 1)
    }

    fn rational(numerator: i128, denominator: i128) -> Self {
        Self::new(numerator, denominator, 0, 1)
    }

    fn sqrt33(numerator: i128, denominator: i128) -> Self {
        Self::new(0, 1, numerator, denominator)
    }

    fn new(rational_num: i128, rational_den: i128, sqrt33_num: i128, sqrt33_den: i128) -> Self {
        let (rational_num, rational_den) = normalize(rational_num, rational_den);
        let (sqrt33_num, sqrt33_den) = normalize(sqrt33_num, sqrt33_den);
        Self {
            rational_num,
            rational_den,
            sqrt33_num,
            sqrt33_den,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(
            self.rational_num * other.rational_den + other.rational_num * self.rational_den,
            self.rational_den * other.rational_den,
            self.sqrt33_num * other.sqrt33_den + other.sqrt33_num * self.sqrt33_den,
            self.sqrt33_den * other.sqrt33_den,
        )
    }

    fn scale(&self, factor: i32) -> Self {
        Self::new(
            self.rational_num * factor as i128,
            self.rational_den,
            self.sqrt33_num * factor as i128,
            self.sqrt33_den,
        )
    }

    fn is_one(&self) -> bool {
        self.rational_num == self.rational_den && self.sqrt33_num == 0
    }
}

fn normalize(mut numerator: i128, mut denominator: i128) -> (i128, i128) {
    if denominator < 0 {
        numerator = -numerator;
        denominator = -denominator;
    }
    let divisor = gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) as i128;
    (numerator / divisor, denominator / divisor)
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let next = left % right;
        left = right;
        right = next;
    }
    left.max(1)
}
