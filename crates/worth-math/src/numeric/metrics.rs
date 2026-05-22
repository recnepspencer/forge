use crate::error::{MathError, NumericContractKind};
use crate::linalg;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitVector2([f64; 2]);

impl UnitVector2 {
    pub fn try_new(vector: [f64; 2]) -> Result<Self, MathError> {
        let normalized =
            linalg::normalize_checked_2d(vector).ok_or(MathError::NumericContractViolation {
                kind: NumericContractKind::UnitVector2,
                context: "unit vector requires a finite non-zero 2D direction",
            })?;
        Ok(Self(normalized))
    }

    pub fn as_array(self) -> [f64; 2] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UnitVector3([f64; 3]);

impl UnitVector3 {
    pub fn try_new(vector: [f64; 3]) -> Result<Self, MathError> {
        let normalized =
            linalg::normalize_checked(vector).ok_or(MathError::NumericContractViolation {
                kind: NumericContractKind::UnitVector3,
                context: "unit vector requires a finite non-zero direction",
            })?;
        Ok(Self(normalized))
    }

    pub fn as_array(self) -> [f64; 3] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FiniteNonNegativeF64(f64);

impl FiniteNonNegativeF64 {
    pub fn try_new(value: f64, context: &'static str) -> Result<Self, MathError> {
        if !value.is_finite() || value < 0.0 {
            return Err(MathError::NumericContractViolation {
                kind: NumericContractKind::FiniteNonNegativeScalar,
                context,
            });
        }
        Ok(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }
}

pub fn angle_between_unit_vectors(
    a: UnitVector3,
    b: UnitVector3,
) -> Result<FiniteNonNegativeF64, MathError> {
    let dot = linalg::dot(a.as_array(), b.as_array()).clamp(-1.0, 1.0);
    FiniteNonNegativeF64::try_new(dot.acos(), "angle between unit vectors")
}

pub fn distance_between_points(
    a: [f64; 3],
    b: [f64; 3],
) -> Result<FiniteNonNegativeF64, MathError> {
    FiniteNonNegativeF64::try_new(linalg::distance_sq(a, b).sqrt(), "distance between points")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_vector_2_rejects_zero_and_nan() {
        assert!(UnitVector2::try_new([0.0, 0.0]).is_err());
        assert!(UnitVector2::try_new([f64::NAN, 0.0]).is_err());
    }

    #[test]
    fn unit_vector_rejects_zero_and_nan() {
        assert!(UnitVector3::try_new([0.0, 0.0, 0.0]).is_err());
        assert!(UnitVector3::try_new([f64::NAN, 0.0, 0.0]).is_err());
    }

    #[test]
    fn angle_between_unit_vectors_is_finite() {
        let angle = angle_between_unit_vectors(
            UnitVector3::try_new([1.0, 0.0, 0.0]).expect("unit x"),
            UnitVector3::try_new([0.0, 1.0, 0.0]).expect("unit y"),
        )
        .expect("angle");
        assert!((angle.get() - std::f64::consts::FRAC_PI_2).abs() < 1e-15);
    }

    #[test]
    fn distance_between_points_is_finite() {
        let distance = distance_between_points([0.0, 0.0, 0.0], [3.0, 4.0, 0.0]).expect("distance");
        assert_eq!(distance.get(), 5.0);
    }
}
