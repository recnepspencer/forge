use crate::error::{MathError, NumericContractKind};
use crate::linalg;

use super::metrics::UnitVector3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FinitePoint2([f64; 2]);

impl FinitePoint2 {
    pub fn try_new(point: [f64; 2]) -> Result<Self, MathError> {
        if point.iter().any(|value| !value.is_finite()) {
            return Err(MathError::NumericContractViolation {
                kind: NumericContractKind::FinitePoint2,
                context: "finite point requires finite 2D coordinates",
            });
        }
        Ok(Self(point))
    }

    pub fn as_array(self) -> [f64; 2] {
        self.0
    }

    pub fn u(self) -> f64 {
        self.0[0]
    }

    pub fn v(self) -> f64 {
        self.0[1]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiniteVector2([f64; 2]);

impl FiniteVector2 {
    pub fn try_new(vector: [f64; 2]) -> Result<Self, MathError> {
        if vector.iter().any(|value| !value.is_finite()) {
            return Err(MathError::NumericContractViolation {
                kind: NumericContractKind::FiniteVector2,
                context: "finite vector requires finite 2D components",
            });
        }
        Ok(Self(vector))
    }

    pub fn as_array(self) -> [f64; 2] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FinitePoint3([f64; 3]);

impl FinitePoint3 {
    pub fn try_new(point: [f64; 3]) -> Result<Self, MathError> {
        if point.iter().any(|value| !value.is_finite()) {
            return Err(MathError::NumericContractViolation {
                kind: NumericContractKind::FinitePoint3,
                context: "finite point requires finite 3D coordinates",
            });
        }
        Ok(Self(point))
    }

    pub fn as_array(self) -> [f64; 3] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiniteVector3([f64; 3]);

impl FiniteVector3 {
    pub fn try_new(vector: [f64; 3]) -> Result<Self, MathError> {
        if vector.iter().any(|value| !value.is_finite()) {
            return Err(MathError::NumericContractViolation {
                kind: NumericContractKind::FiniteVector3,
                context: "finite vector requires finite 3D components",
            });
        }
        Ok(Self(vector))
    }

    pub fn as_array(self) -> [f64; 3] {
        self.0
    }
}

/// Compute the canonical perpendicular unit direction for a given unit vector.
///
/// Policy:
/// - deterministic
/// - axis-priority fallback through the shared linear algebra helper
/// - no domain-specific tolerance or ambiguity policy
///
/// This is the stable math-layer answer to "pick some perpendicular direction"
/// when a caller has already admitted a valid source direction and only needs
/// a canonical fallback basis vector.
pub fn canonical_perpendicular_unit_vector(direction: UnitVector3) -> UnitVector3 {
    let perpendicular = linalg::compute_perpendicular_direction(direction.as_array());
    UnitVector3::try_new(perpendicular)
        .expect("canonical perpendicular generation must preserve unit-direction validity")
}

#[deprecated(
    note = "use canonical_perpendicular_unit_vector for the explicit deterministic policy"
)]
pub fn perpendicular_unit_vector(direction: UnitVector3) -> Result<UnitVector3, MathError> {
    Ok(canonical_perpendicular_unit_vector(direction))
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_perpendicular_unit_vector, FinitePoint2, FinitePoint3, FiniteVector2,
        FiniteVector3,
    };
    use crate::numeric::metrics::UnitVector3;

    #[test]
    fn finite_points_and_vectors_reject_nonfinite_components() {
        assert!(FinitePoint2::try_new([f64::NAN, 0.0]).is_err());
        assert!(FiniteVector2::try_new([f64::NAN, 0.0]).is_err());
        assert!(FinitePoint3::try_new([0.0, f64::INFINITY, 0.0]).is_err());
        assert!(FiniteVector3::try_new([0.0, 0.0, f64::NEG_INFINITY]).is_err());
    }

    #[test]
    fn perpendicular_unit_vector_is_orthogonal_and_unit_length() {
        let direction = UnitVector3::try_new([0.0, 0.0, 1.0]).expect("unit direction");
        let perpendicular = canonical_perpendicular_unit_vector(direction);
        let vector = perpendicular.as_array();

        assert!(vector[2].abs() < 1e-12);
        let length_sq = vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2];
        assert!((length_sq - 1.0).abs() < 1e-12);
    }

    #[test]
    fn canonical_perpendicular_unit_vector_is_deterministic() {
        let direction = UnitVector3::try_new([0.0, 0.0, 1.0]).expect("unit direction");
        let a = canonical_perpendicular_unit_vector(direction).as_array();
        let b = canonical_perpendicular_unit_vector(direction).as_array();
        assert_eq!(a, b);
    }
}
