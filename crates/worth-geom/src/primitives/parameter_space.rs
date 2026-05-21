use serde::{Deserialize, Serialize};
use worth_math::{FinitePoint2, MathError, NumericContractKind};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterSpacePoint {
    uv: [f64; 2],
}

impl ParameterSpacePoint {
    pub fn try_new(uv: [f64; 2]) -> Result<Self, MathError> {
        Ok(Self::from_finite(FinitePoint2::try_new(uv)?))
    }

    pub fn as_array(self) -> [f64; 2] {
        self.uv
    }

    pub fn u(self) -> f64 {
        self.uv[0]
    }

    pub fn v(self) -> f64 {
        self.uv[1]
    }

    pub fn interpolate_linear(self, other: Self, t: f64) -> Result<Self, MathError> {
        if !t.is_finite() {
            return Err(MathError::NumericContractViolation {
                kind: NumericContractKind::FiniteNonNegativeScalar,
                context: "parameter interpolation requires a finite interpolation parameter",
            });
        }
        let [u0, v0] = self.as_array();
        let [u1, v1] = other.as_array();
        Self::try_new([u0 + t * (u1 - u0), v0 + t * (v1 - v0)])
    }

    pub fn offset_polar(self, radius: f64, angle_radians: f64) -> Result<Self, MathError> {
        if !radius.is_finite() || !angle_radians.is_finite() {
            return Err(MathError::NumericContractViolation {
                kind: NumericContractKind::FinitePoint2,
                context: "polar parameter offset requires finite radius and angle",
            });
        }
        let [u, v] = self.as_array();
        Self::try_new([
            u + radius * angle_radians.cos(),
            v + radius * angle_radians.sin(),
        ])
    }

    fn from_finite(point: FinitePoint2) -> Self {
        Self {
            uv: point.as_array(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParameterSpacePoint;

    #[test]
    fn parameter_space_point_rejects_nonfinite_values() {
        assert!(ParameterSpacePoint::try_new([f64::NAN, 0.0]).is_err());
    }

    #[test]
    fn parameter_space_point_exposes_named_components() {
        let point = ParameterSpacePoint::try_new([0.25, 0.75]).expect("parameter space point");
        assert_eq!(point.u(), 0.25);
        assert_eq!(point.v(), 0.75);
    }

    #[test]
    fn parameter_space_point_interpolates_linearly() {
        let a = ParameterSpacePoint::try_new([0.0, 0.0]).unwrap();
        let b = ParameterSpacePoint::try_new([2.0, 4.0]).unwrap();
        let mid = a.interpolate_linear(b, 0.5).unwrap();
        assert_eq!(mid.as_array(), [1.0, 2.0]);
    }
}
