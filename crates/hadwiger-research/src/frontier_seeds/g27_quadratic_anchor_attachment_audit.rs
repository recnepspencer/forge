use crate::mathematical_verification::ExactRational;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_data::retained_g27_coefficients;
use super::g27_outside_moser_anchor::{G27OutsideMoserAnchorCandidate, G27OutsideMoserAxis};
use super::g27_quadratic_anchor_search::search_g27_bounded_quadratic_anchors_checked;
use super::g27_w_circles_exact_geometry_support::{Rat, WExactPoint, K4};

const ROW_685_TARGETS: [usize; 4] = [8, 13, 18, 6];
const INTERNAL_QUADRATIC_RADICANDS: [i128; 3] = [3, 11, 33];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27QuadraticAnchorAttachmentStatus {
    SuppressedInsideRetainedField,
    OutsideFieldNoRequiredAttachments,
    MutationEligibleAllTargets,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27QuadraticAnchorAttachmentAuditRow {
    candidate_id: String,
    radicand: i128,
    status: G27QuadraticAnchorAttachmentStatus,
    unit_targets: Vec<usize>,
}

impl G27QuadraticAnchorAttachmentAuditRow {
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    pub fn radicand(&self) -> i128 {
        self.radicand
    }

    pub fn status(&self) -> G27QuadraticAnchorAttachmentStatus {
        self.status
    }

    pub fn unit_targets(&self) -> &[usize] {
        &self.unit_targets
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27QuadraticAnchorAttachmentAuditReport {
    candidates_audited: usize,
    suppressed_inside_field_count: usize,
    outside_field_count: usize,
    mutation_eligible_count: usize,
    rows: Vec<G27QuadraticAnchorAttachmentAuditRow>,
}

impl G27QuadraticAnchorAttachmentAuditReport {
    pub fn candidates_audited(&self) -> usize {
        self.candidates_audited
    }

    pub fn summary(&self) -> (usize, usize, usize, usize) {
        (
            self.candidates_audited,
            self.suppressed_inside_field_count,
            self.outside_field_count,
            self.mutation_eligible_count,
        )
    }

    pub fn rows(&self) -> &[G27QuadraticAnchorAttachmentAuditRow] {
        &self.rows
    }

    pub fn admits_mutation_candidate(&self) -> bool {
        self.mutation_eligible_count > 0
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

pub fn audit_g27_quadratic_anchor_attachments_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27QuadraticAnchorAttachmentAuditReport, G27GeometricFractionalError> {
    let search = search_g27_bounded_quadratic_anchors_checked(handle)?;
    let g27_points = g27_points_from_coefficients(&retained_g27_coefficients()?)?;
    let rows = search
        .retained_survivors()
        .iter()
        .map(|candidate| audit_candidate(candidate, &g27_points))
        .collect::<Result<Vec<_>, _>>()?;
    let suppressed_inside_field_count = rows
        .iter()
        .filter(|row| {
            row.status == G27QuadraticAnchorAttachmentStatus::SuppressedInsideRetainedField
        })
        .count();
    let outside_field_count = rows.len() - suppressed_inside_field_count;
    let mutation_eligible_count = rows
        .iter()
        .filter(|row| row.status == G27QuadraticAnchorAttachmentStatus::MutationEligibleAllTargets)
        .count();
    Ok(G27QuadraticAnchorAttachmentAuditReport {
        candidates_audited: rows.len(),
        suppressed_inside_field_count,
        outside_field_count,
        mutation_eligible_count,
        rows,
    })
}

fn audit_candidate(
    candidate: &G27OutsideMoserAnchorCandidate,
    g27_points: &[WExactPoint],
) -> Result<G27QuadraticAnchorAttachmentAuditRow, G27GeometricFractionalError> {
    let extension = candidate
        .extension()
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "quadratic_attachment_extension",
        })?;
    if INTERNAL_QUADRATIC_RADICANDS.contains(&extension.radicand()) {
        return Ok(G27QuadraticAnchorAttachmentAuditRow {
            candidate_id: candidate.anchor_id().to_string(),
            radicand: extension.radicand(),
            status: G27QuadraticAnchorAttachmentStatus::SuppressedInsideRetainedField,
            unit_targets: Vec::new(),
        });
    }
    if extension.radicand() != 2 {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "quadratic_attachment_supported_radicand",
        });
    }
    let point = quadratic_anchor_point(candidate)?;
    let unit_targets = ROW_685_TARGETS
        .iter()
        .copied()
        .filter(|target| squared_distance_to_k4_point(point, g27_points[*target - 1]).is_one())
        .collect::<Vec<_>>();
    let status = if unit_targets.len() == ROW_685_TARGETS.len() {
        G27QuadraticAnchorAttachmentStatus::MutationEligibleAllTargets
    } else {
        G27QuadraticAnchorAttachmentStatus::OutsideFieldNoRequiredAttachments
    };
    Ok(G27QuadraticAnchorAttachmentAuditRow {
        candidate_id: candidate.anchor_id().to_string(),
        radicand: extension.radicand(),
        status,
        unit_targets,
    })
}

#[derive(Clone, Copy)]
struct K4Sqrt2 {
    base: K4,
    sqrt2: K4,
}

impl K4Sqrt2 {
    fn from_k4(value: K4) -> Self {
        Self {
            base: value,
            sqrt2: K4::zero(),
        }
    }

    fn sqrt2(value: K4) -> Self {
        Self {
            base: K4::zero(),
            sqrt2: value,
        }
    }

    fn add(self, other: Self) -> Self {
        Self {
            base: self.base.add(other.base),
            sqrt2: self.sqrt2.add(other.sqrt2),
        }
    }

    fn sub(self, other: Self) -> Self {
        Self {
            base: self.base.sub(other.base),
            sqrt2: self.sqrt2.sub(other.sqrt2),
        }
    }

    fn mul(self, other: Self) -> Self {
        let base = self
            .base
            .mul(other.base)
            .add(self.sqrt2.mul(other.sqrt2).scale(2));
        let sqrt2 = self.base.mul(other.sqrt2).add(self.sqrt2.mul(other.base));
        Self { base, sqrt2 }
    }

    fn is_one(self) -> bool {
        self.base == K4::one() && self.sqrt2 == K4::zero()
    }
}

#[derive(Clone, Copy)]
struct QuadraticPoint {
    x: K4Sqrt2,
    y: K4Sqrt2,
}

fn quadratic_anchor_point(
    candidate: &G27OutsideMoserAnchorCandidate,
) -> Result<QuadraticPoint, G27GeometricFractionalError> {
    let base = point_from_coefficients(candidate.moser_coefficients());
    let extension = candidate
        .extension()
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "quadratic_anchor_extension_missing",
        })?;
    let radical = K4Sqrt2::sqrt2(k4_from_rational(extension.coefficient())?);
    let mut point = QuadraticPoint {
        x: K4Sqrt2::from_k4(base.x),
        y: K4Sqrt2::from_k4(base.y),
    };
    match extension.axis() {
        G27OutsideMoserAxis::X => point.x = point.x.add(radical),
        G27OutsideMoserAxis::Y => point.y = point.y.add(radical),
    }
    Ok(point)
}

fn squared_distance_to_k4_point(left: QuadraticPoint, right: WExactPoint) -> K4Sqrt2 {
    let dx = left.x.sub(K4Sqrt2::from_k4(right.x));
    let dy = left.y.sub(K4Sqrt2::from_k4(right.y));
    dx.mul(dx).add(dy.mul(dy))
}

fn g27_points_from_coefficients(
    coefficients: &[[i32; 4]],
) -> Result<Vec<WExactPoint>, G27GeometricFractionalError> {
    if coefficients.len() != 27 {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "g27_point_coefficients",
        });
    }
    Ok(coefficients
        .iter()
        .map(|coefficients| point_from_coefficients(*coefficients))
        .collect())
}

fn point_from_coefficients(coefficients: [i32; 4]) -> WExactPoint {
    coefficients.iter().enumerate().fold(
        WExactPoint {
            x: K4::zero(),
            y: K4::zero(),
        },
        |sum, (index, value)| sum.add(g27_basis_point(index).scale(*value as i128)),
    )
}

fn g27_basis_point(index: usize) -> WExactPoint {
    match index {
        0 => WExactPoint {
            x: K4::rational(1, 1),
            y: K4::zero(),
        },
        1 => WExactPoint {
            x: K4::rational(1, 2),
            y: K4([rat(0, 1), rat(1, 2), rat(0, 1), rat(0, 1)]),
        },
        2 => WExactPoint {
            x: K4::rational(5, 6),
            y: K4([rat(0, 1), rat(0, 1), rat(1, 6), rat(0, 1)]),
        },
        3 => WExactPoint {
            x: K4([rat(5, 12), rat(0, 1), rat(0, 1), rat(-1, 12)]),
            y: K4([rat(0, 1), rat(5, 12), rat(1, 12), rat(0, 1)]),
        },
        _ => WExactPoint {
            x: K4::zero(),
            y: K4::zero(),
        },
    }
}

fn k4_from_rational(value: &ExactRational) -> Result<K4, G27GeometricFractionalError> {
    let token = value.stable_token();
    let (numerator, denominator) =
        token
            .split_once('/')
            .ok_or(G27GeometricFractionalError::MalformedData {
                source: "quadratic_extension_rational",
            })?;
    Ok(K4([
        rat(
            numerator
                .parse()
                .map_err(|_| G27GeometricFractionalError::MalformedData {
                    source: "quadratic_extension_numerator",
                })?,
            denominator
                .parse()
                .map_err(|_| G27GeometricFractionalError::MalformedData {
                    source: "quadratic_extension_denominator",
                })?,
        ),
        rat(0, 1),
        rat(0, 1),
        rat(0, 1),
    ]))
}

fn rat(numerator: i128, denominator: i128) -> Rat {
    Rat::new(numerator, denominator).expect("literal denominator")
}
