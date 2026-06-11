use worth_math::arithmetic::precision::PrecisionEscalation;
use worth_math::sign::CertifiedTriSign;
use worth_math::{MathError, NumericContractKind};

use super::canonical_order::canonical_cyclic_orient2d_points;
use super::digest::predicate_basis_digest_parts;
use super::{PlanarPredicateInputBasis, PlanarPredicateKind};

#[derive(Clone, Debug)]
pub(crate) struct PlanarPredicateMathEvaluation {
    pub(crate) certified_sign: CertifiedTriSign,
    pub(crate) precision_escalation: PrecisionEscalation,
    pub(crate) basis_digest_parts: Vec<String>,
}

pub(crate) fn evaluate_planar_predicate_authority(
    kind: PlanarPredicateKind,
    basis: &PlanarPredicateInputBasis,
) -> Result<PlanarPredicateMathEvaluation, worth_math::MathError> {
    match kind {
        PlanarPredicateKind::Orient2d => evaluate_orient2d(basis),
    }
}

fn evaluate_orient2d(
    basis: &PlanarPredicateInputBasis,
) -> Result<PlanarPredicateMathEvaluation, worth_math::MathError> {
    let canonical_points = canonical_cyclic_orient2d_points(basis.projected_points());
    for point in canonical_points {
        if point.iter().any(|coordinate| !coordinate.is_finite()) {
            return Err(MathError::NumericContractViolation {
                kind: NumericContractKind::FinitePoint2,
                context: "planar predicate authority orient2d projected points",
            });
        }
    }
    let (certified_sign, precision_escalation) = worth_math::predicates::orient2d(
        canonical_points[0],
        canonical_points[1],
        canonical_points[2],
    )?;
    Ok(PlanarPredicateMathEvaluation {
        certified_sign,
        precision_escalation,
        basis_digest_parts: predicate_basis_digest_parts(
            PlanarPredicateKind::Orient2d,
            basis,
            canonical_points,
        ),
    })
}
